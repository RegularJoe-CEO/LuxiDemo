# Luxi public demos and evidence

**What this repo is:** closed **binaries**, **evidence packs**, and **markdown** run docs.  
**What this repo is not:** marketing website source, proprietary engine source, or the luxiedge.com deploy tree.

**Website (Replit, separate):** [luxiedge.com](https://luxiedge.com) - not published from this repository.  
**Boundary:** [`REPO_BOUNDARY.md`](REPO_BOUNDARY.md)

**Benchmarks:** [`BENCHMARKS.md`](BENCHMARKS.md) · **Runnable demos:** [`DEMOS.md`](DEMOS.md) · **Evidence:** [`evidence/README.md`](evidence/README.md) · **Architecture:** [`LUXI_SYSTEM.md`](LUXI_SYSTEM.md)

**Try without an NDA:** [**Luxi Book**](downloads/luxibook/) (CSV European options + deterministic output hash + Ed25519 `lxq2_` receipts - the professional Quant try)
and [**LuxiRisk**](luxirisk/) (free crypto/retail risk CLI). Inference serve binaries and numerical toys are documented in DEMOS below those two.

Luxi also builds energy-aware AI compute (measured H100 results in [`BENCHMARKS.md`](BENCHMARKS.md)). **LuxiEdge** is the inference surface.
**Luxi Book** is the runnable Quant path that can become a design-partner conversation. Other layers are labeled prototype or concept.

This repository has two jobs:

1. Ship compiled evaluation demos that can be run without disclosing proprietary
   implementation source.
2. Publish evidence packs that show exactly what was measured, including
   historical experiments that remain important to the development record.

## Current measured result

### LuxiEdge vs vLLM 0.25.1 - Qwen2-7B on H100

Luxi runs Qwen2-7B on an H100 with bit-identical results at any batch size. On long prompts (2k to 32k tokens) it is 3–7% faster than vLLM 0.25.1 and uses 2–5% less energy per token. When generating tokens it matches vLLM's speed but uses 2–7% more energy per token. Against vLLM's own deterministic (batch-invariant) mode, Luxi is faster and uses less energy on every test, including 1.5–2.7× faster token generation with 16–50% less energy.

Full prefill and decode tables, method, determinism, and what is borrowed vs Luxi's own: [`BENCHMARKS.md`](BENCHMARKS.md)

### Prior third-party baseline (TESTfort Version 99, 2026-07-23)

Independent evaluation of the **earlier** stack: thr still trailed vLLM; board energy slightly better; det + soak held. Kept for honesty and lineage:

[`h100-qwen2-7b-v99-matched-prefill-2026-07-23`](evidence/h100-qwen2-7b-v99-matched-prefill-2026-07-23/) - LuxiEdge ~28,374.7 pos/s @ 0.018718 J/pos (B16); ~80.6% of default vLLM thr with ~3.1% lower board J/pos.

## Run the public demos (no source)

### A) Luxi Book - CSV European options (the Quant sale)

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

**Output vector hash** (`example_book.csv` only - the value you compare across machines):  
`4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a`  

v0.2.0 matrix: Mini arm64 + RTX 4090 / H200 / H100×2 (CPU↔GPU agree on this book); A100 remains a **historical** v1-era line only. Not a universal GPU claim.  
The signed `lxq2_…` receipt is **per-install** - see [`downloads/luxibook/README.md`](downloads/luxibook/README.md) and [`evidence/v0.2.0-matrix/`](downloads/luxibook/evidence/v0.2.0-matrix/).  
Binaries: [`downloads/luxibook/`](downloads/luxibook/) · how to run: [`DEMOS.md`](DEMOS.md)

### B) LuxiRisk - free crypto / retail risk CLI (v0.2)

Freebie closed binary: liquidation price, position size from risk %, max $ loss at
stop - each with an **Ed25519-signed** `lxr1_…` **calculation receipt**.
**Not** the Quant book. Offline by default.

**OS binaries are not code-signed / notarized.** macOS right-click → Open; Windows SmartScreen → Run anyway.
Always verify the published download checksums (SHA-256 of the binary file).

```bash
chmod +x luxirisk/dist/luxirisk-macos-arm64   # or linux-x86_64
shasum -a 256 -c luxirisk/dist/luxirisk-macos-arm64.sha256
./luxirisk/dist/luxirisk-macos-arm64 liq --side long --entry 65000 --leverage 10
# → Liquidation price: 58825 · Receipt: lxr1_…
pip install cryptography && python3 luxirisk/test-vectors/verify_receipts.py
```

- Docs + formulas + vectors: [`luxirisk/`](luxirisk/)
- Release: [**luxirisk-v0.2**](https://github.com/RegularJoe-CEO/LuxiDemo/releases/tag/luxirisk-v0.2)
- Catalog: [`DEMOS.md`](DEMOS.md) · Built by the team behind LuxiEdge - [luxiedge.com](https://luxiedge.com)

### C) Inference serve API demo (demoted)

```bash
chmod +x downloads/luxiedge-serve-macos-arm64
./downloads/luxiedge-serve-macos-arm64 --bind 127.0.0.1:8787
curl -s http://127.0.0.1:8787/v1/models | python3 -m json.tool
```

Not Luxi Book. OpenAI-shaped API with a toy generate path. The `/v1/gtm`, `/health` and `/dashboard` routes include static scoreboard fields compiled into this demo build, for API integration testing; they are not benchmark results. Measured results: [`BENCHMARKS.md`](BENCHMARKS.md). Catalog: [`DEMOS.md`](DEMOS.md).

## Demo and product map

| Surface | What can be run publicly today | Status |
|---|---|---|
| **Luxi Book** | CSV BS/Black-76 + output-vector hash + Ed25519 `lxq2_` receipt; macOS + Linux CPU + Linux CUDA | **Primary Quant try** ([downloads](downloads/luxibook/) · [DEMOS](DEMOS.md)) |
| **LuxiRisk** freebie | Liq / size / max loss + Ed25519 `lxr1_` receipts (**OS binaries not code-signed**) | **[v0.2 freebie](https://github.com/RegularJoe-CEO/LuxiDemo/releases/tag/luxirisk-v0.2)** |
| LuxiEdge numerical engine | REST evaluation, operator list, receipt validation | **Working binary demo** |
| Deterministic tensor primitives | `luxi-tools ate` and `energy` | **Working binary demo** |
| Quant/statistical operators | `validate`, `quant_chain`, normalization operators | **Working binary demo** |
| Scientific and edge examples | Orbital and robotics commands | **Working binary demo** |
| LuxiEdge serve demo (v100) | Stripped HTTP binary: OpenAI-shaped API, toy generate path | **Working binary demo (no source)** |
| LuxiEdge Qwen2-7B vs vLLM 0.25.1 | H100 prefill + decode thr, J/token, determinism ([`BENCHMARKS.md`](BENCHMARKS.md)); engine private | **Measured evidence** |
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

- **Current measurement:** Qwen2-7B on H100 vs vLLM 0.25.1 - prefill + decode throughput, energy per token, determinism ([`BENCHMARKS.md`](BENCHMARKS.md)).
- **Prior third-party measurement:** Version 99 TESTfort matched-prefill pack.
- **Current runnable demonstrations:** Luxi Book (macOS + Linux CPU + Linux CUDA) + LuxiRisk freebie + version-100 serve + v3.0 numerical tools.
- **Independent numerical-engine evaluation:** linked and separately scoped.
- **Historical transformer research:** July 2026 TRADE, Flash comparison,
  long-context and sustain packs, preserved under `evidence/`.

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
| [`BENCHMARKS.md`](BENCHMARKS.md) | Qwen2-7B on H100 vs vLLM 0.25.1 (prefill + decode) |
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
