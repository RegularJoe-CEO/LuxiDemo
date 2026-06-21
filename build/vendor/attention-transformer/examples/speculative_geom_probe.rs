//! Speculative decoding probe — GEOMETRIC (Hadamard) drafter + EXACT verifier.
//!
//! THE FRAMEWORK
//! -------------
//! Autoregressive decoding is normally blind & sequential: finish token N,
//! look at it, then start N+1. Speculative decoding makes it "see-through":
//!
//!   1. A CHEAP DRAFTER proposes the next K tokens ahead of time (allowed to be
//!      wrong — this is where we put the lossy geometry/sqrt tricks for free).
//!   2. The EXACT VERIFIER (the real engine) does ONE wide forward pass over all
//!      K proposed positions at once and checks each against what it would have
//!      produced sequentially.
//!   3. Every proposed token that matches is ACCEPTED; the first mismatch is
//!      CORRECTED with the verifier's own token. Then we repeat.
//!
//! KEY GUARANTEE: the accepted stream is BIT-IDENTICAL to plain sequential
//! greedy decoding. Bit-exactness lives in the VERIFIER, not the drafter, so
//! the drafter is free to be as approximate (Hadamard-projected, sqrt-scaled)
//! as we like. Speed comes from fewer sequential trips, not from approximation.
//!
//! This probe uses a small synthetic "ground-truth model" so it is fully
//! reproducible with no external weights. The point is to prove the CONTROL
//! FLOW is exact and to measure the acceptance rate of a geometric drafter.

use attention_transformer::linalg::hadamard_transform;

const VOCAB: usize = 64; // toy vocab
const DIM: usize = 32; // hidden width (power of two for Hadamard)
const GEN: usize = 64; // tokens to generate
const SPEC_K: usize = 4; // draft window (how far we guess ahead)

// ---------------------------------------------------------------------------
// Ground-truth "model": deterministic next-token logits from a context.
// This stands in for your real Waller/GPT-2 forward pass. It is the SAME for
// both the sequential baseline and the speculative verifier, so any agreement
// between them is a real correctness result, not a coincidence.
// ---------------------------------------------------------------------------
fn embed(token: usize) -> Vec<f32> {
    // Fixed pseudo-embedding (deterministic, no RNG).
    let mut v = vec![0f32; DIM];
    for (d, slot) in v.iter_mut().enumerate() {
        let x = ((token * 131 + d * 17 + 7) % 251) as f32 / 251.0 - 0.5;
        *slot = x;
    }
    v
}

/// The exact, deterministic logits the real engine would produce given a
/// context. Fixed left-to-right accumulation — this is the bit-locked path.
fn true_logits(context: &[usize]) -> Vec<f32> {
    // Summarize context with a fixed, order-preserving fold (recency-weighted).
    let mut acc = vec![0f32; DIM];
    for (pos, &tok) in context.iter().enumerate() {
        let e = embed(tok);
        let w = 1.0 / (1.0 + (context.len() - 1 - pos) as f32);
        for d in 0..DIM {
            acc[d] += e[d] * w; // multiply-then-add, fixed order
        }
    }
    // Project to vocab via fixed deterministic weights.
    let mut logits = vec![0f32; VOCAB];
    for (t, slot) in logits.iter_mut().enumerate() {
        let we = embed(t);
        let mut s = 0f32;
        for d in 0..DIM {
            s += acc[d] * we[d];
        }
        *slot = s;
    }
    logits
}

fn argmax(logits: &[f32]) -> usize {
    let mut best = 0usize;
    let mut best_v = logits[0];
    for (i, &v) in logits.iter().enumerate().skip(1) {
        if v > best_v {
            best_v = v;
            best = i;
        }
    }
    best
}

// ---------------------------------------------------------------------------
// SEQUENTIAL BASELINE — the bit-locked reference. One token at a time, blind.
// ---------------------------------------------------------------------------
fn decode_sequential(prompt: &[usize], n: usize) -> (Vec<usize>, u64) {
    let mut ctx = prompt.to_vec();
    let mut passes = 0u64;
    for _ in 0..n {
        let logits = true_logits(&ctx); // one forward pass
        passes += 1;
        let next = argmax(&logits);
        ctx.push(next);
    }
    (ctx[prompt.len()..].to_vec(), passes)
}

// ---------------------------------------------------------------------------
// GEOMETRIC DRAFTER — the cheap, lossy guesser. Allowed to be wrong.
//
// Trick: instead of the full vocab projection, we rotate the context summary
// into the orthogonal (Hadamard) basis and score candidate tokens by a
// sqrt-scaled match in that rotated space. This is far cheaper than the exact
// projection and tends to agree with it on "easy" tokens. Misses cost nothing
// because the verifier corrects them.
// ---------------------------------------------------------------------------
fn draft_next(context: &[usize]) -> usize {
    let mut acc = vec![0f32; DIM];
    for (pos, &tok) in context.iter().enumerate() {
        let e = embed(tok);
        let w = 1.0 / (1.0 + (context.len() - 1 - pos) as f32);
        for d in 0..DIM {
            acc[d] += e[d] * w;
        }
    }
    // Rotate the context summary into the orthogonal basis (add-only transform).
    let acc_h = hadamard_transform(&acc);

    // Score each candidate token in the ROTATED space with a sqrt-scaled metric.
    // (Cheap approximation of the exact dot-product projection.)
    let mut best = 0usize;
    let mut best_v = f32::NEG_INFINITY;
    for t in 0..VOCAB {
        let we_h = hadamard_transform(&embed(t));
        let mut s = 0f32;
        for d in 0..DIM {
            // sqrt-magnitude alignment: sign-aware, compressed dynamic range.
            let p = acc_h[d] * we_h[d];
            s += p.signum() * p.abs().sqrt();
        }
        if s > best_v {
            best_v = s;
            best = t;
        }
    }
    best
}

// ---------------------------------------------------------------------------
// SPECULATIVE DECODE — drafter proposes SPEC_K ahead, verifier checks in one
// "wide" pass (here modeled as SPEC_K+1 exact logit evals over the proposed
// prefixes), accepting the matching run and correcting the first miss.
// ---------------------------------------------------------------------------
fn decode_speculative(prompt: &[usize], n: usize) -> (Vec<usize>, u64, u64, u64) {
    let mut ctx = prompt.to_vec();
    let start = ctx.len();
    let mut verify_passes = 0u64;
    let mut proposed = 0u64;
    let mut accepted = 0u64;

    while ctx.len() - start < n {
        // 1) DRAFT: cheaply guess the next SPEC_K tokens.
        let mut draft = Vec::with_capacity(SPEC_K);
        {
            let mut tmp = ctx.clone();
            for _ in 0..SPEC_K {
                let g = draft_next(&tmp);
                draft.push(g);
                tmp.push(g);
            }
        }
        proposed += draft.len() as u64;

        // 2) VERIFY: the exact engine checks each proposed position against
        //    what it would itself produce. One wide pass = SPEC_K+1 exact evals
        //    over growing prefixes (in a real batched engine these collapse
        //    into a single parallel forward — the cost model below counts it
        //    as ONE verify pass for the whole window).
        let mut i = 0;
        while i < draft.len() && ctx.len() - start < n {
            let exact = argmax(&true_logits(&ctx)); // verifier's true token
            if exact == draft[i] {
                ctx.push(exact); // accepted — bit-identical to sequential
                accepted += 1;
                i += 1;
            } else {
                ctx.push(exact); // first miss -> take verifier's token, restart
                break;
            }
        }
        verify_passes += 1; // one wide verify pass per window
    }

    (
        ctx[start..start + n].to_vec(),
        verify_passes,
        proposed,
        accepted,
    )
}

fn main() {
    let prompt = vec![1usize, 5, 9, 13];

    println!("SPECULATIVE DECODING — geometric drafter + exact verifier\n");

    let (seq_out, seq_passes) = decode_sequential(&prompt, GEN);
    let (spec_out, verify_passes, proposed, accepted) = decode_speculative(&prompt, GEN);

    // CORRECTNESS: speculative output must equal sequential output, token-for-token.
    let identical = seq_out == spec_out;

    println!("Tokens generated      : {GEN}");
    println!("Draft window (SPEC_K) : {SPEC_K}");
    println!();
    println!("Sequential passes     : {seq_passes}  (one per token, blind)");
    println!("Speculative verify    : {verify_passes}  (wide passes, see-through)");
    let accept_rate = accepted as f64 / proposed as f64 * 100.0;
    println!(
        "Drafter acceptance    : {accepted}/{proposed}  ({accept_rate:.1}% of guesses correct)"
    );
    let speedup = seq_passes as f64 / verify_passes as f64;
    println!("Sequential-trip ratio : {speedup:.2}x fewer dependent passes");
    println!();
    println!(
        "Output bit-identical to sequential greedy : {}",
        if identical { "YES  ✅" } else { "NO  ❌" }
    );
    println!();
    if identical {
        println!("PROVEN: the geometric drafter is lossy and fast, but the exact");
        println!("verifier guarantees the accepted stream is identical to the");
        println!("bit-locked sequential path. Determinism intact; speed from");
        println!("fewer dependent trips. Receipt would be unchanged.");
    } else {
        println!("Mismatch — verifier control flow needs review.");
    }
}
