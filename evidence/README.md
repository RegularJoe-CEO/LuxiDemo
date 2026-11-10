# Public evidence index

This index separates **current claims**, **working demonstrations**, and
**historical research**. Historical packs remain available because they explain
the development path and preserve both wins and losses.

## Inference, November 2026 (current)

Overview: [`../INFERENCE.md`](../INFERENCE.md) · scope: [`../TEST_SCOPE.md`](../TEST_SCOPE.md) · package: [`../downloads/luxi-replay-demo/`](../downloads/luxi-replay-demo/)

### R1 replay package clean-install test - 2026-11-10

[`r1-replay-package-2026-11-10/`](r1-replay-package-2026-11-10/) · complete for this test: every replay record, comparison and log is retained (per-install private keys and the downloaded model files are not)

- Package `luxi-replay-demo-1.0.0-linux-x86_64.tar.gz`, sha256 `b1f6cf0b…39c9`, build `9f7931d9…6694` (a new build identity, not the November 9 binaries)
- 32-token replay: identical records on RTX 5090, A40, AMD EPYC 7543, Intel Xeon Gold 6342 and a sandbox x86_64 CPU (all 10 cross-machine pairs); step root matches the surviving 16-hex-digit prefix of the November 9 H100 NVL short replay, whose full record was not recovered
- Full suite: 136/136 answers and 546,114 tokens identical to the signed November 9 H100 NVL records on RTX 5090 and A40, and identical to each other job by job
- Offline verify, wrong-key rejection and rejection of every tested tamper edit on all three machines

### DeepSeek-R1-Distill-Qwen-7B GPU swap test - 2026-11-09

[`r1-distill-7b-gpu-swap-2026-11-09/`](r1-distill-7b-gpu-swap-2026-11-09/) · partial: see its Completeness section

- Nine GPU types each matched 136/136 sampled answers and 546,114 generated tokens against the signed H100 NVL reference, plus a separate 92/92 Qwen gate; 17 problems x 8 samples, not 136 distinct questions
- Seven CPU models each matched 2/2 small Qwen checks and the short signed R1 replay capped at 32 tokens; they did not run the full GPU workload
- vLLM 0.32 normal was faster and cheaper per token wherever the tested int8 configuration ran
- Includes every per-GPU comparison, gate, CPU-check and soak file, the 17 signed H100 NVL records, the public key and an offline verifier. The per-GPU answer bundles are not in the recovered archive

## Luxi Book - Quant receipt (public try)

Not a thr/J pack. Closed binaries + measured SHA-256 on one example book:

| Field | Value |
|-------|--------|
| Workload | `example_book.csv` · European BS / Black-76 · five Greeks |
| Receipt | `4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a` |
| ATM_CALL | `10.4505835721856215` |
| Boxes (current, v0.2.1) | H200 · H100 · A100 · RTX 5090 · RTX 4090, each on the CPU and CUDA pricing paths: 20 signed receipts across two books ([`../downloads/luxibook/evidence/v0.2.1-phi-fix-matrix/`](../downloads/luxibook/evidence/v0.2.1-phi-fix-matrix/)) |
| Boxes (historical, pre-fix) | Mac Mini CPU · RunPod x86 CPU · A100 · H100 · H200 (two GPU runs each), 2026-08-15 |
| Binaries | [`../downloads/luxibook/`](../downloads/luxibook/) |
| Date | 2026-08-16 (v0.2.1 matrix) |

**Non-claim:** this book / these boxes / this kernel - not universal CPU↔GPU.

Tables: [`../RESULTS.md`](../RESULTS.md) · how to run: [`../DEMOS.md`](../DEMOS.md)

## Historical transformer measurements (September 2026)

Qwen2-7B-Instruct and earlier engine builds. These speed, energy and determinism results do not transfer to the November v27 build or the replay package. Overview: [`../INFERENCE.md`](../INFERENCE.md#historical-qwen2-7b-on-h100-september-2026) · scope: [`../TEST_SCOPE.md`](../TEST_SCOPE.md)

### Qwen2-7B vs vLLM 0.25.1 on H100 - 2026-09-25/26

[`h100-qwen2-7b-vs-vllm-0.25.1-2026-09-25/`](h100-qwen2-7b-vs-vllm-0.25.1-2026-09-25/) · tables and method: [`../BENCHMARKS.md`](../BENCHMARKS.md)

- Long prompts (2k to 32k tokens): 3–7% faster than vLLM 0.25.1, 2–5% less energy per token
- Token generation: matches vLLM's speed, 2–7% more energy per token
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

## Prior third-party-operated transformer measurement

### Version 99 matched prefill, 2026-07-23 (TESTfort)

[`h100-qwen2-7b-v99-matched-prefill-2026-07-23/`](h100-qwen2-7b-v99-matched-prefill-2026-07-23/)

- Earlier stack: thr trailed vLLM; modest board-energy edge; det + soak held
- Kept as independent lineage - **not** the current thr claim
- Covers the earlier Version 99 build only; it does not cover the 2026-09 vLLM or SGLang results or the drift demo

Formal signed narrative: pending.

## Independently evaluated numerical engine

TestFort’s December 2025 report covers a defined deterministic numerical
workload (not Luxi Book, not inference thr/J):

The previous public URL `https://luxiedge.com/luxiedge-validation-report.pdf` returned HTTP 404 on 2026-10-11. The measured figures remain the December 2025 row in [`RESULTS.md`](../RESULTS.md).

This evidence belongs to LuxiQuant numerical execution and is separate from
transformer inference and from the option book.

## Current correctness status

| Surface | Public status |
|---|---|
| R1 replay package 1.0.0 | Public closed binary; RTX 5090, A40 and three CPUs tested on 2026-11-10, raw outputs retained |
| R1 GPU swap test, 2026-11-09 | Per-GPU comparison files and the signed H100 NVL reference public; per-GPU answer bundles not recovered |
| Luxi Book example receipt | Measured on listed boxes; public binaries |
| Faithful Qwen2-7B CUDA (historical) | Website reported a limited acceptance result; raw public pack still required |
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
