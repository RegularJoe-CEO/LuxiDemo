<!-- SPDX-FileCopyrightText: 2025 Eric Waller -->
<!-- SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0 -->

# Luxi Edge Benchmarks

This directory contains Criterion.rs benchmark harnesses for performance testing of CPU SIMD and GPU acceleration.

## Latest Benchmark Results (November 8, 2025)

- **CPU SIMD:** 193,421 ops/sec @ 596mW
- **GPU (NVIDIA L4):** 72,727,273 ops/sec @ 16.4W
- **GPU Speedup:** 377× faster than CPU SIMD

See [../docs/benchmarks/GPU_L4_RESULTS.md](../docs/benchmarks/GPU_L4_RESULTS.md) for comprehensive GPU analysis.

## Running Benchmarks

**CPU Benchmarks (default):**
```bash
# Run all CPU benchmarks
cargo bench

# Run specific benchmark suite
cargo bench --bench edge_suite
cargo bench --bench my_benchmark
cargo bench --bench simd_vs_scalar
cargo bench --bench lambert_benchmark  # Orbital mechanics root-finding
cargo bench --bench neon_benchmark     # ARM Neon SIMD intrinsics
cargo bench --bench gpu_optimizations  # CPU-only batch tests
```

**GPU Benchmarks (requires CUDA):**
```bash
# Build with GPU support
cargo bench --bench gpu_optimizations --features gpu

# Or with Vulkan support
cargo bench --bench gpu_optimizations --features vulkan
```

**GPU Production Benchmark (RunPod/NVIDIA L4):**
```bash
# Build the L4 benchmark server
cargo build --release --bin l4_benchmark

# Run the server
./target/release/l4_benchmark &

# Execute benchmark from Python client
python3 gpu_bench.py
```

## Documentation

For comprehensive benchmark documentation, results, and analysis, see:
- **[`../docs/benchmarks/README.md`](../docs/benchmarks/README.md)** - Central benchmark navigation hub
- **[`../docs/benchmarks/BENCHMARK_DATA.md`](../docs/benchmarks/BENCHMARK_DATA.md)** - Latest performance metrics (CPU + GPU)
- **[`../docs/benchmarks/GPU_L4_RESULTS.md`](../docs/benchmarks/GPU_L4_RESULTS.md)** - GPU validation and analysis
- **[`../docs/benchmarks/COMPARATIVE_ANALYSIS.md`](../docs/benchmarks/COMPARATIVE_ANALYSIS.md)** - Cross-tool comparisons

## Benchmark Suites

**CPU-Focused:**
- **edge_suite.rs**: SIMD-accelerated expression evaluation benchmarks
- **my_benchmark.rs**: Fallback calculus workload benchmarks
- **simd_vs_scalar.rs**: Comparative analysis of SIMD vs scalar implementations
- **lambert_benchmark.rs**: Lambert's problem orbital mechanics benchmark (bisection root-finding)
- **neon_benchmark.rs**: ARM Neon SIMD intrinsics benchmarks (ARM64 optimization)

**GPU-Focused:**
- **gpu_optimizations.rs**: Batch evaluation, FP16 GPU kernels, Vulkan fallback
- **../src/bin/l4_benchmark.rs**: Production GPU server for RunPod deployment

**ARM64 SIMD:**
- **neon_benchmark.rs**: ARM Neon intrinsics performance testing
  - Polynomial evaluation: vectorized arithmetic operations
  - FMA operations: fused multiply-add performance
  - Memory bandwidth: vector load/store performance
  - Platform support: ARM64 (Neon), x86_64 (scalar fallback)
  - See [README_NEON.md](README_NEON.md) for detailed documentation

**Scientific Computing:**
- **lambert_benchmark.rs**: Demonstrates root-finding for orbital mechanics (TOF calculations)
  - Direct Lambert TOF: ~56.5 ns
  - Bisection solve (tol=1e-6): ~421 µs
  - Bisection solve (tol=1e-9): ~496 µs
