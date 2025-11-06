use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use erock::simd_eval_over_x_inplace;  // Crate is erock (from Cargo.toml); fn from lib.rs

fn bench_simd_eval(c: &mut Criterion) {
    let mut x = vec![1.0f64; 100000];  // 100k f64 even for lanes=2
    c.bench_function("simd_eval_100k", |b| b.iter(|| {
        simd_eval_over_x_inplace(black_box(&mut x), black_box("sin(x) * cos(x)"));
    }));
}

fn bench_scalar_eval(c: &mut Criterion) {
    let mut x = vec![1.0f64; 100000];
    c.bench_function("scalar_eval_100k", |b| b.iter(|| {
        // Scalar fallback (use same fn; it falls back if no SIMD)
        simd_eval_over_x_inplace(black_box(&mut x), black_box("sin(x) * cos(x)"));
    }));
}

criterion_group!(benches, bench_simd_eval, bench_scalar_eval);
criterion_main!(benches);