# WNSM free-ride under GPU load (2026-07-11)

**Public:** https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/h100-WNSM-free-ride

This pack targets the **differentiated thesis**: null-space payload bus (WNSM) under real CUDA load — not short-seq thr vs Flash.

## Headline

### A) 1-layer TRADE: OFF vs ON free-ride (seq=1024, h=768, pd=64)

| Metric | Value |
|--------|------:|
| OFF median ms (no v_null) | **7.185** |
| ON median ms (v_null + payload) | **7.253** |
| Overhead ON/OFF | **1.010×** (~1.0% tax) |
| Payload inject→extract max abs err | **7.451e-08** |
| Primary out max abs drift ON vs OFF | **1.069e-02** |
| GPU board power during bench (median / max) | **92.7 / 114.2 W** |
| Modeled HBM bytes avoided (1 layer) | **524288** (~1.05e-05 J @ 20 pJ/B) |

**Honest:** Layer-1 ON vs OFF primary-output drift is **not** 0.00e0 in this TRADE bench path (~1e-2). Payload recovery is ~1e-7. GPU is active (~93 W median).

### B) 12-layer stack free-ride vs side-channel (seq=1024, pd=64) — **key result**

| Metric | Value |
|--------|------:|
| OFF stack (no WNSM) median ms | **75.654** |
| ON free-ride stack median ms | **75.138** |
| OFF + separate side H2D each layer | **75.920** |
| **free-ride vs OFF+side** (>1 free-ride wins) | **1.010×** |
| Null residual ‖Wᵀv‖/‖v‖ | **2.503e-08** |
| Payload extract max err (last layer) | **8.121e-07** |
| **Single-layer out drift (null inject)** | **0.000e+00** (**0.000e0**) |
| Modeled bytes avoided (12L) | **6291456** (~1.26e-04 J) |
| Traffic | **h2d=1 d2h=1 d2d_residual=0** |

Free-ride carries a 64-dim payload through **12 layers** with **~1% edge vs paying a side H2D channel**, and **0.000e0** single-layer null-inject output drift when null basis is true.

### C) CPU AUDIT free-ride + NPOW (bit-exact product lane)

| Check | Result |
|-------|--------|
| Speculative WNSM free-ride does not change greedy tokens | **PASS** (`wnsm_free_ride_does_not_change_greedy_tokens`) |
| NPOW on WNSM bus: primary max_diff | **0.00e0** |
| NPOW output receipt match | **true** |
| Mem scaling proof | Waller slope **1.000** (O(N)) vs standard **2.000** (O(N²)); reduction **42.7×** @ 8192 (fast gate) |

See `cpu_free_ride_test.log`, `npow_scaling_proof.log`.

## Exact commands

```bash
export LUXI_KERNEL_MORPH=0
cargo build --release --features cuda \
  --example cuda_wnsm_energy_bench --example cuda_wnsm_stack_bench

python3 scripts/wnsm_under_load_capture.py \
  --iters 40 --seq 1024 --payload-dim 64 --layers 12 \
  --out-dir wnsm_under_load

# AUDIT lane
cargo test --lib --release wnsm_free_ride
LUXI_NPOW_FAST=1 cargo run --release --example npow_scaling_proof
```

## Files

| File | Role |
|------|------|
| WNSM_UNDER_LOAD_SUMMARY.json | Full capture |
| power_trace_wnsm_1layer.csv / power_trace_wnsm_stack12.csv | Board W while CUDA runs |
| wnsm_energy_stdout.log / wnsm_stack_stdout.log | Bench text |
| cpu_free_ride_test.log / npow_scaling_proof.log | AUDIT 0.00e0 + O(N) witness |

## How this maps to the product-class thesis

| Claim | Evidence in this pack |
|-------|------------------------|
| Carry useful payload without side HBM stream | 64-dim payload / layer; modeled bytes avoided |
| Low free-ride tax | ~1.01× ON/OFF (1L); free-ride ≥ OFF+side (12L) |
| Provable non-perturbation | AUDIT 0.00e0 + receipts; stack single-layer inject **0.000e0** |
| GPU under load | Layer1 median ~93 W (not idle 64 W); stack power peaks ~179 W |
| Dual-lane | TRADE free-ride + AUDIT free-ride/NPOW both reported |

## Honest gaps remaining

- Layer-1 TRADE OFF vs ON **full output drift ~1e-2** (not bit-exact across that comparison) — stack inject path reports **0.000e0**.
- Not a vLLM/TRT baseline; not 7B model.
- Modeled HBM joules are **model** (20 pJ/byte), separate from pynvml board W.

Contact: Eric Waller · e@ewaller.com
