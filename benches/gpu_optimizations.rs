// Benchmark for GPU optimizations: FP16 kernels, batching, and Vulkan fallback
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::collections::HashMap;

use erock::luxi_eval::{interpreter, lexer, parser};

fn parse_expr(expr: &str) -> parser::Arena {
    let tokens = lexer::tokenize(expr);
    parser::parse(tokens).expect("parse failed").0
}

// Benchmark batched evaluation (20% speedup target for 10k+ evals)
fn benchmark_batch_evaluation(c: &mut Criterion) {
    let arena = parse_expr("sin(x)*cos(x)");
    let fixed: HashMap<String, f64> = HashMap::new();
    
    let mut group = c.benchmark_group("batch_eval");
    
    for size in [1000, 5000, 10_000, 50_000].iter() {
        let xs: Vec<f64> = (0..*size).map(|i| -10.0 + (i as f64) * 0.002).collect();
        
        group.bench_with_input(BenchmarkId::new("optimized", size), size, |b, _| {
            b.iter(|| {
                let result = interpreter::batch_eval_optimized(&arena, &fixed, &xs);
                black_box(result)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("simd", size), size, |b, _| {
            b.iter(|| {
                let result = interpreter::simd_eval_over_x_inplace(0, &arena, &fixed, xs.clone());
                black_box(result)
            })
        });
    }
    
    group.finish();
}

// Benchmark GPU FP16 kernels (when available)
#[cfg(feature = "gpu")]
fn benchmark_gpu_fp16(c: &mut Criterion) {
    use erock::gpu_kernels::GpuKernels;
    
    if let Ok(gpu) = GpuKernels::new() {
        let mut group = c.benchmark_group("gpu_fp16");
        
        for size in [10_000, 50_000, 100_000].iter() {
            let input: Vec<f32> = (0..*size).map(|i| -10.0 + (i as f32) * 0.0002).collect();
            
            group.bench_with_input(BenchmarkId::new("fp16_kernel", size), size, |b, _| {
                b.iter(|| {
                    let result = gpu.eval_sincos_fp16(&input);
                    black_box(result)
                })
            });
        }
        
        group.finish();
    } else {
        println!("GPU not available, skipping GPU benchmarks");
    }
}

#[cfg(not(feature = "gpu"))]
fn benchmark_gpu_fp16(_c: &mut Criterion) {
    println!("GPU benchmarks disabled. Build with --features gpu to enable.");
}

// Benchmark Vulkan fallback (when available)
#[cfg(feature = "vulkan")]
fn benchmark_vulkan_fallback(c: &mut Criterion) {
    use erock::vulkan_fallback::VulkanFallback;
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    if let Ok(vulkan) = rt.block_on(VulkanFallback::new()) {
        let mut group = c.benchmark_group("vulkan_fallback");
        
        for size in [10_000, 50_000, 100_000].iter() {
            let input: Vec<f32> = (0..*size).map(|i| -10.0 + (i as f32) * 0.0002).collect();
            
            group.bench_with_input(BenchmarkId::new("wgpu_kernel", size), size, |b, _| {
                b.iter(|| {
                    let result = rt.block_on(vulkan.eval_sincos(&input));
                    black_box(result)
                })
            });
        }
        
        group.finish();
    } else {
        println!("Vulkan not available, skipping Vulkan benchmarks");
    }
}

#[cfg(not(feature = "vulkan"))]
fn benchmark_vulkan_fallback(_c: &mut Criterion) {
    println!("Vulkan benchmarks disabled. Build with --features vulkan to enable.");
}

// Benchmark operations per joule (energy efficiency)
fn benchmark_ops_per_joule(c: &mut Criterion) {
    let arena = parse_expr("sin(x)*cos(x)");
    let fixed: HashMap<String, f64> = HashMap::new();
    let xs: Vec<f64> = (0..100_000).map(|i| -10.0 + (i as f64) * 0.0002).collect();
    
    c.bench_function("ops_per_joule_100k", |b| {
        b.iter(|| {
            let result = interpreter::batch_eval_optimized(&arena, &fixed, &xs);
            black_box(result)
        })
    });
}

criterion_group!(
    benches,
    benchmark_batch_evaluation,
    benchmark_gpu_fp16,
    benchmark_vulkan_fallback,
    benchmark_ops_per_joule
);
criterion_main!(benches);
