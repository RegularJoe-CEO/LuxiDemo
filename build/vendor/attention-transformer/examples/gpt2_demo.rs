//! GPT-2 124M demo (CPU only, gated behind `gpt2` feature).
//!
//! Usage:
//!   cargo run --release --features gpt2 --example gpt2_demo -- "The meaning of life is"
//!
//! It loads the real GPT-2 small weights from the HF cache, runs greedy
//! generation, and prints coherent text + a logits receipt for reproducibility.
//!
//! Generation strategy:
//!   1. ONE full forward over the prompt (this is the receipt-producing call;
//!      the full `forward` path is bit-locked and unchanged).
//!   2. Seed the recurrent Waller KV state from the prompt.
//!   3. Every subsequent token is produced by `forward_incremental`, which only
//!      processes the single new position against the persisted KV state
//!      (O(1) attention growth instead of full O(N^2) recompute every step).

use std::env;
use std::time::Instant;

use attention_transformer::gpt2::{Gpt2Config, Gpt2Model, Gpt2Tokenizer};

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let prompt = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "The meaning of life is".to_string()
    };

    println!("=== GPT-2 124M Demo (pure Rust, CPU only) ===");
    println!("Prompt: \"{}\"", prompt);

    // 1. Locate snapshot (globs as required)
    let snapshot = match attention_transformer::gpt2::loader::find_gpt2_snapshot_dir() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error finding model: {}", e);
            eprintln!("Make sure the GPT-2 snapshot exists at ~/.cache/huggingface/hub/models--gpt2/snapshots/<hash>");
            std::process::exit(1);
        }
    };
    println!("Using snapshot: {:?}", snapshot);

    // 2. Load config + weights
    let config = Gpt2Config::from_snapshot(&snapshot).expect("config load failed");
    println!(
        "Config: n_embd={}, n_head={}, n_layer={}",
        config.n_embd, config.n_head, config.n_layer
    );

    let tensors = attention_transformer::gpt2::loader::load_gpt2_tensors(&snapshot)
        .expect("weight loading failed");
    let mut model = Gpt2Model::from_tensors(tensors, config).expect("model construction failed");

    // 3. Tokenizer
    let tokenizer = Gpt2Tokenizer::find_and_load(&snapshot)
        .expect("tokenizer load failed (see tokenizer.rs for fallback locations)");

    // 4. Encode prompt
    let mut ids: Vec<u32> = tokenizer.encode(&prompt);
    println!("Encoded prompt length: {}", ids.len());

    let max_new_tokens = 40usize;
    let vocab = model.config.vocab_size;

    // --- Step 1: ONE full forward over the prompt (receipt-producing, bit-locked path) ---
    let prompt_start = Instant::now();
    let (logits, receipt) = model.forward(&ids);
    let prompt_ms = prompt_start.elapsed().as_secs_f64() * 1000.0;

    let hex_receipt: String = receipt.iter().map(|b| format!("{:02x}", b)).collect();
    println!("Prompt forward logits receipt: {}", hex_receipt);

    // First generated token = argmax over the prompt's last position.
    let last_pos = ids.len() - 1;
    let first_tok = argmax(&logits[last_pos * vocab..(last_pos + 1) * vocab]);

    // --- Step 2: seed the recurrent KV state from the prompt ---
    model
        .seed_for_incremental_generation(&ids)
        .expect("seeding failed");

    ids.push(first_tok);

    // --- Step 3: generate the rest incrementally (one new position per step) ---
    let mut incr_total_ms = 0.0f64;
    let mut incr_steps = 0usize;
    let mut next_tok = first_tok;

    for _ in 1..max_new_tokens {
        // Absolute position of the token we just pushed.
        let position = ids.len() - 1;

        let step_start = Instant::now();
        let (inc_logits, _inc_receipt) = model.forward_incremental(next_tok, position);
        let step_ms = step_start.elapsed().as_secs_f64() * 1000.0;
        incr_total_ms += step_ms;
        incr_steps += 1;

        next_tok = argmax(&inc_logits);
        ids.push(next_tok);
    }

    // 5. Decode and print
    let full_text = tokenizer.decode(&ids);
    println!("\n=== Generated continuation ===");
    println!("{}", full_text);

    // 6. Timing summary
    let avg_incr_ms = if incr_steps > 0 {
        incr_total_ms / incr_steps as f64
    } else {
        0.0
    };
    let incr_tps = if incr_total_ms > 0.0 {
        incr_steps as f64 / (incr_total_ms / 1000.0)
    } else {
        0.0
    };
    println!("\n=== Timing (recurrent KV incremental path) ===");
    println!(
        "Prompt full forward   : {:.1} ms ({} tokens)",
        prompt_ms,
        last_pos + 1
    );
    println!("Incremental steps     : {}", incr_steps);
    println!("Avg per new token     : {:.1} ms", avg_incr_ms);
    println!("Incremental throughput: {:.2} tokens/sec", incr_tps);
    println!(
        "\n(Deterministic greedy decoding — same prompt always yields identical output + receipt)"
    );
}
