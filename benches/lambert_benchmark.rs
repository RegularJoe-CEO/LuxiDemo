use criterion::{criterion_group, criterion_main, Criterion};
use erock::bisect_root;
use erock::lambert::{lambert_tof, lambert_tof_expression};
use std::hint::black_box;

/// Benchmark direct Lambert TOF calculation
fn bench_lambert_tof_direct(c: &mut Criterion) {
    let r1 = 6980.0;
    let r2 = 10520.0;
    let c_chord = 6655.0;
    let s = 12078.0;
    let mu = 398600.0;
    let a = 6066.0;
    
    c.bench_function("lambert_tof_direct", |b| {
        b.iter(|| {
            let tof = lambert_tof(
                black_box(a),
                black_box(r1),
                black_box(r2),
                black_box(c_chord),
                black_box(s),
                black_box(mu),
            );
            black_box(tof);
        })
    });
}

/// Benchmark Lambert problem solving using bisection
fn bench_lambert_bisect(c: &mut Criterion) {
    let r1 = 6980.0;
    let r2 = 10520.0;
    let c_chord = 6655.0;
    let s = 12078.0;
    let mu = 398600.0;
    let target_tof = 1800.0;
    
    let expr = lambert_tof_expression(r1, r2, c_chord, s, mu, target_tof);
    
    c.bench_function("lambert_bisect_solve", |b| {
        b.iter(|| {
            // Use correct bracket: [6040, 6100] based on the test
            let root = bisect_root(black_box(&expr), black_box(6040.0), black_box(6100.0), black_box(1e-6)).unwrap();
            black_box(root);
        })
    });
}

/// Benchmark Lambert problem with tighter tolerance
fn bench_lambert_bisect_tight(c: &mut Criterion) {
    let r1 = 6980.0;
    let r2 = 10520.0;
    let c_chord = 6655.0;
    let s = 12078.0;
    let mu = 398600.0;
    let target_tof = 1800.0;
    
    let expr = lambert_tof_expression(r1, r2, c_chord, s, mu, target_tof);
    
    c.bench_function("lambert_bisect_tight_tol", |b| {
        b.iter(|| {
            let root = bisect_root(black_box(&expr), black_box(6040.0), black_box(6100.0), black_box(1e-9)).unwrap();
            black_box(root);
        })
    });
}

criterion_group!(lambert_benches, bench_lambert_tof_direct, bench_lambert_bisect, bench_lambert_bisect_tight);
criterion_main!(lambert_benches);
