use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

/// Scalar implementation - baseline for comparison
fn scalar_sin_cos(data: &mut [f64]) {
    for value in data.iter_mut() {
        let v = *value;
        *value = v.sin() * v.cos();
    }
}

/// Scalar implementation - polynomial evaluation
fn scalar_polynomial(data: &mut [f64]) {
    for value in data.iter_mut() {
        let x = *value;
        // Polynomial: 2x^3 - 3x^2 + 5x - 1
        *value = 2.0 * x * x * x - 3.0 * x * x + 5.0 * x - 1.0;
    }
}

/// Scalar implementation - FMA operations
fn scalar_fma(data: &mut [f64]) {
    for value in data.iter_mut() {
        let x = *value;
        // Multiple FMA operations: (x * 2.5 + 1.3) * x + 0.7
        *value = x.mul_add(2.5, 1.3).mul_add(x, 0.7);
    }
}

/// ARM Neon implementation - sin*cos
#[cfg(target_arch = "aarch64")]
fn neon_sin_cos(data: &mut [f64]) {
    use std::arch::aarch64::*;
    
    let len = data.len();
    let lanes = 2; // f64x2 on Neon
    let full_vecs = len / lanes;
    
    unsafe {
        // Process 2 f64 elements at a time
        for i in (0..full_vecs * lanes).step_by(lanes) {
            // Load vector
            let ptr = data.as_ptr().add(i);
            let v = vld1q_f64(ptr);
            
            // Extract lanes, compute sin*cos (no SIMD sin/cos in standard Neon)
            let x0 = vgetq_lane_f64(v, 0);
            let x1 = vgetq_lane_f64(v, 1);
            
            let y0 = x0.sin() * x0.cos();
            let y1 = x1.sin() * x1.cos();
            
            // Build result vector and store
            let result = vsetq_lane_f64(y1, vsetq_lane_f64(y0, vdupq_n_f64(0.0), 0), 1);
            let out_ptr = data.as_mut_ptr().add(i);
            vst1q_f64(out_ptr, result);
        }
    }
    
    // Handle remainder
    for i in (full_vecs * lanes)..len {
        let v = data[i];
        data[i] = v.sin() * v.cos();
    }
}

/// ARM Neon implementation - polynomial evaluation
#[cfg(target_arch = "aarch64")]
fn neon_polynomial(data: &mut [f64]) {
    use std::arch::aarch64::*;
    
    let len = data.len();
    let lanes = 2;
    let full_vecs = len / lanes;
    
    unsafe {
        let two = vdupq_n_f64(2.0);
        let three = vdupq_n_f64(3.0);
        let five = vdupq_n_f64(5.0);
        let one = vdupq_n_f64(1.0);
        
        for i in (0..full_vecs * lanes).step_by(lanes) {
            let ptr = data.as_ptr().add(i);
            let x = vld1q_f64(ptr);
            
            // x^2
            let x2 = vmulq_f64(x, x);
            // x^3
            let x3 = vmulq_f64(x2, x);
            
            // 2x^3
            let term1 = vmulq_f64(two, x3);
            // -3x^2
            let term2 = vmulq_f64(three, x2);
            // 5x
            let term3 = vmulq_f64(five, x);
            
            // 2x^3 - 3x^2
            let result = vsubq_f64(term1, term2);
            // + 5x
            let result = vaddq_f64(result, term3);
            // - 1
            let result = vsubq_f64(result, one);
            
            let out_ptr = data.as_mut_ptr().add(i);
            vst1q_f64(out_ptr, result);
        }
    }
    
    // Handle remainder
    for i in (full_vecs * lanes)..len {
        let x = data[i];
        data[i] = 2.0 * x * x * x - 3.0 * x * x + 5.0 * x - 1.0;
    }
}

/// ARM Neon implementation - FMA operations
#[cfg(target_arch = "aarch64")]
fn neon_fma(data: &mut [f64]) {
    use std::arch::aarch64::*;
    
    let len = data.len();
    let lanes = 2;
    let full_vecs = len / lanes;
    
    unsafe {
        let c1 = vdupq_n_f64(2.5);
        let c2 = vdupq_n_f64(1.3);
        let c3 = vdupq_n_f64(0.7);
        
        for i in (0..full_vecs * lanes).step_by(lanes) {
            let ptr = data.as_ptr().add(i);
            let x = vld1q_f64(ptr);
            
            // x * 2.5 + 1.3
            let temp = vfmaq_f64(c2, x, c1);
            // temp * x + 0.7
            let result = vfmaq_f64(c3, temp, x);
            
            let out_ptr = data.as_mut_ptr().add(i);
            vst1q_f64(out_ptr, result);
        }
    }
    
    // Handle remainder
    for i in (full_vecs * lanes)..len {
        let x = data[i];
        data[i] = x.mul_add(2.5, 1.3).mul_add(x, 0.7);
    }
}

/// Portable SIMD fallback for non-ARM platforms
#[cfg(not(target_arch = "aarch64"))]
fn neon_sin_cos(data: &mut [f64]) {
    scalar_sin_cos(data);
}

#[cfg(not(target_arch = "aarch64"))]
fn neon_polynomial(data: &mut [f64]) {
    scalar_polynomial(data);
}

#[cfg(not(target_arch = "aarch64"))]
fn neon_fma(data: &mut [f64]) {
    scalar_fma(data);
}

/// Benchmark sin*cos operations
fn bench_sin_cos(c: &mut Criterion) {
    let mut group = c.benchmark_group("sin_cos");
    
    for size in [1_000, 10_000, 100_000].iter() {
        let base: Vec<f64> = (0..*size).map(|i| i as f64 * 0.001).collect();
        
        group.bench_with_input(BenchmarkId::new("scalar", size), size, |b, _| {
            b.iter(|| {
                let mut data = base.clone();
                scalar_sin_cos(black_box(&mut data));
                black_box(&data);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("neon", size), size, |b, _| {
            b.iter(|| {
                let mut data = base.clone();
                neon_sin_cos(black_box(&mut data));
                black_box(&data);
            });
        });
    }
    
    group.finish();
}

/// Benchmark polynomial evaluation
fn bench_polynomial(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial");
    
    for size in [1_000, 10_000, 100_000].iter() {
        let base: Vec<f64> = (0..*size).map(|i| i as f64 * 0.001).collect();
        
        group.bench_with_input(BenchmarkId::new("scalar", size), size, |b, _| {
            b.iter(|| {
                let mut data = base.clone();
                scalar_polynomial(black_box(&mut data));
                black_box(&data);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("neon", size), size, |b, _| {
            b.iter(|| {
                let mut data = base.clone();
                neon_polynomial(black_box(&mut data));
                black_box(&data);
            });
        });
    }
    
    group.finish();
}

/// Benchmark FMA operations
fn bench_fma(c: &mut Criterion) {
    let mut group = c.benchmark_group("fma");
    
    for size in [1_000, 10_000, 100_000].iter() {
        let base: Vec<f64> = (0..*size).map(|i| i as f64 * 0.001).collect();
        
        group.bench_with_input(BenchmarkId::new("scalar", size), size, |b, _| {
            b.iter(|| {
                let mut data = base.clone();
                scalar_fma(black_box(&mut data));
                black_box(&data);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("neon", size), size, |b, _| {
            b.iter(|| {
                let mut data = base.clone();
                neon_fma(black_box(&mut data));
                black_box(&data);
            });
        });
    }
    
    group.finish();
}

/// Benchmark memory bandwidth (load/store operations)
fn bench_memory_bandwidth(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_bandwidth");
    
    for size in [10_000, 100_000].iter() {
        let base: Vec<f64> = (0..*size).map(|i| i as f64 * 0.001).collect();
        
        #[cfg(target_arch = "aarch64")]
        group.bench_with_input(BenchmarkId::new("neon_load_store", size), size, |b, _| {
            use std::arch::aarch64::*;
            b.iter(|| {
                let mut data = base.clone();
                let len = data.len();
                let lanes = 2;
                let full_vecs = len / lanes;
                
                unsafe {
                    for i in (0..full_vecs * lanes).step_by(lanes) {
                        let ptr = data.as_ptr().add(i);
                        let v = vld1q_f64(ptr);
                        let out_ptr = data.as_mut_ptr().add(i);
                        vst1q_f64(out_ptr, v);
                    }
                }
                black_box(&data);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("scalar_copy", size), size, |b, _| {
            b.iter(|| {
                let mut data = base.clone();
                for i in 0..data.len() {
                    data[i] = data[i];
                }
                black_box(&data);
            });
        });
    }
    
    group.finish();
}

criterion_group!(
    neon_benches,
    bench_sin_cos,
    bench_polynomial,
    bench_fma,
    bench_memory_bandwidth
);
criterion_main!(neon_benches);
