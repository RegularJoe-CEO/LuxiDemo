# public 7b class benchmark pack evidence index

All packs below are **public** under `evidence/`.

## Primary (7B-class)

| Pack | What it shows |
|------|----------------|
| **[spacex-xai-h100-QWEN7B-TRADE](evidence/spacex-xai-h100-QWEN7B-TRADE/)** | Full Qwen2-7B TRADE stack on H100: multi-run thr + **sustain-only NVML J/tok** (~0.63 @ seq=128) |

Open `evidence/spacex-xai-h100-QWEN7B-TRADE/START_HERE.md` first.

## Supporting (earlier H100 diligence)

| Pack | What it shows |
|------|----------------|
| [spacex-xai-h100-TRADE-cuda](evidence/spacex-xai-h100-TRADE-cuda/) | Device-resident 12L stack energy (GPT-2 width) |
| [spacex-xai-h100-STACK12-H2H](evidence/spacex-xai-h100-STACK12-H2H/) | 12L TRADE vs PyTorch Flash H2H |
| [spacex-xai-h100-WNSM-free-ride](evidence/spacex-xai-h100-WNSM-free-ride/) | WNSM free-ride under load |
| [spacex-xai-h100-LONGCTX-scaling](evidence/spacex-xai-h100-LONGCTX-scaling/) | O(N) vs O(N²) memory + CUDA 32k |
| [spacex-xai-h100-BASELINE-vs-geo](evidence/spacex-xai-h100-BASELINE-vs-geo/) | Single-layer baseline + morph/mesh wedges |
| [spacex-xai-h100-2026-07-11](evidence/spacex-xai-h100-2026-07-11/) | Continuous-batch serve sustain (CPU path) |

## Honest non-claims

- TRADE 7B path is **not** HF chat-quality (architecture map into TRADE kernels).
- Flash may win short-seq thr/J/tok; differentiated axes: free-ride/AUDIT, O(N) mem, measured board energy on real 7B-scale weights.
- Power is **GPU board** (pynvml), not wall-plug AC.
