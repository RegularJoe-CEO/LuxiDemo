//! Hard equivalence gate for the recurrent-KV incremental generation path.
//!
//! Proves that generating tokens via `forward_incremental` (O(1) attention growth
//! against the persisted Waller KV state) yields BIT-IDENTICAL results to the
//! full-recompute `forward` path that re-runs attention over the whole sequence
//! every step.
//!
//! Without this gate, a fast incremental path is worthless: speed only matters if
//! the output is provably the same as the locked reference. This test asserts:
//!   1. The greedy token sequence is identical between the two paths.
//!   2. The per-step argmax logit value is bit-identical (exact f32 equality).
//!
//! Gated behind the `gpt2` feature because it loads the real 124M GPT-2 weights
//! from the local HF cache. Skips gracefully (passes) if the snapshot is absent,
//! so CI without the model still goes green.

#![cfg(feature = "gpt2")]

use attention_transformer::gpt2::{loader, Gpt2Config, Gpt2Model};

fn argmax(logits: &[f32]) -> (u32, f32) {
    let mut best_tok = 0u32;
    let mut best_val = f32::NEG_INFINITY;
    for (v, &val) in logits.iter().enumerate() {
        if val > best_val {
            best_val = val;
            best_tok = v as u32;
        }
    }
    (best_tok, best_val)
}

#[test]
fn incremental_generation_matches_full_recompute_bit_for_bit() {
    // Locate the model; skip cleanly if it's not present on this machine.
    let snapshot = match loader::find_gpt2_snapshot_dir() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("[skip] GPT-2 snapshot not found; equivalence gate skipped.");
            return;
        }
    };

    let config = Gpt2Config::from_snapshot(&snapshot).expect("config load failed");
    let tensors = loader::load_gpt2_tensors(&snapshot).expect("weight loading failed");

    // A short fixed prompt of valid token ids (GPT-2 vocab = 50257).
    // "The" "Ġquick" style ids are not needed; any small valid ids exercise the path.
    let prompt: Vec<u32> = vec![464, 2068, 7586, 21831]; // arbitrary valid ids
    let n_new = 12usize;

    // --- Reference path: full recompute every step (the locked baseline behavior) ---
    let ref_model = Gpt2Model::from_tensors(tensors.clone(), config.clone())
        .expect("model construction failed");
    let mut ref_ids = prompt.clone();
    let mut ref_tokens: Vec<u32> = Vec::with_capacity(n_new);
    let mut ref_vals: Vec<f32> = Vec::with_capacity(n_new);
    let vocab = ref_model.config.vocab_size;
    for _ in 0..n_new {
        let (logits, _r) = ref_model.forward(&ref_ids);
        let last = ref_ids.len() - 1;
        let (tok, val) = argmax(&logits[last * vocab..(last + 1) * vocab]);
        ref_tokens.push(tok);
        ref_vals.push(val);
        ref_ids.push(tok);
    }

    // --- Incremental path: full forward on the prompt, then forward_incremental ---
    let mut inc_model =
        Gpt2Model::from_tensors(tensors, config).expect("model construction failed");
    let mut inc_ids = prompt.clone();
    let mut inc_tokens: Vec<u32> = Vec::with_capacity(n_new);
    let mut inc_vals: Vec<f32> = Vec::with_capacity(n_new);

    // Prompt forward (matches reference first step) + seed.
    let (logits0, _r0) = inc_model.forward(&inc_ids);
    let last0 = inc_ids.len() - 1;
    let (tok0, val0) = argmax(&logits0[last0 * vocab..(last0 + 1) * vocab]);
    inc_tokens.push(tok0);
    inc_vals.push(val0);
    inc_model
        .seed_for_incremental_generation(&inc_ids)
        .expect("seeding failed");
    inc_ids.push(tok0);

    // Remaining tokens via the incremental path.
    let mut next = tok0;
    for _ in 1..n_new {
        let position = inc_ids.len() - 1;
        let (inc_logits, _r) = inc_model.forward_incremental(next, position);
        let (tok, val) = argmax(&inc_logits);
        inc_tokens.push(tok);
        inc_vals.push(val);
        next = tok;
        inc_ids.push(tok);
    }

    // --- Hard assertions ---
    assert_eq!(
        ref_tokens, inc_tokens,
        "incremental token sequence diverged from full recompute:\n  full = {:?}\n  incr = {:?}",
        ref_tokens, inc_tokens
    );

    for (i, (&r, &c)) in ref_vals.iter().zip(inc_vals.iter()).enumerate() {
        assert_eq!(
            r.to_bits(),
            c.to_bits(),
            "argmax logit bits diverged at generated step {} (full={}, incr={})",
            i,
            r,
            c
        );
    }
}
