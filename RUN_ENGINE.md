# How to Run the Luxi / eRock Engine

This document explains how to:
- Open the project on a fresh terminal.
- Run all key CPU benchmarks in one command.
- Run the NVIDIA L4 GPU throughput & energy benchmark in one command.

The canonical project path on this Mac is:

- `~/eRock`

---

## A. Open the Project

1. Open a new Terminal.
2. Go to the repo:

   ```bash
   cd ~/eRock
   ```

3. You should see files like `Cargo.toml`, `src/`, `tools/`, `scripts/`:

   ```bash
   ls
   ```

---

## B. Run All CPU Benchmarks (MacBook Pro, M1 Pro)

**This uses only the Mac CPU. It does not touch RunPod or GPUs.**

1. From the repo root:

   ```bash
   cd ~/eRock
   ./scripts/run_cpu_suite.sh
   ```

2. What this script does:
   - Builds the project in release mode.
   - Runs these Criterion benchmarks in sequence:
     - `evaluate_10k`
     - `evaluate_100k`
     - `bisect_root`
     - `simd_inplace_100k`
   - Saves logs under `benchmark_logs/`:
     - `benchmark_logs/evaluate_10k.log`
     - `benchmark_logs/evaluate_100k.log`
     - `benchmark_logs/bisect_root.log`
     - `benchmark_logs/simd_inplace_100k.log`

3. Expected behavior (on the M1 Pro MacBook):
   - Total runtime: roughly 40–45 minutes.
   - CPU usage: 1–2 cores near 100% during active measurement.
   - Warnings from Criterion are normal; focus on final timing numbers.

---

## C. Run a Single CPU Benchmark

From `~/eRock` you can run individual benchmarks directly:

```bash
cd ~/eRock

# 10k element evaluation
cargo bench evaluate_10k

# 100k element evaluation
cargo bench evaluate_100k

# Root-finding benchmark
cargo bench bisect_root

# SIMD inplace evaluation over 100k elements
cargo bench simd_inplace_100k
```

These commands also run **only on the Mac CPU**.

---

## D. Run the NVIDIA L4 GPU Throughput & Energy Benchmark (RunPod)

This section assumes a RunPod NVIDIA L4 endpoint (e.g. `local_crimson_hamster`) that
clones this GitHub repo inside the container.

### D1. Start the L4 container

1. In RunPod:
   - Go to **Home → Serverless → Endpoints**.
   - Start the endpoint that uses the L4 GPU (24 GB).
2. Open a **terminal / shell** into the running pod.

### D2. Locate the eRock repo inside the container

1. In the container shell, look for the cloned repo:

   ```bash
   ls
   ```

   Common locations are `/workspace/eRock` or `/root/eRock`. Change into the one that
   contains `Cargo.toml`:

   ```bash
   cd /workspace/eRock    # adjust if needed
   ls Cargo.toml
   ```

### D3. Update code and install Python dependencies

```bash
git pull
pip install "cupy-cuda11x" pynvml
```

### D4. Run the L4 GPU benchmark (one command)

```bash
cd /path/to/eRock   # repo root in the container
./scripts/run_l4_gpu_benchmark.sh
```

This script:

- Calls `python tools/gpu_l4_benchmark.py` with a 50M-element workload.
- Measures:
  - Total element-wise operations.
  - Throughput (ops/sec).
  - Energy efficiency (ops/J) via NVML.
- Writes a log to:

```bash
benchmark_logs/gpu_l4_benchmark.log
```

Expected behavior:

- One-time `pip install` may take a couple of minutes the first time.
- Benchmark measurement itself runs for about 20 seconds (controlled by `--duration`).
- At the end it prints:
  - Average GPU power in watts.
  - Total ops.
  - Throughput (ops/s).
  - Energy efficiency (ops/J).

These numbers are what you will show to NVIDIA as the reproducible L4 result.

---

## E. Where to Find Everything

- **Repo root (Mac):** `~/eRock`
- **Backup archive (Mac home dir):** `~/eRock-backup-YYYYMMDD-HHMM.tar.gz`
- **CPU benchmark logs (Mac or container):** `~/eRock/benchmark_logs/*.log`
- **GPU L4 benchmark script:** `~/eRock/tools/gpu_l4_benchmark.py`
- **GPU L4 benchmark logs (L4 container):** `benchmark_logs/gpu_l4_benchmark.log`
