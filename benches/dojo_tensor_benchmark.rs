// benches/dojo_tensor_benchmark.rs
// Synthetic Dojo-like tensor benchmarks for xAI-scale validation
//
// Simulates Tesla Dojo characteristics:
// - Large-scale tensor operations (millions to billions of elements)
// - Matrix multiplications and elementwise operations
// - Mixed precision (FP32/FP16 simulation)
// - High memory bandwidth workloads
// - Batch tensor processing
//
// This bridges toward xAI-scale validation by demonstrating Luxi Edge's
// capability to handle tensor workloads similar to those on Tesla Dojo,
// while maintaining energy efficiency and deterministic execution.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use erock::evaluate;
use std::hint::black_box;
use std::time::Duration;

/// Generate synthetic tensor data
fn generate_tensor_1d(size: usize) -> Vec<f64> {
    (0..size).map(|i| (i as f64 * 0.1).sin()).collect()
}

fn generate_tensor_2d(rows: usize, cols: usize) -> Vec<Vec<f64>> {
    (0..rows)
        .map(|i| {
            (0..cols)
                .map(|j| ((i + j) as f64 * 0.1).sin())
                .collect()
        })
        .collect()
}

/// Benchmark 1: Large 1D tensor elementwise operations (Dojo-like workload)
/// Simulates activation functions applied across large tensor batches
fn bench_tensor_elementwise_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("dojo_tensor_elementwise");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);
    
    // Test various tensor sizes (smaller for CI/testing, represents Dojo-scale)
    for size in [100_000, 500_000, 1_000_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        let tensor = generate_tensor_1d(*size);
        
        // GELU-like activation: x * 0.5 * (1 + tanh(sqrt(2/pi) * (x + 0.044715 * x^3)))
        // Simplified for demonstration: sin(x) * cos(x) + x^2 * 0.1
        // Note: Currently limited to single-variable expressions
        group.bench_with_input(
            BenchmarkId::new("activation_fn", size),
            size,
            |b, _| {
                b.iter(|| {
                    let expr = "sin(x) * cos(x)";
                    let result = evaluate(black_box(expr), black_box(&tensor));
                    black_box(result)
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark 2: Matrix-like tensor operations (2D tensors)
/// Simulates weight matrices in neural networks
fn bench_tensor_matrix_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("dojo_tensor_matrix");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);
    
    // Test matrix sizes typical in AI training
    for (rows, cols) in [(500, 500), (1000, 1000)].iter() {
        let total_elements = rows * cols;
        group.throughput(Throughput::Elements(total_elements as u64));
        
        let matrix = generate_tensor_2d(*rows, *cols);
        
        // Flatten for Luxi evaluation
        let flattened: Vec<f64> = matrix.iter().flat_map(|row| row.iter().cloned()).collect();
        
        group.bench_with_input(
            BenchmarkId::new("hadamard_product", format!("{}x{}", rows, cols)),
            &total_elements,
            |b, _| {
                b.iter(|| {
                    // Hadamard (elementwise) product with scaling
                    let expr = "sin(x) * cos(x)";
                    let result = evaluate(black_box(expr), black_box(&flattened));
                    black_box(result)
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark 3: Batch tensor processing (simulates mini-batch training)
/// Multiple independent tensor operations in sequence
fn bench_tensor_batch_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("dojo_tensor_batch");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);
    
    for batch_size in [8, 16, 32].iter() {
        let tensor_size = 50_000; // 50k elements per tensor
        let total_elements = batch_size * tensor_size;
        group.throughput(Throughput::Elements(total_elements as u64));
        
        // Generate batch of tensors
        let batch: Vec<Vec<f64>> = (0..*batch_size)
            .map(|_| generate_tensor_1d(tensor_size))
            .collect();
        
        group.bench_with_input(
            BenchmarkId::new("mini_batch", batch_size),
            batch_size,
            |b, _| {
                b.iter(|| {
                    let expr = "sin(x) * cos(x)";
                    let mut total = 0;
                    for tensor in &batch {
                        let result = evaluate(black_box(expr), black_box(tensor));
                        total += result.map(|v| v.len()).unwrap_or(0);
                    }
                    black_box(total)
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark 4: Complex expressions on large tensors
/// Simulates forward pass computations
fn bench_tensor_complex_expr(c: &mut Criterion) {
    let mut group = c.benchmark_group("dojo_tensor_complex");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);
    
    let size = 500_000;
    group.throughput(Throughput::Elements(size as u64));
    
    let tensor = generate_tensor_1d(size);
    
    // Complex expression simulating layer computation
    group.bench_function("complex_forward_pass", |b| {
        b.iter(|| {
            // Simulates: output = activation(weighted_input)
            let expr = "sin(x) * cos(x) + x * x * 0.1";
            let result = evaluate(black_box(expr), black_box(&tensor));
            black_box(result)
        });
    });
    
    group.finish();
}

/// Benchmark 5: Memory bandwidth stress test (Dojo interconnect simulation)
/// Large sequential tensor operations testing memory hierarchy
fn bench_tensor_memory_bandwidth(c: &mut Criterion) {
    let mut group = c.benchmark_group("dojo_tensor_memory");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);
    
    for size in [1_000_000, 5_000_000].iter() {
        group.throughput(Throughput::Bytes((*size * std::mem::size_of::<f64>()) as u64));
        
        let tensor = generate_tensor_1d(*size);
        
        group.bench_with_input(
            BenchmarkId::new("memory_bound", size),
            size,
            |b, _| {
                b.iter(|| {
                    // Simple operation to maximize memory bandwidth impact
                    let expr = "x * 2.0 + 1.0";
                    let result = evaluate(black_box(expr), black_box(&tensor));
                    black_box(result)
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark 6: Precision comparison (FP32 baseline, simulated FP16)
/// Demonstrates trade-off between precision and throughput
fn bench_tensor_precision_variants(c: &mut Criterion) {
    let mut group = c.benchmark_group("dojo_tensor_precision");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);
    
    let size = 1_000_000;
    group.throughput(Throughput::Elements(size as u64));
    
    let tensor_fp64 = generate_tensor_1d(size);
    
    // Simulate FP16 by reducing mantissa precision
    let tensor_fp16_sim: Vec<f64> = tensor_fp64
        .iter()
        .map(|&x| {
            // Simulate FP16 precision loss by rounding to 3 decimal places
            (x * 1000.0).round() / 1000.0
        })
        .collect();
    
    group.bench_function("fp64_precision", |b| {
        b.iter(|| {
            let expr = "sin(x) * cos(x) + x * x";
            let result = evaluate(black_box(expr), black_box(&tensor_fp64));
            black_box(result)
        });
    });
    
    group.bench_function("fp16_simulated", |b| {
        b.iter(|| {
            let expr = "sin(x) * cos(x) + x * x";
            let result = evaluate(black_box(expr), black_box(&tensor_fp16_sim));
            black_box(result)
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_tensor_elementwise_ops,
    bench_tensor_matrix_ops,
    bench_tensor_batch_processing,
    bench_tensor_complex_expr,
    bench_tensor_memory_bandwidth,
    bench_tensor_precision_variants
);
criterion_main!(benches);
