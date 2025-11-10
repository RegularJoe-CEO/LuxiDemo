# Luxi Edge Benchmark Documentation

**Central navigation for all performance benchmarks and analysis.**

---

## Quick Links

| Document | Purpose | Last Updated |
|----------|---------|--------------|
| **[GPU_L4_RESULTS.md](GPU_L4_RESULTS.md)** | NVIDIA L4 GPU validation (72.7M ops/sec) | 2025-11-08 |
| **[COMPARATIVE_ANALYSIS.md](COMPARATIVE_ANALYSIS.md)** | Cross-tool comparisons (Python, C++, NumPy) | 2025-11-08 |
| **[../../BENCHMARK_DATA.md](../../BENCHMARK_DATA.md)** | Root-level benchmark summary (CPU + GPU) | 2025-11-08 |

---

## Latest Results (November 10, 2025)

### ARM Neon Energy Efficiency Quantification
- **Platform profiles:** Pre-configured energy models for Pi5, Jetson, Graviton3, Apple M2
- **Theoretical peaks:** 533M ops/J (pessimistic) to 2.67B ops/J (optimistic)
- **Use case:** Battery-powered edge AI, robotics, space-rated computing
- **Documentation:** [../../docs/NEON_ENERGY_PROBABILISTIC_TOF_QUICKSTART.md](../../docs/NEON_ENERGY_PROBABILISTIC_TOF_QUICKSTART.md)

### Probabilistic TOF Analysis
- **Capability:** Monte Carlo uncertainty propagation for orbital mechanics
- **Performance:** 16.3 µs for 8-revolution swarm solve
- **Applications:** SpaceX mission planning, satellite swarms, navigation under uncertainty
- **Documentation:** [../../docs/NEON_ENERGY_PROBABILISTIC_TOF_QUICKSTART.md](../../docs/NEON_ENERGY_PROBABILISTIC_TOF_QUICKSTART.md)

### GPU Acceleration — NVIDIA L4
- **Throughput:** 72,727,273 ops/sec (72.7M)
- **Latency:** 55ms for 4M elements
- **Power:** 16.4W measured via NVML
- **Efficiency:** 4.44M ops/sec/W
- **Speedup:** 377× faster than CPU SIMD

**See:** [GPU_L4_RESULTS.md](GPU_L4_RESULTS.md) for comprehensive analysis

### CPU SIMD Baseline
- **Throughput:** 193,421 ops/sec
- **Power:** 596mW under load
- **Speedup:** 13.7× vs scalar baseline
- **Efficiency:** 18× better energy efficiency

**See:** [../../BENCHMARK_DATA.md](../../BENCHMARK_DATA.md) for detailed CPU metrics

---

## Document Structure

### Primary Benchmark Documents

1. **GPU_L4_RESULTS.md** — Comprehensive GPU Analysis
   - Production validation results (RunPod NVIDIA L4)
   - Performance breakdown (throughput, latency, power)
   - Optimization roadmap (PTX kernels, FP16, multi-GPU)
   - Bottleneck analysis and future targets

2. **COMPARATIVE_ANALYSIS.md** — Cross-Tool Performance
   - Luxi vs Python/NumPy
   - Luxi vs C++ stdlib
   - Luxi vs SciPy Newton solver
   - Methodology and reproducibility

3. **../../BENCHMARK_DATA.md** (Root Level) — Executive Summary
   - Latest CPU and GPU results
   - Performance comparison tables
   - Deployment guidance (when to use CPU vs GPU)
   - Quick reference metrics

### Historical Data

- **data_exports/** — Raw benchmark data exports (CSV, JSON)
- **raw/** — Criterion.rs output and measurement data

---

## Running Benchmarks

### CPU Benchmarks
```bash
# Standard Criterion benchmarks
cargo bench

# Specific benchmark suites
cargo bench --bench edge_suite
cargo bench --bench simd_vs_scalar
cargo bench --bench lambert_benchmark  # Scientific computing (orbital mechanics)
```

### Scientific Computing Benchmarks
```bash
# Lambert's problem (orbital mechanics)
cargo bench --bench lambert_benchmark

# Specific Lambert tests
cargo test --lib lambert
```

### GPU Benchmarks
```bash
# Build GPU-enabled server
cargo build --release --bin l4_benchmark

# Run server
./target/release/l4_benchmark &

# Execute GPU benchmark from Python
python3 gpu_bench.py
```

### Full Methodology
See individual documents for complete reproduction instructions:
- **GPU methodology:** [GPU_L4_RESULTS.md](GPU_L4_RESULTS.md#methodology)
- **CPU methodology:** [../../BENCHMARK_DATA.md](../../BENCHMARK_DATA.md#methodology)
- **Comparative methodology:** [COMPARATIVE_ANALYSIS.md](COMPARATIVE_ANALYSIS.md)

---

## Key Findings

### GPU Validation Success ✅
- **72.7M ops/sec** throughput exceeds 30M SIMD target by 2.4×
- **377× speedup** over CPU SIMD baseline
- **Production-ready** HTTP server deployment validated
- **16.4W power** — GPU at idle-level consumption

### CPU SIMD Performance ✅
- **193k ops/sec** throughput on ARM64/x86_64
- **13.7× speedup** over scalar baseline
- **596mW power** — suitable for battery-powered edge devices
- **18× energy efficiency** improvement

### Scientific Computing Benchmarks ✅
- **Lambert's problem** orbital mechanics solver
- **Sub-millisecond root-finding** (~421 µs for bisection)
- **High accuracy** — result within 0.003% of expected value
- **17.7M direct evaluations/sec** — demonstrating Luxi's computational efficiency

See [../../docs/lambert_benchmark.md](../../docs/lambert_benchmark.md) for details on the orbital mechanics implementation.

### Deployment Guidance
| Factor | Use CPU SIMD | Use GPU |
|--------|--------------|---------|
| Batch Size | <10k elements | >10k elements |
| Latency | <10ms required | 50ms+ acceptable |
| Power Budget | <1W | 10-50W |
| Deployment | Edge/IoT | Data center |

---

## Archive / Deprecated

The following documents have been consolidated:
- ~~`gpu_l4_results.md`~~ → Merged into `GPU_L4_RESULTS.md`
- ~~`gpu_optimizations.md`~~ → Merged into `GPU_L4_RESULTS.md`
- ~~`criterion_simd.md`~~ → Outdated (replaced by latest results)
- ~~`BENCHMARK_DATA.md`~~ → Duplicate (use root-level version)
- ~~`FINDING_DATA.md`~~ → Unnecessary meta-documentation
- ~~`SYNCING_MAIN.md`~~ → Unnecessary meta-documentation
- ~~`repro.md`~~ → Consolidated into individual benchmark docs

---

**Last Updated:** 2025-11-08  
**Maintained By:** Luxi Engineering Team  
**Questions:** See repository README or open an issue

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
