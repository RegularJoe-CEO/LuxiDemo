//! FP8 shadow path gate: 0.00e0 max diff and identical SHA-256 vs f32 gold.
//!
//!   cargo run --release --example fp8_parity_gate

use attention_transformer::fp8::{build_fp8_shadow, forward_fp8_shadow_f32, max_abs_diff};
use attention_transformer::wnsm_transformer::{format_receipt, sha256_of_f32_slice};

fn main() {
    println!("═══════════════════════════════════════════════════════════");
    println!(" FP8 deterministic shadow path — parity gate");
    println!("═══════════════════════════════════════════════════════════\n");

    // Weights chosen on E4M3-exact grid (powers of 1/8)
    let w: Vec<f32> = (0..64)
        .map(|i| match i % 4 {
            0 => 0.0f32,
            1 => 1.0,
            2 => 2.0,
            _ => 4.0,
        })
        .collect();
    let shadow = build_fp8_shadow(&w, 8, 8).expect("all weights E4M3-exact");
    let a: Vec<f32> = (0..32).map(|i| (i as f32) * 0.01 - 0.1).collect();

    let f32_direct = matmul_f32(&a, &w, 4, 8, 8);
    let fp8_path = forward_fp8_shadow_f32(&a, &shadow, 4, 8, 8);

    let diff = max_abs_diff(&f32_direct, &fp8_path);
    let r_f32 = sha256_of_f32_slice(&f32_direct);
    let r_fp8 = sha256_of_f32_slice(&fp8_path);

    println!("max_abs_diff     : {:.2e}", diff);
    println!("f32 receipt      : {}", format_receipt(&r_f32));
    println!("FP8 shadow receipt: {}", format_receipt(&r_fp8));
    println!("receipts match   : {}", r_f32 == r_fp8);

    if diff == 0.0 && r_f32 == r_fp8 {
        println!("\n✅ FP8 shadow path PASS (0.00e0 + identical SHA-256)");
    } else {
        eprintln!("\n❌ FP8 shadow path FAIL");
        std::process::exit(1);
    }
}

fn matmul_f32(a: &[f32], w: &[f32], m: usize, k: usize, n: usize) -> Vec<f32> {
    let mut out = vec![0.0f32; m * n];
    for row in 0..m {
        for col in 0..n {
            let mut acc = 0.0f32;
            for p in 0..k {
                acc += a[row * k + p] * w[p * n + col];
            }
            out[row * n + col] = acc;
        }
    }
    out
}