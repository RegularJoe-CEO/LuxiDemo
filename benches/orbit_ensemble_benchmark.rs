// SPDX-FileCopyrightText: 2025 Eric Waller
// SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0

//! Benchmark orbital ensemble generation and convergence analysis
//! 
//! Tests SIMD-optimized vs scalar performance for:
//! - LEO swarm generation
//! - J2 perturbation propagation
//! - N-body multi-satellite interactions

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use erock::orbit_ensemble::{LeoSwarmConfig, generate_leo_swarm, propagate_j2};
use erock::nbody::{NBodySystem, propagate_nbody};

/// Benchmark LEO swarm generation
fn bench_swarm_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("swarm_generation");
    
    for size in [100, 500, 1000, 5000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let config = LeoSwarmConfig {
                num_sats: size,
                ..Default::default()
            };
            b.iter(|| {
                black_box(generate_leo_swarm(&config))
            });
        });
    }
    
    group.finish();
}

/// Benchmark J2 propagation for single satellite
fn bench_j2_propagation(c: &mut Criterion) {
    let mut group = c.benchmark_group("j2_propagation");
    
    let config = LeoSwarmConfig {
        num_sats: 1,
        ..Default::default()
    };
    let swarm = generate_leo_swarm(&config);
    let state = swarm[0].to_state_vector();
    
    for dt in [1.0, 10.0, 60.0].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(dt), dt, |b, &dt| {
            b.iter(|| {
                black_box(propagate_j2(&state, dt))
            });
        });
    }
    
    group.finish();
}

/// Benchmark N-body propagation for swarms
fn bench_nbody_propagation(c: &mut Criterion) {
    let mut group = c.benchmark_group("nbody_propagation");
    
    for size in [10, 50, 100, 500].iter() {
        let config = LeoSwarmConfig {
            num_sats: *size,
            ..Default::default()
        };
        let swarm = generate_leo_swarm(&config);
        let states: Vec<_> = swarm.iter().map(|oe| oe.to_state_vector()).collect();
        let system = NBodySystem::new_massless(states);
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                black_box(propagate_nbody(&system, 1.0, true))
            });
        });
    }
    
    group.finish();
}

/// Benchmark convergence: SIMD vs scalar baseline
fn bench_convergence_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("convergence_analysis");
    
    // Generate moderate-sized swarm for convergence testing
    let config = LeoSwarmConfig {
        num_sats: 100,
        ..Default::default()
    };
    let swarm = generate_leo_swarm(&config);
    let states: Vec<_> = swarm.iter().map(|oe| oe.to_state_vector()).collect();
    let system = NBodySystem::new_massless(states);
    
    // SIMD path (with J2)
    group.bench_function("simd_j2_100sats", |b| {
        b.iter(|| {
            black_box(propagate_nbody(&system, 1.0, true))
        });
    });
    
    // Without J2 (faster baseline)
    group.bench_function("simd_no_j2_100sats", |b| {
        b.iter(|| {
            black_box(propagate_nbody(&system, 1.0, false))
        });
    });
    
    // Scalar baseline (single satellite for comparison)
    let single_state = swarm[0].to_state_vector();
    group.bench_function("scalar_j2_1sat", |b| {
        b.iter(|| {
            black_box(propagate_j2(&single_state, 1.0))
        });
    });
    
    group.finish();
}

/// Benchmark sub-millisecond timesteps for real-time applications
fn bench_realtime_propagation(c: &mut Criterion) {
    let mut group = c.benchmark_group("realtime_propagation");
    
    // Small swarms for real-time control (e.g., drone formations)
    for size in [5, 10, 20].iter() {
        let config = LeoSwarmConfig {
            num_sats: *size,
            ..Default::default()
        };
        let swarm = generate_leo_swarm(&config);
        let states: Vec<_> = swarm.iter().map(|oe| oe.to_state_vector()).collect();
        let system = NBodySystem::new_massless(states);
        
        group.bench_with_input(
            BenchmarkId::new("target_1ms", size), 
            size, 
            |b, _| {
                b.iter(|| {
                    black_box(propagate_nbody(&system, 0.1, true))
                });
            }
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_swarm_generation,
    bench_j2_propagation,
    bench_nbody_propagation,
    bench_convergence_analysis,
    bench_realtime_propagation,
);
criterion_main!(benches);
