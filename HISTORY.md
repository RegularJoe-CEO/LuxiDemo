# Luxi public evidence history

The older Luxi evidence is important. It records what was tried, what worked,
what failed, and which mechanisms led to the current engine. Historical does
not mean worthless; it means the result belongs to its original configuration
and should not be mistaken for the current headline.

## Timeline

### December 2025: deterministic numerical engine

TestFort independently evaluated a defined LuxiEdge numerical workload on an
NVIDIA H100. The report records repeated CPU/GPU receipt agreement, a sustained
load test, throughput, latency, and power for that workload.

This is the foundation now described as **LuxiQuant numerical** (REST /
operators / microbench). It is separate from transformer-inference measurements
and from the later **Luxi Book** option-pricing path.

### June 2026: attention and receipt mechanisms

Public demonstrations and research artifacts covered Waller/ATE behavior,
WNSM receipts, long-context mechanisms, and early H100/H200 shapes.

### July 11, 2026: public H100 research packs

The repository published:

- full 7B-class TRADE sustain measurements
- 12-layer device-resident energy
- a same-shape PyTorch/Flash comparison that Luxi lost
- WNSM free-ride evidence
- long-context memory scaling
- baseline/geodesic wedges
- continuous-batch sustain context

Those packs remain public without rewriting their raw results.

### July 21 to 22, 2026: packing and runtime breakthrough

The private engine reduced attention launches, integrated a pinned Flash
backend, improved multi-sequence packing, and crossed the project’s peer-energy
gate under local testing. These engineering records explain the step change
between the July 11 historical pack and Version 99.

### July 23, 2026: Version 99 technician-operated measurement

A TESTfort technician ran the frozen matched-prefill protocol on one H100.
LuxiEdge measured:

- 28,374.7 prefill positions/s
- 0.018718 GPU-board J/position
- 80.60% of default-vLLM throughput with 3.10% lower board energy
- 91.78% of batch-invariant-vLLM throughput with 9.15% lower board energy

The current public pack preserves the technician attestation, per-run table,
arithmetic audit, anomaly log, and selected receipts. The signed narrative is
still pending.

### Late July 2026: version-100 GTM lock

Multi-run H100 thr/J/det lock and matched prefill vs vLLM on B16/B32 (TRADE
energy path). Public pack under `evidence/version-100-h100-gtm/`. Local serve
binary embeds the scoreboard on `GET /v1/gtm`.

### Early August 2026: absolute prefill champion + matched freeze

- B72 dual_gemm multi-run absolute thr/J lock
  (`evidence/prefill_accel_lock_20260807T233111Z/`)
- Separate matched vLLM H2H freeze at B16/B32
  (`evidence/prefill_freeze_matched_20260807T210749Z/`)

Rule: absolute B72 is not a matched vLLM claim until a B72 vLLM arm exists.

### Mid August 2026: Luxi Book public Quant surface

Closed `luxi-book` binaries published under `site/downloads/luxibook/`:

- CSV European Black-Scholes / Black-76 + five Greeks + SHA-256
- macOS ARM64 CPU, Linux x86_64 CPU, Linux x86_64 CUDA
- Measured receipt on `example_book.csv`:
  `4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a`
  on Mini CPU · Linux CPU · A100 · H100 · H200

Public site order: **Book (sale) → LuxiRisk freebie → inference demoted**.
LuxiRisk v0.2 remains the free retail/crypto CLI with `lxr1_` receipts — not
the option book.

## How to read old packs

Use each pack’s original:

- model or shape
- work-unit definition
- precision
- hardware
- timing window
- energy boundary
- comparator

Do not combine a throughput number from one pack with watts from another.
Do not carry a result from a research lane into a faithful-model lane.
Do not treat numerical-engine ops/s as option-book thr or as inference thr/J.

The current index is [`evidence/README.md`](evidence/README.md).
Measured tables: [`RESULTS.md`](RESULTS.md).
