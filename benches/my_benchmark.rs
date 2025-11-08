use criterion::{criterion_group, criterion_main, Criterion};
use erock::{bisect_root, evaluate, simd_eval_over_x_inplace};
use std::hint::black_box;

fn bench_evaluate_small(c: &mut Criterion) {
    let expr = "sin(x) * cos(x)";
    let xs: Vec<f64> = (0..10_000).map(|i| i as f64 * 0.001).collect();
    c.bench_function("evaluate_10k", |b| {
        b.iter(|| {
            let ys = evaluate(expr, black_box(&xs)).unwrap();
            black_box(ys);
        })
    });
}

fn bench_evaluate_large(c: &mut Criterion) {
    let expr = "sin(x) * cos(x)";
    let xs: Vec<f64> = (0..100_000).map(|i| i as f64 * 0.001).collect();
    c.bench_function("evaluate_100k", |b| {
        b.iter(|| {
            let ys = evaluate(expr, black_box(&xs)).unwrap();
            black_box(ys);
        })
    });
}

fn bench_bisect(c: &mut Criterion) {
    c.bench_function("bisect_root", |b| {
        b.iter(|| {
            let root = bisect_root("x * x - 2.0", -10.0, 10.0, 1e-9).unwrap();
            black_box(root);
        })
    });
}

fn bench_simd(c: &mut Criterion) {
    let expr = "sin(x) * cos(x)";
    let base: Vec<f64> = (0..100_000).map(|i| i as f64 * 0.001).collect();
    c.bench_function("simd_inplace_100k", |b| {
        b.iter(|| {
            let mut data = base.clone();
            let slice = black_box(data.as_mut_slice());
            simd_eval_over_x_inplace(expr, slice).unwrap();
            black_box(&data);
        })
    });
}

criterion_group!(my_benchmark_groups, bench_evaluate_small, bench_evaluate_large, bench_bisect, bench_simd);
criterion_main!(my_benchmark_groups);
