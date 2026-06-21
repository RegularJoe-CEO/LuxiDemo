//! Performance benchmark for the optimized CPU engine (GPT-2 width).
//!
//! Run with features for maximum speed:
//!   cargo run --release --features "gpt2 rayon simd" --example perf_bench
//!
//! Reports:
//!   - ms per forward pass
//!   - tokens / second (at batch=1)
//!   - rough energy proxy using the existing EnergyReport model

use std::time::Instant;

use attention_transformer::gpt2::loader::{find_gpt2_snapshot_dir, load_gpt2_tensors};
use attention_transformer::gpt2::{Gpt2Config, Gpt2Model};

fn main() {
    println!("=== attention-transformer CPU Performance Bench (GPT-2 124M width) ===");

    let snapshot = match find_gpt2_snapshot_dir() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Could not find GPT-2 snapshot: {}", e);
            return;
        }
    };

    let config = Gpt2Config::from_snapshot(&snapshot).expect("config");
    let tensors = load_gpt2_tensors(&snapshot).expect("weights");

    // Pre-build the model (this is where we can add buffer pre-allocation later)
    let model = Gpt2Model::from_tensors(tensors, config).expect("model build");

    // Use a realistic prompt length
    let prompt_len = 128usize;
    let input_ids: Vec<u32> = (0..prompt_len).map(|i| (i % 1000) as u32).collect();

    // Warmup
    for _ in 0..3 {
        let _ = model.forward(&input_ids);
    }

    // Timed run
    let iters = 5;
    let start = Instant::now();
    for _ in 0..iters {
        let _ = model.forward(&input_ids);
    }
    let elapsed = start.elapsed();
    let ms_per_pass = elapsed.as_secs_f64() * 1000.0 / iters as f64;
    let tokens_per_sec = (prompt_len as f64) / (ms_per_pass / 1000.0);

    println!("Prompt length      : {} tokens", prompt_len);
    println!("ms / forward pass  : {:.1}", ms_per_pass);
    println!("tokens / second    : {:.1}", tokens_per_sec);

    // Rough energy proxy (reuse the existing model)
    let energy = attention_transformer::EnergyReport::compute(prompt_len, 768, 3072, 12, 0, false);
    println!(
        "Energy proxy (baseline) : {:.2e} J (model only)",
        energy.total_estimated_joules
    );

    println!("\n(With rayon + tiling the numbers above should be dramatically better than the original naive loops.)");
}
