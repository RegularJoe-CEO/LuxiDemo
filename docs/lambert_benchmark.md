# Lambert's Problem Benchmark

This implementation solves Lambert's problem using Luxi Edge's bisection root-finding capabilities.

## Problem Statement

Given orbital mechanics parameters, find the semi-major axis `a` where the time of flight (TOF) equals 1800 seconds.

### Test Vector
- r₁ = 6980 km (departure radius)
- r₂ = 10520 km (arrival radius)  
- c = 6655 km (chord length)
- s = 12078 km (semi-perimeter)
- μ = 398600 km³/s² (Earth gravitational parameter)
- Target TOF = 1800 seconds
- **Expected result: a ≈ 6066 km**

## Implementation

### 1. Lambert TOF Function (`src/lambert.rs`)

The Time of Flight for elliptical orbits is calculated using Battin's formulation:

```rust
pub fn lambert_tof(a: f64, _r1: f64, _r2: f64, c: f64, s: f64, mu: f64) -> f64 {
    let alpha_sin = (s / (2.0 * a)).sqrt();
    let beta_sin = ((s - c) / (2.0 * a)).sqrt();
    let alpha = 2.0 * alpha_sin.asin();
    let beta = 2.0 * beta_sin.asin();
    let tof = (a.powi(3) / mu).sqrt() * (alpha - alpha.sin() - (beta - beta.sin()));
    tof
}
```

### 2. Rhai Expression Generator

For use with Luxi's `bisect_root`, we generate a Rhai expression that evaluates `TOF(a) - target_tof`:

```rust
pub fn lambert_tof_expression(_r1: f64, _r2: f64, c: f64, s: f64, mu: f64, target_tof: f64) -> String
```

### 3. Bracket Selection

Critical constraint: `a ≥ s/2` to avoid NaN in `asin()`.

For the test vector:
- s/2 = 6039 km (minimum valid a)
- TOF decreases as a increases (for elliptical orbits)
- Bracket [6040, 6100] km provides sign change
- f(6040) ≈ +145 (TOF > 1800)
- f(6100) ≈ -82 (TOF < 1800)

## Test Results

All tests pass with high accuracy:

```
✅ test_lambert_tof_calculation: Direct TOF at a=6066 → 1799.47s ≈ 1800s
✅ test_lambert_tof_direct: Validation passes
✅ test_lambert_rhai_expression: Rhai evaluates correctly
✅ test_lambert_bisect_root: Bisection finds a=6065.5±0.5 km
```

## Benchmark Results

Performance on the test system:

| Benchmark | Time | Description |
|-----------|------|-------------|
| `lambert_tof_direct` | ~56.5 ns | Direct TOF calculation |
| `lambert_bisect_solve` | ~421 µs | Bisection with tol=1e-6 |
| `lambert_bisect_tight_tol` | ~496 µs | Bisection with tol=1e-9 |

### Analysis

- **Direct calculation**: Extremely fast (~56ns) for single evaluations
- **Bisection solving**: ~421 µs for 1e-6 tolerance (sufficient for most applications)
- **Tight tolerance**: ~496 µs for 1e-9 tolerance (minimal overhead for 1000× better precision)
- The bisection leverages Luxi's efficient Rhai-based expression evaluation
- Performance scales well with tolerance requirements

## Usage

### Running Tests

```bash
cargo test --lib lambert
```

### Running Benchmarks

```bash
cargo bench --bench lambert_benchmark
```

## Future Extensions

This implementation demonstrates:
1. ✅ Efficient root-finding for orbital mechanics
2. ✅ Integration with Luxi's Rhai expression engine
3. ✅ Accurate validation against known test vectors

Potential GPU acceleration opportunities:
- Batch solving multiple Lambert problems in parallel
- Vectorized TOF evaluation for trajectory optimization
- Monte Carlo orbit determination scenarios

## Files

- `src/lambert.rs` - Core Lambert TOF functions
- `benches/lambert_benchmark.rs` - Performance benchmarks
- `src/lib.rs` - Integration tests (in `#[cfg(test)]` module)
