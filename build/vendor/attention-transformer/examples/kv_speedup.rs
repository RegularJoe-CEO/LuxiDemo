//! Measure recurrent-KV incremental generation vs full-recompute generation.
//!
//! Prints per-token cost for both strategies so the speedup is grounded in
//! measured numbers rather than asserted. Gated behind `gpt2`.
//!
//!   cargo run --release --features gpt2 --example kv_speedup -- "Once upon a time"

use std::env;
use std::time::Instant;

use attention_transformer::gpt2::{loader, Gpt2Config, Gpt2Model};

fn argmax(logits: &[f32]) -> u32 {
    let mut best = 0u32;
    let mut bv = f32::NEG_INFINITY;
    for (v, &val) in logits.iter().enumerate() {
        if val > bv {
            bv = val;
            best = v as u32;
        }
    }
    best
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let prompt = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "Once upon a time".into()
    };

    let snapshot = loader::find_gpt2_snapshot_dir().expect("snapshot not found");
    let config = Gpt2Config::from_snapshot(&snapshot).expect("config");
    let tensors = loader::load_gpt2_tensors(&snapshot).expect("weights");

    // Use the real tokenizer for a realistic prompt length.
    let tokenizer =
        attention_transformer::gpt2::Gpt2Tokenizer::find_and_load(&snapshot).expect("tokenizer");
    let prompt_ids: Vec<u32> = tokenizer.encode(&prompt);
    let n_new = 40usize;
    let vocab = config.vocab_size;

    println!("=== KV speedup benchmark (GPT-2 124M, CPU) ===");
    println!(
        "Prompt tokens: {}, generating {} tokens\n",
        prompt_ids.len(),
        n_new
    );

    // --- Strategy A: full recompute every step (the OLD behavior) ---
    {
        let model = Gpt2Model::from_tensors(tensors.clone(), config.clone()).expect("model");
        let mut ids = prompt_ids.clone();
        let mut total = 0.0f64;
        let mut first_ms = 0.0f64;
        let mut last_ms = 0.0f64;
        for step in 0..n_new {
            let t = Instant::now();
            let (logits, _r) = model.forward(&ids);
            let ms = t.elapsed().as_secs_f64() * 1000.0;
            total += ms;
            if step == 0 {
                first_ms = ms;
            }
            last_ms = ms;
            let last = ids.len() - 1;
            ids.push(argmax(&logits[last * vocab..(last + 1) * vocab]));
        }
        println!("[A] FULL RECOMPUTE every step:");
        println!(
            "    first token (seq {:>3}): {:.1} ms",
            prompt_ids.len(),
            first_ms
        );
        println!(
            "    last  token (seq {:>3}): {:.1} ms",
            prompt_ids.len() + n_new - 1,
            last_ms
        );
        println!(
            "    total {} tokens       : {:.1} ms ({:.2} tok/s)\n",
            n_new,
            total,
            n_new as f64 / (total / 1000.0)
        );
    }

    // --- Strategy B: recurrent KV incremental (the NEW behavior) ---
    {
        let mut model = Gpt2Model::from_tensors(tensors, config).expect("model");
        let mut ids = prompt_ids.clone();
        let (logits, _r) = model.forward(&ids);
        let last = ids.len() - 1;
        let mut next = argmax(&logits[last * vocab..(last + 1) * vocab]);
        model.seed_for_incremental_generation(&ids).expect("seed");
        ids.push(next);

        let mut total = 0.0f64;
        let mut first_ms = 0.0f64;
        let mut last_ms = 0.0f64;
        for step in 0..(n_new - 1) {
            let position = ids.len() - 1;
            let t = Instant::now();
            let (inc_logits, _r) = model.forward_incremental(next, position);
            let ms = t.elapsed().as_secs_f64() * 1000.0;
            total += ms;
            if step == 0 {
                first_ms = ms;
            }
            last_ms = ms;
            next = argmax(&inc_logits);
            ids.push(next);
        }
        println!("[B] RECURRENT KV incremental:");
        println!(
            "    first token (seq {:>3}): {:.1} ms",
            prompt_ids.len() + 1,
            first_ms
        );
        println!(
            "    last  token (seq {:>3}): {:.1} ms",
            prompt_ids.len() + n_new - 1,
            last_ms
        );
        println!(
            "    total {} tokens       : {:.1} ms ({:.2} tok/s)",
            n_new - 1,
            total,
            (n_new - 1) as f64 / (total / 1000.0)
        );
    }
}
