# LuxiEdge benchmarks: Qwen2-7B on H100 vs vLLM 0.25.1

Measured 2026-09-25/26 on one NVIDIA H100 SXM · [luxiedge.com](https://luxiedge.com)

## Headline

Luxi runs Qwen2-7B on an H100 with bit-identical results at any batch size. On long prompts (2k to 32k tokens) it is 3–7% faster than vLLM 0.25.1 and uses 2–5% less energy per token. When generating tokens it matches vLLM's speed but uses 2–7% more energy per token. Against vLLM's own deterministic (batch-invariant) mode, Luxi is faster and uses less energy on every test, including 1.5–2.7× faster token generation with 16–50% less energy.

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

## Method

- H100 SXM (RunPod), Qwen2-7B-Instruct, vLLM 0.25.1.
- Engines alternated in matched blocks on the same GPU, 6 runs per cell.
- Energy is from the GPU's NVML total-energy counter, with no idle subtraction.
- CPU pinned with taskset. Run-to-run standard deviation ≤0.7%.
- The one-token last-layer trim optimization is excluded from these numbers.

## Determinism

- Luxi output is bit-exact across batch sizes 1, 16 and 64, across repeats, and across separate processes.
- It matches the Hugging Face reference argmax and top-5.
- vLLM default is repeatable for an identical batch but is not batch-invariant: per-step logprobs for a prompt differed on about 250 of 256 steps when it ran alone versus inside a batch.
- vLLM's batch-invariant mode was identical in all cases tested.

## Why generation uses more energy

To stay batch-invariant, Luxi pins one GEMM configuration per matrix shape at every batch size, while vLLM switches to a low-power small-tile GEMM at batch 1. Luxi's deterministic FA3 decode setting (num_splits=1) takes about 19.2 µs per layer versus vLLM's 13.2 µs. The faster split setting broke batch invariance, so it was rejected.

## What is borrowed and what is Luxi's own

- The attention kernel is FlashAttention-3, taken from vLLM's wheel and called through Luxi's C++ shim. The matrix multiplies use NVIDIA cuBLASLt.
- Luxi's own work: the batch-invariance design, pinned algorithm choices and algorithm cache, the energy-based tuner, fused elementwise kernels, and CUDA-graph decode.
- Luxi's own mma.sync attention kernel is shown in the last column. It is weaker than FA3 at long lengths.

## Run summary

Per-cell means, spread, latency, median board power and SM clock:
[`evidence/h100-qwen2-7b-vs-vllm-0.25.1-2026-09-25/summary.md`](evidence/h100-qwen2-7b-vs-vllm-0.25.1-2026-09-25/summary.md).

Engine labels in that file: `vllm_default` = vLLM 0.25.1 default · `vllm_BI` = vLLM batch-invariant mode · `luxi_c8_FA3` = Luxi (FA3) · `luxi_c8_ownattn` = Luxi own attention kernel.

Scope: single-GPU board energy (not wall-plug), one model, one GPU class. Not a multi-tenant full-server comparison.

Contact: Eric Waller, e@ewaller.com
