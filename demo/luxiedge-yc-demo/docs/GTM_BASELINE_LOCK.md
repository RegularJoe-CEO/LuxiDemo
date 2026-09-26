# LuxiEdge GTM baseline lock (version-100)

**Status:** Internal go-to-market lock - measured H100 prefill executor  
**Lane:** TRADE energy/throughput (not AUDIT bit-exact gold)  
**Date:** 2026-07-31  

This is the configuration to sell and defend until a stronger pack replaces it.

---

## Product configuration (TRADE energy path)

| Knob | Value |
|------|--------|
| Attention | Flash-class control (bridge) |
| Stack | Device-resident multi-layer (1× host upload / 1× download boundary) |
| Weights | FP16 residency on GPU |
| Model class | Qwen2-7B-Instruct |
| Sequence | 128 (prefill positions) |
| Batch | **16** primary · **32** scale |

**Do not sell:** AUDIT lane as the thr/J path. Keep dual-lane story: **AUDIT = trust**, **TRADE = thr + joules**.

---

## Locked metrics (multi-run formal pack - **authoritative**)

**Campaign:** 5 × 15s sustains each · H100 · Flash + device-resident + FP16  
**Source:** multi-run lock on version-100 GTM hour pack  

| Batch | Thr median (pos/s) | Thr min to max | Thr stdev | J/pos median | Det (5-run token match) | Flash |
|------:|-------------------:|------------:|----------:|-------------:|------------------------:|:-----:|
| **16** | **39,865** | 39,504 to 41,050 | ~671 | **0.0168** | **1.0** | yes |
| **32** | **42,967** | 42,635 to 43,570 | ~387 | **0.0160** | **1.0** | yes |

**Primary sell cell: B=16** (tight variance, commercial batch). **B=32** for scale.

### Comparison against vLLM 0.25.1

Luxi runs Qwen2-7B on an H100 with bit-identical results at any batch size. On long prompts (2k to 32k tokens) it is 3–7% faster than vLLM 0.25.1 and uses 2–5% less energy per token. When generating tokens it matches vLLM's speed but uses 2–7% more energy per token. Against vLLM's own deterministic (batch-invariant) mode, Luxi is faster and uses less energy on every test, including 1.5–2.7× faster token generation with 16–50% less energy.

Tables, method and disclosure: [BENCHMARKS.md](https://github.com/RegularJoe-CEO/LuxiDemo/blob/main/BENCHMARKS.md).

---

## GTM claims (allowed)

1. **vs vLLM 0.25.1:** use only the headline wording in [BENCHMARKS.md](https://github.com/RegularJoe-CEO/LuxiDemo/blob/main/BENCHMARKS.md).  
2. **Deterministic dual-run behavior** on the Luxi TRADE path for fixed prompts (token-id agreement).  
3. **Batch scale holds efficiency** (thr rises B1→B32; J/pos falls; det stays 1.0).

## GTM claims (forbidden until more packs)

- Full multi-tenant continuous-batch *server* win vs vLLM OpenAI API  
- Decode-only thr crown  
- Wall-plug / PUE  
- AUDIT bit-exact on GPU TRADE kernels  
- “Always wins every shape”

---

## Serving surface

| Surface | Role for GTM |
|---------|----------------|
| `cuda_qwen7b_trade` sustain | **Money path** - thr + NVML energy |
| `serve_v05` HTTP | OpenAI-shaped API + **`GET /v1/gtm` scoreboard** - **do not** quote HTTP thr as TRADE thr |
| AUDIT receipts | Compliance / dual-lane story |

Commercial scripts:

- `scripts/gtm_demo_one_shot.sh` - TRADE sustain (money path)
- `scripts/gtm_serve_boot.sh` - HTTP + GTM energy mode
- `scripts/gtm_pod_commercial.sh` - TRADE + serve smoke on pod

Serve doc: [`GTM_COMMERCIAL_SERVE.md`](GTM_COMMERCIAL_SERVE.md).

---

## One-line pitch

> Luxi runs Qwen2-7B on an H100 with bit-identical results at any batch size. On long prompts (2k to 32k tokens) it is 3–7% faster than vLLM 0.25.1 and uses 2–5% less energy per token. When generating tokens it matches vLLM's speed but uses 2–7% more energy per token. Against vLLM's own deterministic (batch-invariant) mode, Luxi is faster and uses less energy on every test, including 1.5–2.7× faster token generation with 16–50% less energy.

---

## Contact

e@ewaller.com
