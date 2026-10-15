# LuxiEnergy measurement boundaries

LuxiEnergy is LuxiEdge's secondary division, also described as AI Energy Systems. It is not a separate sellable product. LuxiEdge's only sellable product is the LuxiQuant Risk State Engine, Risk Runtime v0.4.0. This page summarizes measured energy evidence and roadmap boundaries. It does not describe a standalone shipping product.

## Current measured inference evidence

The Qwen2-7B-Instruct fp16 comparisons were measured on NVIDIA H100 SXM GPUs on 2026-09-25 and 2026-09-26.

- Against normal vLLM 0.25.1 during generation, Luxi ran at the same speed within 2% and used 2 to 7% more GPU-board energy per output token.
- On 2k to 32k prompts, Luxi was 3 to 7% faster than normal vLLM and used 2 to 5% less GPU-board energy per prompt token.
- Against vLLM batch-invariant mode, generation was 1.5 to 2.7 times faster and used 16 to 50% less GPU-board energy per token.
- Against SGLang 0.5.19 deterministic mode, the tested 2k to 32k prompt cells were 1.18 to 1.24 times faster with 14 to 19% less GPU-board energy per token. Generation was 1.21 to 2.86 times faster.

The vLLM and SGLang comparisons used different H100 pods. Energy came from the NVML GPU total-energy counter, with no idle subtraction. These are GPU-board measurements, not CPU, memory, cooling, wall-plug, or facility-energy measurements. Prefill is not complete inference.

See [BENCHMARKS.md](../BENCHMARKS.md) and [TEST_SCOPE.md](../TEST_SCOPE.md).

## Historical TestFort evaluation

TestFort independently evaluated a separate seven-function numerical-engine workload in December 2025. It is not LuxiBook and not the current inference comparison. [Read the historical report](https://luxiedge.com/luxiedge-validation-report.pdf).

## Roadmap framing

| Area | Status |
|---|---|
| AI compute | Measured on the specifically named inference workloads |
| Memory movement | Research and roadmap |
| LuxiPak and LuxiPhase | Workload-behavior roadmap, not shipping products |
| LuxiSDG | Historical software-defined-generator work |

No repo result establishes a system-wide energy advantage, facility savings, or lifecycle savings from avoided reruns. Those effects have not been measured.

[Request a confidential evaluation](https://luxiedge.com/pilot) or review [luxiedge.com/proof](https://luxiedge.com/proof).
