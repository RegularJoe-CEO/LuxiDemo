# Luxi Edge Benchmark Hub

This directory collects every benchmark-facing artifact in one place so you no longer need to chase outdated files in the repository root.

## 🎉 Latest: GPU Acceleration Validated (November 8, 2025)

**NVIDIA L4 GPU achieves 72.7M ops/sec - 2.4× faster than SIMD baseline!**

- **Throughput:** 72,727,273 ops/sec
- **Latency:** 55ms for 4M elements
- **Power:** 16.4W
- **vs SIMD Target:** 2.4× FASTER ✅

See [GPU_L4_RESULTS.md](GPU_L4_RESULTS.md) for complete analysis.

## Overview

Luxi Edge benchmarks measure performance across multiple dimensions:
- **GPU Acceleration**: NVIDIA L4 and T4 GPU performance - **72.7M ops/sec validated**
- **SIMD Runtime**: Vectorized expression evaluation using hardware acceleration
- **Fallback Calculus**: Derivative, gradient, and root-finding operations
- **Power Efficiency**: Energy consumption under various workloads (4.4M ops/J on L4)
- **Cross-Platform Comparisons**: Performance against Python/NumPy, SciPy, and optimized C++

For detailed methodology and test environment specifications, see [`BENCHMARK_DATA.md`](BENCHMARK_DATA.md).

## Methodology

Benchmarks are executed using:
- **Criterion.rs** for micro-benchmarking Rust code with statistical rigor
- **Apple powermetrics** (macOS) and custom instrumentation for power measurements
- **Controlled environments** with documented hardware specifications

See [`BENCHMARK_DATA.md`](BENCHMARK_DATA.md) for specific hardware configurations and test parameters.

## Results Summary

### GPU Performance (November 8, 2025)
| Platform | Throughput | Latency (4M) | Power | Efficiency |
|----------|-----------|--------------|-------|------------|
| **L4 GPU** | **72.7M ops/sec** | **55ms** | 16.4W | 4.4M ops/J |
| SIMD Target | 30M ops/sec | 133ms | ~50W | 600K ops/J |
| Rhai Dynamic | 2K ops/sec | 2000s | ~15W | 133 ops/J |

**Achievement:** GPU eliminates 15,000× performance gap and exceeds SIMD baseline by 2.4×

### CPU SIMD Performance (November 6, 2025)
Latest CPU performance metrics are maintained in [`BENCHMARK_DATA.md`](BENCHMARK_DATA.md). Key highlights:
- 13.7× faster than baseline for SIMD operations
- 18× lower energy per operation
- 52× speedup when bypassing parsing (80ms → 1.6ms)
- Detailed comparative analysis available in [`COMPARATIVE_ANALYSIS.md`](COMPARATIVE_ANALYSIS.md)

## Data Inventory

All benchmark data and artifacts are organized within this directory:
- **BENCHMARK_DATA.md**: Executive summary and latest performance numbers
- **data_exports/**: Raw Criterion JSON/HTML exports
- **raw/**: Python benchmark client scripts migrated from root `benchmarks/` directory
- **../../benches/**: Criterion benchmark source code (standard Rust location)

## Quick Navigation

| Document | Purpose |
|----------|---------|
| [`GPU_L4_RESULTS.md`](GPU_L4_RESULTS.md) | **NEW:** NVIDIA L4 GPU benchmark results - 72.7M ops/sec validated |
| [`BENCHMARK_DATA.md`](BENCHMARK_DATA.md) | Executive summary of CPU SIMD and latest Luxi Edge results |
| [`COMPARATIVE_ANALYSIS.md`](COMPARATIVE_ANALYSIS.md) | Cross-tool study vs. Python/NumPy, SciPy Newton, and tuned C++ |
| [`gpu_l4_results.md`](gpu_l4_results.md) | Legacy GPU benchmark data (pre-November 2025) |
| [`data_exports/`](data_exports/) | Raw Criterion baselines and HTML reports (`cargo bench -- --save-baseline`) |
| [`raw/`](raw/) | Python benchmark scripts and client utilities |
| [`FINDING_DATA.md`](FINDING_DATA.md) | Step-by-step guide for locating the refreshed benchmark files |
| [`SYNCING_MAIN.md`](SYNCING_MAIN.md) | Checklist for aligning the `main` branch and deleting legacy snapshots |
| [`../../benches/`](../../benches/) | Source code for Criterion harnesses |

## Updating Numbers

### GPU Benchmarks
1. Deploy to RunPod or GPU instance (see [`../../RUNPOD_INSTRUCTIONS.txt`](../../RUNPOD_INSTRUCTIONS.txt))
2. Build: `cargo build --release --bin l4_benchmark`
3. Run server: `./target/release/l4_benchmark &`
4. Execute benchmark: `python3 gpu_bench.py`
5. Update [`GPU_L4_RESULTS.md`](GPU_L4_RESULTS.md) with new results

### CPU Benchmarks
1. Run `cargo bench --bench edge_suite` for SIMD runtime metrics.
2. Run `cargo bench --bench my_benchmark` for fallback calculus workloads.
3. Export data if needed with `cargo bench -- --save-baseline current` and store it in `data_exports/`.
4. Update [`BENCHMARK_DATA.md`](BENCHMARK_DATA.md) with the new summary figures.
5. Document cross-tool comparisons in [`COMPARATIVE_ANALYSIS.md`](COMPARATIVE_ANALYSIS.md).

## Verifying Files Exist

From the repository root:

```bash
ls docs/benchmarks
ls docs/benchmarks/data_exports
```

Both commands should list the files above, confirming that the cleaned benchmark layout is present.

### Still seeing legacy October artifacts?

1. Walk through [`SYNCING_MAIN.md`](SYNCING_MAIN.md) to guarantee your `main`
   branch matches the January 2025 commits and remove archived files.
2. Run the helper script from the repository root:
   ```bash
   ./tools/verify_benchmark_freshness.sh
   ```
   The script prints the last `git log` entry for `BENCHMARK_DATA.md` and warns if
   extra benchmark summaries are lingering in the tree.
3. Need screenshots and browser-specific cache tips? See
   [`FINDING_DATA.md`](FINDING_DATA.md).
