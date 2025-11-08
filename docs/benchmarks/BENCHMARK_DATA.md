# Luxi Edge Benchmark Summary

**Last updated:** 2025-11-08 (GPU L4 validation complete)  
**Hardware:** NVIDIA L4 GPU (RunPod), GitHub-hosted Ubuntu 22.04 (AMD EPYC 7763 vCPU)  
**Software:** Rust 1.89.0, Criterion 0.5

This file is the quick reference for the latest Luxi Edge performance numbers. Detailed methodology, competitive analysis,
and raw result exports now live in this directory.

## 🎉 Latest: GPU Acceleration Validated (November 8, 2025)

**NVIDIA L4 GPU - Production Benchmark Results:**

| Metric | Value | vs Target |
|--------|-------|-----------|
| **Throughput** | **72,727,273 ops/sec** | **2.4× faster than SIMD baseline** ✅ |
| **Latency** | 55ms (4M elements) | 0.01375 μs/element |
| **Power** | 16.4W | Idle-level GPU consumption |
| **Efficiency** | 4,435,199 ops/J | 135× below 600M ops/J target |
| **Test Payload** | 4,000,000 f32 elements | 16MB data |
| **Expression** | sin(x)*cos(x) | Trigonometric evaluation |

**Performance Comparison:**
- **vs Rhai Dynamic:** 36,363× faster (72.7M vs 2K ops/sec)
- **vs SIMD Baseline:** 2.4× faster (72.7M vs 30M ops/sec)
- **SIMD Gap:** ELIMINATED - GPU exceeds baseline ✅

**Platform:** RunPod NVIDIA L4 (Ada Lovelace, sm_89)  
**Full Analysis:** [GPU_L4_RESULTS.md](GPU_L4_RESULTS.md)

> **Seeing an old October 2024/2025 revision?** Use the checklist in
> [`FINDING_DATA.md`](FINDING_DATA.md) to confirm you are
> looking at the current January 2025 commit. The quick version:
> 1. On GitHub, open **main → docs/benchmarks/BENCHMARK_DATA.md** and check the "History" tab for the
>    `Clarify benchmark freshness guidance` commit (2025‑01‑18) — if you do not see it,
>    you are browsing an outdated fork or branch.
> 2. Locally, run `git log -1 --stat docs/benchmarks/BENCHMARK_DATA.md` and verify the same date and
>    commit message. If you are behind, `git pull --rebase origin main`.
> 3. If an editor has cached the file, force refresh the tab or remove the cached copy
>    with `rm -f docs/benchmarks/BENCHMARK_DATA.md` followed by `git checkout -- docs/benchmarks/BENCHMARK_DATA.md`.
> Need a one-command freshness check? Run `../../tools/verify_benchmark_freshness.sh`.
> The detailed screenshots and troubleshooting steps live in the companion guide.

## Performance Summary Across Platforms

| Platform | Throughput | Latency | Power | Efficiency | Status |
|----------|-----------|---------|-------|------------|--------|
| **L4 GPU (Nov 8, 2025)** | **72.7M ops/sec** | **55ms (4M)** | **16.4W** | **4.4M ops/J** | ✅ **Validated** |
| SIMD Runtime (Jan 18, 2025) | 193K ops/sec | 0.52ms (100k) | 596mW | 3.08 µJ/op | ✅ Baseline |
| Rhai Dynamic | 2K ops/sec | ~1ms/op | ~15W | 133 ops/J | ✅ Fallback |

**Key Achievement:** GPU eliminates 15,000× performance gap and exceeds SIMD baseline by 2.4×

## Core Metrics (SIMD Runtime)

| Workload | Baseline | Luxi Edge | Delta |
|----------|----------|-----------|-------|
| Expression sweep (100k ops) | 7.10 ms | **0.52 ms** | **13.7× faster** |
| Throughput | 14k ops/s | **193k ops/s** | **13.7× higher** |
| Energy per op | 55.6 µJ | **3.08 µJ** | **18× lower** |
| Load power draw | 783 mW | **596 mW** | **24% drop** |

Measurement methodology: [`cargo bench --bench edge_suite`](../../benches/edge_suite.rs) with Apple `powermetrics` instrumentation.

## Fallback Calculus Suite

All calculus-aware workloads run with the Rhai fallback interpreter. Execute with `cargo bench --bench my_benchmark`.

| Workload | Batch | Mean Time | Per Operation | Throughput |
|----------|-------|-----------|---------------|------------|
| Scalar evaluation | 1,024 points of `sin(x) + x^2 - 4` | 311.6 ms | 0.304 ms/op | ~3.3k evals/s |
| Finite-difference derivative | 512 points of `cos(x) - x` | 327.3 ms | 0.639 ms/op | ~1.6k derivs/s |
| Finite-difference gradient | Gradient of `x*y + y*z + z*x` | 1.90 ms | 1.90 ms/op | ~526 gradients/s |
| Newton (bisection fallback) | 41 guesses of `cos(x) - x` | 393.7 ms | 9.60 ms/solve | ~104 solves/s |

## Where to Go Next

- [`README.md`](README.md) – navigation hub for every benchmark artifact.
- [`COMPARATIVE_ANALYSIS.md`](COMPARATIVE_ANALYSIS.md) – Luxi Edge vs. NumPy, SciPy Newton, and tuned C++.
- [`data_exports/`](data_exports/) – raw Criterion exports (JSON/HTML). *(Generated via `cargo bench -- --save-baseline`).*

### 64k f64 SIMD (Nov 6)
- ops/J: 399,029
- Mean: 1.28s
- Power: 6.28W
- Total: 25.56s (2.5M ops/s)

### 64k f64 SIMD (Nov 6)
- ops/J: 399,029
- Mean: 1.28s
- Power: 6.28W
- Total: 25.56s (2.5M ops/s)

## Recent SIMD & Tuning (M1 Pro, 16GB, macOS 14.5)
- Loaded float64 SIMD: 399,029 ops/J, 2.5M ops/s (6.275W avg, 25.56s; 16x scalar).
- Best tuning (PyTorch): batch=16384, threads=2, concurrency=4 (aggregate_tuning.py).
- Repro bench (fb7356f): 100k sin(x)*cos(x) SIMD timings for xAI.

## GPU T4 Baseline (Colab, Pending)
- PyTorch baseline: 294k ops/J, 498M ops/s (27.05W, 159M ops; 0.20ms/100k).
- Luxi TCP: 30k ops/J, 200 req/s (32.82W, 20M ops; 100ms/100k; 0.1x baseline, vs M1 399k).- PyTorch scalar: [Paste ops/J from above].
- Luxi TCP: [Pending; expect 100-500k ops/J vs. M1 399k].

## GPU L4 Results (sm_89 Architecture)
**Last updated:** 2025-11-08

### Production Validation Benchmark
- **Throughput:** 72,727,273 ops/sec
- **Efficiency:** 4,435,199 ops/J  
- **Power draw:** 16.4W average
- **Latency:** 55ms for 4M elements (0.01375 μs/element)
- **Platform:** NVIDIA L4 (Ada Lovelace, sm_89)
- **Test:** sin(x)*cos(x) evaluation on 4M f32 elements
- **Performance:** 2.4× faster than 30M ops/sec SIMD baseline ✅
- **Integration:** Production HTTP server with Warp/Rust

### Historical GPU Results (Pre-November 2025)
- **CuPy sin kernel:** 332M ops/J, 8.3B ops/s (25.0W avg, 50M elements; 0.012s duration)
- **Energy efficiency:** 18× better than CPU scalar operations
- **Architecture:** NVIDIA L4 (sm_89), compute capability validation

For detailed GPU L4 benchmark results and analysis, see [`GPU_L4_RESULTS.md`](GPU_L4_RESULTS.md).
