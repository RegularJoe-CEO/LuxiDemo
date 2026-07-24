# Historical benchmark index

These July 11 measurements are preserved with their original configurations.
For the latest results, open the [current evidence index](evidence/README.md)
or the [Version 99 matched-prefill pack](evidence/h100-qwen2-7b-v99-matched-prefill-2026-07-23/).

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

## Measurement boundaries

- TRADE 7B path is **not** HF chat-quality (architecture map into TRADE kernels).
- Flash may win short-seq thr/J/tok; differentiated axes: free-ride/AUDIT, O(N) mem, measured board energy on real 7B-scale weights.
- Power is **GPU board** (pynvml), not wall-plug AC.
- The sequence-length 5 result is a microbenchmark, not a current product
  result.
