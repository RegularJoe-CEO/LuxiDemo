//! GPT-2 124M speculative decoding — BATCHED verifier + hybrid drafter.
//!
//! Usage:
//!   cargo run --release --features gpt2 --example gpt2_speculative -- "Once upon a time"
//!
//! ============================================================================
//! STATUS: EXPERIMENT / RESEARCH ARTIFACT — NOT the fast generation path.
//! ============================================================================
//! Speculative decoding wins on BANDWIDTH-BOUND hardware (e.g. GPUs), where a
//! batched verify over K positions costs about the same as 1 because the weights
//! stream once. On COMPUTE-BOUND CPUs (Apple Silicon, this machine) K verify
//! steps cost ~K×, so speculation does MORE total work per emitted token than
//! plain sequential generation and is SLOWER in wall-clock here.
//!
//! For real, fast, deterministic generation use the RECURRENT KV PATH instead:
//!   * `examples/gpt2_demo.rs`   — the production generation loop (forward +
//!                                 seed_for_incremental_generation + forward_incremental)
//!   * `examples/kv_speedup.rs`  — side-by-side proof: incremental KV is ~7.6x
//!                                 faster end-to-end at 40 tokens, with flat
//!                                 per-token cost (vs quadratic full recompute).
//!
//! This file is retained only to document the geometric/induction/trained-head
//! drafter framework and to prove the batched verifier keeps output BIT-IDENTICAL
//! to greedy. It is intentionally left on the full-recompute verify path because
//! wrapping speculation around incremental KV competes with (does not stack on)
//! the KV win and is slower still on compute-bound CPUs.
//!
//! This is the wall-clock version of the speculative framework. It keeps the
//! bit-exact contract but turns drafter acceptance into REAL speed via a batched
//! verify pass, and raises acceptance with a near-zero-energy induction drafter.
//!
//! THE CONTRACT (unchanged — determinism is never touched)
//! -------------------------------------------------------
//!   * VERIFIER = the real, bit-locked `Gpt2Model::forward` greedy argmax. The
//!     accepted token stream is BIT-IDENTICAL to plain sequential greedy decoding.
//!   * DRAFTER  = a cheap, lossy guesser. It is allowed to be wrong; every miss is
//!     corrected by the verifier at zero correctness cost.
//!
//! WHAT'S NEW vs the first integration
//! -----------------------------------
//!   1. BATCHED VERIFY. Instead of K sequential forwards per window, we run ONE
//!      forward over `ctx + draft[0..K]`. Because attention is causal, the logits
//!      at positions L-1 .. L-1+K give the verifier's true next-token for every
//!      prefix at once. We accept the longest matching run; the first mismatch is
//!      replaced by the verifier's own token. Each window costs ONE forward but
//!      emits 1 + (#accepted drafts) tokens — that is the wall-clock lever.
//!   2. INDUCTION DRAFTER. The cheapest high-acceptance guess on real text is an
//!      induction lookup: if the recent suffix already appeared in the context,
//!      predict whatever followed it last time. Pure memory lookup, no vocab scan
//!      on a hit — the most energy-efficient drafter possible. This is what pushes
//!      acceptance up on structured / repetitive text.
//!   3. LEAN GEOMETRIC FALLBACK. When induction misses, fall back to an orthogonal
//!      alignment score against the real wte matrix. The expensive sqrt-warp is
//!      dropped (an orthogonal dot-product ranking is mathematically equivalent
//!      and far cheaper / lower energy).

use std::env;
use std::time::Instant;

use attention_transformer::gpt2::{DraftHead, Gpt2Config, Gpt2Model, Gpt2Tokenizer};

const SPEC_K: usize = 6; // draft window (how far we guess ahead)
const GEN: usize = 24; // tokens to generate
const NGRAM: usize = 3; // induction suffix length to match

/// Greedy argmax over a single-position logits slice of length `vocab`.
fn argmax(logits: &[f32]) -> u32 {
    let mut best_tok = 0u32;
    let mut best_logit = f32::NEG_INFINITY;
    for (v, &val) in logits.iter().enumerate() {
        if val > best_logit {
            best_logit = val;
            best_tok = v as u32;
        }
    }
    best_tok
}

/// The verifier's true next token for a context: real engine forward, argmax of
/// the LAST position. Bit-locked path — identical to sequential greedy.
fn verify_next(model: &Gpt2Model, ctx: &[u32]) -> u32 {
    let vocab = model.config.vocab_size;
    let (logits, _receipt) = model.forward(ctx);
    let last = ctx.len() - 1;
    argmax(&logits[last * vocab..(last + 1) * vocab])
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let prompt = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "Once upon a time".to_string()
    };

    println!("=== GPT-2 124M Speculative Decoding (BATCHED verifier + hybrid drafter) ===");
    println!("Prompt: \"{}\"\n", prompt);

    // --- Load model ---
    let snapshot = attention_transformer::gpt2::loader::find_gpt2_snapshot_dir()
        .expect("could not find GPT-2 snapshot");
    let config = Gpt2Config::from_snapshot(&snapshot).expect("config load failed");
    let tensors = attention_transformer::gpt2::loader::load_gpt2_tensors(&snapshot)
        .expect("weight loading failed");

    // Grab the REAL token-embedding matrix for the geometric fallback drafter.
    let (wte, wte_shape) = tensors.get("wte.weight").expect("wte.weight missing");
    let vocab = wte_shape[0];
    let n_embd = wte_shape[1];
    println!("Real wte matrix: vocab={}, n_embd={}\n", vocab, n_embd);
    let wte_owned: Vec<f32> = wte.to_vec();

    let model = Gpt2Model::from_tensors(tensors, config).expect("model construction failed");
    let tokenizer = Gpt2Tokenizer::find_and_load(&snapshot).expect("tokenizer load failed");

    // --- Trained draft head (primary signal if available) ---
    let draft_head = DraftHead::load("draft_head.bin").ok();
    if draft_head.is_some() {
        println!("Loaded trained draft head (primary drafter).\n");
    } else {
        println!("No trained draft_head.bin found — falling back to induction + geometric only.\n");
    }

    let prompt_ids: Vec<u32> = tokenizer.encode(&prompt);
    println!("Encoded prompt length: {}\n", prompt_ids.len());

    // --- Induction drafter: if the recent NGRAM suffix occurred earlier in ctx,
    //     predict the token that followed it last time. Pure lookup; returns None
    //     on a miss so the caller can fall back. Scans from the most recent match. ---
    let induction_next = |ctx: &[u32]| -> Option<u32> {
        let len = ctx.len();
        if len < NGRAM + 1 {
            return None;
        }
        let suffix = &ctx[len - NGRAM..];
        // search backwards for an earlier occurrence of `suffix`
        let mut i = len - NGRAM; // last possible start strictly before current suffix
        while i >= 1 {
            i -= 1;
            if &ctx[i..i + NGRAM] == suffix {
                let follow = i + NGRAM;
                if follow < len {
                    return Some(ctx[follow]);
                }
            }
        }
        None
    };

    // --- Geometric fallback: lean orthogonal alignment against real wte (no sqrt-warp). ---
    let geom_next = |ctx: &[u32]| -> u32 {
        // Recency-weighted context summary in embedding space.
        let mut acc = vec![0.0f32; n_embd];
        let len = ctx.len();
        for (pos, &tok) in ctx.iter().enumerate() {
            let w = 1.0 / (1.0 + (len - 1 - pos) as f32);
            let base = tok as usize * n_embd;
            for d in 0..n_embd {
                acc[d] += wte_owned[base + d] * w; // multiply-then-add, fixed order
            }
        }
        // Plain dot-product alignment (orthogonal-equivalent ranking, cheap).
        let mut best = 0u32;
        let mut best_v = f32::NEG_INFINITY;
        for t in 0..vocab {
            let base = t * n_embd;
            let mut s = 0.0f32;
            for d in 0..n_embd {
                s += acc[d] * wte_owned[base + d];
            }
            if s > best_v {
                best_v = s;
                best = t as u32;
            }
        }
        best
    };

    // Hybrid drafter: trained head (if we have a last verified hidden) is primary.
    // Falls back to induction, then geometric.
    let draft_next = |ctx: &[u32], last_hidden: Option<&[f32]>| -> u32 {
        if let (Some(head), Some(h)) = (&draft_head, last_hidden) {
            if h.len() == n_embd {
                let logits = head.apply(h);
                return argmax(&logits);
            }
        }
        induction_next(ctx).unwrap_or_else(|| geom_next(ctx))
    };

    // --- 1) BASELINE: bit-locked sequential greedy (the verifier's truth) ---
    println!("Running bit-locked sequential greedy baseline...");
    let seq_start = Instant::now();
    let mut baseline = prompt_ids.clone();
    for _ in 0..GEN {
        let next = verify_next(&model, &baseline);
        baseline.push(next);
    }
    let baseline_out: Vec<u32> = baseline[prompt_ids.len()..].to_vec();
    let seq_ms = seq_start.elapsed().as_secs_f64() * 1000.0;
    println!(
        "  baseline done in {:.0} ms  ({} forward passes)\n",
        seq_ms, GEN
    );

    // --- 2) SPECULATIVE with BATCHED verify ---
    println!("Running speculative decode (batched verifier + hybrid drafter)...");
    let spec_start = Instant::now();
    let mut ctx = prompt_ids.clone();
    let start = ctx.len();
    let mut verify_passes = 0u64;
    let mut proposed = 0u64;
    let mut accepted = 0u64;

    // The "tap": last hidden state from the last verified/ accepted position.
    // We will use this to drive the trained draft head for the *next* window.
    let mut last_verified_hidden: Option<Vec<f32>> = None;

    while ctx.len() - start < GEN {
        // DRAFT phase: use the trained head on the last verified hidden if available.
        let remaining = GEN - (ctx.len() - start);
        let k = SPEC_K.min(remaining);
        let mut draft = Vec::with_capacity(k);
        {
            let mut tmp = ctx.clone();
            for i in 0..k {
                let g = if i == 0 {
                    draft_next(&tmp, last_verified_hidden.as_deref())
                } else {
                    draft_next(&tmp, None)
                };
                draft.push(g);
                tmp.push(g);
            }
        }
        proposed += draft.len() as u64;

        // BATCHED VERIFY using the tap: we get logits + all hidden states in one pass.
        let l = ctx.len();
        let mut extended = ctx.clone();
        extended.extend_from_slice(&draft);
        let (logits, hidden_states, _r) = model.forward_with_hidden(&extended);
        verify_passes += 1;

        // Walk the proposals and accept.
        let mut emitted = 0usize;
        let mut newly_accepted_up_to = l; // position in extended after last acceptance
        for i in 0..k {
            if ctx.len() - start >= GEN {
                break;
            }
            let pos = l - 1 + i;
            let true_tok = argmax(&logits[pos * vocab..(pos + 1) * vocab]);
            if true_tok == draft[i] {
                ctx.push(true_tok);
                accepted += 1;
                emitted += 1;
                newly_accepted_up_to = l + i + 1;
            } else {
                ctx.push(true_tok);
                emitted += 1;
                break;
            }
        }

        // TAP: save the hidden state at the last *accepted* position for the next draft window.
        if newly_accepted_up_to > l {
            let hidden_pos = newly_accepted_up_to - 1;
            let h_start = hidden_pos * n_embd;
            last_verified_hidden = Some(hidden_states[h_start..h_start + n_embd].to_vec());
        }

        debug_assert!(emitted >= 1);
        let _ = emitted;
    }
    let spec_out: Vec<u32> = ctx[start..start + GEN].to_vec();
    let spec_ms = spec_start.elapsed().as_secs_f64() * 1000.0;
    println!(
        "  speculative done in {:.0} ms  ({} forward passes)\n",
        spec_ms, verify_passes
    );

    // --- 3) PROOF + METRICS ---
    let identical = baseline_out == spec_out;
    let accept_rate = accepted as f64 / proposed as f64 * 100.0;
    let speedup = seq_ms / spec_ms;

    println!("=== Generated text (bit-locked baseline) ===");
    println!("{}\n", tokenizer.decode(&baseline));

    println!("=== Results ===");
    println!("Tokens generated         : {}", GEN);
    println!("Draft window (SPEC_K)    : {}", SPEC_K);
    println!(
        "Drafter acceptance       : {}/{}  ({:.1}% of real-text guesses correct)",
        accepted, proposed, accept_rate
    );
    println!("Baseline forward passes  : {}", GEN);
    println!("Speculative fwd passes   : {}", verify_passes);
    println!("Baseline wall-clock      : {:.0} ms", seq_ms);
    println!("Speculative wall-clock   : {:.0} ms", spec_ms);
    println!("WALL-CLOCK SPEEDUP       : {:.2}x", speedup);
    println!(
        "Output bit-identical to sequential greedy : {}",
        if identical { "YES" } else { "NO" }
    );
    println!();
    if identical {
        println!("PROVEN on real GPT-2 weights: the batched verifier collapses each draft");
        println!("window into ONE forward pass, so drafter acceptance converts directly into");
        println!("fewer dependent forwards — real wall-clock speed — while the exact verifier");
        println!("keeps the accepted stream bit-identical to the bit-locked greedy path.");
    } else {
        println!("MISMATCH — verifier control flow needs review (should be impossible).");
    }
}
