# The Luxi Energy Architecture

Luxi is building the energy architecture for AI compute, from deterministic
computation inside a processor to scheduling and power control at the facility
boundary.

The unifying thesis is simple: energy is wasted when computation is repeated,
moved unnecessarily, executed inefficiently, admitted too early, poorly timed,
or disconnected from the power system supporting it.

## The layers

LuxiEdge is the company, not a layer. It is one core engine with seven commercial products on [luxiedge.com](https://luxiedge.com): Inference, Haplotastic, LuxiQuant Risk State Engine, Orbital, Catastrophe, Edge and LuxiEnergy. They are at different stages of validation, and most have no public binary. The energy-architecture layers below are research and roadmap except where marked.

| Layer | Control point | Purpose | Current maturity |
|---|---|---|---|
| **LuxiQuant** | Compute | LuxiQuant Risk State Engine (Risk Runtime v0.4.2), one of LuxiEdge's commercial products | **Confidential evaluation; no public download or public prices** · free **LuxiBook** v0.2.1 demo (includes the LuxiSize sizing calculator); TestFort Dec 2025 evaluated a defined seven-function numerical-engine workload, not the current Risk State Engine |
| **Inference** | Execute | Reproducible inference across rented GPUs and CPUs | **Public closed-binary replay package** ([`downloads/luxi-replay-demo/`](downloads/luxi-replay-demo/)) · November 2026 R1 evidence in [`INFERENCE.md`](INFERENCE.md) · own workloads by private GPU-swap evaluation · September 2026 Qwen2-7B results are historical ([`BENCHMARKS.md`](BENCHMARKS.md)) |
| **LuxiPak** | Schedule | Admit work after dependencies are ready and place it to reduce hold-not-work and repeated preparation | **In development** |
| **LuxiPhase** | Shape | Shape aggregate compute timing to reduce power swings while respecting throughput and SLOs | **Prototype/local validation** |
| **LuxiLoad** | Coordinate | Coordinate compute demand with generation, electrical, cooling, and reliability limits | **Roadmap** (not on luxiedge.com) |
| **LuxiSDG** | Control | Guarded local demand management for smaller commercial sites | **Historical** (preserved software-defined-generator work; not a currently available product) |

The shorthand is:

**Compute. Execute. Schedule. Shape. Coordinate. Control.**

## Public portfolio today (what to download)

Public downloads today: the **Luxi inference replay demo** (closed binary, Linux x86_64) and **LuxiBook** (free public demonstration, includes LuxiSize, the sizing calculator). The LuxiQuant Risk State Engine and Haplotastic have no public download. The website on Replit is a separate tree.

| Surface | Role | Where |
|---|---|---|
| **Luxi inference replay demo** | Closed-binary R1 replay: GPU or CPU 32-token replay, offline verification, tamper test, record compare, 136-answer GPU suite | [`downloads/luxi-replay-demo/`](downloads/luxi-replay-demo/) · [`INFERENCE.md`](INFERENCE.md) |
| **LuxiEdge proofs** | Deterministic numerics with receipts: inference, orbital, genomics research, catastrophe, GPU sums and the option book | [`README.md`](README.md) · [`RESULTS.md`](RESULTS.md) |
| **LuxiBook** | Quant try: CSV European options, five Greeks, output-vector hash + Ed25519 `lxq2_` receipt. The 10-row public book is not the 1,000,000-option book | [`DEMOS.md`](DEMOS.md) · [`downloads/luxibook/`](downloads/luxibook/) |
| **LuxiSize** (part of LuxiBook) | Sizing calculator included in the free LuxiBook download, not the LuxiQuant Risk State Engine (Risk Runtime v0.4.2): liquidation, size, max loss, Ed25519 `lxr1_` receipts | [`luxirisk/`](luxirisk/) · release `luxirisk-v0.2` |
| Numerical v3.0 demos | REST `/evaluate`, operators, ATE/tools. Supporting math, not the option book | GitHub release `v3.0` · [`DEMOS.md`](DEMOS.md) |

LuxiEdge is deterministic numerics with receipts. Inference has a public closed-binary replay demo; its engine
source stays private. The LuxiQuant Risk State Engine (Risk Runtime v0.4.2) is available by confidential evaluation,
with no public download. LuxiBook is the free public demonstration. It includes LuxiSize, the sizing calculator.

The later layers (LuxiPak, LuxiPhase, LuxiLoad, LuxiSDG) are research, roadmap, or historical work at different
stages of proof; they are not presented as deployed systems.

Every public result should identify:

- workload and accepted work unit
- hardware and software
- measurement boundary
- reproducibility or numerical contract
- validation status
- retained evidence

GPU-board energy is not facility electricity. A controlled benchmark is not
automatically a production or customer result. Hash claims are the rows in
[`RESULTS.md`](RESULTS.md). The 10-row LuxiBook receipt is one measured book.
It does not say that every CPU and GPU matches, and it is not the
1,000,000-option book.

## Demo rule

Every layer earns “working demo” status only after it has:

1. retained inputs and outputs,
2. a defined baseline,
3. a repeatable command or executable,
4. a measurement boundary,
5. a clear statement of what the demonstration establishes.

See [`DEMOS.md`](DEMOS.md) for current coverage and [`RESULTS.md`](RESULTS.md)
for measured tables.
