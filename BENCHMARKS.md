# LuxiEdge benchmarks: Qwen2-7B on H100 vs vLLM 0.25.1 and SGLang 0.5.19

Measured 2026-09-25/26 on NVIDIA H100 SXM GPUs (the vLLM and SGLang runs used separate pods) · [luxiedge.com](https://luxiedge.com)

## Headline

Luxi runs Qwen2-7B on an H100 with bit-identical results at any batch size. On long prompts (2k to 32k tokens) it is 3–7% faster than vLLM 0.25.1 and uses 2–5% less energy per token. When generating tokens it matches vLLM's speed but uses 2–7% more energy per token. Against vLLM's own deterministic (batch-invariant) mode, Luxi is faster and uses less energy on every test, including 1.5–2.7× faster token generation with 16–50% less energy. It also beats SGLang's deterministic mode on every test, at 1.18–1.24× faster on 2k–32k prompts with 14–19% less energy per token, and 1.21–2.86× faster generation.

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
- CPU pinned with taskset. Run-to-run standard deviation under 1%.
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

## SGLang 0.5.19 (deterministic and normal mode)

Measured 2026-09-26 on a second H100 SXM pod (RunPod, US-NE-1), not the pod used for the vLLM tables above. Numbers are medians of 6 runs.

Against SGLang's deterministic mode, Luxi is faster and uses less energy on every test:

- SGLang deterministic, bf16: on 2k to 32k prompts Luxi is 1.18–1.20× faster with 14–17% less energy per token. Generation is 1.21–1.26× faster with 10–13% less energy per token.
- SGLang deterministic, fp16: on 2k to 32k prompts Luxi is 1.21–1.24× faster with 17–19% less energy per token. Generation is 1.72–2.86× faster with 24–51% less energy per token.
- On 1k-token prompts Luxi is 1.20–1.27× faster than the bf16 mode and 1.23–1.68× faster than the fp16 mode.

Against normal SGLang (not batch-invariant):

- On prompts of 1k to 32k tokens Luxi is 0.3–8.5% faster and uses 0.5–7% less energy per token (2k to 32k: 1.3–4.6% faster, 1.0–4.1% less energy).
- When generating tokens the speed is about equal (Luxi is 1.5% slower at batch 1 and 2.3–4.3% faster at batch 16 and 64) and Luxi uses 1–7% more energy per token.

### Prefill (prompts)

Throughput in prompt tokens/s and energy in joules (J) per prompt token.

| Prompt length × batch | SGLang deterministic (fp16) | SGLang deterministic (bf16) | SGLang normal (fp16) | Luxi (FA3) |
|---|---:|---:|---:|---:|
| 2048 × 16 | 36,580 tok/s, 0.01902 J | 37,690, 0.01846 | 44,760, 0.01555 | 45,320, 0.01540 |
| 8192 × 4 | 33,900, 0.02053 | 34,950, 0.01990 | 41,040, 0.01701 | 41,700, 0.01671 |
| 32767 × 1 | 26,220, 0.02652 | 26,930, 0.02573 | 30,370, 0.02294 | 31,780, 0.02201 |
| 1024 × 1 | 22,430, 0.02694 | 29,630, 0.02304 | 34,790, 0.01977 | 37,740, 0.01839 |
| 1024 × 16 | 36,370, 0.01916 | 37,840, 0.01836 | 45,110, 0.01549 | 45,260, 0.01541 |
| 1024 × 64 | 37,450, 0.01859 | 38,250, 0.01818 | 45,490, 0.01530 | 45,930, 0.01520 |

### Decode (token generation)

1024-token prompt, 256 greedy generated tokens. Throughput in output tokens/s and energy in joules (J) per output token.

| Batch | SGLang deterministic (fp16) | SGLang deterministic (bf16) | SGLang normal (fp16) | Luxi (FA3) |
|---:|---:|---:|---:|---:|
| 1 | 58.84, 5.920 | 135.7, 3.266 | 171.0, 2.700 | 168.4, 2.894 |
| 16 | 863.6, 0.4593 | 1695, 0.3136 | 2043, 0.2642 | 2130, 0.2739 |
| 64 | 2843, 0.1800 | 4050, 0.1510 | 4777, 0.1351 | 4888, 0.1366 |

### Method

- H100 SXM 80GB (RunPod, US-NE-1), driver 570.195.03 (CUDA 12.8), power limit 700 W. Qwen2-7B-Instruct.
- SGLang 0.5.19, the last SGLang release built for CUDA 12 (the pod driver was CUDA 12.8; SGLang 0.5.20 needs CUDA 13). Installed from the official CUDA 12.9 wheels (torch 2.13.0+cu129), with a few dependency versions pinned to their CUDA 12.9 builds.
- All SGLang runs used the fa3 attention backend with the prefix (radix) cache off, matching the vLLM runs. Deterministic mode is SGLang's own `enable_deterministic_inference` setting.
- In deterministic mode SGLang swaps in batch-invariant matrix multiply kernels: a Triton kernel for fp16 and DeepGEMM for bf16. That is why its speed depends on the number format. Luxi runs in fp16 only, so SGLang's deterministic mode was measured in both fp16 (the same format as Luxi) and bf16.
- DeepGEMM compiles its kernels at run time. The pod image's CUDA 12.4 compiler failed on it, so the bf16 runs used the CUDA 12.9 toolkit for that compile. The fp16 runs did not need it.
- SGLang's context limit was raised to 32,800 tokens so it would accept the 32,767-token prompt (SGLang caps input at 5 tokens below the limit). Token positions stay within the model's 32,768.
- SGLang's kernels, scheduler and model code were not modified. One line in its tokenizer manager got a missing None check so the correctness check could ask for top-5 logprobs (`setup/sglang_logprob_guard.patch` in the evidence folder). The speed runs do not ask for logprobs, so that line is not on their path.
- Luxi build: commit c112ef6 plus patches 0001–0007 (config c8), FA3 attention, fp16.
- Luxi and SGLang deterministic mode (fp16 and bf16) alternated in matched blocks on the same GPU: fp16, Luxi, bf16, fp16, Luxi, bf16. Each block ran 3 runs of 12 seconds per test, so each engine has 6 runs per test.
- Normal SGLang ran right after, in a second set of matched blocks alternating with vLLM 0.25.1 on the same GPU and with the same method. So Luxi vs normal SGLang comes from back-to-back rounds, not from interleaved blocks.
- Energy is from the GPU's NVML total-energy counter over each run, with no idle subtraction, the same method as the vLLM runs. CPU pinned with taskset.
- Run-to-run standard deviation was under 1% of the mean in most tests. The largest was 2.1% (Luxi generation at batch 1).

### Determinism and correctness

- Luxi passed its determinism and correctness gate on this pod: for every test (2k, 8k and 32k prompts, a 1k prompt with 256 generated tokens at batch 1, 16 and 64, and five short prompts) it gave one logits hash across batch sizes and repeats. Argmax and top-5 match the Hugging Face fp16 reference on all five short prompts. The hashes were bit-identical to the 2026-09-26 final gate of the same build.
- SGLang deterministic mode passed determinism in both fp16 and bf16: results were identical alone versus inside a batch and from run to run, for 2k, 8k and 32k prompts and for 256 generated tokens at batch 16 and 64 and with staggered arrivals. Argmax and top-5 matched Hugging Face on all five short prompts.
- Normal SGLang is not batch-invariant. It is repeatable for an identical batch, but per-step top-5 logprobs for a prompt differed on 250–254 of 256 steps when it ran inside a batch of 16 or 64 versus alone, and on one of the five short prompts the greedy output changed at token 16 inside a batch of 16.

### Run summary and raw files

Per-test medians with latency, median board power and SM clock:
[`evidence/h100-qwen2-7b-vs-sglang-0.5.19-2026-09-26/summary.md`](evidence/h100-qwen2-7b-vs-sglang-0.5.19-2026-09-26/summary.md). Gate outputs, setup logs, block logs and the runbook are in the same folder.

Engine labels in that file: `luxi_c8` = Luxi (FA3) · `sglang_det_fp16` = SGLang deterministic (fp16) · `sglang_det_bf16` = SGLang deterministic (bf16) · `sglang_normal_fp16` = SGLang normal (fp16) · `vllm_0.25.1_default` = vLLM 0.25.1 default, run on this pod in the same round as normal SGLang.

Scope: single-GPU board energy (not wall-plug), one model, one GPU class. Not a multi-tenant full-server comparison.

Contact: Eric Waller, e@ewaller.com
