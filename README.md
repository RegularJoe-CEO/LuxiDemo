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

## Current measured result (version-100 GTM)

**Progress after TESTfort Version 99:** Flash-class attention + device-resident stack + FP16 on the TRADE energy path. On the same prefill protocol class (Qwen2-7B, S=128, H100 board joules), LuxiEdge now **wins thr and board energy** vs vLLM.

Multi-run lock (5×15 s sustains · NVML board joules · det score = multi-run token agreement):

| Batch | Thr median (pos/s) | Board J/pos | Det | vs vLLM thr | vs vLLM J/pos |
|------:|-------------------:|------------:|----:|------------:|--------------:|
| **16** | **~39,865** | **~0.0168** | **1.0** | **~1.17×** | **~10% lower** |
| **32** | **~42,967** | **~0.0160** | **1.0** | **~1.18×** | **~14% lower** |

**Pack:** [`evidence/version-100-h100-gtm/`](evidence/version-100-h100-gtm/)  
**One-pager:** [`PUBLIC_GTM_ONE_PAGER.md`](evidence/version-100-h100-gtm/PUBLIC_GTM_ONE_PAGER.md)  
**H2H brief:** [`PUBLIC_H2H_PREFILL_ENERGY_BRIEF.md`](evidence/version-100-h100-gtm/PUBLIC_H2H_PREFILL_ENERGY_BRIEF.md)

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

## Demo and product map

| Surface | What can be run publicly today | Status |
|---|---|---|
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
