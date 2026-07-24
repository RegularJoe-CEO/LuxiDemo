# Version 99 matched-prefill measurement on H100

**Date:** 2026-07-23  
**Operator:** TESTfort technician on a client-staged H100 environment  
**Source pin:** `d9ad7e06ff727440a1237e9fbb709d8bad0ba452`  
**Status:** technician execution and attestation present; formal signed narrative pending

## Workload

| Field | Value |
|---|---|
| Model | Qwen2-7B-Instruct staged local copy |
| Hardware | 1× NVIDIA H100 80GB HBM3 |
| Batch | 16 |
| Sequence length | 128 |
| Work unit | completed iterations × B × S **prefill positions** |
| vLLM generation setting | `max_new_tokens=1`; generated token not counted |
| Prefix caching | disabled; unique prompts |
| Energy | cumulative NVML GPU-board energy over the hot window |
| Primary energy calculation | `energy_J_hot / prefill positions` |

## Median results

| Configuration | Positions/s | Board J/position |
|---|---:|---:|
| **LuxiEdge Version 99** | **28,374.7** | **0.018718** |
| vLLM 0.25.1 default | 35,203.1 | 0.019316 |
| vLLM 0.25.1 batch-invariant | 30,914.3 | 0.020604 |

## Derived comparisons

| Comparison | Throughput | Energy |
|---|---:|---:|
| LuxiEdge versus default vLLM | **80.60%** of vLLM | **3.10% lower** board J/position |
| LuxiEdge versus batch-invariant vLLM | **91.78%** of vLLM | **9.15% lower** board J/position |
| vLLM batch-invariant versus default | **12.18% lower** | **6.67% higher** J/position |

LuxiEdge did not win throughput.

## Soak

One 600-second run per engine remained within approximately 1% of the 60-second
medians for throughput and board J/position.

## Receipt results

- Luxi Flash receipt: recorded repeat hashes stable.
- Luxi versus serial oracle: maximum absolute difference approximately
  `2.8e-5`; explicitly not bit-exact.
- Measured cross-sequence contamination: zero in the receipt.
- Flash fallback count: zero in the measured Luxi runs.
- Luxi generation-repeat check: stable first token across three recorded runs.
- vLLM batch-invariant receipt: repeat, composition, and order checks passed in
  the recorded functional arm.

## Run order and limitations

The retained third-party tree shows default vLLM first, then LuxiEdge. It does
not contain a separate reverse-order T1 arm.

This pack does not establish:

- decode or full-serving throughput
- TTFT or ITL
- multi-GPU behavior
- training performance
- other models, batches, or sequence lengths
- facility or wall-plug energy
- universal cross-hardware bitwise determinism

## Anomaly

The pod-built binary SHA-256 differed from two pre-published reference digests.
The technician recorded the exact source pin, clean detached working tree,
successful build, active Flash backend, and zero fallback. See
[`ANOMALY_LOG.txt`](ANOMALY_LOG.txt).

## Files

| File | Purpose |
|---|---|
| [`T5_PER_RUN.csv`](T5_PER_RUN.csv) | Claim-bearing per-run throughput and energy table |
| [`T5_ARITHMETIC_AUDIT.txt`](T5_ARITHMETIC_AUDIT.txt) | Independent arithmetic recomputation and gates |
| [`ATTESTATION.txt`](ATTESTATION.txt) | Technician statement on source/packages/run inclusion |
| [`ANOMALY_LOG.txt`](ANOMALY_LOG.txt) | Recorded deviations |
| [`RESULTS.json`](RESULTS.json) | Machine-readable medians and ratios |
| [`receipts/`](receipts/) | Selected determinism and stability receipts |
| [`SHA256SUMS`](SHA256SUMS) | Integrity manifest |

The public selection intentionally excludes infrastructure IP addresses, SSH
ports/session data, credentials, pod identifiers, private source, and model
weights.
