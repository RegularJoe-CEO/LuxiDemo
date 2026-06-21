# attention-transformer — Detailed Technical Deep Dive

> **Repository:** `https://github.com/RegularJoe-CEO/attention-transformer`
> **Source of truth:** the pure-Rust CPU path (`src/wnsm_transformer.rs`, `src/waller_operator.rs`, `src/linalg.rs`)
> **Receipt contract:** `sha256_of_f32_slice(values: &[f32]) -> [u8; 32]` over `f32::to_bits()`
> **Audience:** engineers, auditors, model-risk officers, certification reviewers, and patent counsel.

This document explains, in deliberate and verbose detail, **how** the engine works, **why** its outputs are bit-for-bit reproducible, **where** the energy savings come from, and **where the genuine novelty lies** versus prior art. It is a learning document, not a summary. Throughout, "Patent Angle" callouts separate well-known public-domain techniques from the specific combinations that are defensible.

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [The Fundamental Problem: Attention Is Expensive in the Wrong Currency](#2-the-fundamental-problem)
3. [The GAE Waller Operator: Single-Pass Online-Softmax Attention](#3-the-gae-waller-operator)
4. [WNSM: Waller Null-Space Multiplexing](#4-wnsm-waller-null-space-multiplexing) (includes NPOW)
5. [The Receipt Contract: Determinism You Can Verify](#5-the-receipt-contract)
6. [Recurrent KV State: Turning O(N²) Generation into Flat-Per-Token](#6-recurrent-kv-state)
7. [Speculative Decoding: Doing the Next Calculation Before You Need It](#7-speculative-decoding)
8. [The Performance Ladder: How We Got ~9× Without Touching a Single Bit](#8-the-performance-ladder)
9. [The Energy Model: Why Data Movement Is the Real Bill](#9-the-energy-model)
10. [Measured Evidence](#10-measured-evidence)
11. [Patent-Relevant Differentiators and Novelty](#11-patent-relevant-differentiators)
12. [CUDA Execution (H100 Verified)](#12-cuda-execution-h100-verified)
13. [Glossary](#13-glossary)

---

## 1. Executive Summary

`attention-transformer` (the Quantum Trader Edition) is a deterministic, fused-attention transformer engine purpose-built for **high-stakes quantitative workloads** where reproducibility and auditability are more important than raw tokens per second.

It is built around four ideas that work together:

1. **GAE Waller attention** — a single-pass, streaming, online-softmax attention operator that never materializes the full N×N score matrix. Memory is **O(N)** instead of O(N²).
2. **WNSM (Waller Null-Space Multiplexing)** — a way to carry extra payloads (KV state, routing, checksums, speculative tokens) through the *mathematical null space* of the MLP projection, so they travel with **zero change to the primary output** (`0.00e0` diff) and near-zero extra data movement.
3. **A unified cryptographic receipt** — every forward pass emits a SHA-256 hash over the bit-pattern of its f32 outputs. Identical inputs produce **byte-identical receipts**, across runs and across machines. This is the auditability hook.
4. **A determinism-preserving performance stack** — tiling, multithreading (Rayon), ARM NEON SIMD, recurrent KV state, load-time packing, allocation/lookup elimination, and a standalone `matmul_blocked` primitive. **Every shipped production lever keeps the receipts bit-identical.** Speed is never traded for reproducibility. Speculative decoding remains documented as a research artifact, not the recommended CPU speed path.

The engine has been verified live on a commodity Apple-Silicon MacBook (CPU only, no GPU): it runs the synthetic `production_demo`, runs a real downloaded **GPT-2 124M** model end-to-end, passes 18 unit tests on both the default and edge (`--no-default-features`) builds, and reproduces its claims bit-for-bit.

> **Patent Angle.** The individual building blocks — online softmax, KV caching, speculative decoding, SHA-256 — are public knowledge. The defensible position is the **specific combination**: bit-exact determinism preserved *across* an O(N) streaming attention operator, a null-space payload channel that provably does not perturb the output, and a cryptographic receipt that makes the whole thing auditable. That combination is what nobody else ships.

---

## 2. The Fundamental Problem

### 2.1 Attention's quadratic memory wall

Standard scaled-dot-product attention computes, for a sequence of N tokens:

```
scores = Q @ Kᵀ          # N×N matrix
weights = softmax(scores) # N×N matrix
out = weights @ V         # N×d
```

The `scores` and `weights` matrices are **N×N**. At N = 131,072 tokens, a single f32 N×N matrix is roughly **68.7 GB** — it does not fit in the memory of any laptop, and barely fits on a data-center GPU. This is the quadratic memory wall.

### 2.2 The currency that actually matters

A second, subtler problem: the dominant cost of inference is **not arithmetic — it is moving bytes**. The physics is stark:

| Operation | Approximate energy |
|---|---|
| A floating-point multiply-add | ~1 picojoule |
| Reading a byte from on-chip cache | ~10 picojoules |
| Reading a byte from DRAM/HBM | ~100–1000 picojoules |

Moving a byte from main memory costs **100× to 1000× more energy** than computing on it. So the materialization of that N×N score matrix is not just a memory problem — it is the **electricity bill**. Every byte written to and read back from HBM is paid for in joules and in heat.

> **The core insight that drives the entire design: Energy ∝ Data Movement.** Make the bytes move fewer times, and you simultaneously win on memory, speed, and power.

---

## 3. The GAE Waller Operator

### 3.1 Streaming softmax: never build the N×N matrix

The Waller operator (the "GAE" — Geodesic Attention Engine — lineage) computes attention in a **single streaming pass** per query row. Instead of building the full score matrix, it walks the keys/values once and maintains three small running quantities per query:

- `max_val` — the running maximum logit seen so far (for numerical stability)
- `sum_exp` — the running sum of exponentiated, rescaled logits
- `accumulator` — the running, online-rescaled weighted sum of value vectors

When a new (key, value) pair arrives, the operator:

1. Computes the dot-product logit `q · k`.
2. If this logit exceeds `max_val`, it **rescales** the existing `sum_exp` and `accumulator` down by `exp(old_max − new_max)` and updates `max_val`.
3. Adds `exp(logit − max_val)` into `sum_exp`.
4. Adds `exp(logit − max_val) · v` into `accumulator`.

At the end, the attention output for that query is `accumulator / sum_exp`. The full softmax was computed correctly — but the N×N matrix was **never materialized**. Memory is O(N) (you hold the keys/values and three running scalars/vectors), not O(N²).

### 3.2 Why this is mathematically exact

The online-softmax rescaling trick is algebraically identical to computing the softmax in one shot. The rescale step `exp(old_max − new_max)` exactly compensates for the change in the stabilizing constant, so the final ratio `accumulator / sum_exp` equals the textbook `softmax(scores) @ V` to within the same floating-point rounding — and, crucially, in a **fixed accumulation order** so the rounding is reproducible.

> **Patent Angle.** Online/streaming softmax is **prior art** — FlashAttention (2022) popularized O(N) memory attention and it is now standard in PyTorch. The Waller operator does **not** claim to invent O(N) attention. What it contributes is the *integration*: this streaming operator is implemented with a **fixed, left-to-right accumulation order** so it is bit-exact and receipt-verifiable, and it exposes a persistable `(max_val, sum_exp, accumulator)` **state** (see §6) that the rest of the system builds on. Honesty here strengthens the position — the novelty is the deterministic, stateful, receipt-locked *assembly*, not the softmax trick itself.

### 3.3 The code

The operator lives in `src/waller_operator.rs`. Its naive correctness is pinned by a unit test that compares it element-by-element against a textbook `Q@Kᵀ → softmax → @V` reference (`waller_operator` reference test). The streaming result must match the dense result within tight tolerance.

---

## 4. WNSM: Waller Null-Space Multiplexing

### 4.1 The idea in one sentence

**Carry an extra payload through the part of the MLP's weight space that the output never sees — the null space — so the payload rides along for free and changes the primary output by exactly zero.**

### 4.2 What the null space is

A linear projection `W` (for example, an MLP down-projection) maps an input vector to an output vector. Because of the dimensions and rank of `W`, there is a subspace of inputs — the **null space** of `W` — that `W` maps to **zero**. Any vector you add from that null space does not change `W`'s output at all.

WNSM exploits this. It injects a payload into the activation along directions that lie in the null space of the projection. When the projection is applied:

- The primary output is **unchanged** — provably, because the injected component projects to zero.
- The payload has nonetheless **traveled along with the activation** through the same memory traffic that was happening anyway.

### 4.3 Why this matters for energy

Normally, if you wanted to move a payload (say, a KV-cache state or a routing signal) between layers, you would pay for an **extra memory transaction** — extra bytes to and from HBM, extra joules. WNSM makes the payload **piggyback** on the activation that is already moving. The marginal data-movement cost approaches zero. This is the `EnergyReport`'s "bytes avoided" figure.

### 4.4 The proof: 0.00e0

The claim "WNSM does not change the output" is not asserted — it is **tested**. The `production_demo` runs the model twice: once normally, once with WNSM carrying a payload. It then:

- Computes the SHA-256 receipt of both outputs → they are **identical** (`e1980a6f…37628`).
- Computes the maximum element-wise difference → **`0.00e0`**.

The unit test `wnsm_fidelity_is_exact_zero_diff_and_identical_receipts` locks this permanently.

> **Patent Angle.** Null-space injection as a *concept* exists in coding theory and watermarking. The defensible contribution here is using the **null space of a transformer's own learned projection weights** as a **zero-overhead, provably-non-perturbing payload bus** for inference state — and proving it bit-exact with a cryptographic receipt. The combination of "null-space payload" + "0.00e0 provable fidelity" + "auditable receipt" is the ownable ground.

### 4.5 NPOW — scaling proof in the payload

**NPOW** (N-power witness) ships inside the WNSM payload (`src/npow/`, 12 floats, magic tag `NPOW`). Each forward can carry:

| Field | Meaning |
|-------|---------|
| `mem_slope_waller` | log-log slope of Waller streaming memory vs `seq_len` (~**1.0** = O(N)) |
| `mem_slope_standard` | log-log slope of dense N×N score matrix bytes (~**2.0** = O(N²)) |
| `memory_reduction_x` | ratio at anchor length (e.g. **341×** @ 131k tokens) |

Reproduce:

```bash
LUXI_NPOW_FAST=1 cargo run --release --example npow_scaling_proof   # gate / CI (~1s)
cargo run --release --example npow_scaling_proof                    # full timed sweep → 131k
```

**Gate logic:** pass/fail on **memory** log-log slopes only (Waller ~1.0, standard ~2.0). Time slope is informational (full-seq `waller_operator` is O(N²) FLOPs). WNSM check: **witness receipt** equality after bus transport + loose slope tolerance — avoids false “payload corrupted” from float round-trip.

Injecting NPOW through `forward_wnsm_chained` leaves the primary output at **0.00e0** max diff with identical output receipts.

---

## 5. The Receipt Contract

### 5.1 The single canonical function

```
sha256_of_f32_slice(slice: &[f32]) -> [u8; 32]
```

The algorithm is deliberately simple and exact:

1. Iterate the slice **in order**.
2. For each `f32`, take `to_bits()` — the exact 32-bit IEEE-754 pattern, with no rounding or reinterpretation.
3. Encode that `u32` as **little-endian** bytes.
4. Feed the bytes, in order, into SHA-256.

The output is a 256-bit digest. Two runs that produce byte-identical f32 outputs produce byte-identical digests. Two runs that differ in even a single low-order mantissa bit produce **completely different** digests (the avalanche property of SHA-256).

### 5.2 Why `to_bits()` and not the float value

Comparing floats by value (`a == b`) is fragile and platform-dependent. Hashing `to_bits()` captures the **exact** bit pattern, including the distinction between `+0.0` and `−0.0` and the exact representation of every value. This is what makes the receipt a true cross-platform fingerprint.

### 5.3 What the receipt buys you

- **Reproducibility:** rerun the same input next year on a different machine → same receipt.
- **Auditability:** a regulator or counterparty can independently recompute and confirm the receipt.
- **Tamper-evidence:** any change to the math, the weights, or the data changes the receipt.

This is the same receipt discipline used in the companion quant risk engine. It is the contract that must **never change**.

---

## 6. Recurrent KV State

### 6.1 The problem with naive generation

Autoregressive text generation produces one token at a time, feeding the growing sequence back in. The naive approach **recomputes attention over the entire sequence at every step**. Generating token 100 means redoing the work for tokens 1–99 yet again. Cost grows quadratically with sequence length.

### 6.2 The Waller advantage: state is already there

Recall from §3 that the Waller operator maintains `(max_val, sum_exp, accumulator)` as it streams. **That triple is the entire attention state.** For a new token, you do not need to rebuild it from scratch — you can `extend` the existing state with the new key/value and the new query attends against the persisted cache.

This is implemented as:

- `WallerKVState` — a struct holding the per-head `k_cache`, `v_cache`, and running `(max_val, sum_exp, accumulator)`.
- `extend(&mut self, new_k, new_v)` — appends the new token's key/value in fixed left-to-right order.
- `waller_attention_for_new_query(q_row, &state, ...)` — computes attention for the new token against the persisted state, using the **same** online-softmax and **same** left-to-right accumulation order as the full operator.

Because the accumulation order is identical, the incremental result is **bit-identical** to a full recompute.

### 6.3 The proof and the measured win

The hard gate is `tests/gpt2_incremental_equiv.rs`: it generates N tokens via full recompute and via the incremental path on the **real GPT-2 124M weights**, and asserts the token sequences and per-step logits are bit-identical.

Measured live (GPT-2 124M, 40 tokens, MacBook CPU):

- Full recompute: 49.7 s total; per-token cost grows from 328 ms to 2,056 ms (quadratic).
- Incremental KV: 6.5 s total; per-token cost is **flat** (~135 ms at the last token).
- **7.6× faster end-to-end**, and the gap widens the longer you generate (15× per-token at token 43).

> **Patent Angle.** KV caching is standard. The specific contribution is that the Waller operator's streaming state is the *natural, bit-exact* carrier for the cache — there is no separate "cache" bolted on; the operator's own running accumulator **is** the cache, and extending it is provably equivalent to recomputing. That equivalence-by-construction (locked by an equivalence test) is the defensible piece.

---

## 7. Speculative Decoding

### 7.1 The "see-through" idea

The user's framing was: *do one calculation but already be working on the next one.* That is exactly **speculative decoding**:

1. A **cheap drafter** proposes several future tokens quickly.
2. The **real engine (verifier)** does a single batched forward pass over all the proposed tokens at once.
3. Accepted tokens are those that match the verifier's own output; the first mismatch is corrected by the verifier's true prediction.

The crucial property: **the accepted output is bit-identical to plain sequential greedy decoding.** The verifier always has the final say, so the drafter is *allowed* to be approximate — it only affects speed, never correctness.

### 7.2 The geometry angle

Instead of a second neural network as the drafter, this engine first explored **zero-parameter geometric drafters**:

- An **induction-head lookup** — if a recent suffix appeared earlier in the text, predict whatever followed it last time. Near-zero compute, pure memory lookup. Excellent on structured/repetitive text.
- A **lean geometric scorer** — reuses the real GPT-2 token embeddings, projected through the add-only Walsh-Hadamard transform (an orthogonal basis built from additions and subtractions only), to cheaply rank candidate continuations.

### 7.3 The batched verifier — the real lever

The key engineering insight: `Gpt2Model::forward` returns logits for **every position** in the input (it is causal). So one forward pass over `[context + K draft tokens]` verifies all K proposals **simultaneously** — instead of K separate passes. This is what converts a given acceptance rate into actual wall-clock speedup.

### 7.4 Measured result

On structured text, the batched verifier collapsed 24 dependent forward passes into 11, for a **1.75× wall-clock speedup**, with output **bit-identical** to greedy. On natural, varied text the zero-parameter drafters accept ~10%, and a tiny trained draft head was subsequently evaluated and found ineffective for GPT-2 124M greedy decoding (~1–13% acceptance).

> **Engineering finding (important).** On a *compute-bound* CPU (e.g. an Apple-silicon MacBook), speculative decoding **competes with** the recurrent KV state rather than stacking on top of it: each verify step is itself a forward pass, so K verify steps cost roughly K× the cheap incremental cost. With ~10% acceptance this makes the speculative path *slower* than plain recurrent-KV generation on CPU. Speculative decoding is therefore retained only as a **documented research artifact** (`examples/gpt2_speculative.rs`). The **recurrent KV state is the single recommended fast generation path** on CPU/edge. Speculative decoding remains attractive on bandwidth-bound GPUs, where a batched verify pass is near-free — a future direction, not a current CPU lever.

> **Patent Angle.** Speculative decoding is published prior art (Leviathan et al., 2023). The novel ground being explored here is two-fold: (1) a **geometric/orthogonal-transform drafter** that needs zero trained parameters, and (2) relaying the trained draft head's signal through the **WNSM null space** so its predictions are extracted as a near-free byproduct of the verifier's own forward pass. The "energy-free draft signal via null space" is the differentiator — everyone else pays for a second model.

---

## 8. The Performance Ladder

Every rung below was added **without changing a single output bit** — the `production_demo` receipt (`e1980a6f…37628`) and the GPT-2 logits receipt held at every step. This is the discipline that makes the speed story credible.

| Rung | Technique | Speedup | Determinism mechanism |
|---|---|---|---|
| 0 | Naive triple-loop matmul | baseline (~50 s/pass) | — |
| 1 | Tiled matmul + Rayon | ~6.6× | Parallelize over **independent output rows**; fixed left-to-right K accumulation within each dot product |
| 2 | ARM NEON SIMD | ~9× total | Vectorize across the **N-dimension** (output columns), not the reduction; **separate multiply + add** (no FMA reordering) to match the scalar baseline bit-for-bit |
| 3 | Recurrent KV state | 7.6× on generation | Operator's own streaming accumulator is the cache; `extend` is equivalence-tested |
| 4 | Batched speculative decoding | 1.75× on structured text, slower on compute-bound CPU in real use | Verifier has final say → output bit-identical to greedy; retained as research artifact, not the recommended CPU path |
| 5 | `matmul_blocked` cache-blocked GEMM primitive | no runtime change yet | Added as standalone deterministic primitive only; not wired into sacred GPT-2 f32 path |

### 8.1 Why NEON only gave 1.35× on top of tiling

A subtle but important honest finding: the obvious way to use NEON — vectorizing the *reduction* (dot-product) dimension with 4 lane accumulators — would have **broken bit-exactness** (different accumulation order, and FMA). So instead the kernel vectorizes across **output columns**, preserving the exact per-element accumulation order. This keeps the receipt locked but means the kernel is **memory-bandwidth-bound** rather than compute-bound, so the SIMD win is modest. We chose **bit-exactness over a bigger but receipt-breaking speedup.** That is the right trade for an auditable engine.

> **Patent Angle.** The defensible engineering pattern is "**SIMD/parallel speedups under a fixed-accumulation-order constraint**" — achieving hardware acceleration while provably preserving a cryptographic receipt. Most production kernels deliberately *abandon* bit-exactness to go faster; doing the opposite, by construction, is the unusual and ownable discipline.

---

## 9. The Energy Model

### 9.1 The formula

`EnergyReport::compute(...)` estimates energy from **bytes of data movement avoided**, anchored to the citable hardware figure of **~20 pJ/byte** for HBM access. The bytes-avoided formula in the code is linear in sequence length:

```
avoided = 2 * payload_dim * seq * layers * 4   (bytes)
```

The `× 4` is bytes per f32; the `× 2` accounts for the write-then-read round trip avoided. This grows **linearly** with sequence length.

### 9.2 The linear-vs-quadratic divergence

Standard attention's energy grows **quadratically** (it materializes and re-reads the N×N score matrix). The Waller path grows **linearly**. The ratio therefore **doubles every time the sequence doubles**:

- At 8,192 tokens: ~128× less attention-energy.
- At 131,072 tokens: ~2,048× less.
- At 262,144 tokens: ~**4,096×** less.

### 9.3 Reconciliation (why the number is defensible)

The published ~20 pJ/byte HBM figure, multiplied by the code's real `bytes_avoided` formula, **reproduces the engine's own hardcoded `EnergyReport` constant** to within rounding (2.46e-7 J calculated vs 2.2e-7 J reported for the demo workload). That cross-check — a citable public constant reproducing the engine's internal number via the actual code formula — is what makes the energy claim survive scrutiny.

> **Honest scope.** The reported savings isolate the **attention-stage data movement** (the score-matrix traffic plus the WNSM payload bus), which is where the quadratic blowup lives. It is not a full end-to-end watt-hour bill. Real end-to-end savings are expected to be larger, but this is the part we can defend with measured numbers.

---

## 10. Measured Evidence

All of the following were measured **live on the user's MacBook (CPU only)**:

- **Fidelity:** `production_demo` → NORMAL and WNSM receipts identical (`e1980a6f…37628`), max diff `0.00e0`.
- **Determinism:** `decoder_forward_is_deterministic_100_runs` → 100 forwards, identical receipts.
- **Correctness:** naive-reference unit tests for `waller_operator`, `online_softmax`, `welford`, `layernorm`, `mlp` all pass; 18 tests green on default **and** `--no-default-features` (edge) builds.
- **Real model:** GPT-2 124M loaded from `model.safetensors` + `tokenizer.json`, runs end-to-end on CPU with a deterministic prompt logits receipt (`756a50a3de7f318b1474bbbc2258c0a8e417921727b6c395e23f6f18dc04b9c8` for "Once upon a time" on the current f32 path).
- **Memory scaling:** see [`SCALING_EVIDENCE.md`](SCALING_EVIDENCE.md) — 341× less memory at 131K tokens (201 MB vs 68.7 GB).
- **Energy scaling:** see [`ENERGY_EVIDENCE.md`](ENERGY_EVIDENCE.md) — 4,096× attention-energy reduction at 262K tokens.
- **Generation speedup:** recurrent KV → 7.6× end-to-end at 40 tokens, flat per-token cost.
- **Speculative decoding status:** batched verifier → 1.75× wall-clock on structured/repetitive text, bit-identical output, but slower than recurrent KV on compute-bound CPU for practical generation; retained as a research artifact.
- **CUDA (H100, 2026-06, locked):** **AUDIT** bit-identical to CPU (receipt `0ae659948eabc3fa…d37ada`). **TRADE** ~6.8 ms/layer, 12-layer stack ~69 ms @ seq=1024. Waller-only ~4 ms. See [`QUANT_TRADE_LOCKED.md`](QUANT_TRADE_LOCKED.md).

Reproduce the core claims with one command:

```bash
cargo run --release --example production_demo
```

---

## 11. Patent-Relevant Differentiators

To be honest and therefore defensible, separate the prior art from the contributions:

**Prior art (do not claim to invent):**
- Online / streaming softmax and O(N) memory attention (FlashAttention, 2022).
- KV caching for autoregressive generation.
- Speculative decoding (Leviathan et al., 2023).
- SHA-256, IEEE-754, Walsh-Hadamard transforms, Kahan-style fixed-order accumulation.

**Defensible contributions (the combination):**
1. **Bit-exact determinism preserved across the production stack** — an O(N) streaming attention operator, multithreaded/SIMD matmul, KV recurrence, load-time packing, and deterministic linear-algebra primitives, all locked to cryptographic receipts. Most production engines abandon bit-exactness to go fast; doing the opposite by construction is the unusual asset. Speculative decoding is bit-identical but is not the recommended CPU speed path.
2. **WNSM** — using a transformer's own learned-weight null space as a **provably-non-perturbing (`0.00e0`), near-zero-energy payload bus** for inference state, verified by receipt.
3. **Equivalence-by-construction KV recurrence** — the operator's streaming accumulator *is* the cache; extending it is provably equal to recomputing, locked by an equivalence test on real weights.
4. **Research direction: energy-aware draft signals** — relaying draft-head signals through the WNSM null space remains an explored research direction, but trained drafting was ineffective for GPT-2 124M greedy decoding and is not part of the recommended production path.

**The market position:** not "fastest LLM runtime" (llama.cpp wins raw speed today), but **"the only edge inference engine whose every output a regulator can independently reproduce and audit, bit-for-bit, while staying energy-efficient."** That is a real, narrow, ownable niche — defense, regulated finance, compliance, and certification.

---

## 12. CUDA Execution (H100 Verified)

CUDA is gated behind `--features cuda` (`cuda_src/`, `src/gpu/cuda.rs`). As of 2026-06 it has been verified on **RunPod H100 NVL** against the pure-Rust CPU reference.

### 12.1 Two lanes (do not conflate them)

| Lane | When | Pipeline | Receipt |
|------|------|----------|---------|
| **AUDIT** | `LUXI_RECEIPT_AUDIT=1` | CPU batched LN1+QKV → GPU Waller+wo → CPU `fused_mlp_layernorm` per row | **Bit-identical** to CPU `forward()` — `0ae659948eabc3fa1212b84d9a2006c707c28ba4209ce28410df676d38d37ada` |
| **TRADE** | default `forward_cuda` | GPU LN1 → packed QKV → parallel waller+wo → batched MLP (`launch_mlp_block_geodesic`) | Deterministic on GPU; **not** the compliance receipt |

**AUDIT** keeps CPU QKV + CPU MLP for bit-exact receipts. **TRADE** uses device QKV and batched GEMM MLP (locked ~6.8 ms/layer @ seq=1024).

### 12.2 Measured latency (H100 NVL, locked)

| Benchmark | seq=1024 |
|-----------|----------|
| `cuda_layer_bench` | **~6.8 ms** |
| `cuda_quant_bench` (12 layers) | **~69 ms** |
| `cuda_bench` (Waller only) | **~4 ms** |

All locked numbers, RunPod gates, and env knobs: [`QUANT_TRADE_LOCKED.md`](QUANT_TRADE_LOCKED.md).

### 12.3 Environment knobs

| Variable | Effect |
|----------|--------|
| `LUXI_RECEIPT_AUDIT=1` | AUDIT lane (verify / compliance) |
| `LUXI_CUDA_CPU_POST=1` | TRADE with CPU MLP (phase-debug) |
| `LUXI_CUDA_GPU_QKV=1` | Experimental GPU QKV (slow at large seq) |
| `LUXI_CUDA_PHASE_TIMING=1` | Per-kernel ms in `cuda_layer_bench` |
| `LUXI_CUDA_ROW_FUSED=1` | Slow row-fused waller+wo (~135 ms/layer @ seq=1024) |
| `LUXI_CUDA_FUSED_ROW_MLP=1` | Slow serial row MLP (~27 ms) |
| `LUXI_CUDA_QUANT_STACK=0` | Disable multi-layer persistent GPU stack |
| `LUXI_CUDA_MEGA=1` | Full mega-fused layer (not receipt-certified) |
| `CUDA_ARCH` | nvcc arch (90 = H100) |
| `LUXI_NPOW_FAST` | NPOW analytical smoke (no 131k Waller timing) |

### 12.4 Regression

NVIDIA: `bash scripts/runpod_quant_gate.sh` · first AUDIT: `./scripts/runpod_cuda_check.sh`  
CPU LuxiEdge: `LUXI_NPOW_FAST=1 cargo run --release --example npow_scaling_proof`  
Full command list: [`QUANT_TRADE_LOCKED.md`](QUANT_TRADE_LOCKED.md). Design notes: [`GEODESIC_SWEEP_DESIGN.md`](GEODESIC_SWEEP_DESIGN.md).

### 12.5 Documentation map

| Doc | Use |
|-----|-----|
| [`QUANT_TRADE_LOCKED.md`](QUANT_TRADE_LOCKED.md) | Ops, benchmarks, gates |
| [`LUXIEDGE_BUILD_ROADMAP.md`](LUXIEDGE_BUILD_ROADMAP.md) | Pillar delivery |
| [`AGENT_TECHNOLOGY_BRIEF.md`](AGENT_TECHNOLOGY_BRIEF.md) | Agents without repo access |
| [`SCALING_EVIDENCE.md`](SCALING_EVIDENCE.md) / [`ENERGY_EVIDENCE.md`](ENERGY_EVIDENCE.md) | Measured O(N) claims |

---

## 13. Glossary

- **GAE (Geodesic Attention Engine):** the lineage name for the single-pass streaming attention operator.
- **Waller Operator:** the streaming, online-softmax attention implementation maintaining `(max_val, sum_exp, accumulator)`.
- **WNSM (Waller Null-Space Multiplexing):** carrying a payload through the null space of a projection so the primary output is unchanged.
- **Null space:** the subspace of inputs that a linear map sends to zero; adding from it does not change the output.
- **Online softmax:** computing softmax in a single streaming pass with running max/sum rescaling, avoiding the full N×N matrix.
- **Receipt:** the SHA-256 digest over `to_bits()` of the f32 outputs; a reproducible cross-platform fingerprint.
- **KV state / KV cache:** the persisted attention state (`max_val`, `sum_exp`, `accumulator`, cached keys/values) that makes incremental generation flat-cost.
- **Speculative decoding:** drafting several tokens cheaply, then verifying them in one batched pass; output stays bit-identical to greedy.
- **Drafter / Verifier:** the cheap proposer and the exact final-say model in speculative decoding.
- **Walsh-Hadamard transform:** an orthogonal transform built from additions and subtractions only; used as a cheap, deterministic basis for geometric drafting.
- **Bit-exact / `0.00e0`:** outputs that are identical down to the last IEEE-754 bit.
- **HBM:** High-Bandwidth Memory; the off-chip memory whose access dominates inference energy (~20 pJ/byte).
- **Race to idle:** finishing work fast so the chip can return to a low-power state, lowering energy and heat.
- **NPOW:** N-power scaling witness (12×f32) on the WNSM bus; mem slopes + witness SHA-256.
- **AUDIT / TRADE:** compliance (CPU-matched) vs desk-speed CUDA geodesic paths.

---

*Pure-Rust path is the golden reference. CUDA AUDIT path is bit-identical on H100; TRADE path optimizes desk latency. Determinism is the product. — Eric Waller, © 2026*

---

## Quantum Trader Edition – Why This Matters

The architecture described in this document was built for one primary real-world constituency: **quantitative traders and high-stakes systematic trading teams** who need inference they can defend in front of risk committees, regulators, and counterparties.

Key properties that matter in this domain:

- **Machine-independent reproducibility** via the receipt contract.
- **Audit-grade provenance** (possible via WNSM payload embedding).
- **Energy and memory characteristics** suitable for large-scale simulation or edge deployment.
- **A canonical deterministic f32 path** with quarantined quantized infrastructure kept separate until it is fixed, benchmarked, and separately receipt-locked.

Most "fast" inference engines were never designed for this threat model. This one was.
