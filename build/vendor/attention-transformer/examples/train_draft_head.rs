//! Offline, deterministic training of the linear draft head (gpt2 feature only).
//!
//! Usage (overnight run):
//!   cargo run --release --features "gpt2 rayon simd" --example train_draft_head
//!
//! Reads `corpus.txt` (a large natural-text file) from the repo root, tokenizes it,
//! and trains the head to predict GPT-2's own greedy next token from each position's
//! final hidden state. CRITICAL: it harvests EVERY position in each forward window,
//! so one forward pass yields ~window training examples instead of one.
//!
//! Produces `draft_head.bin` in the current dir.

use attention_transformer::gpt2::loader::{find_gpt2_snapshot_dir, load_gpt2_tensors};
use attention_transformer::gpt2::{DraftHead, Gpt2Config, Gpt2Model};
use std::time::Instant;

// Tuning knobs for the overnight run.
const WINDOW: usize = 64; // forward-window width
const MAX_TOKENS: usize = 30_000; // cap corpus tokens (≈ minutes of data collection)
const EPOCHS: usize = 8;
const LR: f32 = 0.05;

fn main() {
    let t0 = Instant::now();
    println!("=== Training draft head for GPT-2 speculative decoding (overnight) ===");

    let snapshot = find_gpt2_snapshot_dir().expect("GPT-2 snapshot required for training");
    let config = Gpt2Config::from_snapshot(&snapshot).expect("config");
    let tensors = load_gpt2_tensors(&snapshot).expect("weights");

    let model = Gpt2Model::from_tensors(tensors, config).expect("model");
    let n_embd = model.config.n_embd;
    let vocab = model.config.vocab_size;
    println!("Model ready: n_embd={}, vocab={}", n_embd, vocab);

    let tokenizer = attention_transformer::gpt2::Gpt2Tokenizer::find_and_load(&snapshot)
        .expect("tokenizer required for meaningful head training");

    // Load the real corpus.
    let raw = std::fs::read_to_string("corpus.txt")
        .expect("corpus.txt required in repo root for overnight training");
    println!("Corpus loaded: {} chars", raw.len());

    let mut tokens: Vec<u32> = tokenizer.encode(&raw);
    if tokens.len() > MAX_TOKENS {
        tokens.truncate(MAX_TOKENS);
    }
    println!(
        "Tokenized corpus: {} tokens (window={}, epochs={})",
        tokens.len(),
        WINDOW,
        EPOCHS
    );

    // ---- Data collection: harvest EVERY position in each non-overlapping window ----
    // For position i in a window, the target is GPT-2's greedy argmax at position i,
    // and the feature is that position's final hidden state.
    let mut examples: Vec<(Vec<f32>, u32)> = Vec::new();
    let n_windows = tokens.len() / WINDOW;
    println!("Collecting hidden states from {} windows...", n_windows);

    for w in 0..n_windows {
        let start = w * WINDOW;
        let slice = &tokens[start..start + WINDOW];
        let (logits, hidden, _r) = model.forward_with_hidden(slice);

        // Harvest positions 0..WINDOW-1 (each predicts the next token; last position's
        // target would be outside this window, so we use its own greedy argmax which is
        // still a valid "what the model predicts next" signal).
        for pos in 0..WINDOW {
            let h_start = pos * n_embd;
            let hidden_vec = hidden[h_start..h_start + n_embd].to_vec();
            let l_start = pos * vocab;
            let target = argmax(&logits[l_start..l_start + vocab]);
            examples.push((hidden_vec, target));
        }

        if w % 50 == 0 {
            println!(
                "  window {}/{}  ({} examples so far, {:.0}s elapsed)",
                w,
                n_windows,
                examples.len(),
                t0.elapsed().as_secs_f32()
            );
        }
    }
    println!(
        "Collected {} training examples in {:.0}s",
        examples.len(),
        t0.elapsed().as_secs_f32()
    );

    if examples.is_empty() {
        eprintln!("No examples collected. Aborting.");
        return;
    }

    // ---- Train a linear head with deterministic, fixed-order gradient descent ----
    let mut head = DraftHead::new(n_embd, vocab);

    for epoch in 0..EPOCHS {
        let mut total_loss = 0.0f32;
        let lr = LR * (0.85f32).powi(epoch as i32); // gentle decay

        for (hidden, target) in &examples {
            let logits = head.apply(hidden);

            // Softmax + cross-entropy against GPT-2's greedy token.
            let max_logit = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            let mut exps = vec![0.0f32; vocab];
            let mut sum = 0.0f32;
            for (i, &l) in logits.iter().enumerate() {
                exps[i] = (l - max_logit).exp();
                sum += exps[i];
            }
            let inv = 1.0 / sum;
            let probs: Vec<f32> = exps.iter().map(|&e| e * inv).collect();

            let loss = -probs[*target as usize].max(1e-12).ln();
            total_loss += loss;

            // grad of cross-entropy wrt logits = probs - onehot(target)
            let mut grad_logit = probs;
            grad_logit[*target as usize] -= 1.0;

            // Fixed-order weight update (lossy drafter — exactness not required).
            for v in 0..vocab {
                let g = grad_logit[v] * lr;
                head.bias[v] -= g;
                let row = v * n_embd;
                for d in 0..n_embd {
                    head.weight[row + d] -= g * hidden[d];
                }
            }
        }

        println!(
            "Epoch {}/{}: avg loss {:.4}  (lr {:.4}, {:.0}s elapsed)",
            epoch + 1,
            EPOCHS,
            total_loss / examples.len() as f32,
            lr,
            t0.elapsed().as_secs_f32()
        );

        // Checkpoint after every epoch so an interrupted run still leaves a usable head.
        head.save("draft_head.bin").expect("failed to save head");
    }

    println!("\nTrained draft head saved to draft_head.bin");
    println!("Total wall time: {:.0}s", t0.elapsed().as_secs_f32());
    println!("Next: cargo run --release --features gpt2 --example gpt2_speculative");
}

fn argmax(slice: &[f32]) -> u32 {
    let mut best = 0u32;
    let mut best_v = f32::NEG_INFINITY;
    for (i, &v) in slice.iter().enumerate() {
        if v > best_v {
            best_v = v;
            best = i as u32;
        }
    }
    best
}
