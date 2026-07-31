# Public evidence index

This index separates **current claims**, **working demonstrations**, and
**historical research**. Historical packs remain available because they explain
the development path and preserve both wins and losses.

## Current GTM transformer measurement (version-100)

[`version-100-h100-gtm/`](version-100-h100-gtm/)

- Qwen2-7B-Instruct class · S=128 · B=16/32
- Flash + device-resident + FP16 TRADE path
- Multi-run 5×15 s thr + NVML board J/pos + det=1.0
- Matched H2H vs vLLM: **~1.17–1.18× thr**, **~10–14% lower** board J/pos
- Authoritative: `MULTI_RUN_LOCK_SLIM.json` · buyer: `PUBLIC_GTM_ONE_PAGER.md`

## Prior third-party-operated transformer measurement

### Version 99 matched prefill, 2026-07-23 (TESTfort)

[`h100-qwen2-7b-v99-matched-prefill-2026-07-23/`](h100-qwen2-7b-v99-matched-prefill-2026-07-23/)

- Earlier stack: thr trailed vLLM; modest board-energy edge; det + soak held
- Kept as independent lineage — **not** the version-100 thr claim

Formal signed narrative: pending.

## Independently evaluated numerical engine

TestFort’s December 2025 report covers a defined deterministic numerical
workload:

[Open the public report](https://luxiedge.com/luxiedge-validation-report.pdf)

This evidence belongs to LuxiQuant/numerical execution and is separate from
transformer inference.

## Current correctness status

| Surface | Public status |
|---|---|
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
