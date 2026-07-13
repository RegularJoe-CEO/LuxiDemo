# Public 7B-class benchmark pack index

All packs below are **public** under `evidence/`.  
Folder names are **neutral H100 benchmark labels** (no company diligence codenames).

## Primary (7B-class)

| Pack | What it shows |
|------|----------------|
| **[h100-7b-class-TRADE](evidence/h100-7b-class-TRADE/)** | Full 7B-class TRADE stack on H100: multi-run thr + **sustain-only NVML J/tok** (~0.63 @ seq=128, ~403 tok/s) |

Open `evidence/h100-7b-class-TRADE/START_HERE.md` first.

## Supporting (H100)

| Pack | What it shows |
|------|----------------|
| [h100-stack12-TRADE-cuda](evidence/h100-stack12-TRADE-cuda/) | Device-resident 12L stack energy (GPT-2 width) |
| [h100-stack12-H2H](evidence/h100-stack12-H2H/) | 12L TRADE vs PyTorch Flash H2H (we publish the loss) |
| [h100-WNSM-free-ride](evidence/h100-WNSM-free-ride/) | WNSM free-ride under load |
| [h100-LONGCTX-scaling](evidence/h100-LONGCTX-scaling/) | O(N) vs O(N²) memory + CUDA 32k |
| [h100-BASELINE-vs-geo](evidence/h100-BASELINE-vs-geo/) | Single-layer baseline + morph/mesh wedges |
| [h100-serve-sustain-2026-07-11](evidence/h100-serve-sustain-2026-07-11/) | Continuous-batch serve sustain (CPU path context) |

## Honest non-claims

- TRADE 7B path is **not** HF chat-quality (architecture map into TRADE kernels).
- Flash may win short-seq thr/J/tok; differentiated axes: free-ride/AUDIT, O(N) mem, measured board energy on real 7B-scale weights.
- Power is **GPU board** (pynvml), not wall-plug AC.
- Lead product thr/energy at **sustain_seq ≥ 128**; seq=5 ~3.56 J/tok is microbench only.

## Site

Human-readable site rebuild: [`site/`](site/) · claims log: [`CLAIMS_REGISTER.md`](CLAIMS_REGISTER.md) · publish notes: [`PUBLISH.md`](PUBLISH.md)
