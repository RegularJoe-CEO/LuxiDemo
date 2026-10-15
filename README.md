# LuxiEdge

**Deterministic numerics with receipts.**

**Deterministic computation you can verify.**

LuxiEdge is the company.

## The only sellable product

**The LuxiQuant Risk State Engine, Risk Runtime v0.4.0, is LuxiEdge's only sellable product.** It is offered only through a paid confidential evaluation under NDA. It is proprietary, has no public download, and has no public price.

It provides deterministic authenticated risk state as a scenario-by-position matrix changes. Customer pricing, market data, scenarios, and methodology remain under customer control. Authoritative rebuilds remain comparators and reconciliation paths.

[Request a confidential evaluation](https://luxiedge.com/pilot).

The public proof claim is scoped reproducibility, not a universal cross-platform promise. Every proof below names its workload, hardware, date, and limits.

## Company structure and free demos

| Name | Official status |
|---|---|
| **LuxiEdge** | Company |
| **LuxiQuant** | Primary commercial division |
| **LuxiQuant Risk State Engine** | LuxiEdge's only sellable product. Risk Runtime v0.4.0 is offered only through a paid confidential evaluation under NDA. |
| **LuxiEnergy** | Secondary division, also described as AI Energy Systems |
| **LuxiBook v0.2.1** | Free option-book pricer demo with Ed25519 receipts |
| **LuxiRisk v0.2** | Free legacy calculator demo |

The markets discussed on luxiedge.com are hypotheses. They are not customer claims and not a list of shipping products.

Review the [official proof page](https://luxiedge.com/proof).

## Proof families on luxiedge.com

### Financial and catastrophe risk

On 2026-10-12, Luxi was tested with Oasis LMF 2.5.8 and the public OasisPiWind model on Linux x86-64 and Apple M1 Pro under Docker linux/arm64. Across the tested 1, 2, and 4 worker configurations, Luxi produced one output hash while stock defaults changed. This is one public model and two tested machine configurations, not a customer result or a universal Oasis claim. See [proofs/oasis-drift](proofs/oasis-drift/OASIS_DRIFT.md).

### GPU numerical primitives

On 2026-10-13, 10,000,000 synthetic float32 dollar values were summed on one NVIDIA L4. Luxi's fixed-order Kahan result matched the exact total and the same result hash was reproduced on the L4, an x86 CPU, and an Apple Silicon Mac. The exact GPU path is a correctness path, not a speed path. No energy result is claimed. See [proofs/gpu-sum](proofs/gpu-sum/README.md).

### Orbital propagation

On 2026-10-10 and 2026-10-11, the named jax100 and starlink-382a419 SGP4 grids produced matching bytes on macOS arm64, Linux arm64, qemu Linux x86_64, and a near-earth NVIDIA H100 build. The x86 result used qemu emulation, not a native x86 host. The H100 build covered near-earth propagation only. The Starlink file contained 79 rejected states on every platform that ran it. This is not a universal cross-platform promise. See [proofs/luxi-sgp4](proofs/luxi-sgp4/README.md).

### Qwen2-7B inference on H100

Measured 2026-09-25 and 2026-09-26 with Qwen2-7B-Instruct fp16 on NVIDIA H100 SXM GPUs:

- Against normal vLLM 0.25.1 during generation, Luxi ran at the same speed within 2% and used 2 to 7% more GPU-board energy per output token.
- On 2k to 32k prompts, Luxi was 3 to 7% faster than normal vLLM and used 2 to 5% less GPU-board energy per prompt token.
- Against vLLM batch-invariant mode, Luxi was faster in every tested cell. Generation was 1.5 to 2.7 times faster and used 16 to 50% less GPU-board energy per token.
- Against SGLang 0.5.19 deterministic mode, Luxi was 1.18 to 1.24 times faster on 2k to 32k prompts with 14 to 19% less GPU-board energy per token. Generation was 1.21 to 2.86 times faster.
- Against normal SGLang, Luxi was 1.3 to 4.6% faster on 2k to 32k prompts with 1.0 to 4.1% less GPU-board energy per token. Generation speed was about equal and Luxi used 1 to 7% more GPU-board energy per token.

The vLLM and SGLang comparisons used different H100 pods. Energy is the NVML GPU-board boundary, not wall-plug or facility energy. Correctness checks matched the top token and ordered top five tokens on the stated prompts; they do not establish full-sequence equivalence. The drift demonstration includes a published long-running-process fault. See [INFERENCE.md](INFERENCE.md), [BENCHMARKS.md](BENCHMARKS.md), and [TEST_SCOPE.md](TEST_SCOPE.md).

## Additional evidence

These results remain public evidence, but they are not top-line proof families on luxiedge.com.

| Evidence | Workload, hardware, and date | Limits |
|---|---|---|
| [Kessler LAFT](proofs/kessler-laft/KESSLER.md) | NCAR LAFT Kessler warm-rain scheme, float64, Intel Xeon, Apple M1 Pro, and NVIDIA A100. The evidence page was added 2026-10-15. | The evidence page does not state a separate run date. The 1,000,000-column case was GPU-only. JAX stayed within rounding of the Fortran reference, so this is a reproducibility result, not an accuracy criticism. |
| [SAAL comparison](proofs/saal-compare/SAAL_COMPARISON.md) | Public 15,940-object catalog, 24-hour grid, Linux x86-64, Linux arm64, macOS arm64, Rosetta 2, and Luxi H100 evidence, dated 2026-10-11. | Linux arm64 was under qemu for the SAAL comparison. Rosetta speed and energy reflect emulation. Timing harnesses differed, so ratios are indicative. |
| [OrbVeil catalog](proofs/orbveil-edge/README.md) | Public 15,940-object catalog, 1,441 epochs, Apple M1 Pro, native Linux arm64 container, and qemu Linux x86_64, measured 2026-10-11. | The Mac was not a Jetson. Linux x86_64 was emulated. Linux rows were hash-only. Apple energy figures are estimated subsystem energy. |
| Genomics | Public GIAB HG002 MHC PairHMM aggregate-score reduction on CPU and NVIDIA H200, measured 2026-06-17. | Not a genotype caller, not a HaplotypeCaller replacement, not a GATK-equivalence claim, and not a claim of identical CPU and GPU bits. See [RESULTS.md](RESULTS.md). |
| [Attention v2](https://github.com/RegularJoe-CEO/attention-v2-benchmarks) | H200 morph versus Flash on the 16k rag_tokenized fixture, measured 2026-06-20. | The 9.84 times result is scoped to compressible KV. On the short-sequence 12-layer H100 comparison, PyTorch Flash fp16 won on joules per token. |

## TestFort status

TestFort independently evaluated a separate seven-function numerical-engine workload in December 2025. It remains valid for that exact workload and does not validate LuxiBook or the September vLLM and SGLang comparisons. [Read the December 2025 report](https://luxiedge.com/luxiedge-validation-report.pdf).

The July 2026 packed-prefill pack remains only in the historical archive and is not part of the current third-party claim.

The proposed independent go-forward test is in [TESTFORT_FORWARD_PROTOCOL.md](TESTFORT_FORWARD_PROTOCOL.md).

## Public demos and evidence

- [Run LuxiBook v0.2.1 and LuxiRisk v0.2](DEMOS.md)
- [Review the measured result ledger](RESULTS.md)
- [Browse the evidence index](evidence/README.md)
- [Read the repository boundary](REPO_BOUNDARY.md)

## Contact

Eric Waller, e@ewaller.com

Copyright 2026 Eric Waller. Public binaries are evaluation builds.
