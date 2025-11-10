// benches/neural_surrogate_benchmark.rs
// Benchmark for hybrid Monte Carlo with neural surrogates
// Comparing convergence speed vs xAI orbit forecasters

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use erock::lambert::monte_carlo_tof;
use erock::neural_surrogate::{hybrid_monte_carlo_tof, SurrogateConfig, ConvergenceStats};
use std::time::Duration;

/// Benchmark pure Monte Carlo approach (baseline)
fn bench_pure_monte_carlo(c: &mut Criterion) {
    let r1 = 6980.0;
    let r2 = 10520.0;
    let c_chord = 6655.0;
    let s = 12078.0;
    let mu = 398600.0;
    let a_nominal = 6066.0;
    let a_std_dev = 10.0;
    
    let mut group = c.benchmark_group("convergence_comparison");
    group.measurement_time(Duration::from_secs(10));
    
    for n_samples in [100, 500, 1000, 5000].iter() {
        group.bench_with_input(
            BenchmarkId::new("pure_monte_carlo", n_samples),
            n_samples,
            |b, &n_samples| {
                b.iter(|| {
                    let samples = monte_carlo_tof(
                        black_box(a_nominal),
                        black_box(a_std_dev),
                        black_box(r1),
                        black_box(r2),
                        black_box(c_chord),
                        black_box(s),
                        black_box(mu),
                        black_box(0),
                        black_box(n_samples),
                    );
                    black_box(samples.len())
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark hybrid Monte Carlo with surrogate (no actual model, simulates speedup)
fn bench_hybrid_monte_carlo(c: &mut Criterion) {
    let r1 = 6980.0;
    let r2 = 10520.0;
    let c_chord = 6655.0;
    let s = 12078.0;
    let mu = 398600.0;
    let a_nominal = 6066.0;
    let a_std_dev = 10.0;
    
    let mut group = c.benchmark_group("convergence_comparison");
    group.measurement_time(Duration::from_secs(10));
    
    for n_samples in [100, 500, 1000, 5000].iter() {
        group.bench_with_input(
            BenchmarkId::new("hybrid_surrogate", n_samples),
            n_samples,
            |b, &n_samples| {
                b.iter(|| {
                    // Without neural feature, this falls back to pure Monte Carlo
                    // but still provides the ConvergenceStats interface
                    let result = hybrid_monte_carlo_tof(
                        black_box(a_nominal),
                        black_box(a_std_dev),
                        black_box(r1),
                        black_box(r2),
                        black_box(c_chord),
                        black_box(s),
                        black_box(mu),
                        black_box(0),
                        black_box(n_samples),
                        black_box(None),
                    );
                    if let Ok((samples, stats)) = result {
                        black_box((samples.len(), stats.total_evals))
                    } else {
                        black_box((0, 0))
                    }
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark convergence analysis
fn bench_convergence_analysis(c: &mut Criterion) {
    use erock::lambert::tof_probabilistic_bounds;
    
    let r1 = 6980.0;
    let r2 = 10520.0;
    let c_chord = 6655.0;
    let s = 12078.0;
    let mu = 398600.0;
    let a_nominal = 6066.0;
    let a_std_dev = 10.0;
    
    c.bench_function("convergence_analysis", |b| {
        b.iter(|| {
            // Generate samples
            let samples = monte_carlo_tof(
                a_nominal, a_std_dev, r1, r2, c_chord, s, mu, 0, 1000
            );
            
            // Extract semi-major axis values
            let a_values: Vec<f64> = samples.iter().map(|(a, _)| *a).collect();
            
            // Calculate probabilistic bounds
            let stats = tof_probabilistic_bounds(
                black_box(&a_values),
                black_box(r1),
                black_box(r2),
                black_box(c_chord),
                black_box(s),
                black_box(mu),
                black_box(0),
            );
            
            black_box(stats)
        });
    });
}

/// Demonstrate convergence speed comparison (for documentation)
fn bench_convergence_speed_comparison(c: &mut Criterion) {
    let r1 = 6980.0;
    let r2 = 10520.0;
    let c_chord = 6655.0;
    let s = 12078.0;
    let mu = 398600.0;
    let a_nominal = 6066.0;
    let a_std_dev = 10.0;
    
    let mut group = c.benchmark_group("xai_orbit_forecaster_comparison");
    group.significance_level(0.1).sample_size(50);
    
    // Traditional approach: full physics for every sample
    group.bench_function("traditional_orbit_forecaster", |b| {
        b.iter(|| {
            let samples = monte_carlo_tof(
                a_nominal, a_std_dev, r1, r2, c_chord, s, mu, 0, 1000
            );
            black_box(samples.len())
        });
    });
    
    // Hybrid approach: surrogate + selective physics
    group.bench_function("hybrid_ml_physics_forecaster", |b| {
        b.iter(|| {
            let result = hybrid_monte_carlo_tof(
                a_nominal, a_std_dev, r1, r2, c_chord, s, mu, 0, 1000, None
            );
            if let Ok((samples, _)) = result {
                black_box(samples.len())
            } else {
                black_box(0)
            }
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_pure_monte_carlo,
    bench_hybrid_monte_carlo,
    bench_convergence_analysis,
    bench_convergence_speed_comparison
);
criterion_main!(benches);
