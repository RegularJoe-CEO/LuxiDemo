//! **Full transformer layer** bench — one `WNSM_GAE_Layer::forward_cuda` (not attention-only).
//!
//! For the ~3–4 ms Waller kernel number (Q,K,V already on GPU), use `cuda_bench`:
//!   cargo run --release --features cuda --example cuda_bench -- 50 1024 1024 16
//!
//!   cargo run --release --features cuda --example cuda_layer_bench -- [ITERS] [SEQ] [HIDDEN] [HEADS] [MLP]
//!
//! Locked H100 @ seq=1024: median ~6.8 ms full layer. See `docs/QUANT_TRADE_LOCKED.md`.
//! Receipt gate: `cuda_verify` with `LUXI_RECEIPT_AUDIT=1`. See `tests/cuda_lanes.md`.

#![allow(unexpected_cfgs)]

#[cfg(cuda_compilation_failed)]
fn main() {
    eprintln!("cuda_layer_bench requires nvcc-built kernels.");
    std::process::exit(1);
}

#[cfg(not(cuda_compilation_failed))]
fn main() {
    use std::time::Instant;

    use attention_transformer::config::Config;
    use attention_transformer::wnsm_transformer::WNSM_GAE_Layer;

    fn arg<T: std::str::FromStr>(idx: usize, default: T) -> T {
        std::env::args()
            .nth(idx)
            .and_then(|s| s.parse().ok())
            .unwrap_or(default)
    }

    let iters: usize = arg(1, 20);
    let seq: usize = arg(2, 128);
    let hidden: usize = arg(3, 1024);
    let heads: usize = arg(4, 16);
    let mlp: usize = arg(5, 256);

    assert!(hidden % heads == 0);
    let cfg = Config::new(hidden, heads, mlp, seq);

    let mut layer = WNSM_GAE_Layer::new(cfg.clone());
    for w in [&mut layer.wq, &mut layer.wk, &mut layer.wv, &mut layer.wo] {
        for x in w.iter_mut() {
            *x = (*x * 0.7 + 0.01).sin();
        }
    }

    let input: Vec<f32> = (0..seq * hidden)
        .map(|i| ((i as f32) * 0.003).sin() * 0.05)
        .collect();

    let geodesic = !std::env::var("LUXI_CUDA_CPU_QKV")
        .map(|v| v == "1")
        .unwrap_or(false)
        && !std::env::var("LUXI_RECEIPT_AUDIT")
            .map(|v| v == "1")
            .unwrap_or(false);

    // This example always records GPU phases (extra sync — fine for profiling only).
    if geodesic && std::env::var("LUXI_CUDA_PHASE_TIMING").is_err() {
        // SAFETY: single-threaded main before any threads; bench-only.
        unsafe { std::env::set_var("LUXI_CUDA_PHASE_TIMING", "1") };
    }

    println!("============================================================");
    println!(" FULL LAYER bench — one WNSM_GAE_Layer::forward_cuda");
    println!(" (NOT the ~4 ms cuda_bench number — that is Waller-only, QKV on GPU)");
    println!(" iters={} seq={} hidden={} heads={} mlp={}", iters, seq, hidden, heads, mlp);
    if geodesic {
        println!(" TRADE = GEODESIC (GPU LN1 + packed QKV + parallel waller+wo + GPU MLP + H2D/D2H)");
    } else {
        println!(" Lane = CPU QKV and/or AUDIT (see env)");
    }
    println!(" LUXI_CUDA_ROW_FUSED=1 → broken-slow path (~135 ms @ seq=1024). Do not use for TRADE.");
    println!(" Compare attention-only: cuda_bench -- 50 {} {} {}", seq, hidden, heads);
    println!("============================================================");

    let warmup = layer.forward_cuda(&input, seq).expect("warmup");
    let warmup_sum: f32 = warmup.iter().sum();
    println!(" warmup checksum (must be non-zero): {:.6}", warmup_sum);

    let mut total_ms = Vec::with_capacity(iters);
    let mut qkv_ms = Vec::with_capacity(iters);
    let mut gpu_ms = Vec::with_capacity(iters);
    let mut post_ms = Vec::with_capacity(iters);
    let mut phase_h2d = Vec::new();
    let mut phase_ln1 = Vec::new();
    let mut phase_qkv = Vec::new();
    let mut phase_waller = Vec::new();
    let mut phase_res_ln2 = Vec::new();
    let mut phase_mlp = Vec::new();
    let mut phase_d2h = Vec::new();

    for _ in 0..iters {
        let t0 = Instant::now();
        let out = layer.forward_cuda(&input, seq).expect("forward_cuda");
        #[cfg(all(feature = "cuda", not(cuda_compilation_failed)))]
        {
            attention_transformer::gpu::cuda::cuda_device_sync();
            if geodesic {
                if let Some(p) = attention_transformer::gpu::cuda::take_geodesic_phase_ms() {
                    phase_h2d.push(p.h2d);
                    phase_ln1.push(p.ln1);
                    phase_qkv.push(p.qkv);
                    phase_waller.push(p.waller_wo);
                    phase_res_ln2.push(p.res_ln2);
                    phase_mlp.push(p.mlp);
                    phase_d2h.push(p.d2h);
                }
            }
        }
        std::hint::black_box(out.iter().sum::<f32>());
        total_ms.push(t0.elapsed().as_secs_f64() * 1000.0);
    }

    if !geodesic {
        for _ in 0..iters {
            let t0 = Instant::now();
            let (q, k, v) = layer.build_qkv_cpu(&input, seq);
            qkv_ms.push(t0.elapsed().as_secs_f64() * 1000.0);

            let t1 = Instant::now();
            let attn = layer.cuda_attn_with_qkv(seq, &q, &k, &v).expect("cuda attn");
            gpu_ms.push(t1.elapsed().as_secs_f64() * 1000.0);

            let t2 = Instant::now();
            if std::env::var("LUXI_CUDA_CPU_POST")
                .map(|v| v == "1")
                .unwrap_or(false)
            {
                let _ = layer.cuda_post_proj(&input, &attn, seq);
            }
            post_ms.push(t2.elapsed().as_secs_f64() * 1000.0);
        }
    }

    fn report(label: &str, samples: &[f64]) {
        let mut s = samples.to_vec();
        s.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!(
            " {:20} median {:>8.3} ms  mean {:>8.3} ms",
            label,
            s[s.len() / 2],
            s.iter().sum::<f64>() / s.len() as f64
        );
    }

    report("TOTAL (full layer)", &total_ms);
    if geodesic && !phase_h2d.is_empty() {
        report("  H2D input", &phase_h2d);
        report("  LN1", &phase_ln1);
        report("  QKV GEMM", &phase_qkv);
        report("  Waller+wo (attn)", &phase_waller);
        report("  res+LN2", &phase_res_ln2);
        report("  MLP (dominant)", &phase_mlp);
        report("  D2H output", &phase_d2h);
        let attn_stack: Vec<f64> = phase_ln1
            .iter()
            .zip(phase_qkv.iter())
            .zip(phase_waller.iter())
            .map(|((a, b), c)| a + b + c)
            .collect();
        let post_stack: Vec<f64> = phase_res_ln2
            .iter()
            .zip(phase_mlp.iter())
            .map(|(a, b)| a + b)
            .collect();
        report("  → pre-MLP stack", &attn_stack);
        report("  → MLP+LN2 stack", &post_stack);
        println!(
            " cuda_bench (Waller-only, device QKV) is typically ~3–4 ms @ seq=1024 — not this TOTAL."
        );
    } else if geodesic {
        println!("  (enable geodesic; phases missing — rebuild with cuda feature)");
    } else {
        report("  CPU QKV", &qkv_ms);
        report("  GPU attn+wo", &gpu_ms);
        if std::env::var("LUXI_CUDA_CPU_POST")
            .map(|v| v == "1")
            .unwrap_or(false)
        {
            report("  CPU post", &post_ms);
        } else {
            println!("  (MLP on GPU — included in TOTAL)");
        }
    }
    println!("============================================================");
}