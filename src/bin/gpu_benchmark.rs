// src/bin/gpu_benchmark.rs (entire file)
use anyhow::Result;
use half::f16;
use std::env;
use std::time::Instant;

// Use the new GPU module type
use erock::gpu_kernels::Fp16SincosModule;

fn parse_arg(name: &str, default: usize) -> usize {
    let mut args = env::args().collect::<Vec<_>>();
    let mut i = 0;
    while i + 1 < args.len() {
        if args[i] == format!("--{name}") {
            if let Ok(v) = args[i + 1].parse::<usize>() {
                return v;
            }
        }
        i += 1;
    }
    default
}

fn main() -> Result<()> {
    // Defaults (adjust via flags: --elements N --iters K)
    let elements = parse_arg("elements", 1_000_000);
    let iters = parse_arg("iters", 10);

    println!("gpu_benchmark: elements={}, iters={}", elements, iters);

    // Prepare input
    let input = vec![f16::from_f32(1.0); elements];

    // Initialize GPU module
    let module = Fp16SincosModule::new()?;

    // Warmup
    let _ = module.launch(&input, elements)?;

    // Timed runs
    let start = Instant::now();
    for _ in 0..iters {
        let _out = module.launch(&input, elements)?;
    }
    let elapsed = start.elapsed().as_secs_f64();

    // Very simple ops estimate: 2 results (sin, cos) per element per iter
    let total_outputs = (elements as f64) * (iters as f64) * 2.0;
    let ops_per_sec = total_outputs / elapsed.max(1e-9);

    println!("elapsed_sec={:.6}", elapsed);
    println!("outputs={} (sin+cos per element per iter)", total_outputs as u64);
    println!("throughput_ops_per_sec={:.3}", ops_per_sec);

    Ok(())
}
