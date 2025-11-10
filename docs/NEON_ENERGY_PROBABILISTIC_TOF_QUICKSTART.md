# Neon Ops/Joule and Probabilistic TOF Quick Start

## Overview

This document provides quick examples for using the new ARM Neon energy efficiency calculations and probabilistic TOF bounds added to Luxi Edge.

## 1. ARM Neon Energy Efficiency Calculations

### Get Platform Energy Profile

```rust
use erock::energy::neon_profiles;

// Select your platform
let pi5 = neon_profiles::raspberry_pi5();
let jetson = neon_profiles::jetson_orin_nano();
let graviton = neon_profiles::aws_graviton3();
let m2 = neon_profiles::apple_m2();

println!("Platform: {}", pi5.platform);
println!("Power: {}W", pi5.power_watts);
println!("SIMD width: {}x f64", pi5.simd_width);
```

### Calculate Theoretical Peak Efficiency

```rust
use erock::energy::{neon_profiles, theoretical_peak_ops_per_joule};

let pi5 = neon_profiles::raspberry_pi5();
let peak = theoretical_peak_ops_per_joule(&pi5);

println!("Pi5 theoretical peak: {:.2}B ops/J", peak / 1e9);
// Output: Pi5 theoretical peak: 2.67B ops/J
```

### Get Realistic Efficiency Bounds

```rust
use erock::energy::{neon_profiles, energy_efficiency_bounds};

let pi5 = neon_profiles::raspberry_pi5();
let (pessimistic, realistic, optimistic) = energy_efficiency_bounds(&pi5);

println!("Pessimistic (20%): {:.2}M ops/J", pessimistic / 1e6);
println!("Realistic (50%):   {:.2}M ops/J", realistic / 1e6);
println!("Optimistic (80%):  {:.2}M ops/J", optimistic / 1e6);

// Output:
// Pessimistic (20%): 533.33M ops/J
// Realistic (50%):   1333.33M ops/J
// Optimistic (80%):  2133.33M ops/J
```

### Measure Actual Energy Efficiency

```rust
use erock::energy::{neon_profiles, EnergyMetrics};
use std::time::Instant;

let platform = neon_profiles::raspberry_pi5();

// Run your benchmark
let start = Instant::now();
let ops_completed = run_neon_benchmark(); // Your function
let duration = start.elapsed().as_secs_f64();

// Calculate metrics
let ops_per_second = ops_completed as f64 / duration;
let power_watts = platform.power_watts; // Or measure with INA219 sensor

let metrics = EnergyMetrics::from_measurements(
    ops_per_second,
    power_watts,
    duration
);

println!("Throughput: {:.2}M ops/sec", metrics.ops_per_second / 1e6);
println!("Efficiency: {:.2}M ops/J", metrics.ops_per_joule / 1e6);
println!("Energy: {:.2}J total", metrics.total_energy_joules);
println!("Energy/op: {:.2}nJ", metrics.nanojoules_per_op);
```

## 2. Probabilistic TOF Bounds

### Basic Probabilistic Analysis

```rust
use erock::lambert::tof_probabilistic_bounds;

// Lambert problem parameters
let r1 = 6980.0;   // km
let r2 = 10520.0;  // km
let c = 6655.0;    // km
let s = 12078.0;   // km
let mu = 398600.0; // km³/s²

// Generate samples with uncertainty (e.g., from sensor noise)
let a_nominal = 6066.0;
let a_samples: Vec<f64> = (0..1000)
    .map(|i| a_nominal + (i as f64 - 500.0) * 0.1)
    .collect();

// Calculate probabilistic bounds
let stats = tof_probabilistic_bounds(&a_samples, r1, r2, c, s, mu, 0);

println!("TOF Statistics:");
println!("  Mean: {:.1}s ± {:.1}s", stats.mean, stats.std_dev);
println!("  Range: [{:.1}, {:.1}]s", stats.min, stats.max);
println!("  Median (p50): {:.1}s", stats.p50);
println!("  p95: {:.1}s", stats.p95);
println!("  p99: {:.1}s", stats.p99);
println!("  95% CI: [{:.1}, {:.1}]s", 
         stats.confidence_95_lower, 
         stats.confidence_95_upper);
```

### Monte Carlo Simulation

```rust
use erock::lambert::monte_carlo_tof;

// Nominal parameters
let a_nominal = 6066.0;
let a_std_dev = 10.0;  // 10 km uncertainty in semi-major axis

let r1 = 6980.0;
let r2 = 10520.0;
let c = 6655.0;
let s = 12078.0;
let mu = 398600.0;
let n_rev = 0;  // Direct transfer

// Run Monte Carlo with 1000 samples
let samples = monte_carlo_tof(
    a_nominal,
    a_std_dev,
    r1, r2, c, s, mu,
    n_rev,
    1000
);

// Analyze results
let tof_values: Vec<f64> = samples.iter()
    .map(|(_, tof)| *tof)
    .filter(|tof| tof.is_finite())
    .collect();

let mean_tof: f64 = tof_values.iter().sum::<f64>() / tof_values.len() as f64;
println!("Monte Carlo results:");
println!("  Samples: {}", tof_values.len());
println!("  Mean TOF: {:.1}s", mean_tof);
```

### Combined Example: Mission Planning with Uncertainty

```rust
use erock::lambert::{monte_carlo_tof, tof_probabilistic_bounds};
use erock::energy::{neon_profiles, EnergyMetrics};
use std::time::Instant;

// Mission parameters with uncertainty
let r1 = 6980.0;   // LEO altitude
let r2 = 42164.0;  // GEO altitude
let c = 35184.0;
let s = 42567.0;
let mu = 398600.0;

let a_nominal = 24572.0;  // km
let a_std_dev = 50.0;     // GPS uncertainty propagated to semi-major axis

// Platform energy budget
let platform = neon_profiles::raspberry_pi5();

// Run Monte Carlo simulation
let start = Instant::now();
let samples = monte_carlo_tof(a_nominal, a_std_dev, r1, r2, c, s, mu, 0, 1000);
let duration = start.elapsed().as_secs_f64();

// Calculate statistics
let a_samples: Vec<f64> = samples.iter().map(|(a, _)| *a).collect();
let stats = tof_probabilistic_bounds(&a_samples, r1, r2, c, s, mu, 0);

// Energy metrics
let ops = 1000.0; // 1000 TOF calculations
let metrics = EnergyMetrics::from_measurements(
    ops / duration,
    platform.power_watts,
    duration
);

println!("Mission Planning Results:");
println!("  Nominal TOF: {:.1}s", stats.mean);
println!("  Uncertainty: ±{:.1}s (1σ)", stats.std_dev);
println!("  95% CI: [{:.1}, {:.1}]s", stats.confidence_95_lower, stats.confidence_95_upper);
println!("  Worst case (p99): {:.1}s", stats.p99);
println!("  ΔV margin: {:.1}%", (stats.p99 - stats.mean) / stats.mean * 100.0);
println!("\nComputation Efficiency:");
println!("  Time: {:.3}ms", duration * 1000.0);
println!("  Energy: {:.2}mJ", metrics.total_energy_joules * 1000.0);
println!("  Efficiency: {:.2}M ops/J", metrics.ops_per_joule / 1e6);
```

## 3. Running Benchmarks

### Lambert Probabilistic Benchmarks

```bash
# All lambert benchmarks
cargo bench --bench lambert_benchmark

# Only probabilistic benchmarks
cargo bench --bench lambert_benchmark -- probabilistic

# Only Monte Carlo benchmarks
cargo bench --bench lambert_benchmark -- monte_carlo
```

### Neon Energy Efficiency Benchmarks

```bash
# All neon benchmarks
cargo bench --bench neon_benchmark

# Only energy efficiency benchmarks
cargo bench --bench neon_benchmark -- energy_efficiency

# Quick test (no full benchmark run)
cargo bench --bench neon_benchmark -- --test
```

## 4. Rad-Hard Space Applications

For spacecraft and satellite applications with radiation-hardened requirements, see the comprehensive guide:

[docs/RAD_HARD_SPACE_APPLICATIONS.md](RAD_HARD_SPACE_APPLICATIONS.md)

Topics covered:
- Energy-constrained mission design (cubesats, interplanetary probes)
- Radiation-induced uncertainty quantification
- Platform selection (Pi5, Jetson, Graviton for TMR configurations)
- Real-world examples: Lunar Gateway, Mars Sample Return, constellation formation flying
- Integration with flight software

## 5. xAI Grok Stochastic Simulations

For xAI applications requiring stochastic trajectory analysis:

### Swarm Optimization

```rust
use erock::lambert::solve_multirev_batch;

// Evaluate multiple revolution counts simultaneously
let rev_counts = vec![0, 1, 2, 3, 4, 5, 6, 7];
let solutions = solve_multirev_batch(r1, r2, c, s, mu, target_tof, &rev_counts, 1e-3);

// Benchmark shows: 8-revolution swarm solved in 16.3 µs
println!("Found {} trajectory options", solutions.len());
for (n_rev, a) in solutions {
    println!("  {} rev: a = {:.1} km", n_rev, a);
}
```

### Uncertainty Propagation

```rust
// For each trajectory candidate, compute probabilistic bounds
for (n_rev, a_solution) in solutions {
    // Generate samples with sensor uncertainty
    let a_samples: Vec<f64> = generate_uncertainty_samples(a_solution, sensor_noise);
    let stats = tof_probabilistic_bounds(&a_samples, r1, r2, c, s, mu, n_rev);
    
    // Select robust trajectory with best p95 performance
    if stats.p95 < best_p95 && stats.confidence_95_upper < fuel_budget {
        best_trajectory = (n_rev, a_solution, stats);
    }
}
```

## 6. Energy Budget Analysis

### Example: Pi5 vs Jetson vs Graviton3

```rust
use erock::energy::{neon_profiles, theoretical_peak_ops_per_joule};

let platforms = vec![
    neon_profiles::raspberry_pi5(),
    neon_profiles::jetson_orin_nano(),
    neon_profiles::aws_graviton3(),
];

println!("Platform Energy Comparison:");
println!("{:<25} {:>10} {:>15}", "Platform", "Power (W)", "Peak (M ops/J)");
println!("{}", "-".repeat(52));

for platform in platforms {
    let peak = theoretical_peak_ops_per_joule(&platform);
    println!("{:<25} {:>10.1} {:>15.1}", 
             platform.platform, 
             platform.power_watts, 
             peak / 1e6);
}

// Output:
// Platform Energy Comparison:
// Platform                   Power (W)   Peak (M ops/J)
// ----------------------------------------------------
// Raspberry Pi 5 (...)            3.0          2666.7
// Jetson Orin Nano (...)          7.0           800.0
// AWS Graviton3 (...)             5.0          1485.7
```

## References

- [BENCHMARK_DATA.md](../BENCHMARK_DATA.md) — Full benchmark results
- [docs/RAD_HARD_SPACE_APPLICATIONS.md](RAD_HARD_SPACE_APPLICATIONS.md) — Space applications guide
- [docs/XAI_EXECUTIVE_SUMMARY.md](XAI_EXECUTIVE_SUMMARY.md) — xAI integration overview
- [benches/README_NEON.md](../benches/README_NEON.md) — Neon benchmark documentation
- [docs/lambert_benchmark.md](lambert_benchmark.md) — Lambert problem benchmarking

---

**© 2025 Luxi Edge. All rights reserved.**
