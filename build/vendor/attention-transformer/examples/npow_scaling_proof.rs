//! NPOW scaling proof: O(N) power-law in WNSM payload + 0.00e0 output fidelity.
//!
//!   LUXI_NPOW_FAST=1 cargo run --release --example npow_scaling_proof   # gate smoke (~1s)
//!   cargo run --release --example npow_scaling_proof                    # full sweep incl. 131k timing

use attention_transformer::config::Config;
use attention_transformer::npow::{
    decode_npow_payload, run_scaling_samples, run_scaling_samples_full, NpowPayload,
    NPOW_PAYLOAD_DIM,
};
use attention_transformer::wnsm_transformer::{
    format_receipt, sha256_of_f32_slice, WNSM_GAE_Decoder,
};

fn main() {
    println!("═══════════════════════════════════════════════════════════");
    println!(" NPOW — N-power scaling witness (WNSM payload bus)");
    println!("═══════════════════════════════════════════════════════════\n");

    let head_dim = 64usize;
    let fast = std::env::var("LUXI_NPOW_FAST").ok().as_deref() == Some("1");
    let samples = if fast {
        eprintln!("LUXI_NPOW_FAST=1 — analytical mem only (no 131k Waller timing)");
        run_scaling_samples(head_dim)
    } else {
        eprintln!("Running full scaling sweep (5 seq lengths; 131072 ~30–90s)...");
        run_scaling_samples_full(head_dim)
    };
    let anchor = samples.last().expect("anchor").clone();

    let npow = NpowPayload::from_samples(&samples, &anchor);
    println!("{}", npow.summary_line());
    println!(
        "Power-law proof (mem): Waller slope {:.3} (~1.0 = O(N)), standard {:.3} (~2.0 = O(N²))",
        npow.mem_slope_waller, npow.mem_slope_standard
    );
    println!(
        "Power-law proof (time): Waller slope {:.3} (informational; full-seq waller_operator is O(N²) FLOPs)",
        npow.time_slope_waller
    );

    if !npow.proves_memory_scaling() {
        eprintln!("❌ NPOW memory scaling proof FAILED");
        std::process::exit(1);
    }
    println!("✅ NPOW memory scaling proof PASS (O(N) Waller vs O(N²) standard)\n");

    // WNSM carrier: inject NPOW on row 0, primary output unchanged
    let cfg = Config::new(64, 4, 256, 8);
    let mut decoder = WNSM_GAE_Decoder::new(cfg, 2);
    decoder.install_npow_wnsm(NPOW_PAYLOAD_DIM);

    let seq = 4usize;
    let h = 64usize;
    let input: Vec<f32> = (0..seq * h)
        .map(|i| ((i as f32) * 0.013).sin() * 0.15)
        .collect();

    let normal = decoder.forward(input.clone(), seq);
    let normal_receipt = sha256_of_f32_slice(&normal);

    let mut init_payload = vec![vec![0.0f32; NPOW_PAYLOAD_DIM]; seq];
    init_payload[0] = npow.to_payload_vec(&samples, &anchor);

    let (wnsm_out, extracted) = decoder.forward_wnsm_chained(input, seq, Some(init_payload));
    let wnsm_receipt = sha256_of_f32_slice(&wnsm_out);

    let max_diff: f32 = normal
        .iter()
        .zip(wnsm_out.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f32, f32::max);

    println!("WNSM fidelity (NPOW payload on bus):");
    println!("  max_diff       : {:.2e}", max_diff);
    println!("  output receipt : {}", format_receipt(&normal_receipt));
    println!("  match          : {}", normal_receipt == wnsm_receipt);

    let decoded = decode_npow_payload(&extracted[0]).expect("decode row0");
    println!("  NPOW recovered : slope_waller={:.3}", decoded.mem_slope_waller);
    println!(
        "  payload receipt: {}",
        format_receipt(&decoded.witness_receipt)
    );

    if max_diff != 0.0 || normal_receipt != wnsm_receipt {
        eprintln!("❌ WNSM fidelity FAIL with NPOW payload");
        std::process::exit(1);
    }
    if decoded.witness_receipt != npow.witness_receipt {
        eprintln!("❌ NPOW payload corrupted in WNSM bus (witness receipt mismatch)");
        std::process::exit(1);
    }
    if (decoded.mem_slope_waller - npow.mem_slope_waller).abs() > 1e-3
        || (decoded.mem_slope_standard - npow.mem_slope_standard).abs() > 1e-3
    {
        eprintln!(
            "❌ NPOW slope mismatch after bus: waller {} vs {}, std {} vs {}",
            decoded.mem_slope_waller,
            npow.mem_slope_waller,
            decoded.mem_slope_standard,
            npow.mem_slope_standard
        );
        std::process::exit(1);
    }

    println!("\n✅ NPOW scaling proof complete (O(N) witness + WNSM 0.00e0)");
}