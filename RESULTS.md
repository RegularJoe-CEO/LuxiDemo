# Published results

Last updated: 2026-09-26

This page indexes **measured** public results. Each result is limited to the
workload and hardware described in its evidence. Do not blend numbers across
sections.

## Luxi Book - CSV European options (primary Quant try)

**Workload:** `example_book.csv` only · European Black-Scholes / Black-76 · five
Greeks · SHA-256 over the canonical f64 little-endian price+Greeks vector.  
**Kernel:** `black_scholes_book_kernel` (CPU gold; optional CUDA on Linux).  
**Engine:** v0.2.0 (`b4645c2`) ships Ed25519 `lxq2_` seals; the **comparable** value across boxes remains the output vector hash below.

| Surface | Result |
|---|---|
| Output vector SHA-256 | `4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a` |
| ATM_CALL price | `10.4505835721856215` |
| v0.2.0 matrix (engine `b4645c2`) | **RTX 4090 (Ada)** + H200 + H100×2 devices (CPU+GPU agree) + Mac Mini arm64 CPU; receipts under [`downloads/luxibook/evidence/v0.2.0-matrix/`](downloads/luxibook/evidence/v0.2.0-matrix/) |
| Historical A100 (2026-08-15, pre-attestation binary) | same **output hash** only - not re-measured under v0.2.0 |
| Public binaries | macOS ARM64 CPU · Linux x86_64 CPU · Linux x86_64 CUDA |

**Non-claims:** this book, these devices, this kernel - not “all GPUs always
match,” not desk VaR, not live market data, not `risk-pipeline`. Do not treat the
`lxq2_…` string as a published constant (per-install seal).

- Binaries: [`downloads/luxibook/`](downloads/luxibook/)
- Matrix receipts: [`downloads/luxibook/evidence/v0.2.0-matrix/`](downloads/luxibook/evidence/v0.2.0-matrix/)
- How to run: [`DEMOS.md`](DEMOS.md)

## LuxiEdge vs vLLM 0.25.1 - Qwen2-7B on H100 (2026-09-25/26)

Luxi runs Qwen2-7B on an H100 with bit-identical results across every batch size we tested (1, 16 and 64 when generating tokens; 1 against 16, 4 and 2 on 2k, 8k and 32k-token prompts), across repeated runs and separate processes. On long prompts (2k to 32k tokens) it is 3–7% faster than vLLM 0.25.1 and uses 2–5% less energy per token. When generating tokens it matches vLLM's speed but uses 2–7% more energy per token. Against vLLM's own deterministic (batch-invariant) mode, Luxi is faster and uses less energy on every test, including 1.5–2.7× faster token generation with 16–50% less energy.

**Prefill (long prompts)** - throughput tok/s and energy J per token:

| Prompt length × batch | vLLM 0.25.1 default | vLLM batch-invariant mode | Luxi (FA3) | Luxi own attention kernel |
|---|---:|---:|---:|---:|
| 2048 × 16 | 44,680 tok/s, 0.01543 J | 43,190, 0.01601 | 46,180, 0.01515 | 44,270, 0.01579 |
| 8192 × 4 | 41,120, 0.01688 | 40,070, 0.01734 | 42,350, 0.01647 | 36,700, 0.01905 |
| 32767 × 1 | 29,980, 0.02269 | 29,240, 0.02325 | 32,190, 0.02162 | 21,820, 0.03158 |

**Decode** (1024-token prompt, 256 greedy generated tokens) - throughput tok/s and energy J per output token:

| Batch | vLLM default | vLLM batch-invariant mode | Luxi (FA3) | Luxi own attention kernel |
|---:|---:|---:|---:|---:|
| 1 | 165.8, 2.645 | 61.69, 5.626 | 165.5, 2.820 | 162.1, 2.784 |
| 16 | 2078, 0.2558 | 941, 0.4233 | 2111, 0.2672 | 1832, 0.2813 |
| 64 | 4875, 0.1313 | 3217, 0.1596 | 4898, 0.1338 | 3740, 0.1511 |

Method, determinism, why generation uses more energy, and what is borrowed vs
Luxi's own: [`BENCHMARKS.md`](BENCHMARKS.md). Run summary:
[`evidence/h100-qwen2-7b-vs-vllm-0.25.1-2026-09-25/`](evidence/h100-qwen2-7b-vs-vllm-0.25.1-2026-09-25/)

SGLang 0.5.19 tables (separate H100 pod): [`BENCHMARKS.md`](BENCHMARKS.md#sglang-0519-deterministic-and-normal-mode) · drift demo: [`INFERENCE.md`](INFERENCE.md#drift-demo)

Scope: one H100 SXM · GPU board energy (NVML) · not wall-plug · not
multi-tenant full-server leadership · full scope: [`TEST_SCOPE.md`](TEST_SCOPE.md)

## LuxiEdge Version 99 (prior third-party baseline)

Technician-operated matched prefill on one NVIDIA H100 80GB (2026-07-23).
**Earlier stack** - thr trailed vLLM; kept for lineage.

| Engine | Prefill positions/s | GPU-board J/position |
|---|---:|---:|
| LuxiEdge Version 99 | **28,374.7** | **0.018718** |
| vLLM 0.25.1, default | 35,203.1 | 0.019316 |
| vLLM 0.25.1, batch-invariant | 30,914.3 | 0.020604 |

On this test, LuxiEdge reached 80.60% of default vLLM throughput and used 3.10%
less GPU-board energy per position. Against vLLM batch-invariant mode,
LuxiEdge reached 91.78% of its throughput and used 9.15% less GPU-board energy
per position.

Test configuration: Qwen2-7B-Instruct, batch 16, sequence length 128, packed
prefill, prefix caching disabled, cumulative NVML energy, one H100 80GB.

Evidence:
[`h100-qwen2-7b-v99-matched-prefill-2026-07-23`](evidence/h100-qwen2-7b-v99-matched-prefill-2026-07-23/)

## LuxiQuant numerical engine (microbench / REST - not the option book)

A December 2025 TestFort evaluation reported:

| Measurement | Result |
|---|---:|
| Aggregate rate across the seven tested functions | 286.94 billion operations/s |
| Peak tested square-root rate | 331.13 billion operations/s |
| Operations reported during the one-hour run | 444.4 trillion |
| Request failures | 0 |
| Matching GPU and CPU output hash | 5 of 5 runs on each path |
| Average GPU power during sustained load | approximately 117.2 W |

[Open the numerical-engine validation report](https://luxiedge.com/luxiedge-validation-report.pdf)

These measurements apply to the **numerical expression engine** and its defined
test suite. They are **not** transformer-inference measurements and **not**
Luxi Book option-pricing results.

## LuxiRisk freebie

Retail / crypto liquidation, size, and stop-loss CLI with Ed25519 `lxr1_`
receipts. Not institutional Quant. See [`luxirisk/`](luxirisk/) and release
[luxirisk-v0.2](https://github.com/RegularJoe-CEO/LuxiDemo/releases/tag/luxirisk-v0.2).
No thr/J table - product is signed calculation receipts, not a GPU scoreboard.

## Historical transformer research

Earlier H100 experiments remain available with their original configurations,
including TRADE, Flash comparisons, WNSM, long-context scaling, and sustained
runtime work. See [`evidence/README.md`](evidence/README.md) and
[`HISTORICAL_BENCHMARKS.md`](HISTORICAL_BENCHMARKS.md).
