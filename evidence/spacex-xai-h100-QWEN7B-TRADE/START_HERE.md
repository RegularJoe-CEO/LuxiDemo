# Qwen2-7B TRADE — H100 NVL diligence pack

**Hardware:** 1× NVIDIA H100 NVL (RunPod)  
**Model:** `Qwen/Qwen2-7B-Instruct` → LuxiEdge native (**28L × h=3584**)  
**Date:** 2026-07-11  
**Power:** pynvml **board watts, sustain-only** (`SUSTAIN_BEGIN`→`END` @ 50 Hz)  
**Energy:** `J = median_W × sustain.wall_s` · `J/tok = J / (iters × sustain_seq)`  
**Stack path:** **device-resident** (1 H2D / 1 D2H per stack forward)

---

## Product headline (use these)

| Metric | Value | Protocol |
|--------|------:|----------|
| **Prefill thr** | **~403 tok/s** | sustain_seq=**128**, 2×60s, 28L full stack |
| **J/token** | **0.630 ± 0.002** | same; sustain-only NVML |
| **Median board W** | **254.2 ± 1.7** | same |
| **ms / stack iter** | **317.3** | each iter processes 128 tokens |

**Longer context (seq=256, 30s sustain):**  
~**464 tok/s** · **~0.604 J/tok** · median **~280 W**

### Why not lead with ~3.56 J/tok?

That number is from **sustain_seq=5** (3×5 min multirun in this pack root). At seq=5 the loop is **launch-overhead dominated** (~44 tok/s). It is a valid microbench, **not** the product thr/energy story.

| sustain_seq | tok/s (approx) | J/tok (NVML) | Notes |
|------------:|---------------:|-------------:|-------|
| 5 | ~44 | **3.560 ± 0.005** | original 3×300s pack (root `run_1..3`) |
| 32 | ~221 | — | thr sweep |
| 64 | ~359 | — | thr sweep |
| **128** | **~403** | **0.630** | **lead with this** (`thr_sustain_seq128/`) |
| 256 | ~464 | **~0.604** | `thr_sustain_seq256/` |
| 512 | ~493 | — | thr sweep |

Full ladder: `thr_sweep/LADDER.json`.

---

## Claims (defensible)

1. Full **28-layer** Qwen2-7B-scale weights load and run on GPU TRADE residual stack.
2. **Multi-run** sustains with **sustain-only** board power (not load/map idle).
3. At **sustain_seq≥128**, measured **~0.63 J/tok** and **~400+ prefill tok/s** on H100 NVL.
4. WNSM free-ride on principal block of real L0 `down_proj`: null residual ~**1e-8**, ON/OFF drift **0**.

## Non-claims

- Not HF-faithful chat quality (GQA/SwiGLU mapped into TRADE GELU-MHA path).
- Not a claim to beat Flash thr/J/tok at short seq (H2H optional follow-up).
- Power is **GPU board**, not wall-plug AC.
- `ms/iter` is **stack prefill time**, not single-token decode latency.

## Files

| Path | Purpose |
|------|---------|
| `START_HERE.md` | This file |
| `thr_sustain_seq128/` | **Product thr/energy** — 2×60s @ seq=128 |
| `thr_sustain_seq256/` | Longer-ctx thr/energy — 30s @ seq=256 |
| `thr_sweep/LADDER.json` | Seq ladder + framing |
| `AGGREGATE.json` + `run_1..3/` | Original 3×300s @ **seq=5** (microbench) |
| `SHA256SUMS` | Integrity (regenerated) |

## Reproduce

```bash
python3 scripts/convert_hf_to_luxi.py \
  --src /path/to/Qwen2-7B-Instruct \
  --out /path/to/luxi/qwen2-7b-instruct \
  --source-id Qwen/Qwen2-7B-Instruct --link

cargo build --release --features cuda,gpt2 --example cuda_qwen7b_trade
unset LUXI_RECEIPT_AUDIT
export LUXI_TRADE_ENERGY=1 LUXI_CUDA_FP16_MLP=1 LUXI_CUDA_QUANT_STACK=1 LUXI_TRADE_ATTN=waller

# Product thr (default sustain-seq=128)
python3 scripts/qwen7b_trade_multirun_capture.py \
  --model /path/to/luxi/qwen2-7b-instruct \
  --out /tmp/qwen7b_s128 --runs 2 --sustain-sec 60 --sustain-seq 128
```
