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
| **LuxiEdge** | Execute | Deterministic numerics with receipts, including orbital propagation and energy-aware inference | **Measured** · orbital, genomics, and H100 inference rows in [`RESULTS.md`](RESULTS.md); Qwen2-7B vs vLLM 0.25.1 and SGLang 0.5.19 in [`BENCHMARKS.md`](BENCHMARKS.md) |
| **LuxiPack** | Schedule | Admit work after dependencies are ready and place it to reduce hold-not-work and repeated preparation | **In development** |
| **LuxiPhase** | Shape | Shape aggregate compute timing to reduce power swings while respecting throughput and SLOs | **Prototype/local validation** |
| **LuxiLoad** | Coordinate | Coordinate compute demand with generation, electrical, cooling, and reliability limits | **Early concept** |
| **LuxiSDG** | Control | Guarded local demand management for smaller commercial sites | **Early concept** |

The shorthand is:

**Compute. Execute. Schedule. Shape. Coordinate. Control.**

## Public portfolio today (what to download)

Public portfolio order on this page: **LuxiRisk, then the proof record, then Luxi Book**. The website on Replit is a separate tree.

| Surface | Role | Where |
|---|---|---|
| **LuxiRisk** | Free offline retail / crypto CLI: liquidation, size, max loss, Ed25519 `lxr1_` receipts | [`luxirisk/`](luxirisk/) · release `luxirisk-v0.2` |
| **LuxiEdge proofs** | Deterministic numerics with receipts. Inference on H100 is one measured surface, next to orbital, genomics, and the option book | [`README.md`](README.md) · [`RESULTS.md`](RESULTS.md) |
| **Luxi Book** | Quant try: CSV European options, five Greeks, output-vector hash + Ed25519 `lxq2_` receipt. The 10-row public book is not the 1,000,000-option book | [`DEMOS.md`](DEMOS.md) · [`downloads/luxibook/`](downloads/luxibook/) |
| Numerical v3.0 demos | REST `/evaluate`, operators, ATE/tools. Supporting math, not the option book | GitHub release `v3.0` · [`DEMOS.md`](DEMOS.md) |

LuxiEdge is deterministic numerics with receipts. Inference is one measured
surface. Luxi Book is the Quant path. LuxiRisk is the free retail CLI.

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
automatically a production or customer result. Hash claims are the rows in
[`RESULTS.md`](RESULTS.md). The 10-row Luxi Book receipt is one measured book.
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
