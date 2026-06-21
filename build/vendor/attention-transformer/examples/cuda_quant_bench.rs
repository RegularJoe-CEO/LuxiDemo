//! Quant TRADE: multi-layer `forward_cuda_trade` (one H2D, GPU stack, one D2H).
//!
//! Locked H100 @ seq=1024, 12 layers: median ~69 ms (~5.8 ms/layer amortized).
//! See `docs/QUANT_TRADE_LOCKED.md`. Gate: `bash scripts/runpod_quant_gate.sh`.
//!
//!   cargo run --release --features cuda --example cuda_quant_bench -- [ITERS] [SEQ] [LAYERS]

#![allow(unexpected_cfgs)]

#[cfg(cuda_compilation_failed)]
fn main() {
    eprintln!("cuda_quant_bench requires nvcc-built kernels.");
    std::process::exit(1);
}

#[cfg(not(cuda_compilation_failed))]
fn main() {
    use std::time::Instant;

    use attention_transformer::config::Config;
    use attention_transformer::wnsm_transformer::WNSM_GAE_Decoder;

    fn arg<T: std::str::FromStr>(idx: usize, default: T) -> T {
        std::env::args()
            .nth(idx)
            .and_then(|s| s.parse().ok())
            .unwrap_or(default)
    }

    fn report(label: &str, samples: &[f64]) {
        let mut s = samples.to_vec();
        s.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!(
            " {:24} median {:>8.3} ms  mean {:>8.3} ms",
            label,
            s[s.len() / 2],
            s.iter().sum::<f64>() / s.len() as f64
        );
    }

    let iters: usize = arg(1, 20);
    let seq: usize = arg(2, 1024);
    let num_layers: usize = arg(3, 12);
    let hidden: usize = 1024;
    let heads: usize = 16;
    let mlp: usize = 256;

    let cfg = Config::new(hidden, heads, mlp, seq);
    let mut decoder = WNSM_GAE_Decoder::new(cfg, num_layers);
    for layer in &mut decoder.layers {
        for w in [&mut layer.wq, &mut layer.wk, &mut layer.wv, &mut layer.wo] {
            for x in w.iter_mut() {
                *x = (*x * 0.7 + 0.01).sin();
            }
        }
    }

    let input: Vec<f32> = (0..seq * hidden)
        .map(|i| ((i as f32) * 0.003).sin() * 0.05)
        .collect();

    println!("============================================================");
    println!(" QUANT TRADE bench (batched MLP, parallel waller+wo, GPU stack)");
    println!(" iters={} seq={} layers={} hidden={}", iters, seq, num_layers, hidden);
    println!(" ~7 ms/layer @ seq=1024; cuda_bench ~4ms = Waller ONLY");
    println!("============================================================");

    // 1-layer first (before 12-layer stack allocates all layer buffers).
    let _ = decoder.layers[0]
        .forward_cuda(&input, seq)
        .expect("warmup 1 layer");
    let mut layer_ms = Vec::with_capacity(iters);
    for _ in 0..iters {
        let t0 = Instant::now();
        let out = decoder.layers[0]
            .forward_cuda(&input, seq)
            .expect("single layer");
        attention_transformer::gpu::cuda::cuda_device_sync();
        std::hint::black_box(out.iter().sum::<f32>());
        layer_ms.push(t0.elapsed().as_secs_f64() * 1000.0);
    }

    let _ = decoder
        .forward_cuda_trade(&input, seq)
        .expect("warmup quant stack");
    let mut stack_ms = Vec::with_capacity(iters);
    for _ in 0..iters {
        let t0 = Instant::now();
        let out = decoder
            .forward_cuda_trade(&input, seq)
            .expect("quant stack");
        attention_transformer::gpu::cuda::cuda_device_sync();
        std::hint::black_box(out.iter().sum::<f32>());
        stack_ms.push(t0.elapsed().as_secs_f64() * 1000.0);
    }

    report("1 layer (forward_cuda)", &layer_ms);
    let stack_label = format!("{num_layers} layers (quant stack)");
    report(&stack_label, &stack_ms);
    if num_layers > 1 {
        let per_layer = stack_ms.iter().sum::<f64>() / stack_ms.len() as f64 / num_layers as f64;
        println!("   ~{per_layer:.3} ms amortized per layer (stack / {num_layers})");
        let ratio = stack_ms[stack_ms.len() / 2] / layer_ms[layer_ms.len() / 2];
        if ratio < (num_layers as f64) * 0.5 {
            println!("   WARN: stack much faster than N×1-layer — likely a bug if ratio << {num_layers}");
        }
    }
    println!("============================================================");
}