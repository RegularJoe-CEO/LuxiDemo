use criterion::{black_box, criterion_group, criterion_main, Criterion};
use erock;

fn benchmark_evaluate(c: &mut Criterion) {
    let expr = "x^2 - 4";
    let x = vec![3.0, 4.0, 5.0, 6.0];
    let vars = std::collections::HashMap::new();
    c.bench_function("evaluate_small", |b| b.iter(|| erock::evaluate(black_box(expr), black_box(&x), black_box(&vars))));
}

fn benchmark_find_root(c: &mut Criterion) {
    let expr = "x^2 - 4";
    let lo = 1.0;
    let hi = 3.0;
    let tol = 1e-6;
    let vars = std::collections::HashMap::new();
    c.bench_function("find_root_basic", |b| b.iter(|| erock::find_root(black_box(expr), black_box(lo), black_box(hi), black_box(tol), black_box(&vars))));
}

criterion_group!(benches, benchmark_evaluate, benchmark_find_root);
criterion_main!(benches);
