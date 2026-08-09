# Luxi public demos and evidence

**Website:** [luxiedge.com](https://luxiedge.com)

**Current evidence:** [`evidence/README.md`](evidence/README.md)

**Runnable binary demos:** [`DEMOS.md`](DEMOS.md)
**Luxi product architecture:** [`LUXI_SYSTEM.md`](LUXI_SYSTEM.md)

Luxi is building the energy architecture for AI compute: deterministic numerical
execution, energy-aware GPU execution, scheduling, load shaping, and power
coordination.

**LuxiEdge** is the first public AI product. **LuxiQuant** is the working
deterministic numerical foundation. Other Luxi layers are clearly marked as in
development, prototype, or concept.

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

### A) Commercial serve + locked scoreboard (version-100)

Stripped binary — OpenAI-shaped HTTP + `GET /v1/gtm` embeds the multi-run lock. **No engine source.**

```bash
# From repo (or GitHub release assets)
chmod +x site/downloads/luxiedge-serve-macos-arm64   # or linux-x86_64
./site/downloads/luxiedge-serve-macos-arm64 --bind 127.0.0.1:8787

curl -s http://127.0.0.1:8787/v1/gtm | python3 -m json.tool
curl -s -X POST http://127.0.0.1:8787/v1/completions \
  -H 'content-type: application/json' \
  -d '{"prompt":"Why measure joules per token?","max_tokens":24}'
curl -s -X POST http://127.0.0.1:8787/v1/audit -d '{}'
```

- Site page: [`site/demo.html`](site/demo.html) · live: https://luxiedge.com/demo.html (after Replit publish)
- Package: [`demo/luxiedge-yc-demo/`](demo/luxiedge-yc-demo/)
- Release: [version-100-serve](https://github.com/RegularJoe-CEO/LuxiDemo/releases/tag/version-100-serve) (when published)

### B) Numerical / quant demos (v3.0)

```bash
# Linux x86-64 example
chmod +x luxiedge-demo-linux-x86_64 luxi-tools-linux-x86_64
./luxiedge-demo-linux-x86_64 validate
./luxi-tools-linux-x86_64 energy
```

Download: [v3.0 release](https://github.com/RegularJoe-CEO/LuxiDemo/releases/tag/v3.0) · Full catalog: [`DEMOS.md`](DEMOS.md)

### C) LuxiRisk — offline trader risk CLI (v0.2)

Free closed binary: liquidation price, position size from risk %, max $ loss at
stop — each with an **Ed25519-signed** `lxr1_…` receipt (per-install identity).

```bash
chmod +x luxirisk/dist/luxirisk-macos-arm64   # or linux-x86_64
./luxirisk/dist/luxirisk-macos-arm64 liq --side long --entry 65000 --leverage 10
# → Liquidation price: 58825 · Receipt: lxr1_…
pip install cryptography && python3 luxirisk/test-vectors/verify_receipts.py
```

- Docs + formulas + vectors: [`luxirisk/`](luxirisk/)
- Planned release: `luxirisk-v0.2` after Apple/Windows code-signing certs (see [`luxirisk/SIGNING.md`](luxirisk/SIGNING.md))
- Catalog row: [`DEMOS.md`](DEMOS.md)

## Demo and product map

| Surface | What can be run publicly today | Status |
|---|---|---|
| **LuxiRisk** trader risk CLI | Liq / size / max loss + Ed25519 `lxr1_` receipts (offline) | **Working closed binary (v0.2)** |
| LuxiQuant numerical engine | REST evaluation, operator list, receipt validation | **Working binary demo** |
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
- **Current runnable demonstrations:** version-100 serve binary + v3.0 numerical tools.
- **Independent numerical-engine evaluation:** linked and separately scoped.
- **Historical transformer research:** July 2026 TRADE, Flash comparison,
  WNSM, long-context, and sustain packs, preserved under `evidence/`.

Start at [`evidence/README.md`](evidence/README.md); read
[`HISTORY.md`](HISTORY.md) for why the older packs still matter.

## Verify the Version 99 arithmetic

This verifier reads the public per-run CSV and recomputes the medians and
comparative ratios. It does not contain or reveal the inference engine.

```bash
python3 scripts/verify_v99_pack.py
```

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
| [`DEMOS.md`](DEMOS.md) | Runnable public demo catalog |
| [`LUXI_SYSTEM.md`](LUXI_SYSTEM.md) | Product-family architecture and maturity |
| [`evidence/README.md`](evidence/README.md) | Current and historical evidence index |
| [`RESULTS.md`](RESULTS.md) | Published results and measurement scope |
| [`HISTORY.md`](HISTORY.md) | Development chronology and historical pack value |
| [`docs/`](docs/) | Domain demonstrations and technical notes |

## Contact

Eric Waller, e@ewaller.com

© 2026 Eric Waller. Public binaries are evaluation builds; private source and
commercial rights are not conveyed by this repository.
