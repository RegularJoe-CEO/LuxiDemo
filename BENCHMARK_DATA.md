# Luxi Edge Benchmark Data — 2025‑11‑06

All measurements produced with Criterion.rs (`--sample-size 100`, harness disabled).

## evaluate_10k
- Results: [8.5519 ms, 8.9424 ms, 9.3807 ms]
- Description: Rhai expression `sin(x)*cos(x)` across 10 000 inputs.

## evaluate_100k
- Run 1: [84.657 ms, 89.866 ms, 95.395 ms]
- Run 2: [80.176 ms, 82.312 ms, 84.769 ms] (2–14 % faster after warm-up)
- Insight: Compared to `simd_inplace_100k` (≈1.63 ms), this demonstrates ≈52× turnaround improvement when skipping parsing.

## bisect_root
- Run 1: [231.90 µs, 241.00 µs, 252.16 µs]
- Run 2: [237.43 µs, 243.64 µs, 251.08 µs]
- Note: p = 0.11 (> 0.05) — no statistically significant change between runs.

## simd_inplace_100k
- Run 1: [1.6239 ms, 1.6485 ms, 1.6761 ms]
- Run 2: [1.5969 ms, 1.6337 ms, 1.6841 ms]

## scalar_loop_100k
- Results: [1.6971 ms, 1.7307 ms, 1.7690 ms]
- Note: Dominated by `f64::sin`/`cos`; loop form adds little overhead.

## simd_loop_100k
- Results: [1.6058 ms, 1.6324 ms, 1.6637 ms]

## simd_repro_100k
- Results: [1.6627 ms, 1.7175 ms, 1.7799 ms]

## Criterion Warning Context
- Message: “Unable to complete 100 samples in 5.0 s.”
- Action: Increase measurement window (`--measurement-time 10`) or reduce samples (`--sample-size 60`) only if smoother plots are needed; the warning is benign for our fast functions.

## Usage
- Feed these metrics into the ROI / energy-savings rollups for stakeholders.
- Schedule longer-measurement reruns only upon request.
