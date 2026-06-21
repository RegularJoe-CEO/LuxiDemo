# Quant TRADE — Current Stack, Locked Numbers & Instructions

**Version:** 2026-06-01 · **Hardware:** NVIDIA H100 NVL, `CUDA_ARCH=90`  
**Config:** `hidden=1024`, `heads=16`, `head_dim=64`, `mlp_dim=256`, `seq=1024`

**Single source of truth** for AUDIT vs TRADE, locked benchmarks, RunPod/NVIDIA gates, and LuxiEdge regression. Do not cite `cuda_bench` (~4 ms) as full-layer latency.

---

## Current stack (what ships)

| Area | Status | Entry |
|------|--------|--------|
| CPU golden path | Production reference | `production_demo`, `WNSM_GAE_Decoder::forward` |
| CUDA **AUDIT** | Bit-exact to CPU on H100 | `cuda_verify` + `LUXI_RECEIPT_AUDIT=1` |
| CUDA **TRADE** | Desk geodesic (~6.8 ms/layer @ 1024) | `forward_cuda` / quant stack |
| **NPOW** | O(N) witness on WNSM bus (certified) | `npow_scaling_proof`, `src/npow/` |
| Receipt backtest | Scaffold | `receipt_backtest`, `src/quant/` |
| FP8 shadow | 0.00e0 + same SHA-256 gate | `fp8_parity_gate`, `src/fp8/` |
| PyTorch FFI | C ABI + Python ops | `src/ffi/`, `integrations/torch/` |
| 70B scale profile | Dims + contract (load TBD) | `src/scaling/` |

---

## 1. Two lanes (never mix receipts)

| Lane | Enable | Path | Receipt |
|------|--------|------|---------|
| **AUDIT** | `LUXI_RECEIPT_AUDIT=1` | CPU QKV + GPU Waller+wo + CPU MLP | `0ae659948eabc3fa1212b84d9a2006c707c28ba4209ce28410df676d38d37ada` (bit-exact CPU) |
| **TRADE** | default (unset) | GPU: LN1 → packed QKV → parallel waller+wo → batched MLP | Deterministic GPU; **separate** namespace |

---

## 2. Locked benchmarks (H100 @ seq=1024)

| Example | Measures | Median | Notes |
|---------|----------|--------|--------|
| `cuda_bench` | Waller only | **~4.1 ms** | Not a full layer |
| `cuda_layer_bench` | One `forward_cuda` layer | **~6.8 ms** | Full TRADE layer |
| `cuda_quant_bench` | 12-layer stack | **~69.5 ms** | One H2D, GPU stack, one D2H |
| Amortized | 69.5 / 12 | **~5.8 ms/layer** | vs ~82 ms if 12× isolated |

### Phase breakdown (`LUXI_CUDA_PHASE_TIMING=1`)

| Phase | ~ms |
|-------|-----|
| H2D | 0.18 · LN1 0.22 · QKV 1.01 · Waller+wo 3.85 · res+LN2 0.22 · MLP 0.40 · D2H 0.24 |
| **TOTAL** | **~6.8** |

---

## 3. Instructions — every machine (CPU)

```bash
cd attention-transformer
cargo build --release
cargo test --release
cargo clippy -- -D warnings
cargo run --release --example production_demo          # sacred: e1980a6f…37628, 0.00e0 WNSM
LUXI_NPOW_FAST=1 cargo run --release --example npow_scaling_proof
cargo run --release --example fp8_parity_gate
cargo run --release --example receipt_backtest
```

Evidence sweeps (optional): `scaling_sweep`, `energy_sweep` → see [`SCALING_EVIDENCE.md`](SCALING_EVIDENCE.md), [`ENERGY_EVIDENCE.md`](ENERGY_EVIDENCE.md).

---

## 4. Instructions — NVIDIA / RunPod

Always work from the repo root (e.g. `/workspace/attention-transformer` on RunPod — not `/`).

```bash
export CUDA_ARCH=90 PATH="/usr/local/cuda/bin:$HOME/.cargo/bin:$PATH"
git pull
cargo build --release --features cuda
```

### First-time AUDIT check (installs Rust if missing)

```bash
chmod +x scripts/runpod_cuda_check.sh
./scripts/runpod_cuda_check.sh
```

Expect: `FULL DECODER CUDA PATH VERIFIED`, `Receipts match: true`, `Max diff: 0.00e0`, receipt `0ae659948eabc3fa…d37ada`.

### Full regression gate (AUDIT + TRADE + NPOW)

```bash
bash scripts/runpod_quant_gate.sh
```

**Pass criteria:**

| Step | Pass |
|------|------|
| `cuda_verify` (AUDIT) | `FULL DECODER CUDA PATH VERIFIED`, `max_diff 0.00e0` |
| `cuda_layer_bench` | median ~4–12 ms |
| `cuda_quant_bench` (12 layers) | median ~55–95 ms, no `WARN:` stack bug |
| `cuda_bench` | DEVICE-RESIDENT present |
| `npow_scaling_proof` (`LUXI_NPOW_FAST=1`) | memory PASS + scaling proof complete |

### Manual TRADE benches

```bash
cargo run --release --features cuda --example cuda_layer_bench -- 20 1024 1024 16 256
cargo run --release --features cuda --example cuda_quant_bench -- 20 1024 12
cargo run --release --features cuda --example cuda_bench -- 50 1024 1024 16
```

---

## 5. `cuda_verify` pass / fail

**Standard A (required for AUDIT):** identical CPU/CUDA receipts, `max_diff 0.00e0`, no NaN/Inf.

**Standard B (isolated Waller only):** max_abs ≤ 1e-4, mean_abs ≤ 1e-5, no NaN/Inf — not the decoder compliance bar.

Hard fail: any NaN/Inf, or AUDIT decoder fails Standard A.

---

## 6. Default TRADE stack

- Packed GPU QKV · parallel waller+wo · batched MLP · shared `cuda_trade_stream`
- Multi-layer: `cuda_use_quant_stack()` when `layers.len() > 1`
- `WNSM_GAE_Decoder::forward()` / `forward_cuda_trade()` use quant stack automatically

---

## 7. Do not use

| Env | Effect |
|-----|--------|
| `LUXI_CUDA_ROW_FUSED=1` | ~105–135 ms/layer @ 1024 |
| `LUXI_CUDA_FUSED_ROW_MLP=1` | ~27 ms MLP only |
| `LUXI_CUDA_CPU_QKV=1` | ~560 ms/layer |

---

## 8. Environment reference

| Env | Effect |
|-----|--------|
| `LUXI_RECEIPT_AUDIT=1` | AUDIT lane |
| `LUXI_CUDA_PHASE_TIMING=1` | Per-phase ms in `cuda_layer_bench` |
| `LUXI_CUDA_QUANT_STACK=0` | Disable persistent multi-layer stack |
| `LUXI_NPOW_FAST=1` | NPOW smoke (gate; no 131k timing) |
| `CUDA_ARCH` | nvcc arch (`90` = H100) |

---

## 9. Related docs

| Doc | Role |
|-----|------|
| [`LUXIEDGE_BUILD_ROADMAP.md`](LUXIEDGE_BUILD_ROADMAP.md) | Pillar delivery order |
| [`AGENT_TECHNOLOGY_BRIEF.md`](AGENT_TECHNOLOGY_BRIEF.md) | Shareable agent context (no repo) |
| [`TECHNICAL_DEEP_DIVE.md`](TECHNICAL_DEEP_DIVE.md) | Math, determinism, architecture |
| [`GEODESIC_SWEEP_DESIGN.md`](GEODESIC_SWEEP_DESIGN.md) | Geodesic TRADE design (shipped + optional mega-kernel) |
| [`verification/MILESTONE_H100_CUDA.md`](verification/MILESTONE_H100_CUDA.md) | CUDA performance history |
| [`verification/SUBSTANTIAL_TEST_REPORT.md`](verification/SUBSTANTIAL_TEST_REPORT.md) | Certified substantial-gate snapshot |
| [`tests/cuda_lanes.md`](../tests/cuda_lanes.md) | Manual CUDA regression checklist |