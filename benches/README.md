# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0

# Luxi Edge Benchmarks

This directory contains Criterion.rs benchmark harnesses for performance testing.

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark suite
cargo bench --bench edge_suite
cargo bench --bench my_benchmark
cargo bench --bench simd_vs_scalar
```

## Documentation

For comprehensive benchmark documentation, results, and analysis, see:
- **[`../docs/benchmarks/README.md`](../docs/benchmarks/README.md)** - Central benchmark navigation hub
- **[`../docs/benchmarks/BENCHMARK_DATA.md`](../docs/benchmarks/BENCHMARK_DATA.md)** - Latest performance metrics
- **[`../docs/benchmarks/COMPARATIVE_ANALYSIS.md`](../docs/benchmarks/COMPARATIVE_ANALYSIS.md)** - Cross-tool comparisons

## Benchmark Suites

- **edge_suite.rs**: SIMD-accelerated expression evaluation benchmarks
- **my_benchmark.rs**: Fallback calculus workload benchmarks
- **simd_vs_scalar.rs**: Comparative analysis of SIMD vs scalar implementations
