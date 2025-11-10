use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use erock::bisect_root;
use erock::lambert::{lambert_tof, lambert_tof_expression, lambert_tof_multirev, 
                      solve_multirev_batch, batch_tof_scalar, batch_tof_neon};
use std::hint::black_box as hint_black_box;

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
            hint_black_box(tof);
        })
    });
}

/// Benchmark multi-revolution TOF calculation
fn bench_lambert_tof_multirev(c: &mut Criterion) {
    let r1 = 6980.0;
    let r2 = 10520.0;
    let c_chord = 6655.0;
    let s = 12078.0;
    let mu = 398600.0;
    let a = 6066.0;
    
    let mut group = c.benchmark_group("multirev_tof");
    
    for n_rev in [0, 1, 2, 3].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n_rev), n_rev, |b, &n_rev| {
            b.iter(|| {
                let tof = lambert_tof_multirev(
                    black_box(a),
                    black_box(r1),
                    black_box(r2),
                    black_box(c_chord),
                    black_box(s),
                    black_box(mu),
                    black_box(n_rev),
                );
                hint_black_box(tof);
            })
        });
    }
    
    group.finish();
}

/// Benchmark batch TOF calculation - scalar vs SIMD
fn bench_batch_tof(c: &mut Criterion) {
    let r1 = 6980.0;
    let r2 = 10520.0;
    let c_chord = 6655.0;
    let s = 12078.0;
    let mu = 398600.0;
    
    let mut group = c.benchmark_group("batch_tof");
    
    for size in [10, 100, 1000].iter() {
        // Generate semi-major axis values (all > s/2 = 6039)
        let a_values: Vec<f64> = (0..*size)
            .map(|i| 6050.0 + (i as f64) * 10.0)
            .collect();
        
        group.bench_with_input(BenchmarkId::new("scalar", size), size, |b, _| {
            b.iter(|| {
                let results = batch_tof_scalar(
                    black_box(&a_values),
                    black_box(r1),
                    black_box(r2),
                    black_box(c_chord),
                    black_box(s),
                    black_box(mu),
                    black_box(0),
                );
                hint_black_box(&results);
            })
        });
        
        group.bench_with_input(BenchmarkId::new("neon", size), size, |b, _| {
            b.iter(|| {
                let results = batch_tof_neon(
                    black_box(&a_values),
                    black_box(r1),
                    black_box(r2),
                    black_box(c_chord),
                    black_box(s),
                    black_box(mu),
                    black_box(0),
                );
                hint_black_box(&results);
            })
        });
    }
    
    group.finish();
}

/// Benchmark multi-revolution batch solver (swarm simulation use case)
fn bench_multirev_batch_solver(c: &mut Criterion) {
    let r1 = 6980.0;
    let r2 = 10520.0;
    let c_chord = 6655.0;
    let s = 12078.0;
    let mu = 398600.0;
    let target_tof = 1800.0;
    
    let mut group = c.benchmark_group("multirev_batch_solver");
    
    // Benchmark solving for increasing numbers of revolutions
    let test_cases = vec![
        ("single_rev", vec![0]),
        ("dual_rev", vec![0, 1]),
        ("quad_rev", vec![0, 1, 2, 3]),
        ("swarm_8rev", vec![0, 1, 2, 3, 4, 5, 6, 7]),
    ];
    
    for (name, rev_counts) in test_cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), &rev_counts, |b, rev_counts| {
            b.iter(|| {
                let solutions = solve_multirev_batch(
                    black_box(r1),
                    black_box(r2),
                    black_box(c_chord),
                    black_box(s),
                    black_box(mu),
                    black_box(target_tof),
                    black_box(rev_counts),
                    black_box(1e-3),
                );
                hint_black_box(&solutions);
            })
        });
    }
    
    group.finish();
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

criterion_group!(
    lambert_benches,
    bench_lambert_tof_direct,
    bench_lambert_tof_multirev,
    bench_batch_tof,
    bench_multirev_batch_solver,
    bench_lambert_bisect,
    bench_lambert_bisect_tight
);
criterion_main!(lambert_benches);
