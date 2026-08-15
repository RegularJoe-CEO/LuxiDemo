# Luxi public demos and evidence

**What this repo is:** closed **binaries**, **evidence packs**, and **markdown** run docs.  
**What this repo is not:** marketing website source, proprietary engine source, or the luxiedge.com deploy tree.

**Website (Replit, separate):** [luxiedge.com](https://luxiedge.com) — not published from this repository.  
**Boundary:** [`REPO_BOUNDARY.md`](REPO_BOUNDARY.md)

**Runnable demos:** [`DEMOS.md`](DEMOS.md) · **Evidence:** [`evidence/README.md`](evidence/README.md) · **Architecture:** [`LUXI_SYSTEM.md`](LUXI_SYSTEM.md)

**Try without an NDA:** [**Luxi Book**](downloads/luxibook/) (CSV European options + SHA-256 receipts — the professional Quant try)
and [**LuxiRisk**](luxirisk/) (free crypto/retail risk CLI). Inference serve binaries and numerical toys are documented in DEMOS below those two.

Luxi also builds energy-aware AI compute (prefill thr/J evidence on H100). **LuxiEdge** is the inference surface.
**Luxi Book** is the runnable Quant path that can become a design-partner conversation. Other layers are labeled prototype or concept.

This repository has two jobs:

1. Ship compiled evaluation demos that can be run without disclosing proprietary
   implementation source.
2. Publish evidence packs that show exactly what was measured, including
   historical experiments that remain important to the development record.

## Current measured result (two facts — do not blend)

### Fact A — Best Luxi absolute prefill (dual_gemm @ B72)

| Operating point | Thr median (pos/s) | Board J/pos |
|-----------------|-------------------:|------------:|
| **B72 dual_gemm** (`LUXI_GEMM_DUAL_STREAM=1`) | **~44,860** | **~0.0153** |

5×15s multi-run · flash + device-resident + FP16 · S=128 · H100 NVML board joules.  
Replaces prior Luxi absolute B16 freeze (~41.2k / 0.0169): ~**1.09× thr**, ~**9% lower J/pos**.

**Pack:** [`evidence/prefill_accel_lock_20260807T233111Z/`](evidence/prefill_accel_lock_20260807T233111Z/) · [`CHAMPION_LOCK.json`](evidence/prefill_accel_lock_20260807T233111Z/CHAMPION_LOCK.json) · [`PUBLIC_CHAMPION_BRIEF.md`](evidence/prefill_accel_lock_20260807T233111Z/PUBLIC_CHAMPION_BRIEF.md)

### Fact B — Matched vLLM H2H (B16 / B32 only)

| Batch | Luxi thr | Board J/pos | vs vLLM thr | vs vLLM J/pos |
|------:|---------:|------------:|------------:|--------------:|
| **16** | **~41,221** | **~0.0169** | **~1.18×** | **~12% lower** |
| **32** | **~43,464** | **~0.0158** | **~1.19×** | **~14% lower** |

**Pack:** [`evidence/prefill_freeze_matched_20260807T210749Z/`](evidence/prefill_freeze_matched_20260807T210749Z/)  

**Rule:** Absolute B72 is **not** claimed as 1.18× vLLM until a matched B72 vLLM arm exists. Single-shot thr (e.g. 45,986) is not a product headline.

Scope: prefill positions (iters × batch × seq), single GPU board energy — not decode-only, not multi-tenant full-server leadership, not wall-plug.

### Prior third-party baseline (TESTfort Version 99, 2026-07-23)

Independent evaluation of the **earlier** stack: thr still trailed vLLM; board energy slightly better; det + soak held. Kept for honesty and lineage:

[`h100-qwen2-7b-v99-matched-prefill-2026-07-23`](evidence/h100-qwen2-7b-v99-matched-prefill-2026-07-23/) — LuxiEdge ~28,374.7 pos/s @ 0.018718 J/pos (B16); ~80.6% of default vLLM thr with ~3.1% lower board J/pos.

## Run the public demos (no source)

### A) Luxi Book — CSV European options (the Quant sale)

```bash
# Mac CPU
chmod +x downloads/luxibook/luxi-book-macos-arm64
./downloads/luxibook/luxi-book-macos-arm64 price \
  --book downloads/luxibook/example_book.csv \
  --out report.csv --receipt receipt.json

# Linux CPU
./downloads/luxibook/luxi-book-linux-x86_64 price \
  --book downloads/luxibook/example_book.csv \
  --out report.csv --receipt receipt.json

# Linux CUDA (NVIDIA required)
./downloads/luxibook/luxi-book-linux-x86_64-cuda price \
  --book downloads/luxibook/example_book.csv \
  --out report.csv --receipt receipt.json --mode gpu
```

Measured receipt (`example_book.csv` only):  
`4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a`  
(Mini CPU · Linux CPU · A100 · H100 · H200). Not a universal GPU claim.  
Binaries: [`downloads/luxibook/`](downloads/luxibook/) · how to run: [`DEMOS.md`](DEMOS.md)

### B) LuxiRisk — free crypto / retail risk CLI (v0.2)

Freebie closed binary: liquidation price, position size from risk %, max $ loss at
stop — each with an **Ed25519-signed** `lxr1_…` **calculation receipt**.
**Not** the Quant book. Offline by default.

**OS binaries are unsigned.** macOS right-click → Open; Windows SmartScreen → Run anyway.
Always verify the published SHA-256.

```bash
chmod +x luxirisk/dist/luxirisk-macos-arm64   # or linux-x86_64
shasum -a 256 -c luxirisk/dist/luxirisk-macos-arm64.sha256
./luxirisk/dist/luxirisk-macos-arm64 liq --side long --entry 65000 --leverage 10
# → Liquidation price: 58825 · Receipt: lxr1_…
pip install cryptography && python3 luxirisk/test-vectors/verify_receipts.py
```

- Docs + formulas + vectors: [`luxirisk/`](luxirisk/)
- Release: [**luxirisk-v0.2**](https://github.com/RegularJoe-CEO/LuxiDemo/releases/tag/luxirisk-v0.2)
- Catalog: [`DEMOS.md`](DEMOS.md) · Built by the team behind LuxiEdge — [luxiedge.com](https://luxiedge.com)

### C) Inference serve + locked scoreboard (demoted)

```bash
chmod +x downloads/luxiedge-serve-macos-arm64
./downloads/luxiedge-serve-macos-arm64 --bind 127.0.0.1:8787
curl -s http://127.0.0.1:8787/v1/gtm | python3 -m json.tool
```

Not Luxi Book. Catalog: [`DEMOS.md`](DEMOS.md).

## Demo and product map

| Surface | What can be run publicly today | Status |
|---|---|---|
| **Luxi Book** | CSV BS/Black-76 + SHA-256; macOS + Linux CPU + Linux CUDA binaries | **Primary Quant try** ([downloads](downloads/luxibook/) · [DEMOS](DEMOS.md)) |
| **LuxiRisk** freebie | Liq / size / max loss + `lxr1_` receipts (**unsigned**) | **[v0.2 freebie](https://github.com/RegularJoe-CEO/LuxiDemo/releases/tag/luxirisk-v0.2)** |
| LuxiEdge numerical engine | REST evaluation, operator list, receipt validation | **Working binary demo** |
| ATE / Waller / WNSM primitives | `luxi-tools ate` and `energy` | **Working binary demo** |
| Quant/statistical operators | `validate`, `quant_chain`, Welford, normalization operators | **Working binary demo** |
| Scientific and edge examples | Orbital and robotics commands | **Working binary demo** |
| LuxiEdge serve scoreboard (v100) | Stripped HTTP binary + `/v1/gtm` lock | **Working binary demo (no source)** |
| LuxiEdge Version 100 TRADE thr/J | Multi-run H100 pack; engine private | **Measured evidence (wins thr+J vs vLLM)** |
| LuxiEdge Version 99 inference | TESTfort prior baseline; thr trailed vLLM | **Third-party measured lineage** |
| Faithful Qwen2-7B CUDA | Website reports current acceptance result | **Raw public correctness pack pending** |
| Llama 3.1 resident inference | No public performance binary or energy pack | **Internal milestone** |
| LuxiPack | Public demo not yet published | **In development** |
| LuxiPhase | Public control trace/demo not yet published | **Prototype/local validation** |
| LuxiLoad | No validated public demo | **Early concept** |
| LuxiSDG | No validated public demo | **Early concept** |

An unfinished product layer is not presented as a finished product. Its next
public milestone is a real demo with retained inputs, outputs, baseline, and
measurement boundary.

## Evidence organization

The historical work has **not** been deleted or hidden.

- **Current GTM measurement:** version-100 multi-run thr+J+det + H2H vs vLLM.
- **Prior third-party measurement:** Version 99 TESTfort matched-prefill pack.
- **Current runnable demonstrations:** Luxi Book (macOS + Linux CPU + Linux CUDA) + LuxiRisk freebie + version-100 serve + v3.0 numerical tools.
- **Independent numerical-engine evaluation:** linked and separately scoped.
- **Historical transformer research:** July 2026 TRADE, Flash comparison,
  WNSM, long-context, and sustain packs, preserved under `evidence/`.

Start at [`evidence/README.md`](evidence/README.md); read
[`HISTORY.md`](HISTORY.md) for why the older packs still matter.

## Verify published numbers

```bash
# Version 99 thr/J arithmetic from retained CSV
python3 scripts/verify_v99_pack.py

# Luxi Book binary digests
shasum -a 256 -c downloads/luxibook/luxi-book-macos-arm64.sha256

# LuxiRisk formula vectors
pip install cryptography && python3 luxirisk/test-vectors/verify_receipts.py
```

Script index: [`scripts/README.md`](scripts/README.md). Book receipt target is in
[`RESULTS.md`](RESULTS.md).

## Source and access boundary

This is a **public demo and evidence repository**, not the private engine source
tree. Public demos are compiled evaluation binaries with published checksums.
They let evaluators run defined inputs, inspect outputs, and compare receipts
without receiving the proprietary implementation.

No model weights, private source, credentials, active infrastructure addresses,
or SSH access information belong in this repository.

## Repository guide

| Path | Purpose |
|---|---|
| [`downloads/`](downloads/) | Closed binaries (Luxi Book, serve) + checksums |
| [`luxirisk/`](luxirisk/) | Freebie risk CLI binaries + public formulas/vectors |
| [`evidence/`](evidence/) | Current and historical measurement packs |
| [`DEMOS.md`](DEMOS.md) | Runnable public demo catalog |
| [`RESULTS.md`](RESULTS.md) | Published results and measurement scope |
| [`LUXI_SYSTEM.md`](LUXI_SYSTEM.md) | Product-family architecture and maturity |
| [`HISTORY.md`](HISTORY.md) | Development chronology |
| [`REPO_BOUNDARY.md`](REPO_BOUNDARY.md) | What belongs here vs Replit vs private engines |
| [`docs/`](docs/) | Domain notes (markdown only) |
| [`scripts/`](scripts/) | Public evidence verifiers (no engine source) |

## Contact

Eric Waller, e@ewaller.com

© 2026 Eric Waller. Public binaries are evaluation builds; private source and
commercial rights are not conveyed by this repository.
