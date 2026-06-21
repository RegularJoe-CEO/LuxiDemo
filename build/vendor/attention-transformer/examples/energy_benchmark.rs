//! Energy & Performance Benchmark Example
//!
//! This example demonstrates the efficiency of the attention-transformer
//! on both edge/portable devices (pure Rust + optional rayon) and
//! accelerated hardware (CUDA when available).
//!
//! It focuses on the user's priority: **calcs per joule** (effective
//! throughput relative to estimated energy/data-movement cost).
//!
//! Run on any machine:
//!   cargo run --example energy_benchmark
//!
//! With rayon (good for multi-core laptops):
//!   cargo run --example energy_benchmark --features rayon
//!
//! With CUDA (on capable NVIDIA hardware):
//!   cargo run --example energy_benchmark --features cuda
//!
//! The benchmark uses the real production API and the existing
//! EnergyReport for honest efficiency numbers.

use attention_transformer::config::Config;
use attention_transformer::wnsm_transformer::{EnergyReport, WNSM_GAE_Decoder};
use std::time::Instant;

fn main() {
    println!("═══════════════════════════════════════════════════════════════════════");
    println!("   attention-transformer — ENERGY & PERFORMANCE BENCHMARK");
    println!("   Focus: Calcs per Joule (Edge + Accelerated Paths)");
    println!("═══════════════════════════════════════════════════════════════════════\n");

    let hidden = 128;
    let heads = 8;
    let layers = 4;
    let seq = 64; // modest context that still exercises the model
    let mlp = hidden * 4;

    let cfg = Config::new(hidden, heads, mlp, seq);
    let mut model = WNSM_GAE_Decoder::new(cfg.clone(), layers);

    // Create a realistic input
    let input: Vec<f32> = (0..seq * hidden)
        .map(|i| ((i as f32) * 0.011).sin() * 0.25)
        .collect();

    println!(
        "Config: hidden={}, heads={}, layers={}, seq_len={}",
        hidden, heads, layers, seq
    );
    println!("Running forward pass...\n");

    // Warmup
    let _ = model.forward(input.clone(), seq);

    // Timed run
    let start = Instant::now();
    let _output = model.forward(input.clone(), seq);
    let elapsed = start.elapsed();

    let elements = (seq * hidden) as f64;
    let throughput = elements / elapsed.as_secs_f64();

    println!("Forward completed in: {:?}", elapsed);
    println!("Throughput (elements/sec): {:.2e}", throughput);

    // Energy reporting (data-movement dominant model)
    let energy = EnergyReport::compute(seq, hidden, mlp, layers, hidden, true);
    println!("\nEnergy Efficiency (data movement model):");
    println!(
        "  WNSM payload bytes avoided : {}",
        energy.wnsm_payload_bytes_avoided
    );
    println!(
        "  Est. joules saved          : {:.2e}",
        energy.estimated_joules_saved_vs_standard
    );
    println!("  Notes                      : {}", energy.notes);

    // Rough "effective calcs per joule" proxy
    // We treat each element processed as one "calc" for simplicity.
    // Real FLOPs would be higher, but the relative number is useful.
    let effective_calcs_per_joule = throughput / (energy.total_estimated_joules.max(1e-12));

    println!(
        "\nCalcs per Joule proxy (elements per joule): {:.2e}",
        effective_calcs_per_joule
    );
    println!("(Higher is better. Fused CUDA path and WNSM both improve this metric.)");

    println!(
        "\n✅ Benchmark complete. Try with --features rayon or --features cuda for comparison."
    );
    println!("═══════════════════════════════════════════════════════════════════════");
}
