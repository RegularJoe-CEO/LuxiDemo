# LuxiEdge - Go-to-Market Technical One-Pager

**Status:** Historical inference evidence brief, not the current paid product description.  
**Hardware class:** NVIDIA H100  
**Model class:** Qwen2-7B-Instruct  
**Workload:** Prefill (2k to 32k-token prompts) and decode, compared against vLLM 0.25.1  

---

## Headline

On the 2026-09-25 H100 Qwen2-7B-Instruct fp16 generation tests, Luxi matched normal vLLM 0.25.1 speed within 2% and used 2–7% more GPU-board energy per output token. On 2k–32k prompts, Luxi was 3–7% faster and used 2–5% less GPU-board energy per prompt token. Luxi produced bit-identical outputs across the stated tested batch sizes and repeats. Against vLLM batch-invariant mode it was faster in every tested cell, including 1.5–2.7× faster generation with 16–50% less GPU-board energy.

Tables, method and disclosure: [BENCHMARKS.md](https://github.com/RegularJoe-CEO/LuxiDemo/blob/main/BENCHMARKS.md)

---

## Dual product lanes

| Lane | Promise |
|------|---------|
| **TRADE** | Joules + thr (see BENCHMARKS.md) |
| **AUDIT** | Bit-exact / receipt path for trust (separate from thr claims) |

---

## What we are not claiming

- Full multi-tenant OpenAI-server parity bake-off  
- Decode-only crown  
- Facility wall-plug energy  
- Every model / every sequence length  

---

## Contact

**e@ewaller.com**
