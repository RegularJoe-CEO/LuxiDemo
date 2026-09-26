# LuxiEdge - Go-to-Market Technical One-Pager

**Product:** Energy-aware, dual-lane AI compute (throughput + board joules + determinism)  
**Hardware class:** NVIDIA H100 80GB  
**Model class:** Qwen2-7B-Instruct (FP16)  
**Workload:** Prefill-heavy · sequence 128 · batch 16 / 32  

---

## Headline

| Metric | Batch 16 | Batch 32 |
|--------|--------:|--------:|
| **Throughput** (prompt positions/s) | **~39.9k** median | **~43.0k** median |
| **Board energy** (J per position) | **~0.0168** | **~0.0160** |
| **Determinism** (5-run token match) | **1.0** | **1.0** |

Measured under multi-run sustain (5×15 s). Flash-class attention control · device-resident stack · FP16 weight residency.

---

## Comparison against vLLM 0.25.1

Luxi runs Qwen2-7B on an H100 with bit-identical results at any batch size. On long prompts (2k to 32k tokens) it is 3–7% faster than vLLM 0.25.1 and uses 2–5% less energy per token. When generating tokens it matches vLLM's speed but uses 2–7% more energy per token. Against vLLM's own deterministic (batch-invariant) mode, Luxi is faster and uses less energy on every test, including 1.5–2.7× faster token generation with 16–50% less energy.

Tables, method and disclosure: [BENCHMARKS.md](https://github.com/RegularJoe-CEO/LuxiDemo/blob/main/BENCHMARKS.md)

---

## Dual product lanes

| Lane | Promise |
|------|---------|
| **TRADE** | Joules + thr (numbers above) |
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
