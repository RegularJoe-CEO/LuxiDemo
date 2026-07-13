# 12-layer stack H2H (TRADE vs PyTorch+Flash)

**Public:** https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/h100-stack12-H2H  
**Date:** 2026-07-11 · **GPU:** NVIDIA H100 NVL · **Method:** pynvml board power, J = median_W × wall_s

## Shape (identical both sides)

**12 layers · seq=1024 · h=768 · heads=12 · mlp=3072** · ~10 s sustain each

| Side | Implementation |
|------|----------------|
| **TRADE** | `cuda_stack_device_resident_audit` — device-resident residual stack (CUDA) |
| **Baseline** | PyTorch 12× pre-norm LN+QKV+**Flash SDPA** (causal)+GELU MLP **fp16** |

## Head-to-head results

| Metric | TRADE 12L stack | PyTorch+Flash 12L | Ratio TRADE/PT |
|--------|----------------:|------------------:|---------------:|
| Median board W | **177.2** | **176.4** | ~1.0× (both loaded) |
| ms / full stack forward | **74.32** | **3.90** | **19.1× slower** |
| Prefill tokens / s | **13778** | **262859** | PT **19.1×** higher thr |
| **J / token (board)** | **0.0129** | **0.0007** | **19.2×** (TRADE higher) |

**Bottom line:** On this multi-layer prefill workload, **highly optimized PyTorch FP16 + FlashAttention beats the current TRADE stack** on both throughput and board J/token. GPU power is similar (~176–177 W) — TRADE loses on **work per joule / wall time**, not on “GPU idle.”

We publish this **without spin**. It is the multi-layer baseline comparison requested after the single-layer microbench.

## Context vs prior packs

| Pack | Finding |
|------|---------|
| TRADE-only 12L energy | Absolute J/tok ~0.013 close to city-block claim, GPU ~177 W |
| Single-layer microbench | PT wins ~9× J/tok |
| **This 12L H2H** | **PT+Flash wins ~19× J/tok and ~19× thr** |

Absolute TRADE energy is **internally consistent** with earlier claims; it is **not competitive** with production PyTorch/Flash on the same shape today.

## Exact commands

```bash
export LUXI_KERNEL_MORPH=0 LUXI_CUDA_GRAPH=1 LUXI_CUDA_QUANT_STACK=1
python3 scripts/stack12_h2h_baseline.py --sec 10 --out-dir stack12_h2h
```

## Files

- `STACK12_H2H_SUMMARY.json` — full numbers
- `power_trace_trade_12L.csv` / `power_trace_pytorch_12L.csv`
- `dmon.csv` — SM util / power @ 1 Hz
- `run.log`

## Honest scope

- Not full HF GPT-2 124M weights; GPT-2-**small width** residual stack.
- Not vLLM/TensorRT-LLM (next bar if needed); PT+Flash is the strong standard stack baseline.
- Board power only (not AC wall plug).
- Morph/mesh long-seq wedges (prior pack) are a **different regime** and not re-run here.

Contact: Eric Waller · e@ewaller.com
