use criterion::{criterion_group, criterion_main, Criterion};
use erock::simd_eval_over_x_inplace;
use std::hint::black_box;

fn bench_simd_repro(c: &mut Criterion) {
    let expr = "sin(x) * cos(x)";
    let base: Vec<f64> = (0..100_000).map(|i| i as f64 * 0.001).collect();
    c.bench_function("simd_repro_100k", |b| {
        b.iter(|| {
            let mut data = base.clone();
            let slice = black_box(data.as_mut_slice());
            simd_eval_over_x_inplace(expr, slice).unwrap();
            black_box(&data);
        })
    });
}

criterion_group!(repro_groups, bench_simd_repro);
criterion_main!(repro_groups);
