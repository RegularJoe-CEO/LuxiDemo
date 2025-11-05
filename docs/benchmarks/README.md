# Luxi Edge Benchmark Hub

This directory collects every benchmark-facing artifact in one place so you no longer need to chase outdated files in the
repository root.

## Quick Navigation

| Document | Purpose |
|----------|---------|
| [`COMPARATIVE_ANALYSIS.md`](COMPARATIVE_ANALYSIS.md) | Cross-tool study vs. Python/NumPy, SciPy Newton, and tuned C++ |
| [`data_exports/`](data_exports/) | Raw Criterion baselines and HTML reports (`cargo bench -- --save-baseline`) |
| [`../../BENCHMARK_DATA.md`](../../BENCHMARK_DATA.md) | Executive summary of the latest Luxi Edge results |
| [`FINDING_DATA.md`](FINDING_DATA.md) | Step-by-step guide for locating the refreshed benchmark files |
| [`SYNCING_MAIN.md`](SYNCING_MAIN.md) | Checklist for aligning the `main` branch and deleting legacy snapshots |
| [`../../benches/`](../../benches/) | Source code for Criterion harnesses |

## Updating Numbers

1. Run `cargo bench --bench edge_suite` for SIMD runtime metrics.
2. Run `cargo bench --bench my_benchmark` for fallback calculus workloads.
3. Export data if needed with `cargo bench -- --save-baseline current` and store it in `data_exports/`.
4. Update [`../../BENCHMARK_DATA.md`](../../BENCHMARK_DATA.md) with the new summary figures.
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
