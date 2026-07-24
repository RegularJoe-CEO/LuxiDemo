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

## Current measured result

TESTfort-operated Version 99 measurement, 2026-07-23:

| Configuration | Prefill positions/s | GPU-board J/position | LuxiEdge comparison |
|---|---:|---:|---|
| **LuxiEdge Version 99** | **28,374.7** | **0.018718** | n/a |
| vLLM 0.25.1, default | 35,203.1 | 0.019316 | **80.60%** throughput; **3.10% lower** board energy |
| vLLM 0.25.1, batch-invariant | 30,914.3 | 0.020604 | **91.78%** throughput; **9.15% lower** board energy |

**Measured scope:** Qwen2-7B-Instruct, batch 16, sequence length 128,
packed-prefill positions, one NVIDIA H100 80GB, prefix caching off, cumulative
NVML GPU-board energy. These are not decode tokens, full-serving throughput, or
facility/wall-plug electricity. LuxiEdge did not win throughput in either
comparison.

Open the claim-bearing pack:
[`h100-qwen2-7b-v99-matched-prefill-2026-07-23`](evidence/h100-qwen2-7b-v99-matched-prefill-2026-07-23/).
The technician attestation and selected raw arithmetic/receipt files are
included. The formal signed TESTfort narrative remains pending.

## Run the public demos

The v3.0 release contains compiled evaluation binaries for macOS ARM64, Linux
x86-64, Linux ARM64, Windows x86-64, and a Linux GPU-labelled build.

```bash
# Linux x86-64 example
chmod +x luxiedge-demo-linux-x86_64 luxi-tools-linux-x86_64

./luxiedge-demo-linux-x86_64 validate
./luxiedge-demo-linux-x86_64 list

./luxi-tools-linux-x86_64 validate
./luxi-tools-linux-x86_64 ate
./luxi-tools-linux-x86_64 orbital
./luxi-tools-linux-x86_64 robotics
./luxi-tools-linux-x86_64 energy
```

The public Linux x86-64 v3.0 binaries were downloaded, checksum-matched, and
smoke-tested on 2026-07-23. Both validation entrypoints reported **25 passed,
0 failed**. See
[`docs/demo-receipts/v3.0-linux-x86_64-2026-07-23.md`](docs/demo-receipts/v3.0-linux-x86_64-2026-07-23.md).

Download: [v3.0 release](https://github.com/RegularJoe-CEO/LuxiDemo/releases/tag/v3.0)

Full instructions: [`DEMOS.md`](DEMOS.md)

## Demo and product map

| Surface | What can be run publicly today | Status |
|---|---|---|
| LuxiQuant numerical engine | REST evaluation, operator list, receipt validation | **Working binary demo** |
| ATE / Waller / WNSM primitives | `luxi-tools ate` and `energy` | **Working binary demo** |
| Quant/statistical operators | `validate`, `quant_chain`, Welford, normalization operators | **Working binary demo** |
| Scientific and edge examples | Orbital and robotics commands | **Working binary demo** |
| LuxiEdge Version 99 inference | Public evidence verifier; full engine remains private | **Third-party measured evidence** |
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

- **Current third-party measurement:** Version 99 matched-prefill pack.
- **Current runnable demonstrations:** v3.0 binary receipt and commands.
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
