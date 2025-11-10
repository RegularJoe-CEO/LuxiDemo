// SPDX-FileCopyrightText: 2025 Eric Waller
// SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0

//! Cross-platform SIMD benchmarks for xAI telemetry pipelines
//!
//! Compares performance across:
//! - Scalar (baseline)
//! - AVX2 (x86_64 with 256-bit vectors)
//! - AVX-512 (x86_64 with 512-bit vectors)
//! - ARM Neon (aarch64 with 128-bit vectors)
//!
//! Tests representative workloads for edge telemetry processing:
//! - Polynomial evaluation (sensor data transforms)
//! - FMA operations (physics calculations)
//! - Sin/Cos operations (orientation/navigation)

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use erock::simd_ops::{detect_simd_capability, polynomial_eval, fma_eval, sin_cos_eval};
use std::hint::black_box;

/// Benchmark polynomial evaluation across different sizes
fn bench_polynomial_cross_platform(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial_cross_platform");
    
    let capability = detect_simd_capability();
    println!("\n=== SIMD Capability Detection ===");
    println!("Detected: {:?}", capability);
    
    #[cfg(target_arch = "x86_64")]
    {
        if std::arch::is_x86_feature_detected!("avx512f") {
            println!("AVX-512F: Available ✓");
        } else {
            println!("AVX-512F: Not available");
        }
        if std::arch::is_x86_feature_detected!("avx2") {
            println!("AVX2: Available ✓");
        } else {
            println!("AVX2: Not available");
        }
    }
    
    #[cfg(target_arch = "aarch64")]
    {
        println!("ARM Neon: Available ✓ (always on aarch64)");
    }
    
    for size in [1_000, 10_000, 100_000, 1_000_000].iter() {
        let base: Vec<f64> = (0..*size).map(|i| i as f64 * 0.001).collect();
        
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(
            BenchmarkId::new("adaptive_simd", size),
            size,
            |b, _| {
                b.iter(|| {
                    let mut data = base.clone();
                    polynomial_eval(black_box(&mut data));
                    black_box(&data);
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark FMA operations across different sizes
fn bench_fma_cross_platform(c: &mut Criterion) {
    let mut group = c.benchmark_group("fma_cross_platform");
    
    for size in [1_000, 10_000, 100_000, 1_000_000].iter() {
        let base: Vec<f64> = (0..*size).map(|i| i as f64 * 0.001).collect();
        
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(
            BenchmarkId::new("adaptive_simd", size),
            size,
            |b, _| {
                b.iter(|| {
                    let mut data = base.clone();
                    fma_eval(black_box(&mut data));
                    black_box(&data);
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark sin*cos operations (currently scalar)
fn bench_sin_cos_cross_platform(c: &mut Criterion) {
    let mut group = c.benchmark_group("sin_cos_cross_platform");
    
    for size in [1_000, 10_000, 100_000].iter() {
        let base: Vec<f64> = (0..*size).map(|i| i as f64 * 0.001).collect();
        
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(
            BenchmarkId::new("scalar", size),
            size,
            |b, _| {
                b.iter(|| {
                    let mut data = base.clone();
                    sin_cos_eval(black_box(&mut data));
                    black_box(&data);
                });
            },
        );
    }
    
    group.finish();
}

/// Memory bandwidth test - measures impact of vector width on memory subsystem
fn bench_memory_bandwidth(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_bandwidth");
    
    let capability = detect_simd_capability();
    
    for size in [10_000, 100_000, 1_000_000].iter() {
        let base: Vec<f64> = (0..*size).map(|i| i as f64 * 0.001).collect();
        
        // Bytes transferred: read + write
        let bytes = (*size as u64) * std::mem::size_of::<f64>() as u64 * 2;
        group.throughput(Throughput::Bytes(bytes));
        
        group.bench_with_input(
            BenchmarkId::new(format!("{:?}", capability), size),
            size,
            |b, _| {
                b.iter(|| {
                    let mut data = base.clone();
                    // Simple operation to test memory bandwidth
                    fma_eval(black_box(&mut data));
                    black_box(&data);
                });
            },
        );
    }
    
    group.finish();
}

/// Telemetry pipeline simulation - realistic mixed workload
/// Simulates edge device processing sensor data:
/// 1. Polynomial transform (calibration)
/// 2. FMA operations (scaling/offset)
/// 3. Trigonometric calculations (orientation)
fn bench_telemetry_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("telemetry_pipeline");
    
    let capability = detect_simd_capability();
    println!("\n=== Telemetry Pipeline Test ===");
    println!("SIMD Mode: {:?}", capability);
    
    // Realistic edge telemetry sizes
    for size in [256, 1_024, 4_096, 16_384].iter() {
        let base: Vec<f64> = (0..*size).map(|i| (i as f64) * 0.01).collect();
        
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(
            BenchmarkId::new(format!("{:?}", capability), size),
            size,
            |b, _| {
                b.iter(|| {
                    let mut data = base.clone();
                    
                    // Stage 1: Sensor calibration (polynomial)
                    polynomial_eval(black_box(&mut data));
                    
                    // Stage 2: Unit conversion (FMA)
                    fma_eval(black_box(&mut data));
                    
                    // Stage 3: Orientation calculation (trig)
                    // Take every 8th element to keep realistic
                    let mut trig_data: Vec<f64> = data.iter()
                        .step_by(8)
                        .copied()
                        .collect();
                    sin_cos_eval(black_box(&mut trig_data));
                    
                    black_box(&data);
                    black_box(&trig_data);
                });
            },
        );
    }
    
    group.finish();
}

/// Energy efficiency estimation
/// Estimates ops/J for different SIMD implementations
/// Note: Actual power measurement requires hardware instrumentation
fn bench_energy_estimation(c: &mut Criterion) {
    use std::time::Duration;
    
    let mut group = c.benchmark_group("energy_estimation");
    group.measurement_time(Duration::from_secs(10));
    
    let capability = detect_simd_capability();
    
    // Large workload for power measurement
    let size = 1_000_000;
    let base: Vec<f64> = (0..size).map(|i| i as f64 * 0.001).collect();
    
    group.throughput(Throughput::Elements(size as u64));
    
    println!("\n=== Energy Efficiency Estimation ===");
    println!("SIMD Mode: {:?}", capability);
    println!("Workload: {} elements", size);
    println!("\nEstimated Power Consumption:");
    
    match capability {
        erock::simd_ops::SimdCapability::Avx512 => {
            println!("  AVX-512: ~20-30W (wider vectors, more power)");
            println!("  Expected: 25% faster than AVX2");
        }
        erock::simd_ops::SimdCapability::Avx2 => {
            println!("  AVX2: ~15-20W (balanced)");
            println!("  Baseline for x86_64");
        }
        erock::simd_ops::SimdCapability::Neon => {
            println!("  ARM Neon: ~5-15W (power efficient)");
            println!("  Best ops/J for mobile/edge");
        }
        erock::simd_ops::SimdCapability::Scalar => {
            println!("  Scalar: ~10-15W (no SIMD overhead)");
            println!("  Baseline for all platforms");
        }
    }
    
    group.bench_function(format!("{:?}", capability), |b| {
        b.iter(|| {
            let mut data = base.clone();
            polynomial_eval(black_box(&mut data));
            fma_eval(black_box(&mut data));
            black_box(&data);
        });
    });
    
    group.finish();
}

criterion_group!(
    cross_platform_benches,
    bench_polynomial_cross_platform,
    bench_fma_cross_platform,
    bench_sin_cos_cross_platform,
    bench_memory_bandwidth,
    bench_telemetry_pipeline,
    bench_energy_estimation
);
criterion_main!(cross_platform_benches);
