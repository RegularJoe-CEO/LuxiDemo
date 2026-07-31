# Version 100 H100 prefill head-to-head - LuxiEdge vs vLLM

**Pack type:** Public evidence pack  
**Date:** 2026-07-31 (UTC)  
**Hardware:** 1x NVIDIA H100 80GB HBM3  
**Model class:** Qwen2-7B-Instruct, FP16  
**Workload:** Prefill-heavy, sequence length 128, batch 16 and 32  
**Measurement:** Sequential arms on the same GPU; NVML GPU-board energy

## Result

On this matched workload, the LuxiEdge energy/throughput path completed more
prompt positions per second and used fewer GPU-board joules per prompt position
than the tested vLLM 0.25.1 stack.

| Batch | Engine | Throughput (positions/s) | Board J/position | Determinism |
|------:|:-------|-------------------------:|-----------------:|------------:|
| 16 | **LuxiEdge** | **41,737.1** | **0.017134** | **1.0** |
| 16 | vLLM 0.25.1 | 35,798.8 | 0.019046 | 1.0 |
| 32 | **LuxiEdge** | **43,934.1** | **0.015757** | **1.0** |
| 32 | vLLM 0.25.1 | 37,104.6 | 0.018249 | 1.0 |

| Batch | LuxiEdge/vLLM throughput | LuxiEdge/vLLM energy |
|------:|--------------------------:|---------------------:|
| 16 | **1.166x** | **0.900x** |
| 32 | **1.184x** | **0.863x** |

Lower energy ratio is better.

## Protocol boundary

- Qwen2-7B-Instruct class, FP16, tensor parallelism 1.
- Sequence length 128; batches 16 and 32.
- One generated token per iteration; only matched prompt positions are counted.
- Prefix caching disabled.
- Two 20-second sustain runs per engine and batch after smoke checks.
- Engines ran sequentially on the same H100, never concurrently.
- LuxiEdge used Flash-class attention, device-resident stack execution, and
  FP16 weight/MLP residency.
- vLLM used version 0.25.1 with Flash attention and greedy sampling.
- Energy is cumulative NVML GPU-board energy divided by counted prompt
  positions.

## Determinism

The reported score of 1.0 is agreement on the measured fixed-input execution
path. It is not a claim that the LuxiEdge energy path is bit-exact to the
separate high-fidelity AUDIT lane.

## Files

| File | Purpose |
|------|---------|
| `README.md` | Human-readable method, results, and limits |
| `RESULTS.json` | Machine-readable protocol, run values, medians, and ratios |
| `SHA256SUMS` | Integrity hashes for the public pack |

## Non-claims

- Not full multi-tenant or OpenAI-compatible serving-stack parity.
- Not continuous batching, decode-only throughput, or latency leadership.
- Not multi-GPU tensor or pipeline parallelism.
- Not facility wall-plug energy, cooling energy, PUE, or carbon accounting.
- Not a claim for every model, batch, sequence length, or serving recipe.
- Not a claim of higher open-chat quality.

This pack publishes bounded measurement data only. Source code and internal
engineering documentation are not part of this public pack.

Contact: Eric Waller - e@ewaller.com
