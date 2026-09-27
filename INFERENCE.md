# LuxiEdge inference: Qwen2-7B on H100

**LuxiEdge keeps Qwen2-7B outputs bit-identical across every batch size we tested on an H100, runs at normal serving speed, and outperforms the deterministic modes of vLLM and SGLang.**

Identical outputs usually cost speed: vLLM's batch-invariant mode and SGLang's deterministic mode both run slower than their normal modes. LuxiEdge gives identical outputs at about the speed of normal vLLM 0.25.1 and SGLang 0.5.19.

"Every batch size we tested" means batch 1, 16 and 64 when generating tokens, and batch 1 against 16, 4 and 2 on 2k, 8k and 32k-token prompts, across repeated runs and separate processes. Full scope: [`TEST_SCOPE.md`](TEST_SCOPE.md).

## Results

Each ratio compares Luxi with one engine on the same GPU, in alternating blocks with 6 runs per engine per test. The vLLM and SGLang comparisons come from two different H100 pods, and each row uses only the Luxi runs from its own pod.

| Luxi compared with | Long prompts (2k–32k tokens) | Token generation (1k-token prompt, 256 tokens, batch 1, 16 and 64) |
|---|---|---|
| **Normal modes (not batch-invariant)** | | |
| vLLM 0.25.1 | 3–7% faster, 2–5% less energy per token | Same speed (within 2%), 2–7% more energy per token |
| SGLang 0.5.19 (fp16) | 1.3–4.6% faster, 1.0–4.1% less energy per token | From 1.5% slower (batch 1) to 4.3% faster, 1–7% more energy per token |
| **Deterministic modes** | | |
| vLLM 0.25.1 batch-invariant mode | 6–10% faster, 5–7% less energy per token | 1.5–2.7× faster, 16–50% less energy per token |
| SGLang 0.5.19 deterministic, bf16 | 1.18–1.20× faster, 14–17% less energy per token | 1.21–1.26× faster, 10–13% less energy per token |
| SGLang 0.5.19 deterministic, fp16 | 1.21–1.24× faster, 17–19% less energy per token | 1.72–2.86× faster, 24–51% less energy per token |

The tradeoff: when generating tokens, Luxi uses 1–7% more energy per token than normal vLLM and SGLang. To stay batch-invariant it keeps one matrix-multiply setup per shape at every batch size, while normal vLLM switches to a lower-power setup at batch 1. Details: [Why generation uses more energy](BENCHMARKS.md#why-generation-uses-more-energy).

Normal vLLM and SGLang are not batch-invariant: a prompt's per-step top-5 logprobs changed on 249–254 of 256 steps when it ran inside a batch of 16 or 64 instead of alone.

Energy is GPU board energy (NVML). One model, one GPU type, single-GPU harness; see [`TEST_SCOPE.md`](TEST_SCOPE.md) for what was not tested.

## Details and raw files

- Full tables, method, determinism and correctness checks: [`BENCHMARKS.md`](BENCHMARKS.md)
- What was and wasn't tested: [`TEST_SCOPE.md`](TEST_SCOPE.md)
- vLLM evidence, with per-run rows: [`evidence/h100-qwen2-7b-vs-vllm-0.25.1-2026-09-25/`](evidence/h100-qwen2-7b-vs-vllm-0.25.1-2026-09-25/)
- SGLang evidence, with per-run rows: [`evidence/h100-qwen2-7b-vs-sglang-0.5.19-2026-09-26/`](evidence/h100-qwen2-7b-vs-sglang-0.5.19-2026-09-26/)

## Drift demo

Results will be posted here.

Contact: Eric Waller, e@ewaller.com
