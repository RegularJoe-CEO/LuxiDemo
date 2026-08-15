# The Luxi Energy Architecture

Luxi is building the energy architecture for AI compute, from deterministic
computation inside a processor to scheduling and power control at the facility
boundary.

The unifying thesis is simple: energy is wasted when computation is repeated,
moved unnecessarily, executed inefficiently, admitted too early, poorly timed,
or disconnected from the power system supporting it.

## The layers

| Layer | Control point | Purpose | Current maturity |
|---|---|---|---|
| **LuxiQuant** | Compute | Reproducible numerical and quantitative execution with receipts | **Working engine** · public **Luxi Book** (CSV BS/Black-76 + SHA-256) is the Quant try without NDA; older numerical REST/microbench demos still ship; TestFort Dec 2025 on a defined numerical suite |
| **LuxiEdge** | Execute | Energy-aware GPU execution for AI inference, with packed work and scoped deterministic receipts | **Public AI wedge** · absolute + matched prefill measured on H100; Version 99 third-party lineage |
| **LuxiPack** | Schedule | Admit work after dependencies are ready and place it to reduce hold-not-work and repeated preparation | **In development** |
| **LuxiPhase** | Shape | Shape aggregate compute timing to reduce power swings while respecting throughput and SLOs | **Prototype/local validation** |
| **LuxiLoad** | Coordinate | Coordinate compute demand with generation, electrical, cooling, and reliability limits | **Early concept** |
| **LuxiSDG** | Control | Guarded local demand management for smaller commercial sites | **Early concept** |

The shorthand is:

**Compute. Execute. Schedule. Shape. Coordinate. Control.**

## Public portfolio today (what to download)

Order on the public site: **sale → freebie → evidence**.

| Surface | Role | Where |
|---|---|---|
| **Luxi Book** | **Primary Quant try** — CSV European options, five Greeks, SHA-256 receipt; macOS + Linux CPU + Linux CUDA closed binaries | [`site/demo.html`](site/demo.html) · [`site/downloads/luxibook/`](site/downloads/luxibook/) |
| **LuxiRisk** | **Freebie** retail / crypto risk CLI + Ed25519 `lxr1_` receipts — not institutional Quant | [`luxirisk/`](luxirisk/) · release `luxirisk-v0.2` |
| **LuxiEdge serve / thr·J packs** | Inference evidence and stripped serve binary (scoreboard + toy generate path) | [`site/demo.html#also`](site/demo.html#also) · [`evidence/`](evidence/) |
| Numerical v3.0 demos | REST `/evaluate`, operators, ATE/tools — supporting quant math, not the option book | GitHub release `v3.0` · [`DEMOS.md`](DEMOS.md) |

LuxiEdge remains the public **AI inference** wedge. Luxi Book is the public
**Quant** path that can become a design-partner conversation. LuxiRisk is a
separate free product surface, not a substitute for Book.

The later layers (Pack, Phase, Load, SDG) are a product roadmap at different
stages of proof; they are not presented as deployed systems.

Every public result should identify:

- workload and accepted work unit
- hardware and software
- measurement boundary
- reproducibility or numerical contract
- validation status
- retained evidence

GPU-board energy is not facility electricity. A controlled benchmark is not
automatically a production or customer result. A receipt on one CSV book is not
a universal CPU↔GPU claim.

## Demo rule

Every layer earns “working demo” status only after it has:

1. retained inputs and outputs,
2. a defined baseline,
3. a repeatable command or executable,
4. a measurement boundary,
5. a clear statement of what the demonstration establishes.

See [`DEMOS.md`](DEMOS.md) for current coverage and [`RESULTS.md`](RESULTS.md)
for measured tables.
