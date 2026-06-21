# attention-transformer

**The Deterministic Inference Engine for High-Stakes Quantitative Workloads — The Quantum Trader Edition**

This is a production-grade, bit-exact, auditable transformer inference engine designed specifically for environments where **reproducibility, auditability, and energy efficiency are non-negotiable** — quantitative trading desks, quantum trading strategies, model risk management, regulated AI systems, and scientific computing.

Two operating modes, both fully deterministic:

- **Gold Standard (f32)** — Maximum mathematical fidelity with cryptographic receipts. The source of truth.
- **Experimental int8 turbo (quarantined)** — Optional `forward_turbo(...)` method, deterministic within a fixed quantization configuration and producing its own distinct receipt, but **not the canonical path**. On this Mac it is slower than optimized f32, and the SIMD int8 path has shown a segfault. It is retained for future edge/bandwidth-starved experiments only.

The **pure-Rust CPU path is the golden reference** — every claim below is verified live on commodity hardware (Apple Silicon MacBook, no GPU).

> **Deep dive:** For a verbose, step-by-step explanation of *how* and *why* this works — the math, the determinism discipline, the energy model, and where the novelty lies — see [`docs/TECHNICAL_DEEP_DIVE.md`](docs/TECHNICAL_DEEP_DIVE.md).
>
> **Agent brief (shareable, no repo required):** [`docs/AGENT_TECHNOLOGY_BRIEF.md`](docs/AGENT_TECHNOLOGY_BRIEF.md) — IP notice, full technology explanation, AUDIT vs TRADE, and how to help without leaking.
>
> **LuxiEdge build (ordered):** [`docs/LUXIEDGE_BUILD_ROADMAP.md`](docs/LUXIEDGE_BUILD_ROADMAP.md) — P&L backtest + **NPOW** scaling → 70B AUDIT → FP8 parity → PyTorch.

---

## Why Quantum Traders & High-Stakes Teams Choose This Engine

Traditional inference engines optimize for raw speed and sacrifice reproducibility. This engine inverts the priority:

- **Cryptographic, machine-independent receipts** — Prove exactly what the model output was for any input. Critical for model risk, audit, dispute resolution, and regulatory compliance.
- **One canonical deterministic path** — Run optimized f32 (`rayon + simd`) for the verified path of record. The experimental int8 path is deterministic within its own quantized configuration, but it is quarantined because it is slower on this Mac and unsafe under the SIMD feature.
- **Proven energy & memory efficiency** — O(N) attention + load-time packing + WNSM deliver dramatic reductions in data movement (the dominant cost of inference). This translates directly to lower latency variance and lower infrastructure cost at scale.
- **Edge / Laptop first** — Runs well on CPU (Apple Silicon, x86, ARM) with no GPU dependency. Ideal for research, simulation farms, and air-gapped environments.
- **Future-proof GPU path** — All core primitives (deterministic kernels, packing, WNSM, recurrent KV) are designed to transfer to GPU while preserving the same determinism guarantees.

If your P&L, risk limits, or regulatory obligations depend on knowing *exactly* what a model did on a given set of inputs — this is the engine built for you.

---

## Quick Start (Any Machine — No GPU Needed)

```bash
git clone https://github.com/RegularJoe-CEO/attention-transformer.git
cd attention-transformer
cargo run --release --example production_demo
```

Expected output:
- Identical NORMAL / WNSM receipts (`e1980a6f…37628`)
- Max output difference of `0.00e0`
- Concrete payload bytes avoided + joules saved
- ✅ Production claims verified

This single command exercises the **complete production path**: real weight-based QKV + output projections, Waller fused online-softmax attention, full WNSM null-space payload transport, cryptographic receipts, and explicit energy accounting. It works identically on a MacBook Air, a Windows laptop, an ARM device, or a server.

For a multi-core speed boost (Apple M-series, Intel, AMD):

```bash
cargo run --release --example production_demo --features rayon
```

---

## Scaling Evidence (Measured)

The O(N)-vs-O(N²) memory claim is backed by real measured data, not estimates. At 131,072 tokens the Waller Operator uses **201 MB** versus standard attention's **68.7 GB** — a **341× reduction** — while runtime scales linearly. Full table, chart, and reproduction steps are in [`docs/SCALING_EVIDENCE.md`](docs/SCALING_EVIDENCE.md).

```bash
cargo run --release --example scaling_sweep > scaling.csv
```

## Energy Evidence (Measured)

Data movement — not arithmetic — is the dominant electricity cost in transformer inference, at roughly **~20 pJ per byte** moved to/from HBM. Standard attention's energy grows **quadratically** with sequence length; the Waller Operator grows **linearly**. The savings factor **doubles every time the sequence doubles** — reaching a **4,096× attention-energy reduction** at 262,144 tokens. The citable HBM figure also reproduces the engine's own `EnergyReport` constant. Full table, chart, and reproduction steps are in [`docs/ENERGY_EVIDENCE.md`](docs/ENERGY_EVIDENCE.md).

```bash
cargo run --release --example energy_sweep > energy.csv
```

---

## Quantum Trader / High-Stakes Workflow (Recommended)

For quantum trading desks and any workload requiring provable reproducibility:

```rust
let model = Gpt2Model::from_tensors(tensors, config).expect("model");

// Gold standard (f32) — the DEFAULT path. Maximum determinism + full
// cryptographic receipts. Bit-locked to receipt 756a50a3…b9c8.
let (logits, receipt) = model.forward(&prompt_tokens);
let audit = model.create_audit_report(&prompt_tokens, "live decision 2026-02-18");

// Experimental turbo mode (OPTIONAL, deterministic int8) — own distinct receipt.
// Quarantined: slower than optimized f32 on this Mac and unsafe under SIMD.
// Do not use as the canonical path of record.
let (turbo_logits, turbo_receipt) = model.forward_turbo(&prompt_tokens);
```

Run the real GPT-2 (124M) end-to-end on CPU (f32 default path):

```bash
cargo run --release --features gpt2 --example gpt2_demo -- "Once upon a time"
```

Store the `AuditReport` with every decision. Receipts are the audit trail.

---

## Performance Levers (Verified)

The pure-Rust path has been progressively optimized while keeping the receipts bit-exact at every step:

- **Tiled matmul + Rayon** — parallelized over independent output rows with fixed left-to-right accumulation order. ~6.6× over the naive baseline.
- **ARM NEON SIMD** (`--features simd`) — N-dimension vectorized kernel that stays bit-identical to the scalar reference (separate multiply + add, no fused-multiply-add reordering). ~9× total over naive.
- **Recurrent KV State (the fast generation path)** — the Waller operator is a streaming online-softmax, so its `(max, sum_exp, accumulator)` state can be carried across tokens. This makes autoregressive generation cost **flat per token instead of quadratic**, measured at **7.6× faster end-to-end at 40 tokens** (the gap widens with length). Proven bit-identical to full recompute by `tests/gpt2_incremental_equiv.rs`. **This is the single recommended fast path for text generation** and is wired into `gpt2_demo` and benchmarked by `kv_speedup`.
- **Waste elimination (allocation/lookup)** — per-layer weight tensors are resolved once at load (no per-token `format!` + HashMap hashing); attention heads are parallelized over rayon; `layer_norm` writes in-place and residual adds / head scatter use slice copies. All bit-exact; biggest benefit on bandwidth-starved edge devices.
- **Cache-blocked GEMM primitive (`matmul_blocked`)** — added as a deterministic standalone linear-algebra primitive in `src/linalg.rs`. It is **not wired into the sacred GPT-2 f32 path**, so it does not change receipts or runtime behavior yet. Future agents may benchmark or selectively wire it only after preserving or intentionally versioning receipts.
- **Speculative Decoding (research artifact — not a CPU speed lever)** — a cheap geometric/induction drafter plus a batched verifier was implemented and proven **bit-identical to greedy**. It measured 1.75× on highly repetitive text but is **slower than the recurrent KV path on compute-bound CPUs** (it competes with, rather than stacks on, KV recurrence). A trained draft head was also evaluated and found ineffective for GPT-2 124M greedy decoding (~1–13% acceptance). `examples/gpt2_speculative.rs` is retained as a documented research artifact; use `gpt2_demo` for real generation.

Every lever above preserves the `production_demo` receipt (`e1980a6f…37628`) and the GPT-2 logits receipt. Determinism is never traded for speed.

---

## Public API (Production Surface)

```rust
use attention_transformer::{Config, WNSM_GAE_Decoder, sha256_of_f32_slice, format_receipt, EnergyReport};

let cfg = Config::new(64, 4, 256, 128);
let model = WNSM_GAE_Decoder::new(cfg, 3);

let (out, payload) = model.forward_wnsm_chained(input, seq_len, None);
let receipt = sha256_of_f32_slice(&out);
let energy = EnergyReport::compute(seq_len, hidden, mlp, layers, payload_dim, /*wnsm_active*/ true);
```

## Architecture (Clean Layout)

- `src/wnsm_transformer.rs` — the complete production assembly (Waller attention + real WNSM math + receipts + energy)
- `src/linalg.rs` — deterministic matmul (tiled / NEON), add-only Walsh-Hadamard transform
- `src/waller_operator.rs` — single-pass online-softmax attention + `WallerKVState` recurrent cache
- `src/gpt2/` — feature-gated GPT-2 runner (loader, tokenizer, model forward, draft head)
- `examples/production_demo.rs` — the runnable verification of all core claims
- GPU backends (CUDA / Metal / WebGPU) live in their own directories — see GPU section below for honest status

## Edge, Laptops & Constrained Devices (Primary Strength)

The core algorithmic wins land hardest exactly where they matter most — laptops, ARM devices, phones, and other power- or memory-constrained hardware:

- **Waller fused attention** — O(N) memory instead of O(N²), dramatically reducing memory-bandwidth pressure and power draw.
- **WNSM** — payloads (KV cache, routing, state) travel in the null space of existing MLP weights with near-zero extra data movement.
- **Pure Rust** runs everywhere with excellent determinism and cryptographic receipts. The edge/`no_std`-friendly build is verified by `--no-default-features` (18 tests pass).

---

## GPU Paths — CUDA Verified on H100 (Two Lanes)

The pure-Rust CPU path remains the **golden reference**. **CUDA** (`--features cuda`) is **verified on NVIDIA H100** (RunPod, 2026-06): full decoder receipt matches CPU bit-for-bit in **Lane AUDIT**.

| Lane | Enable | Path | Receipt |
|------|--------|------|---------|
| **AUDIT** | `LUXI_RECEIPT_AUDIT=1` (set in `cuda_verify`) | CPU QKV → GPU Waller+wo → CPU MLP | `0ae659948eabc3fa1212b84d9a2006c707c28ba4209ce28410df676d38d37ada` |
| **TRADE** (default) | unset | GPU geodesic layer (LN1+QKV+attn+MLP) | Faster desk path; separate receipt namespace |

**Two different benchmarks — do not mix them:**

| Bench | What it measures | seq=1024 (H100) |
|-------|------------------|-----------------|
| `cuda_bench` | **Waller attention only** (Q,K,V on GPU) | **~3–4 ms** |
| `cuda_layer_bench` | **One full layer** (quant TRADE) | **~6.8 ms** |
| `cuda_quant_bench` | **12-layer decoder** (one H2D, GPU stack, one D2H) | **~69 ms** (~5.8 ms/layer) |

**Locked spec:** [`docs/QUANT_TRADE_LOCKED.md`](docs/QUANT_TRADE_LOCKED.md) · RunPod gate: `bash scripts/runpod_quant_gate.sh`

TRADE defaults: parallel waller+`wo`, batched GEMM MLP, shared `cuda_trade_stream`, multi-layer `cuda_use_quant_stack()`. AUDIT: `LUXI_RECEIPT_AUDIT=1`.

- **Metal / WebGPU**: stubs only, not wired.

### RunPod / NVIDIA

```bash
git clone https://github.com/RegularJoe-CEO/attention-transformer.git
cd attention-transformer
export CUDA_ARCH=90
./scripts/runpod_cuda_check.sh
```

```bash
bash scripts/runpod_quant_gate.sh
# or manually:
LUXI_RECEIPT_AUDIT=1 cargo run --release --features cuda --example cuda_verify
cargo run --release --features cuda --example cuda_layer_bench -- 20 1024 1024 16 256
cargo run --release --features cuda --example cuda_quant_bench -- 20 1024 12
LUXI_NPOW_FAST=1 cargo run --release --example npow_scaling_proof
```

**NPOW** (O(N) memory witness on WNSM bus): fast smoke above; full sweep `cargo run --release --example npow_scaling_proof` for 131k timing evidence.

See [`docs/QUANT_TRADE_LOCKED.md`](docs/QUANT_TRADE_LOCKED.md) (ops + gates) and [`docs/TECHNICAL_DEEP_DIVE.md`](docs/TECHNICAL_DEEP_DIVE.md).

---

## Status Summary

- **Pure-Rust CPU / edge path**: verified live, 18 tests green (default and `--no-default-features`).
- **GPT-2 124M** (`--features gpt2`): real weights + tokenizer, deterministic logits receipt.
- **CUDA (H100, locked)**: **AUDIT** bit-identical to CPU. **TRADE** ~6.8 ms/layer, 12-layer stack ~69 ms @ seq=1024. **Waller only** ~4 ms (`cuda_bench`) — not full layer. **NPOW** O(N) witness certified (`LUXI_NPOW_FAST=1` in gate). See [`docs/QUANT_TRADE_LOCKED.md`](docs/QUANT_TRADE_LOCKED.md).
- **CPU performance**: tiling+Rayon+NEON (~9× naive forward); recurrent KV (~7.6× generation). Speculative decoding is a research artifact only.

The original research repos (GAE, ATE, etc.) are intentionally left untouched on GitHub as historical archives.

## Documentation

| Doc | Role |
|-----|------|
| [`INSTRUCTIONS.md`](INSTRUCTIONS.md) | Contributor commands and sacred checks |
| [`docs/QUANT_TRADE_LOCKED.md`](docs/QUANT_TRADE_LOCKED.md) | **Ops single source of truth** — AUDIT/TRADE, locked H100 numbers, RunPod gates |
| [`docs/TECHNICAL_DEEP_DIVE.md`](docs/TECHNICAL_DEEP_DIVE.md) | Math, determinism, architecture depth |
| [`docs/AGENT_TECHNOLOGY_BRIEF.md`](docs/AGENT_TECHNOLOGY_BRIEF.md) | Shareable agent context (no repo required) |
| [`docs/LUXIEDGE_BUILD_ROADMAP.md`](docs/LUXIEDGE_BUILD_ROADMAP.md) | Pillar delivery order (NPOW → 70B → FP8 → PyTorch) |
| [`docs/SCALING_EVIDENCE.md`](docs/SCALING_EVIDENCE.md) / [`docs/ENERGY_EVIDENCE.md`](docs/ENERGY_EVIDENCE.md) | Measured O(N) memory and energy claims |
| [`docs/GEODESIC_SWEEP_DESIGN.md`](docs/GEODESIC_SWEEP_DESIGN.md) | Geodesic TRADE design (shipped + optional mega-kernel) |
| [`docs/verification/MILESTONE_H100_CUDA.md`](docs/verification/MILESTONE_H100_CUDA.md) | CUDA performance history |
| [`tests/cuda_lanes.md`](tests/cuda_lanes.md) | Manual CUDA regression checklist |
| [`integrations/torch/README.md`](integrations/torch/README.md) | PyTorch receipt hooks |

## License & Contact

Proprietary — Eric Waller (e@ewaller.com)

© 2026
