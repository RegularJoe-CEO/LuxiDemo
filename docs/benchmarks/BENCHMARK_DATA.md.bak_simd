# Luxi Edge Benchmark Summary

**Last updated:** 2025-01-18 (see `git log -1 --stat BENCHMARK_DATA.md`)  \
**Hardware:** GitHub-hosted Ubuntu 22.04 (AMD EPYC 7763 vCPU)  \
**Software:** Rust 1.89.0, Criterion 0.5

This file is the quick reference for the latest Luxi Edge performance numbers. Detailed methodology, competitive analysis,
and raw result exports now live in this directory.

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
