# Published results

Last updated: 2026-08-16

This page indexes **measured** public results. Each result is limited to the
workload and hardware described in its evidence. Do not blend numbers across
sections.

## Luxi Book — CSV European options (primary Quant try)

**Workload:** `example_book.csv` only · European Black-Scholes / Black-76 · five
Greeks · SHA-256 over the canonical f64 little-endian price+Greeks vector.  
**Kernel:** `black_scholes_book_kernel` (CPU gold; optional CUDA on Linux).  
**Engine:** v0.2.0 (`b4645c2`) ships Ed25519 `lxq2_` seals; the **comparable** value across boxes remains the output vector hash below.

| Surface | Result |
|---|---|
| Output vector SHA-256 | `4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a` |
| ATM_CALL price | `10.4505835721856215` |
| v0.2.0 matrix (engine `b4645c2`) | **RTX 4090 (Ada)** + H200 + H100×2 devices (CPU+GPU agree) + Mac Mini arm64 CPU; receipts under [`downloads/luxibook/evidence/v0.2.0-matrix/`](downloads/luxibook/evidence/v0.2.0-matrix/) |
| Historical A100 (2026-08-15, pre-attestation binary) | same **output hash** only — not re-measured under v0.2.0 |
| Public binaries | macOS ARM64 CPU · Linux x86_64 CPU · Linux x86_64 CUDA |

**Non-claims:** this book, these devices, this kernel — not “all GPUs always
match,” not desk VaR, not live market data, not `risk-pipeline`. Do not treat the
`lxq2_…` string as a published constant (per-install seal).

- Binaries: [`downloads/luxibook/`](downloads/luxibook/)
- Matrix receipts: [`downloads/luxibook/evidence/v0.2.0-matrix/`](downloads/luxibook/evidence/v0.2.0-matrix/)
- How to run: [`DEMOS.md`](DEMOS.md)

## LuxiEdge absolute prefill (internal multi-run champion)

**Workload:** Qwen2-7B-Instruct class · S=128 · B72 · dual_gemm  
(`LUXI_GEMM_DUAL_STREAM=1`) · flash + device-resident + FP16 · one H100 80GB ·
NVML board joules · 5×15 s multi-run · 2026-08-07.

| Operating point | Thr median (pos/s) | Board J/pos |
|---|---:|---:|
| **B72 dual_gemm** | **~44,860** | **~0.0153** |

Pack:
[`evidence/prefill_accel_lock_20260807T233111Z/`](evidence/prefill_accel_lock_20260807T233111Z/)

**Not** a matched vLLM claim at B72.

## LuxiEdge matched vLLM H2H (B16 / B32 only)

| Batch | Luxi thr (pos/s) | Luxi J/pos | vs vLLM thr | vs vLLM J/pos |
|------:|-----------------:|-----------:|------------:|--------------:|
| **16** | **~41,221** | **~0.0169** | **~1.18×** | **~12% lower** |
| **32** | **~43,464** | **~0.0158** | **~1.19×** | **~14% lower** |

Pack:
[`evidence/prefill_freeze_matched_20260807T210749Z/`](evidence/prefill_freeze_matched_20260807T210749Z/)

Scope: prefill positions only · GPU board energy · not decode · not wall-plug ·
not multi-tenant full-server leadership.

## LuxiEdge Version 99 (prior third-party baseline)

Technician-operated matched prefill on one NVIDIA H100 80GB (2026-07-23).
**Earlier stack** — thr trailed vLLM; kept for lineage.

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

## LuxiQuant numerical engine (microbench / REST — not the option book)

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
No thr/J table — product is signed calculation receipts, not a GPU scoreboard.

## Historical transformer research

Earlier H100 experiments remain available with their original configurations,
including TRADE, Flash comparisons, WNSM, long-context scaling, and sustained
runtime work. See [`evidence/README.md`](evidence/README.md) and
[`HISTORICAL_BENCHMARKS.md`](HISTORICAL_BENCHMARKS.md).
