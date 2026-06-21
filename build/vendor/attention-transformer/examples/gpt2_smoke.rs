//! GPT-2 Scale Smoke Test — attention-transformer
//!
//! Purpose: prove the forward pass is STABLE, FINITE, and DETERMINISTIC at
//! GPT-2 "small" dimensions (768 hidden, 12 heads, 12 layers) BEFORE building
//! the real safetensors weight loader + tokenizer.
//!
//! This does NOT load GPT-2 weights. It runs the engine at GPT-2 shape with
//! synthetic input and checks:
//!   1. No NaN / Inf in the output (numerical stability at depth/width)
//!   2. Bounded output magnitude (no blow-up across 12 layers)
//!   3. Bit-exact determinism (same input -> identical SHA-256 receipt, 5 runs)
//!   4. WNSM fidelity at scale (payload does not perturb primary output)
//!
//! Run with: cargo run --release --example gpt2_smoke

use attention_transformer::config::Config;
use attention_transformer::wnsm_transformer::{
    format_receipt, sha256_of_f32_slice, WNSM_GAE_Decoder,
};

fn main() {
    println!("════════════════════════════════════════════════════════════");
    println!("   GPT-2 SCALE SMOKE TEST (768 hidden / 12 heads / 12 layers)");
    println!("════════════════════════════════════════════════════════════\n");

    // GPT-2 "small" geometry
    let hidden = 768;
    let heads = 12;
    let layers = 12;
    let mlp = hidden * 4; // 3072
    let seq = 256; // context window for the smoke test (keep runtime sane)

    println!("Config: hidden={hidden} heads={heads} layers={layers} mlp={mlp} seq={seq}");
    println!("Head dim: {}\n", hidden / heads);

    let cfg = Config::new(hidden, heads, mlp, seq);
    let mut model = WNSM_GAE_Decoder::new(cfg.clone(), layers);

    // Deterministic synthetic input (bounded, GPT-2-like embedding scale)
    let input: Vec<f32> = (0..seq * hidden)
        .map(|i| ((i as f32) * 0.0007).sin() * 0.1)
        .collect();

    // ---- 1. Forward pass + stability checks -------------------------------
    let out = model.forward(input.clone(), seq);

    let mut nan_count = 0usize;
    let mut inf_count = 0usize;
    let mut max_abs = 0.0f32;
    let mut sum = 0.0f64;
    for &v in &out {
        if v.is_nan() {
            nan_count += 1;
        } else if v.is_infinite() {
            inf_count += 1;
        } else {
            max_abs = max_abs.max(v.abs());
            sum += v as f64;
        }
    }
    let mean = sum / (out.len() as f64);

    println!("FORWARD PASS @ GPT-2 SCALE");
    println!("  Output elements : {}", out.len());
    println!("  NaN count       : {nan_count}");
    println!("  Inf count       : {inf_count}");
    println!("  Max |value|     : {max_abs:.4e}");
    println!("  Mean value      : {mean:.4e}");

    let stable = nan_count == 0 && inf_count == 0 && max_abs.is_finite() && max_abs < 1.0e3;
    println!(
        "  Stability       : {}\n",
        if stable {
            "PASS — finite & bounded"
        } else {
            "FAIL"
        }
    );

    // ---- 2. Determinism: 5 runs must yield identical receipts -------------
    let base_receipt = sha256_of_f32_slice(&out);
    let mut deterministic = true;
    for run in 1..5 {
        let r = model.forward(input.clone(), seq);
        let rec = sha256_of_f32_slice(&r);
        if rec != base_receipt {
            deterministic = false;
            println!("  Run {run}: receipt MISMATCH {}", format_receipt(&rec));
        }
    }
    println!("DETERMINISM (5 runs)");
    println!("  Receipt         : {}", format_receipt(&base_receipt));
    println!(
        "  Result          : {}\n",
        if deterministic {
            "PASS — bit-identical across runs"
        } else {
            "FAIL"
        }
    );

    // ---- 3. WNSM fidelity at scale ---------------------------------------
    let (wnsm_out, _payload) = model.forward_wnsm_chained(input.clone(), seq, None);
    let wnsm_receipt = sha256_of_f32_slice(&wnsm_out);
    let max_diff: f32 = out
        .iter()
        .zip(wnsm_out.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0, |m, d| m.max(d));

    println!("WNSM FIDELITY @ GPT-2 SCALE");
    println!("  NORMAL receipt  : {}", format_receipt(&base_receipt));
    println!("  WNSM   receipt  : {}", format_receipt(&wnsm_receipt));
    println!("  Max output diff : {max_diff:.2e}");
    let fidelity = max_diff == 0.0 && wnsm_receipt == base_receipt;
    println!(
        "  Result          : {}\n",
        if fidelity {
            "PASS — payload does not perturb output"
        } else {
            "FAIL"
        }
    );

    // ---- Verdict ----------------------------------------------------------
    let all = stable && deterministic && fidelity;
    println!("════════════════════════════════════════════════════════════");
    println!(
        "  SMOKE TEST: {}",
        if all {
            "✅ ALL PASS — engine holds at GPT-2 scale"
        } else {
            "❌ FAIL"
        }
    );
    println!("════════════════════════════════════════════════════════════");

    if !all {
        std::process::exit(1);
    }
}
