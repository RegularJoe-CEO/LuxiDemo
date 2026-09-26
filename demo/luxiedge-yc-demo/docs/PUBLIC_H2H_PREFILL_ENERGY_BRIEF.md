# LuxiEdge vs vLLM 0.25.1 - Throughput & Energy on H100

**Document type:** Shareable technical brief  
**Status:** Measured on one NVIDIA H100 SXM, Qwen2-7B-Instruct  
**Date:** 2026-09-25/26  

---

## Summary

Luxi runs Qwen2-7B on an H100 with bit-identical results at any batch size. On long prompts (2k to 32k tokens) it is 3–7% faster than vLLM 0.25.1 and uses 2–5% less energy per token. When generating tokens it matches vLLM's speed but uses 2–7% more energy per token. Against vLLM's own deterministic (batch-invariant) mode, Luxi is faster and uses less energy on every test, including 1.5–2.7× faster token generation with 16–50% less energy.

---

## Prefill (long prompts)

Throughput in prompt tokens/s and energy in joules (J) per prompt token.

| Prompt length × batch | vLLM 0.25.1 default | vLLM batch-invariant mode | Luxi (FA3) | Luxi own attention kernel |
|---|---:|---:|---:|---:|
| 2048 × 16 | 44,680 tok/s, 0.01543 J | 43,190, 0.01601 | 46,180, 0.01515 | 44,270, 0.01579 |
| 8192 × 4 | 41,120, 0.01688 | 40,070, 0.01734 | 42,350, 0.01647 | 36,700, 0.01905 |
| 32767 × 1 | 29,980, 0.02269 | 29,240, 0.02325 | 32,190, 0.02162 | 21,820, 0.03158 |

## Decode (token generation)

1024-token prompt, 256 greedy generated tokens. Throughput in output tokens/s and energy in joules (J) per output token.

| Batch | vLLM default | vLLM batch-invariant mode | Luxi (FA3) | Luxi own attention kernel |
|---:|---:|---:|---:|---:|
| 1 | 165.8, 2.645 | 61.69, 5.626 | 165.5, 2.820 | 162.1, 2.784 |
| 16 | 2078, 0.2558 | 941, 0.4233 | 2111, 0.2672 | 1832, 0.2813 |
| 64 | 4875, 0.1313 | 3217, 0.1596 | 4898, 0.1338 | 3740, 0.1511 |

---

## Method (short)

- H100 SXM (RunPod), Qwen2-7B-Instruct, vLLM 0.25.1.
- Engines alternated in matched blocks on the same GPU, 6 runs per cell.
- Energy is from the GPU's NVML total-energy counter, with no idle subtraction.

Full method, determinism results, why generation uses more energy, and what is
borrowed vs Luxi's own: [BENCHMARKS.md](https://github.com/RegularJoe-CEO/LuxiDemo/blob/main/BENCHMARKS.md)

---

## Non-claims

- Not a full multi-tenant serving stack comparison (scheduling, continuous batching product surface, multi-GPU TP/PP).  
- Not wall-plug or PUE.  
- Not a claim of higher open-chat quality than the peer model implementation.

---

## Contact

**LuxiEdge** · e@ewaller.com · [luxiedge.com](https://luxiedge.com)
