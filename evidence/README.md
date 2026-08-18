# Public evidence index

This index separates **current claims**, **working demonstrations**, and
**historical research**. Historical packs remain available because they explain
the development path and preserve both wins and losses.

## Luxi Book - Quant receipt (public try)

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

## Current GTM transformer measurement (absolute + matched)

### Absolute prefill champion (B72 dual_gemm) - 2026-08-07

[`prefill_accel_lock_20260807T233111Z/`](prefill_accel_lock_20260807T233111Z/)

- ~44,860 pos/s · ~0.0153 board J/pos · multi-run median
- **Not** a matched vLLM claim at B72

### Matched vLLM H2H (B16 / B32 only)

[`prefill_freeze_matched_20260807T210749Z/`](prefill_freeze_matched_20260807T210749Z/)

- B16 ~1.18× thr · ~12% lower J/pos vs vLLM
- B32 ~1.19× thr · ~14% lower J/pos vs vLLM

### version-100 GTM lineage

[`version-100-h100-gtm/`](version-100-h100-gtm/)

- Qwen2-7B-Instruct class · S=128 · B=16/32
- Flash + device-resident + FP16 TRADE path
- Multi-run 5×15 s thr + NVML board J/pos + det=1.0
- Matched H2H lineage vs vLLM (~1.17 to 1.18× thr, ~10 to 14% lower board J/pos)
- Authoritative: `MULTI_RUN_LOCK_SLIM.json` · buyer: `PUBLIC_GTM_ONE_PAGER.md`

## Prior third-party-operated transformer measurement

### Version 99 matched prefill, 2026-07-23 (TESTfort)

[`h100-qwen2-7b-v99-matched-prefill-2026-07-23/`](h100-qwen2-7b-v99-matched-prefill-2026-07-23/)

- Earlier stack: thr trailed vLLM; modest board-energy edge; det + soak held
- Kept as independent lineage - **not** the current thr claim

Formal signed narrative: pending.

## Independently evaluated numerical engine

TestFort’s December 2025 report covers a defined deterministic numerical
workload (not Luxi Book, not inference thr/J):

[Open the public report](https://luxiedge.com/luxiedge-validation-report.pdf)

This evidence belongs to LuxiQuant numerical execution and is separate from
transformer inference and from the option book.

## Current correctness status

| Surface | Public status |
|---|---|
| Luxi Book example receipt | Measured on listed boxes; public binaries |
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
