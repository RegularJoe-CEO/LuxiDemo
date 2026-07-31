# LuxiEdge vs vLLM — Prefill Throughput & Board Energy

**Document type:** Shareable technical brief  
**Status:** Measured head-to-head (same hardware, same shape)  
**Date:** 2026-07  

---

## Summary

On a matched **prefill-heavy** workload, the LuxiEdge energy/throughput path is both **faster** and **more energy-efficient** (lower board joules per token position) than a current open serving stack (vLLM), while holding **deterministic** dual-run behavior on the Luxi path.

| Question | Answer |
|----------|--------|
| Faster? | **Yes** — about **1.17–1.18×** higher throughput |
| Better or equal energy per token? | **Yes** — about **10–14% lower** board J per position |
| Determinism held (Luxi TRADE path)? | **Yes** — dual-run score **1.0** |

Peer stack used **greedy sampling** (`temperature = 0`), not a separate “deterministic product mode.” Token accounting is matched prefill positions.

---

## Protocol (what was compared)

| Setting | Value |
|---------|--------|
| Model class | Qwen2-7B-Instruct (FP16) |
| Sequence length | 128 |
| Batch sizes | 16 and 32 |
| Workload | Prefill-heavy (one generated token per iteration; positions = iterations × batch × 128) |
| Hardware | Single NVIDIA H100 80GB-class GPU |
| Comparison | Sequential arms (not concurrent on the same GPU) |
| Metrics | Throughput (positions/s), board energy (NVML joules / position), dual-run determinism |

Peer stack: **vLLM** (current release in test), tensor parallel 1, prefix cache off, same token accounting.

Luxi path: production **energy/throughput** configuration (Flash-class attention control + device-resident multi-layer stack + FP16 weight residency). Not the high-fidelity AUDIT receipt lane.

---

## Results

| Batch | System | Throughput (pos/s) | Board J/position | Determinism |
|------:|:-------|-------------------:|-----------------:|------------:|
| 16 | **LuxiEdge** | **~41,700** | **~0.0171** | **1.0** |
| 16 | vLLM | ~35,800 | ~0.0190 | 1.0 |
| 32 | **LuxiEdge** | **~43,900** | **~0.0158** | **1.0** |
| 32 | vLLM | ~37,100 | ~0.0182 | 1.0 |

**Ratios (Luxi / vLLM):**

| Batch | Throughput ratio | Energy ratio (lower is better) |
|------:|-----------------:|-------------------------------:|
| 16 | **1.17×** | **0.90×** |
| 32 | **1.18×** | **0.86×** |

Board power was measured under load via standard GPU power/energy interfaces (NVML). Energy is **board joules**, not facility wall-plug AC.

---

## How to read this

- **Throughput** counts **prompt positions** under a fixed sequence length and batch (prefill-oriented), not chat tokens/s under a full OpenAI-compatible server.  
- **J/position** is board energy per useful position under that same definition.  
- **Determinism** is dual-run agreement on the measured path (same inputs → same reported behavior).  
- This is an **absolute, same-day head-to-head** on one GPU class—not a claim against every serving recipe, every model, or every sequence length.

---

## Why it matters

For operators, the useful question is rarely “who wins a microkernel.” It is:

1. Do we move more useful work per second at commercial batch?  
2. Do we spend fewer joules per unit of that work?  
3. Can we still stand behind reproducible behavior?

On this shape, the answer to all three is **yes** relative to the peer stack tested.

---

## Non-claims

- Not a full multi-tenant serving stack comparison (scheduling, continuous batching product surface, multi-GPU TP/PP).  
- Not wall-plug or PUE.  
- Not bit-exact identity between energy path and high-fidelity audit path (those remain separate product lanes).  
- Not a claim of higher open-chat quality than the peer model implementation.

---

## Contact

**LuxiEdge** · e@ewaller.com  
For deeper diligence packs (methods, multi-run series, broader shapes), contact for a controlled evaluation.
