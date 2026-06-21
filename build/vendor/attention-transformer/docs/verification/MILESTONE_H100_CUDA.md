# Milestone — H100 CUDA Path Verified (Bit-Exact)

> **Performance history log.** For current locked benchmarks, gates, and env knobs, use [`QUANT_TRADE_LOCKED.md`](../QUANT_TRADE_LOCKED.md).

**Date:** 2026-06-01  
**Hardware:** RunPod H100  
**Repo:** https://github.com/RegularJoe-CEO/attention-transformer

---

## Phase 0 — Correctness baseline (commit 206e637 / bf075f6)

### What was proven
- Full decoder CUDA path is **bit-identical** to the verified Rust CPU reference.
  - CPU receipt:  `0ae659948eabc3fa1212b84d9a2006c707c28ba4209ce28410df676d38d37ada`
  - CUDA receipt: `0ae659948eabc3fa…d37ada` (identical)
- Waller operator: Standard B tolerance (max_abs 5.96e-8, mean_abs 9.63e-9).
- Cross-hardware determinism: Apple Silicon CPU and NVIDIA H100.

### Sustained throughput (cuda_bench) — **unoptimized kernel**
Config: 500 iters, seq_len 1024, hidden 1024, 16 heads.

| Metric | Value |
|--------|-------|
| Median latency | 13.084 ms |
| Throughput | 70.86 iters/sec |
| Approx GFLOP/s | ~304 |

---

## Phase 1 — Persistent buffers (ccae9b8 → beeb597)

- `CudaWallerBuffers`, no per-call `cudaMalloc` in hot path.
- Median **~8.3 ms**, **~518 GFLOP/s**, **~121 iters/sec**.

---

## Phase 2 — hd-specialized register kernel (d5a75c0)

**Commit:** `d5a75c0`  
**Verified on RunPod:** 2026-06-01

### Correctness (unchanged)
- Decoder: **bit-identical** — receipt `0ae659948eabc3fa1212b84d9a2006c707c28ba4209ce28410df676d38d37ada`, max_diff `0.00e0`.
- Waller isolated: Standard B (receipt differs from per-head CPU harness; decoder is the contract).

### Sustained throughput (cuda_bench)
Config: 500 iters, seq_len 1024, hidden 1024, 16 heads (head_dim 64).

| Metric | Phase 0 | Phase 2 |
|--------|---------|---------|
| Median latency | 13.084 ms | **4.164 ms** |
| Mean latency | 14.112 ms | **4.178 ms** |
| Throughput | 70.9 iters/s | **239.3 iters/s** |
| Approx GFLOP/s | ~304 | **~1028** |
| Kernel only (mean) | ~7.5 ms | **3.363 ms** |
| H2D (mean) | — | 0.520 ms |
| D2H (mean) | — | 0.198 ms |
| Power (nvidia-smi median) | — | 115.7 W |
| Useful GFLOP/s per W | — | **~8.89** |

**Speedup vs Phase 0:** ~3.1× median latency, ~3.4× GFLOP/s.  
**Speedup vs Phase 1:** ~2.0× median latency.

### Technique
- C++ template `waller_multihead_hd_t_kernel<HD>`: Q in registers, `__ldg`, `#pragma unroll`, zero `__syncthreads`, serial `d`-order for bit-exact decoder.

---

## Phase 3 — Aggressive waste elimination (c03c5cf, verified RunPod 2026-06-01)

**Backup tag:** `backup/pre-aggressive-cuda-20260602` (revert: see `BACKUP_RESTORE.md`)

Shipped:
- **Occupancy fix:** hd64 launches `64×256` blocks (was `256×64` → only 64 blocks on H100)
- Pinned QKV staging (`cudaMallocHost`) + CUDA stream + async H2D/D2H
- **DEVICE-RESIDENT** bench: one upload, loop kernel+D2H only
- Single `expf` per softmax column (same math, less work)
- Fix: persistent path always D2H (was empty when `timings` was `None`)

### Correctness (c03c5cf)
- Isolated Waller: Standard B PASS (max_abs 5.96e-8, mean_abs 9.63e-9)
- **Full decoder:** bit-identical — receipt `0ae659948eabc3fa1212b84d9a2006c707c28ba4209ce28410df676d38d37ada`, max_diff `0.00e0`

### Sustained throughput (cuda_bench, 500×1024×1024×16)

| Mode | Median | Mean GFLOP/s | Notes |
|------|--------|--------------|-------|
| PERSISTENT (H2D+kernel+D2H) | **4.064 ms** | ~1048 | H2D 0.70 ms, kernel 3.10 ms, D2H 0.20 ms |
| **DEVICE-RESIDENT** | **3.062 ms** | **~1400** | kernel+D2H only after one upload |
| BLOCKING (20 iters) | 13.11 ms | ~244 | malloc per call (not hot path) |

Power median: **115.7 W** → **~9.06 GFLOP/J** (persistent mean).

**Speedup vs Phase 2 median (4.164 ms):** ~1.03× persistent, **~1.36×** device-resident.

## Phase 4 — Quant stack (four CUDA features)

| # | Feature | Enable | Role |
|---|---------|--------|------|
| 1 | Mega-fused layer | `LUXI_CUDA_MEGA=1` | Full layer on GPU (LN1+QKV+multi-head Waller+wo+MLP) |
| 2 | GPU layer composer | `LUXI_CUDA_GPU_LAYER=1` | **CPU QKV** + device Waller + CPU post (receipt-safe). Device LN+QKV: `LUXI_CUDA_GPU_QKV=1` (experimental) |
| 3 | KV cache decode | `forward_cuda_kv_step` | O(seq) incremental quant windows |
| 4 | Lane B INT8 GEMM | `--features cuda-quant` | GPT-2 turbo matmul on GPU |
| — | SMEM-tiled Waller | `LUXI_WALLER_SMEM=1` | Lane A attention tiling |

Demo: `cargo run --release --features "cuda cuda-quant" --example cuda_quant_stack`

## Phase 5 — Split-path layers (TRADE vs AUDIT, 2026-06)

**Commits:** `3db2d56` (CPU QKV default), `6a786cc` (GPU fused MLP post), `78a5a1c` (fix broken fused kernel / fake 0.01 ms bench)

### Lanes

| Lane | Env | Pipeline |
|------|-----|----------|
| AUDIT | `LUXI_RECEIPT_AUDIT=1` | CPU QKV → GPU Waller+wo → CPU MLP |
| TRADE | default | GPU LN1+QKV → parallel waller+wo → batched MLP (quant stack) |

### `cuda_layer_bench` — pre-geodesic (CPU QKV + GPU MLP)

| seq | TOTAL | CPU QKV | GPU attn+wo |
|-----|-------|---------|-------------|
| 128 | 68 ms | 53 ms | 0.7 ms |
| 1024 | 466 ms | 433 ms | 5.3 ms |

### Quant TRADE — LOCKED (2026-06, H100 NVL)

**Canonical:** [`docs/QUANT_TRADE_LOCKED.md`](../QUANT_TRADE_LOCKED.md)

| Benchmark | seq=1024 median |
|-----------|-----------------|
| `cuda_layer_bench` (1 layer) | **~6.8 ms** |
| `cuda_quant_bench` (12 layers) | **~69.5 ms** (~5.8 ms/layer) |
| `cuda_bench` (Waller only) | **~4.1 ms** |

Phase @ seq=1024: H2D 0.18, LN1 0.22, QKV 1.01, waller+wo 3.85, res+LN2 0.22, MLP 0.40, D2H 0.24 ms.

**Do not use:** `LUXI_CUDA_ROW_FUSED=1` (~135 ms), `LUXI_CUDA_FUSED_ROW_MLP=1` (~27 ms MLP). `LUXI_CUDA_CPU_QKV=1` @ 1024: **~560 ms**.

AUDIT receipt: `0ae659948eabc3fa…d37ada`, max_diff `0.00e0`.

### Regression gate

```bash
export CUDA_ARCH=90
bash scripts/runpod_quant_gate.sh
# or:
LUXI_RECEIPT_AUDIT=1 cargo run --release --features cuda --example cuda_verify
cargo run --release --features cuda --example cuda_layer_bench -- 20 1024 1024 16 256
cargo run --release --features cuda --example cuda_quant_bench -- 20 1024 12
cargo run --release --features cuda --example cuda_bench -- 500 1024 1024 16
```

Decoder receipt (AUDIT): `0ae659948eabc3fa1212b84d9a2006c707c28ba4209ce28410df676d38d37ada`

---

## Phase 6 — NPOW scaling witness (2026-06, H100 RunPod)

**Gate:** `LUXI_NPOW_FAST=1 cargo run --release --example npow_scaling_proof` (also in `runpod_quant_gate.sh`).

| Check | Result |
|-------|--------|
| Memory slopes | Waller **1.000**, standard **2.000** |
| WNSM fidelity | **0.00e0**, identical output receipt with NPOW on bus |
| Payload | Witness receipt round-trip (SHA-256 over 12×f32 payload) |

Full timed sweep (no `LUXI_NPOW_FAST`): 512→131072; **341×** memory reduction @ 131k documented in [`SCALING_EVIDENCE.md`](../SCALING_EVIDENCE.md).