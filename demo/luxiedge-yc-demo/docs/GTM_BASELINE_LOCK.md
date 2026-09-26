# LuxiEdge GTM baseline (version-100)

**Status:** Go-to-market reference - claims and serving surface  
**Lane:** TRADE energy/throughput (not AUDIT bit-exact gold)  
**Measured results:** [BENCHMARKS.md](https://github.com/RegularJoe-CEO/LuxiDemo/blob/main/BENCHMARKS.md)  

---

## Product configuration (TRADE energy path)

| Knob | Value |
|------|--------|
| Attention | Flash-class control (bridge) |
| Stack | Device-resident multi-layer (1× host upload / 1× download boundary) |
| Weights | FP16 residency on GPU |
| Model class | Qwen2-7B-Instruct |

**Do not sell:** AUDIT lane as the thr/J path. Keep dual-lane story: **AUDIT = trust**, **TRADE = thr + joules**.

---

## Measured results

### Comparison against vLLM 0.25.1

Luxi runs Qwen2-7B on an H100 with bit-identical results at any batch size. On long prompts (2k to 32k tokens) it is 3–7% faster than vLLM 0.25.1 and uses 2–5% less energy per token. When generating tokens it matches vLLM's speed but uses 2–7% more energy per token. Against vLLM's own deterministic (batch-invariant) mode, Luxi is faster and uses less energy on every test, including 1.5–2.7× faster token generation with 16–50% less energy.

Tables, method and disclosure: [BENCHMARKS.md](https://github.com/RegularJoe-CEO/LuxiDemo/blob/main/BENCHMARKS.md).

---

## GTM claims (allowed)

1. **vs vLLM 0.25.1:** use only the headline wording in [BENCHMARKS.md](https://github.com/RegularJoe-CEO/LuxiDemo/blob/main/BENCHMARKS.md).  
2. **Determinism:** bit-identical results at any batch size, as described in [BENCHMARKS.md](https://github.com/RegularJoe-CEO/LuxiDemo/blob/main/BENCHMARKS.md).

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
| `cuda_qwen7b_trade` sustain | TRADE executor - thr + NVML energy |
| `serve_v05` HTTP | OpenAI-shaped API. `GET /v1/gtm` returns build info, a determinism note and a link to BENCHMARKS.md; completions carry a `token_receipt` and `latency_ms`. **Do not** quote HTTP thr as TRADE thr |
| AUDIT receipts | Compliance / dual-lane story |

Commercial scripts:

- `scripts/gtm_demo_one_shot.sh` - TRADE sustain
- `scripts/gtm_serve_boot.sh` - HTTP serve boot
- `scripts/gtm_pod_commercial.sh` - TRADE + serve smoke on pod

Demo package: [`../README.md`](../README.md).

---

## One-line pitch

> Luxi runs Qwen2-7B on an H100 with bit-identical results at any batch size. On long prompts (2k to 32k tokens) it is 3–7% faster than vLLM 0.25.1 and uses 2–5% less energy per token. When generating tokens it matches vLLM's speed but uses 2–7% more energy per token. Against vLLM's own deterministic (batch-invariant) mode, Luxi is faster and uses less energy on every test, including 1.5–2.7× faster token generation with 16–50% less energy.

---

## Contact

e@ewaller.com
