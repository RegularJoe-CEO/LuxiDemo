# Historical benchmark index

These July 11 (and related) measurements are preserved with their original
configurations. They are **not** the current public headline.

**Current results (start here):**

| Surface | Where |
|---------|--------|
| LuxiBook receipt matrix | [`RESULTS.md`](RESULTS.md) · [`downloads/luxibook/`](downloads/luxibook/) |
| Qwen2-7B vs vLLM 0.25.1 and SGLang 0.5.19 (H100 prefill + decode, drift demo) | [`INFERENCE.md`](INFERENCE.md) · [`BENCHMARKS.md`](BENCHMARKS.md) |
| Full measured tables | [`RESULTS.md`](RESULTS.md) |
| Chronology | [`HISTORY.md`](HISTORY.md) |

Archived third-party-operated prefill lineage:
[Version 99 matched-prefill](evidence/h100-qwen2-7b-v99-matched-prefill-2026-07-23/). It covers a different build and workload and does not validate the September vLLM or SGLang comparisons.

All packs below are **public** under `evidence/`.  
Folder names identify the hardware and workload represented by each pack.

## Primary (7B-class)

| Pack | What it shows |
|------|----------------|
| **[h100-7b-class-TRADE](evidence/h100-7b-class-TRADE/)** | Full 7B-class TRADE stack on H100: multi-run thr + **sustain-only NVML J/tok** (~0.63 @ seq=128, ~403 tok/s) |

Open `evidence/h100-7b-class-TRADE/START_HERE.md` first.

## Supporting (H100)

| Pack | What it shows |
|------|----------------|
| [h100-stack12-TRADE-cuda](evidence/h100-stack12-TRADE-cuda/) | Device-resident 12L stack energy (GPT-2 width) |
| [h100-stack12-H2H](evidence/h100-stack12-H2H/) | 12L TRADE and PyTorch Flash comparison |
| [h100-WNSM-free-ride](evidence/h100-WNSM-free-ride/) | WNSM free-ride under load |
| [h100-LONGCTX-scaling](evidence/h100-LONGCTX-scaling/) | O(N) vs O(N²) memory + CUDA 32k |
| [h100-BASELINE-vs-geo](evidence/h100-BASELINE-vs-geo/) | Single-layer baseline + morph/mesh wedges |
| [h100-serve-sustain-2026-07-11](evidence/h100-serve-sustain-2026-07-11/) | Continuous-batch serve sustain (CPU path context) |
| [version-100-h100-gtm](evidence/version-100-h100-gtm/) | Late-July 2026 short-prompt (S=128) B16/B32 multi-run raw files |
| [prefill_freeze_matched_20260807T210749Z](evidence/prefill_freeze_matched_20260807T210749Z/) | August 2026 short-prompt (S=128) B16/B32 multi-run raw files |
| [prefill_accel_lock_20260807T233111Z](evidence/prefill_accel_lock_20260807T233111Z/) | August 2026 short-prompt (S=128) recipe and batch sweep raw files |

## Measurement boundaries

- TRADE 7B path is **not** HF chat-quality (architecture map into TRADE kernels).
- On the 12-layer H100 same-shape pack in [`evidence/h100-stack12-H2H`](evidence/h100-stack12-H2H/), seq 1024, PyTorch Flash fp16 wins joules per token. The recorded ratio `trade_over_pytorch_J_per_token_ratio` is 19.169052480042783. Differentiated axes elsewhere in these packs: free-ride and AUDIT, O(N) memory, and measured board energy on real 7B-scale weights.
- Power is **GPU board** (pynvml), not wall-plug AC.
- The sequence-length 5 result is a microbenchmark, not a current product
  result.
- These packs are **not** LuxiBook option-pricing results.
