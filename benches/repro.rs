use criterion::{black_box, criterion_group, criterion_main, Criterion};
use erock::interpreter::simd_eval_over_x_inplace;  // Your SIMD fn

fn repro_bench(c: &mut Criterion) {
    let mut x = (0..100000).map(|i| ((i as f64 * 0.1 - 5000.0) / 10000.0 * 20.0 - 10.0)).collect::<Vec<_>>();  // uniform(-10,10), seed=42 equiv
    c.bench_function("repro_100k_sin_cos_simd", |b| b.iter(|| {
        let _ = simd_eval_over_x_inplace(black_box(&mut x), black_box("sin(x) * cos(x)"));
    }));
}

criterion_group!(repro, repro_bench);
criterion_main!(repro);
