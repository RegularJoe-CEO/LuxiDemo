// GPU benchmark for L4 FP16 sin*cos evaluation
// Measures throughput in ops/sec for energy comparison

use erock::gpu_kernels::GpuKernels;
use std::time::Instant;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    // Parse batch size from args (default 10M for L4 saturation)
    let batch_size: usize = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10_000_000);
    
    println!("Initializing GPU kernels...");
    let kernels = GpuKernels::new()?;
    
    // Generate test data
    println!("Generating {} test values...", batch_size);
    let input: Vec<f32> = (0..batch_size)
        .map(|i| (i as f32) * 0.001)
        .collect();
    
    // Warmup run
    println!("Warmup run...");
    let _ = kernels.eval_sincos_fp16(&input)?;
    
    // Benchmark run
    println!("Running GPU benchmark...");
    let start = Instant::now();
    let results = kernels.eval_sincos_fp16(&input)?;
    let elapsed = start.elapsed();
    
    // Calculate metrics
    let ops = input.len() as f64;
    let ops_per_sec = ops / elapsed.as_secs_f64();
    let gops_per_sec = ops_per_sec / 1e9;
    
    println!("\n=== GPU FP16 Benchmark Results ===");
    println!("Operations:      {:>15}", input.len());
    println!("Time:            {:>12.3} s", elapsed.as_secs_f64());
    println!("Throughput:      {:>12.2} B ops/sec", gops_per_sec);
    println!("First result:    {:>12.6}", results[0]);
    println!("Last result:     {:>12.6}", results[results.len()-1]);
    println!("==================================\n");
    
    Ok(())
}
