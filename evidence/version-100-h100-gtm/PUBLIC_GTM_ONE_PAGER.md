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

## Head-to-head (matched prefill)

Same GPU day · sequential comparison · same token definition · peer: vLLM (greedy, prefix cache off).

| | Batch 16 | Batch 32 |
|--|--------:|--------:|
| Throughput vs peer | **~1.17× faster** | **~1.18× faster** |
| Board J/pos vs peer | **~10% lower** | **~14% lower** |

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
