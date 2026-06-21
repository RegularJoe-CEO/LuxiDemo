//! turbo_bench — measures the int8 `forward_turbo` path vs the f32 `forward` path
//! at a fixed 128-token width, and prints the turbo receipt across 3 runs to prove
//! the int8 path is deterministic (stable receipt) even though it is NOT bit-identical
//! to the f32 path.
//!
//! Run: cargo run --release --features gpt2 --example turbo_bench

use attention_transformer::gpt2::model::{Gpt2Config, Gpt2Model};
use std::time::Instant;

fn hex(receipt: &[u8; 32]) -> String {
    receipt.iter().map(|b| format!("{:02x}", b)).collect()
}

fn main() {
    let snapshot =
        attention_transformer::gpt2::loader::find_gpt2_snapshot_dir().expect("snapshot not found");
    let config = Gpt2Config::from_snapshot(&snapshot).expect("config load failed");
    let tensors = attention_transformer::gpt2::loader::load_gpt2_tensors(&snapshot)
        .expect("weight loading failed");
    let model = Gpt2Model::from_tensors(tensors, config).expect("model construction failed");

    // Fixed synthetic 128-token input (deterministic, no tokenizer needed).
    let n = 128usize;
    let ids: Vec<u32> = (0..n).map(|i| (i * 7 + 13) as u32 % 50257).collect();

    println!("=== Turbo Benchmark (128 tokens, GPT-2 124M) ===\n");

    // --- f32 default path ---
    let t0 = Instant::now();
    let (_f32_logits, f32_receipt) = model.forward(&ids);
    let f32_ms = t0.elapsed().as_secs_f64() * 1000.0;
    println!(
        "f32 forward       : {:>8.1} ms   receipt {}",
        f32_ms,
        hex(&f32_receipt)
    );

    // --- int8 turbo path, 3 runs to prove receipt stability ---
    println!("\nint8 forward_turbo (3 runs for determinism check):");
    let mut turbo_receipts = Vec::new();
    let mut turbo_times = Vec::new();
    for run in 1..=3 {
        let t = Instant::now();
        let (_logits, receipt) = model.forward_turbo(&ids);
        let ms = t.elapsed().as_secs_f64() * 1000.0;
        turbo_times.push(ms);
        turbo_receipts.push(hex(&receipt));
        println!("  run {} : {:>8.1} ms   receipt {}", run, ms, hex(&receipt));
    }

    let all_same = turbo_receipts.iter().all(|r| r == &turbo_receipts[0]);
    let avg_turbo: f64 = turbo_times.iter().sum::<f64>() / turbo_times.len() as f64;

    println!("\n--- Summary ---");
    println!("f32   : {:>8.1} ms", f32_ms);
    println!("turbo : {:>8.1} ms (avg of 3)", avg_turbo);
    println!("speedup (f32 / turbo): {:.2}x", f32_ms / avg_turbo);
    println!(
        "turbo receipt stable across runs: {}",
        if all_same {
            "YES (deterministic)"
        } else {
            "NO (non-deterministic!)"
        }
    );
    println!(
        "turbo receipt == f32 receipt    : {} (expected NO — int8 rounds low bits)",
        if turbo_receipts[0] == hex(&f32_receipt) {
            "YES"
        } else {
            "NO"
        }
    );
}
