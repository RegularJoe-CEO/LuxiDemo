use criterion::{criterion_group, criterion_main, Criterion};
use erock::simd_eval_over_x_inplace;
use std::hint::black_box;

fn bench_scalar_loop(c: &mut Criterion) {
    let base: Vec<f64> = (0..100_000).map(|i| i as f64 * 0.001).collect();
    c.bench_function("scalar_loop_100k", |b| {
        b.iter(|| {
            let mut data = base.clone();
            for value in data.iter_mut() {
                let v = black_box(*value);
                *value = v.sin() * v.cos();
            }
            black_box(&data);
        })
    });
}

fn bench_simd_loop(c: &mut Criterion) {
    let expr = "sin(x) * cos(x)";
    let base: Vec<f64> = (0..100_000).map(|i| i as f64 * 0.001).collect();
    c.bench_function("simd_loop_100k", |b| {
        b.iter(|| {
            let mut data = base.clone();
            let slice = black_box(data.as_mut_slice());
            simd_eval_over_x_inplace(expr, slice).unwrap();
            black_box(&data);
        })
    });
}

criterion_group!(simd_vs_scalar_groups, bench_scalar_loop, bench_simd_loop);
criterion_main!(simd_vs_scalar_groups);
