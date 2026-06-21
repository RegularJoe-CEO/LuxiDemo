// cuda_bench.rs — Sustained throughput benchmark for the CUDA Waller operator.
//
// Usage:
//   cargo run --release --features cuda --example cuda_bench -- [ITERS] [SEQ] [HIDDEN] [HEADS]

#![allow(unexpected_cfgs)]

#[cfg(cuda_compilation_failed)]
fn main() {
    eprintln!("cuda_bench requires nvcc-built kernels (CUDA Toolkit + GPU).");
    eprintln!("Build on RunPod/H100 with nvcc in PATH, then re-run.");
    std::process::exit(1);
}

#[cfg(not(cuda_compilation_failed))]
fn main() {
    use std::process::Command;
    use std::time::Instant;

    use attention_transformer::gpu::cuda::{
        waller_operator_cuda_blocking, waller_operator_cuda_persistent, CudaWallerBuffers,
        CudaWallerTimings,
    };

    fn arg<T: std::str::FromStr>(idx: usize, default: T) -> T {
        std::env::args()
            .nth(idx)
            .and_then(|s| s.parse().ok())
            .unwrap_or(default)
    }

    fn median_power_watts(samples: usize) -> Option<f64> {
        let out = Command::new("nvidia-smi")
            .args([
                "--query-gpu=power.draw",
                "--format=csv,noheader,nounits",
            ])
            .output()
            .ok()?;
        if !out.status.success() {
            return None;
        }
        let line = std::str::from_utf8(&out.stdout).ok()?.lines().next()?.trim();
        let first: f64 = line.split_whitespace().next()?.parse().ok()?;
        if samples <= 1 {
            return Some(first);
        }
        let mut vals = vec![first];
        for _ in 1..samples {
            std::thread::sleep(std::time::Duration::from_millis(50));
            let o2 = Command::new("nvidia-smi")
                .args([
                    "--query-gpu=power.draw",
                    "--format=csv,noheader,nounits",
                ])
                .output()
                .ok()?;
            if o2.status.success() {
                if let Some(l) = std::str::from_utf8(&o2.stdout).ok()?.lines().next() {
                    if let Ok(v) = l.trim().split_whitespace().next()?.parse::<f64>() {
                        vals.push(v);
                    }
                }
            }
        }
        vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Some(vals[vals.len() / 2])
    }

    fn run_timed_loop<F>(iters: usize, mut f: F) -> Vec<f64>
    where
        F: FnMut() -> (),
    {
        let mut samples = Vec::with_capacity(iters);
        for _ in 0..iters {
            let t0 = Instant::now();
            f();
            samples.push(t0.elapsed().as_secs_f64() * 1000.0);
        }
        samples
    }

    fn print_stats(label: &str, samples: &[f64], flops_per_iter: f64) {
        let mut s = samples.to_vec();
        s.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let n = s.len();
        let sum: f64 = s.iter().sum();
        let mean = sum / n as f64;
        let min = s[0];
        let max = s[n - 1];
        let median = s[n / 2];
        let p95_idx = (((n as f64) * 0.95) as usize).min(n - 1);
        let p95 = s[p95_idx];
        let gflops = (flops_per_iter / (mean / 1000.0)) / 1e9;

        println!("------------------------------------------------------------");
        println!(" {label} ({} timed iters):", n);
        println!("   min    : {:>8.3} ms", min);
        println!("   median : {:>8.3} ms", median);
        println!("   mean   : {:>8.3} ms", mean);
        println!("   p95    : {:>8.3} ms", p95);
        println!("   max    : {:>8.3} ms", max);
        println!("   throughput : {:.2} iters/sec", n as f64 / sum * 1000.0);
        println!("   approx attention compute : {:.2} GFLOP/s (per mean iter)", gflops);
    }

    let iters: usize = arg(1, 50);
    let seq: usize = arg(2, 512);
    let hidden: usize = arg(3, 512);
    let heads: usize = arg(4, 8);

    assert!(hidden % heads == 0, "hidden must be divisible by heads");
    let head_dim = hidden / heads;
    let scale = 1.0f32 / (head_dim as f32).sqrt();
    let total = seq * hidden;

    let q: Vec<f32> = (0..total).map(|i| ((i as f32) * 0.013).sin() * 0.4).collect();
    let k: Vec<f32> = (0..total).map(|i| ((i as f32) * 0.017).cos() * 0.4).collect();
    let v: Vec<f32> = (0..total).map(|i| ((i as f32) * 0.019).sin() * 0.4).collect();

    let flops_per_iter = 4.0 * (seq as f64) * (seq as f64) * (head_dim as f64) * (heads as f64);

    println!("============================================================");
    println!(" CUDA Waller Operator — Sustained Throughput Benchmark");
    println!("============================================================");
    println!(
        " iters={}  seq={}  hidden={}  heads={}  head_dim={}  scale={:.6}",
        iters, seq, hidden, heads, head_dim, scale
    );
    println!(" elements per tensor: {}  ({} bytes f32)", total, total * 4);
    println!("------------------------------------------------------------");

    print!(" Warmup (persistent)... ");
    let mut buffers = CudaWallerBuffers::new();
    match waller_operator_cuda_persistent(
        &q, &k, &v, &mut buffers, seq, head_dim, heads, scale, None,
    ) {
        Ok(out) => println!("ok (out len {})", out.len()),
        Err(e) => {
            eprintln!("\nCUDA path unavailable: {e}");
            std::process::exit(1);
        }
    }

    let mut checksum = 0.0f64;
    let mut phase = CudaWallerTimings::default();
    let mut phase_samples = Vec::with_capacity(iters);

    let persistent_samples = run_timed_loop(iters, || {
        let out = waller_operator_cuda_persistent(
            &q,
            &k,
            &v,
            &mut buffers,
            seq,
            head_dim,
            heads,
            scale,
            Some(&mut phase),
        )
        .expect("CUDA kernel failed mid-run");
        phase_samples.push(phase);
        checksum += out[0] as f64 + out[out.len() - 1] as f64;
    });

    print_stats("PERSISTENT (H2D+kernel+D2H each iter)", &persistent_samples, flops_per_iter);

    if !phase_samples.is_empty() {
        let n = phase_samples.len() as f64;
        let h2d: f64 = phase_samples.iter().map(|t| t.h2d_ms).sum::<f64>() / n;
        let kern: f64 = phase_samples.iter().map(|t| t.kernel_ms).sum::<f64>() / n;
        let d2h: f64 = phase_samples.iter().map(|t| t.d2h_ms).sum::<f64>() / n;
        println!("   phase mean : H2D {:>7.3} ms | kernel {:>7.3} ms | D2H {:>7.3} ms", h2d, kern, d2h);
        println!("   phase sum  : {:.3} ms (should ~= mean iter latency)", h2d + kern + d2h);
    }

    // Device-resident: upload QKV once, loop kernel+D2H only (true sustained inference ceiling).
    print!("\n Uploading QKV to device (once)... ");
    let mut resident = CudaWallerBuffers::new();
    unsafe {
        resident
            .upload_inputs(&q, &k, &v, total)
            .expect("upload_inputs failed");
    }
    println!("ok");
    let mut resident_samples = Vec::with_capacity(iters);
    for _ in 0..iters {
        let t0 = std::time::Instant::now();
        let out = unsafe {
            let (_, _, o) = resident
                .launch_only(total, seq, head_dim, heads, scale, true)
                .expect("launch_only failed");
            o
        };
        resident_samples.push(t0.elapsed().as_secs_f64() * 1000.0);
        checksum += out[0] as f64 + out[out.len() - 1] as f64;
    }
    print_stats("DEVICE-RESIDENT (kernel+D2H only, QKV on GPU)", &resident_samples, flops_per_iter);

    // Split-path fused Waller+wo (production attn GPU); QKV already on device.
    print!("\n SPLIT-FUSED waller+wo (pinned D2H)... ");
    let mut fused_samples = Vec::with_capacity(iters);
    if let Ok(weights) = {
        use attention_transformer::gpu::cuda::CudaLayerWeights;
        use attention_transformer::wnsm_transformer::WNSM_GAE_Layer;
        let cfg = attention_transformer::config::Config::new(hidden, heads, 256, seq);
        let layer = WNSM_GAE_Layer::new(cfg);
        unsafe { CudaLayerWeights::upload_from_layer(&layer) }
    } {
        for _ in 0..iters {
            let t0 = Instant::now();
            let out = unsafe {
                resident
                    .launch_waller_then_wo(
                        total,
                        seq,
                        hidden,
                        head_dim,
                        heads,
                        scale,
                        weights.d_wo as *const f32,
                    )
                    .expect("launch_waller_then_wo")
            };
            fused_samples.push(t0.elapsed().as_secs_f64() * 1000.0);
            checksum += out[0] as f64;
        }
        print_stats("SPLIT-FUSED (GPU waller+wo, pinned D2H)", &fused_samples, flops_per_iter);
    } else {
        println!("skip (weights upload failed)");
    }

    print!("\n Reference one-shot blocking ({} iters)... ", iters.min(20));
    let blocking_iters = iters.min(20);
    let blocking_samples = run_timed_loop(blocking_iters, || {
        let out = waller_operator_cuda_blocking(&q, &k, &v, seq, head_dim, heads, scale)
            .expect("blocking CUDA failed");
        checksum += out[0] as f64;
    });
    println!("done");
    print_stats("BLOCKING (malloc per call)", &blocking_samples, flops_per_iter);

    if let Some(watts) = median_power_watts(5) {
        let mean_ms = persistent_samples.iter().sum::<f64>() / persistent_samples.len() as f64;
        let gflops = (flops_per_iter / (mean_ms / 1000.0)) / 1e9;
        println!("------------------------------------------------------------");
        println!(" Power (nvidia-smi median): {:.1} W", watts);
        println!(" Useful GFLOP/s per W (approx): {:.4}", gflops / watts);
    } else {
        println!("------------------------------------------------------------");
        println!(" Power: nvidia-smi unavailable (skip GFLOP/J on this host)");
    }

    println!("   (checksum guard: {:.6})", checksum);
    println!("============================================================");
}