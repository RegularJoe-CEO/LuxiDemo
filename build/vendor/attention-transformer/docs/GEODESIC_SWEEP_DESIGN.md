# Geodesic Sweep — Design Notes (Shipped + Optional Future)

**Status (2026-06):** Quant TRADE **shipped and locked** at ~**6.8 ms/layer** @ seq=1024 on H100 NVL — a **~68×** improvement over the pre-geodesic ~466 ms path. Batched MLP, device QKV, parallel waller+wo, and the 12-layer quant stack are in production. Locked numbers and gates: [`QUANT_TRADE_LOCKED.md`](QUANT_TRADE_LOCKED.md).

**Constraints (fixed):**
- Same tensors in/out: `input[seq × hidden]` → `output[seq × hidden]`, same weight layout, same `sha256_of_f32_slice` receipt contract for **Lane AUDIT**.
- Fully deterministic: fixed traversal order, no atomics on reductions, Welford LN matches CPU, Waller matches `waller_operator.rs`.

**Original target (~46 ms/layer):** Exceeded. Current TRADE median is ~6.8 ms. Remaining optional work is a single mega-kernel sweep (P3–P4 below), not latency recovery.

---

## 1. Historical diagnosis (pre-geodesic, June 2026)

Before device QKV and batched MLP shipped, one TRADE layer looked like this:

| Phase | ms @ 1024 | Root cause |
|-------|-----------|------------|
| CPU QKV | ~433 | Host GEMM + PCIe + math in wrong place |
| GPU attn+wo | ~5 | Already fast |
| GPU MLP | ~28 | Acceptable |
| **TOTAL** | **~466** | **Data visited the wrong memory hierarchy** |

Attention was never the bottleneck. **Host QKV and phase boundaries** were. The geodesic program moved QKV and orchestration onto the device while keeping AUDIT on the CPU-matched split path.

---

## 2. Shipped today vs optional mega-kernel

**Shipped TRADE path (locked ~6.8 ms/layer):**

```
H2D x → device LN1 → device QKV → parallel waller+wo → batched MLP → D2H y
```

Multiple kernels, one shared stream, persistent weights — proven in `cuda_layer_bench` and `cuda_quant_bench`.

**Optional future (P3–P4):** fuse into one launch per layer:

```
H2D x (once) → geodesic_layer_sweep → D2H y (once)
```

Inside the kernel, causal attention remains a **fixed-order column sweep** (Waller state machine), not an N² orchestration loop.

**Bifurcation (unchanged):**
- `LUXI_RECEIPT_AUDIT=1` → CPU QKV + GPU Waller+wo + CPU MLP (compliance receipt frozen).
- Default TRADE → full device geodesic stack (desk speed).

---

## 3. Mega-kernel design (not yet shipped)

### 3.1 Grid geometry

| CUDA block | Owns | Warps do |
|------------|------|----------|
| `(blockIdx.x = row, blockIdx.y = head)` | One query row, one head | Register-resident Q; stream K/V along causal prefix |

Same geometry as `waller_multihead_hd_t_kernel`, extended through LN + MLP without returning to HBM between sub-phases.

### 3.2 Register pipeline (single-kernel target)

For each `(row, head)` block, in **fixed order**:

```
1. LOAD row x[row,:] → smem (or registers if h≤128)
2. WELFORD LN1 → q̂
3. HADAMARD_BUTTERFLY(q̂) → q̃   [optional; add-only, deterministic]
4. QKV READOUT (single device GEMM or fused proj_dot at hd=64)
5. WALLER SWEEP col = 0..row with fused wo head-slice accumulation
6. BARRIER (block row): all heads → full attn_proj[row,:] in smem
7. RESIDUAL + WELFORD LN2 → MLP expand → GELU → project → residual → output[row,:]
```

### 3.3 Multi-layer persistent decoder

```
d_x = upload(tokens_embedded)   // once
for layer in 0..L:
    geodesic_layer_sweep<<<grid>>>(d_x, d_weights[layer], d_x)
D2H once
```

Host never loops over `seq` for math. Only `L` launches (or one multi-layer kernel later).

---

## 4. Geometric pieces

| Idea | Role | Determinism |
|------|------|-------------|
| **Waller geodesic** | Shortest causal path through keys: one forward sweep per query | Serial `col` order fixed |
| **Walsh–Hadamard** | Orthogonal energy redistribution before QKV; add/sub only | Fixed butterfly stages |
| **Null-space WNSM** | Payload in MLP null space (unchanged) | Already 0.00e0 proven |
| **Online softmax state** | `(m,s,acc)` sufficient statistics | Receipt-locked order |

---

## 5. Receipt strategy (two lanes, one API)

| Lane | Implementation | Receipt |
|------|----------------|---------|
| **AUDIT** | CPU `forward()` / AUDIT CUDA split | `0ae65994…` (frozen) |
| **TRADE** | Device geodesic stack (shipped) | Deterministic GPU; separate namespace |

A future single-kernel `geodesic_layer_sweep` would need its own proof pass (`LUXI_GEODESIC=1` harness) before merging receipts.

---

## 6. Implementation status

| Step | Description | Status |
|------|-------------|--------|
| **P0** | Kill host QKV — device LN1 + `launch_matmul_f32` | ✅ Shipped |
| **P1** | Parallel waller + wo GEMM | ✅ Shipped (~4 ms attention @ 1024) |
| **P2** | Tiled deterministic GEMM (`launch_matmul_f32_geodesic`) | ✅ Shipped |
| **P3** | Single-launch `geodesic_layer_sweep` v1 | Optional |
| **P4** | Multi-layer persistent mega-decoder | Optional |

**Do not use (debug/slow paths):** `LUXI_CUDA_ROW_FUSED=1`, `LUXI_CUDA_CPU_QKV=1` — see [`QUANT_TRADE_LOCKED.md`](QUANT_TRADE_LOCKED.md) §7.

---

## 7. What we explicitly do not do

- No approximate attention, no sampling, no Flash tiling that reorders sums across blocks.
- No `par_iter().sum()` on host for receipt paths.
- No second model, no speculative loop over draft tokens on the hot path.

---

## 8. Regression (must still pass)

```bash
LUXI_RECEIPT_AUDIT=1 cargo run --release --features cuda --example cuda_verify
cargo run --release --features cuda --example cuda_layer_bench -- 20 1024 1024 16 256
# Expect TOTAL median ~6.8 ms on H100 NVL (locked)
```

Full gate: `bash scripts/runpod_quant_gate.sh`

---

*Shipped geodesic TRADE is live at ~6.8 ms/layer. Optional P3–P4 mega-kernel fusion is incremental — bifurcate only for audit.*