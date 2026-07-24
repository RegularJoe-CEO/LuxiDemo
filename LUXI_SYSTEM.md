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
| **LuxiQuant** | Compute | Reproducible numerical, quantitative, statistical, and scientific execution with receipts | **Working engine**; independently evaluated on a defined numerical workload |
| **LuxiEdge** | Execute | Energy-aware GPU execution for AI inference, with packed work and scoped deterministic receipts | **Primary public product**; Version 99 third-party measured |
| **LuxiPack** | Schedule | Admit work after dependencies are ready and place it to reduce hold-not-work and repeated preparation | **In development** |
| **LuxiPhase** | Shape | Shape aggregate compute timing to reduce power swings while respecting throughput and SLOs | **Prototype/local validation** |
| **LuxiLoad** | Coordinate | Coordinate compute demand with generation, electrical, cooling, and reliability limits | **Early concept** |
| **LuxiSDG** | Control | Guarded local demand management for smaller commercial sites | **Early concept** |

The shorthand is:

**Compute. Execute. Schedule. Shape. Coordinate. Control.**

## What is public today

LuxiEdge is the public AI wedge. LuxiQuant has runnable compiled demonstrations.
The later layers are a product roadmap at different stages of proof; they are
not presented as deployed systems.

Every public result should identify:

- workload and accepted work unit
- hardware and software
- measurement boundary
- reproducibility or numerical contract
- validation status
- retained evidence

GPU-board energy is not facility electricity. A controlled benchmark is not
automatically a production or customer result.

## Demo rule

Every layer earns “working demo” status only after it has:

1. retained inputs and outputs,
2. a defined baseline,
3. a repeatable command or executable,
4. a measurement boundary,
5. a clear statement of what the demonstration establishes.

See [`DEMOS.md`](DEMOS.md) for current coverage.
