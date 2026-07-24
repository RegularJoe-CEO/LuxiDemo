# Published results

Last updated: 2026-07-23

This page provides a compact index of Luxi measurements. Each result is limited
to the workload and hardware described in its evidence.

## LuxiEdge Version 99

The table below summarizes a technician-operated matched prefill test on one
NVIDIA H100 80GB GPU.

| Engine | Prefill positions/s | GPU-board J/position |
|---|---:|---:|
| LuxiEdge Version 99 | **28,374.7** | **0.018718** |
| vLLM 0.25.1, default | 35,203.1 | 0.019316 |
| vLLM 0.25.1, batch-invariant | 30,914.3 | 0.020604 |

On this test, LuxiEdge reached 80.60% of default vLLM throughput and used 3.10%
less GPU-board energy per position. Against vLLM batch-invariant mode,
LuxiEdge reached 91.78% of its throughput and used 9.15% less GPU-board energy
per position.

Test configuration: Qwen2-7B-Instruct, batch 16, sequence length 128, packed
prefill, prefix caching disabled, cumulative NVML energy, and one NVIDIA H100
80GB GPU.

Evidence:
[`h100-qwen2-7b-v99-matched-prefill-2026-07-23`](evidence/h100-qwen2-7b-v99-matched-prefill-2026-07-23/)

The measurements cover prefill work. They do not measure decode throughput,
complete serving performance, or facility electricity.

## LuxiQuant numerical engine

A December 2025 TestFort evaluation reported:

| Measurement | Result |
|---|---:|
| Aggregate rate across the seven tested functions | 286.94 billion operations/s |
| Peak tested square-root rate | 331.13 billion operations/s |
| Operations reported during the one-hour run | 444.4 trillion |
| Request failures | 0 |
| Matching GPU and CPU output hash | 5 of 5 runs on each path |
| Average GPU power during sustained load | approximately 117.2 W |

[Open the numerical-engine validation report](https://luxiedge.com/luxiedge-validation-report.pdf)

These measurements apply to the numerical engine and its defined test suite.
They are not transformer-inference measurements.

## Historical transformer research

Earlier H100 experiments remain available with their original configurations,
including TRADE, Flash comparisons, WNSM, long-context scaling, and sustained
runtime work. See [`evidence/README.md`](evidence/README.md).
