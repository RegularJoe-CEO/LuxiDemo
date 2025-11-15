# DEMO INSTRUCTIONS – Luxi / eRock Engine

This file is the primary guide for running the engine and benchmarks.
It lives at the top of the repo:

- **Mac path:** `~/eRock/DEMO_INSTRUCTIONS.md`

---

## 1. Open the Project (MacBook Pro)

1. Open a new Terminal.
2. Go to the repo:

   ```bash
   cd ~/eRock
   ls
   ```

   You should see `Cargo.toml`, `src/`, `tools/`, and this file `DEMO_INSTRUCTIONS.md`.

---

## 2. Run CPU Benchmarks on the Mac (M1 Pro)

These commands use **only the Mac CPU**. They do **not** touch RunPod or GPUs.

From `~/eRock`:

```bash
cd ~/eRock

# Build in release mode (do this once before benchmarks)
cargo build --release

# 10k element evaluation
cargo bench evaluate_10k

# 100k element evaluation
cargo bench evaluate_100k

# Root-finding benchmark
cargo bench bisect_root

# SIMD inplace evaluation over 100k elements
cargo bench simd_inplace_100k
```

If you have the helper script `scripts/run_cpu_suite.sh`, you can run them all in one shot:

```bash
cd ~/eRock
./scripts/run_cpu_suite.sh
```

This script (if present) runs the four benchmarks above and writes logs into:

- `benchmark_logs/evaluate_10k.log`
- `benchmark_logs/evaluate_100k.log`
- `benchmark_logs/bisect_root.log`
- `benchmark_logs/simd_inplace_100k.log`

---

## 3. Run the NVIDIA L4 GPU Throughput & Energy Benchmark (RunPod)

This requires a **RunPod L4 endpoint** (e.g., `local_crimson_hamster`) and will incur GPU cost
while the pod is running. It measures throughput (ops/sec) and energy efficiency (ops/J)
using CuPy and NVML [L4 harness lives in `tools/gpu_l4_benchmark.py`].

### 3.1 Start the L4 container

1. In RunPod, go to **Home → Serverless → Endpoints**.
2. Start the endpoint that uses:
   - Container image: `runpod/pytorch:...`
   - GPU: NVIDIA L4 (24 GB).
3. Open a **terminal/shell** into the running pod.

### 3.2 Find the eRock repo inside the container

In the container shell:

```bash
ls
```

Look for a folder containing `Cargo.toml` and `DEMO_INSTRUCTIONS.md`, e.g.:

```bash
cd /workspace/eRock    # adjust if needed
ls Cargo.toml DEMO_INSTRUCTIONS.md
```

### 3.3 Install Python dependencies

```bash
pip install "cupy-cuda11x" pynvml
```

(You usually do this once per container.)

### 3.4 Run the L4 GPU benchmark

From the repo root in the container:

```bash
cd /path/to/eRock

python tools/gpu_l4_benchmark.py \
  --elements 50000000 \
  --duration 20 \
  --dtype fp16 \
  --op fma
```

This will:

- Warm up the GPU.
- Run a 50M-element kernel in a loop for about 20 seconds.
- Sample GPU power via NVML.
- Print:
  - Total element-wise operations.
  - Throughput (ops/sec).
  - Average power (W).
  - Energy efficiency (ops/J).

If you later create `scripts/run_l4_gpu_benchmark.sh`, you can instead run:

```bash
./scripts/run_l4_gpu_benchmark.sh
```

which wraps the command above and saves the output to `benchmark_logs/gpu_l4_benchmark.log`.

---

## 4. Where to Find Things

- **Repo root on this Mac:** `~/eRock`
- **This instructions file:** `~/eRock/DEMO_INSTRUCTIONS.md`
- **GPU benchmark harness:** `~/eRock/tools/gpu_l4_benchmark.py`
- **CPU benchmark logs (if scripts are used):** `~/eRock/benchmark_logs/`
- **GPU L4 benchmark log (if wrapper script is used):** `benchmark_logs/gpu_l4_benchmark.log`
