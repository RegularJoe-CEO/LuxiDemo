# Public evidence index

This index separates **current claims**, **working demonstrations**, and
**historical research**. Historical packs remain available because they explain
the development path and preserve both wins and losses.

## LuxiBook - Quant receipt (public try)

Not a thr/J pack. Closed binaries + measured SHA-256 on one example book:

| Field | Value |
|-------|--------|
| Workload | `example_book.csv` · European BS / Black-76 · five Greeks |
| Receipt | `4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a` |
| ATM_CALL | `10.4505835721856215` |
| Boxes | Mac Mini CPU · RunPod x86 CPU · A100 · H100 · H200 (two GPU runs each) |
| Binaries | [`../downloads/luxibook/`](../downloads/luxibook/) |
| Date | 2026-08-15 |

**Non-claim:** this book / these boxes / this kernel - not universal CPU↔GPU.

Tables: [`../RESULTS.md`](../RESULTS.md) · how to run: [`../DEMOS.md`](../DEMOS.md)

## Current transformer measurements

Overview: [`../INFERENCE.md`](../INFERENCE.md) · what was and wasn't tested: [`../TEST_SCOPE.md`](../TEST_SCOPE.md)

### Qwen2-7B vs vLLM 0.25.1 on H100 - 2026-09-25/26

[`h100-qwen2-7b-vs-vllm-0.25.1-2026-09-25/`](h100-qwen2-7b-vs-vllm-0.25.1-2026-09-25/) · tables and method: [`../BENCHMARKS.md`](../BENCHMARKS.md)

- Token generation: matches vLLM's speed within 2%, 2–7% more GPU-board energy per token
- Long prompts (2k to 32k tokens): 3–7% faster than vLLM 0.25.1, 2–5% less GPU-board energy per token
- vs vLLM batch-invariant mode: faster and less energy on every test
- Bit-identical output across every batch size tested (1, 16 and 64 when generating; 1 against 16, 4 and 2 on long prompts)
- Includes per-run rows, gate rows and Hugging Face comparisons

### Qwen2-7B vs SGLang 0.5.19 on H100 - 2026-09-26

[`h100-qwen2-7b-vs-sglang-0.5.19-2026-09-26/`](h100-qwen2-7b-vs-sglang-0.5.19-2026-09-26/) · tables and method: [`../BENCHMARKS.md`](../BENCHMARKS.md)

- Different H100 pod from the vLLM pack; do not mix absolute numbers across the two packs
- vs SGLang deterministic mode (fp16 and bf16): faster and less energy on every test; 2k to 32k prompts 1.18–1.24× faster with 14–19% less energy per token, generation 1.21–2.86× faster
- vs normal SGLang: 2k to 32k prompts 1.3–4.6% faster with 1.0–4.1% less energy per token; generation about equal speed with 1–7% more energy per token
- Includes per-run rows, gate results and setup logs

### Qwen2-7B output drift under batching and live traffic on H100 - 2026-09-26

[`h100-qwen2-7b-drift-demo-2026-09-26/`](h100-qwen2-7b-drift-demo-2026-09-26/) · summary: [`../INFERENCE.md#drift-demo`](../INFERENCE.md#drift-demo)

- Each prompt's output under batch load compared with the same engine's output alone: 20 prompts, 256 greedy tokens, fp16
- Luxi: 80 of 80 batch conditions identical in tokens and logprobs (batch conditions only; the long prompt from a fresh-process re-run)
- Normal vLLM 0.25.1 changed the visible answer in 12 of 140 conditions and normal SGLang 0.5.19 in 11 of 140; their deterministic modes stayed identical in 140 of 140
- Includes an open Luxi fault seen in one long-running process, with its raw records
- Includes the method sheet, per-prompt table, scripts, prompts and summary files

## Historical independent evaluation

TestFort independently evaluated a separate seven-function numerical-engine workload in December 2025. It is not LuxiBook and not the current inference comparison.

- [December 2025 TestFort report](https://luxiedge.com/luxiedge-validation-report.pdf)
- [Measured result row](../RESULTS.md)

The July 23, 2026 packed-prefill pack remains in the historical archive for lineage only. It is not used as current third-party validation.

The December report does not validate the September vLLM or SGLang comparisons or the drift demo. Proposed independent go-forward protocol: [TESTFORT_FORWARD_PROTOCOL.md](../TESTFORT_FORWARD_PROTOCOL.md).

## Current correctness status

| Surface | Public status |
|---|---|
| LuxiBook example receipt | Measured on listed boxes; public binaries |
| Faithful Qwen2-7B CUDA | Website reports current limited acceptance result; raw public pack still required |
| Llama 3.1 resident inference | Internal correctness milestone; no public performance/energy claim |

## Historical H100 research

| Pack | Original purpose |
|---|---|
| [`h100-7b-class-TRADE`](h100-7b-class-TRADE/) | Full 28-layer 7B-class TRADE sustain ladder |
| [`h100-stack12-TRADE-cuda`](h100-stack12-TRADE-cuda/) | Device-resident 12-layer stack energy |
| [`h100-stack12-H2H`](h100-stack12-H2H/) | Same-shape TRADE versus PyTorch/Flash, including the Luxi loss |
| [`h100-WNSM-free-ride`](h100-WNSM-free-ride/) | WNSM payload/free-ride behavior under load |
| [`h100-LONGCTX-scaling`](h100-LONGCTX-scaling/) | O(N) versus dense O(N²) memory scaling |
| [`h100-BASELINE-vs-geo`](h100-BASELINE-vs-geo/) | Single-layer baseline and geodesic wedges |
| [`h100-serve-sustain-2026-07-11`](h100-serve-sustain-2026-07-11/) | Continuous-batch sustain context; CPU-bound serve path on H100 host |
| [`h100-qwen2-7b-v99-matched-prefill-2026-07-23`](h100-qwen2-7b-v99-matched-prefill-2026-07-23/) | Archived packed-prefill lineage for a different build and workload; not current validation |
| [`version-100-h100-gtm`](version-100-h100-gtm/) | Late-July 2026 short-prompt (S=128) B16/B32 multi-run raw files |
| [`prefill_freeze_matched_20260807T210749Z`](prefill_freeze_matched_20260807T210749Z/) | August 2026 short-prompt (S=128) B16/B32 multi-run raw files |
| [`prefill_accel_lock_20260807T233111Z`](prefill_accel_lock_20260807T233111Z/) | August 2026 short-prompt (S=128) recipe and batch sweep raw files |

These packs have not been deleted or reinterpreted. Their numbers remain tied
to their original model/shape, code, comparator, and measurement method.

See also [`../HISTORICAL_BENCHMARKS.md`](../HISTORICAL_BENCHMARKS.md).

## Reading rule

For every result, identify:

1. hardware,
2. model or mathematical workload,
3. accepted work unit,
4. precision and backend,
5. timing window,
6. energy boundary,
7. validation status.

Do not combine throughput from one pack with power from another.
Do not treat Book receipts as thr/J evidence, or thr/J packs as option pricing.


Official proof page: [luxiedge.com/proof](https://luxiedge.com/proof). Confidential evaluation: [luxiedge.com/pilot](https://luxiedge.com/pilot).
