# LuxiEdge — current brief for any LLM

**Snapshot date:** 2026-07-23  
**Product / engine:** LuxiEdge deterministic packed transformer executor (`attention-transformer-v2`)  
**Current branch:** `version-99`  
**Frozen validation commit:** `d9ad7e06ff727440a1237e9fbb709d8bad0ba452`  
**Contact:** e@ewaller.com  
**Current status:** Working H100 prototype; internal validation complete; independent TESTfort validation pending

## How to use this file

Paste this file at the beginning of a new AI conversation when asking for a technical explanation, diagram, presentation, website, investor summary, engineering plan, test protocol, or product assessment involving LuxiEdge.

Treat this document as the current source of truth until it is replaced by a later dated version. Preserve the distinctions between:

- The AI model and the LuxiEdge execution engine beneath it.
- Prefill positions and generated serving tokens.
- Scoped deterministic execution and universal bit-for-bit determinism.
- Internal reproduction and independent third-party validation.
- GPU-board energy and total facility electricity.
- LuxiEdge's execution architecture and the third-party FlashAttention kernel it currently uses.

Do not reuse performance conclusions from the previous July 21 brief. Those figures were superseded by `version-99`.

---

## The simple explanation

LuxiEdge is an engine that runs underneath an AI model. It is not the model or chatbot itself.

It makes AI computation cleaner by:

- Keeping the model and working data on the GPU instead of repeatedly moving them.
- Packing multiple independent requests together for the heavy mathematical work.
- Keeping every request separate during attention so their information does not mix.
- Combining operations and eliminating unnecessary conversions, launches, synchronization, and memory movement.
- Following a controlled numerical path that repeats in the tested environment.
- Producing a receipt showing which backend ran, whether a fallback occurred, and whether the output repeated correctly.

The objective is straightforward:

> Perform useful AI work with less wasted movement, lower energy per completed unit of work, and more predictable results.

The easiest analogy is a combination of a train and a laser. LuxiEdge packs the large, heavy matrix workload like freight onto one powerful train for the GPU Tensor Cores. Small control and verification information moves separately and precisely, like a narrow beam. The heavy data remains on the GPU for reuse.

---

## Current bottom line

| Question | Current answer |
|---|---|
| Does the tool work? | **Yes.** It runs a real, 28-layer Qwen2-7B-class packed-prefill path on one NVIDIA H100. |
| Is it deterministic? | **Yes within the pinned and tested execution contract.** Full-stack fingerprints repeat exactly in the tested environment. |
| Is it bit-for-bit identical to the serial oracle? | **No.** The Flash path is repeatable but has an explicit non-bit-exact numerical tolerance of about `2.8e-5` maximum absolute difference. |
| Is sequence isolation working? | **Yes.** Measured cross-sequence contamination is zero. |
| Is the current path fast? | **Yes, materially faster than the prior Luxi path.** The frozen B16 package is about 27,824 prefill positions/s. |
| Is it a full inference server? | **No.** The current validated product is a packed-prefill executor, not complete online serving or decode parity. |
| Has it been independently validated yet? | **Not yet for this version.** Clean internal recreation is complete; the TESTfort protocol is prepared and pending. |

**Plain-language position:** LuxiEdge is a working deterministic GPU execution prototype with strong evidence controls and an internally reproduced energy-efficiency result. It is not yet a mature serving platform.

---

## What the current transformer executor does

The current engine is a Rust/CUDA, Qwen2-7B-class, device-resident packed-prefill executor tested on an NVIDIA H100 80GB HBM3.

### Main dataflow

1. Multiple independent input sequences enter the executor.
2. LuxiEdge packs them into a larger matrix workload. For example, batch 16 at sequence length 128 becomes `16 × 128 = 2,048` packed rows.
3. Model weights and working activations remain resident in GPU high-bandwidth memory across the measured stack.
4. Each of the 28 transformer layers performs:
   - Parallel deterministic two-pass normalization.
   - Packed QKV Tensor Core matrix multiplication.
   - Causal attention through the Luxi-owned `attention_forward(...)` boundary.
   - Packed output projection.
   - Residual combination and second normalization.
   - Half-precision MLP expansion, activation, and reduction.
   - Residual combination before the next layer.
5. Heavy matrix work is shared efficiently across the packed rows, while attention remains isolated inside each original sequence.
6. Only the necessary result and verification information leave the device-resident path.

### Current attention structure

| Path | Purpose |
|---|---|
| FlashAttention `2.8.3.post1` | Current primary accelerated backend |
| Batched Waller attention | Luxi fallback implementation |
| Serial attention | Numerical oracle used for comparison |

FlashAttention is a third-party dependency. LuxiEdge does not claim ownership of the FlashAttention kernel. LuxiEdge's work includes the replaceable attention boundary, packed and device-resident dataflow, backend selection and provenance, fallback monitoring, sequence isolation, normalization and MLP mechanisms, deterministic receipts, and measurement harnesses.

### Current champion mechanisms

- Packed GEMMs with `M = B × S`.
- FlashAttention invoked behind the Luxi-owned backend boundary.
- Half-precision QKV packing and reduced copy work.
- Flash half output feeding the half-precision output projection path.
- Half-precision MLP intermediates and activation path.
- Fused bias, residual, and second normalization work.
- Parallel two-pass normalization.
- Light device-to-host work during sustained measurement.
- Explicit backend and fallback stamping.

---

## Determinism: what is proven

LuxiEdge has a deterministic transformer executor within the tested contract. This is stronger than merely setting a random seed.

### Full-stack receipt

| Test | Result |
|---|---:|
| B1 fixed-input full-stack fingerprint | Stable across 3 repeats |
| B16 fixed-input full-stack fingerprint | Stable across 3 repeats |
| B32 fixed-input full-stack fingerprint | Stable across 3 repeats |
| Layers covered | All 28 layers |
| Rows covered | All tested `B × S` rows |
| Maximum repeat difference | `0` |
| Finite-output check | Pass |
| Backend used | `flash` |
| Fallback count | `0` |
| Cross-sequence contamination | `0` |
| Maximum Flash difference vs serial oracle | Approximately `2.8e-5` |
| Repeated generated completion in internal screen | Identical across 3 repeats |

This proves that the pinned model, code, H100, software environment, settings, shapes, and fixed inputs follow a repeatable tested path.

### What deterministic does not mean here

- It does not mean the Flash result is bit-for-bit identical to the serial oracle.
- It does not prove identical output across different GPU models.
- It does not prove identical output after driver, CUDA, PyTorch, or FlashAttention changes.
- It does not yet prove arbitrary online request-order or batch-composition invariance through a public Luxi serving API.
- It does not prove model truth, safety, accuracy, or universal correctness.
- It does not prove that every other engine is nondeterministic.

The correct statement is:

> LuxiEdge provides a deterministic, receipt-driven execution contract for the pinned and tested environment, with an explicit non-bit-exact tolerance against its serial numerical oracle.

---

## Current absolute performance and energy

The current frozen evidence package uses a Qwen2-7B-class, 28-layer packed-prefill workload on one H100 with batch 16 and sequence length 128.

The counted unit is a **prefill position**:

```text
positions = completed iterations × batch × sequence length
```

This is not the same as a generated serving token.

### Frozen three-run B16 package

| Run | Positions/s | Cumulative hot-window energy | Completed positions | Board J/position |
|---:|---:|---:|---:|---:|
| 1 | 27,266 | 10,325.295 J | 548,864 | 0.018812 |
| 2 | 28,400 | 10,586 J | 569,344 | 0.018593 |
| 3 | 27,824 | 10,412 J | 557,056 | 0.018692 |
| Median | **27,824** | — | — | **0.018692** |

The energy result comes from the H100's cumulative NVML board-energy counter over the synchronized hot window. It is not reconstructed from a median power sample.

### Longer clean-recreation confirmation

The clean-clone dress rehearsal repeated the B16 workload with longer 60-second windows and reversed the engine order. Luxi's two internal headline medians were:

| Internal arm | Luxi positions/s | Luxi board J/position |
|---|---:|---:|
| Five 60-second runs | 27,227 | 0.01894 |
| Three 60-second reverse-order runs | 27,132.8 | 0.018975 |

The longer runs support the same general operating range as the frozen 20-second package.

### Ten-minute stability

Luxi completed the internal ten-minute soak with approximately:

- Throughput drift: `+0.03%` versus the primary median.
- Board J/position drift: `+0.44%` versus the primary median.

Both remained well inside the approximately 3% rehearsal stability gate.

### B32 status

B32 performance improved materially, but its audited peer-energy result did not earn a final claim. Audited Luxi throughput was roughly 28,860–29,008 positions/s, while median energy of approximately 0.01819–0.01826 J/position narrowly missed the fresh comparison ceiling of 0.018176.

Do not round that miss into a pass.

---

## Current matched comparison status

The previous brief incorrectly said that no same-H100 peer-energy evidence existed. That is no longer true.

### Default performance mode, longer reverse-order arm

| Engine | Median positions/s | Median board J/position |
|---|---:|---:|
| LuxiEdge | 27,132.8 | 0.018975 |
| Matched default serving engine | 35,000.5 | 0.019265 |

In this internal arm, Luxi retained approximately 77.5% of the peer throughput and used approximately 1.5% less GPU-board energy per prefill position.

### Determinism-aligned peer mode

The matched serving engine's batch-invariant mode produced a median of:

- 30,686.6 positions/s.
- 0.020487 board J/position.

Against that internal arm, Luxi retained approximately 88.4% of the throughput and used approximately 7.4% less board energy per position, equivalent to about 8.0% more positions per board joule.

This is an energy-efficiency result, not a speed victory. Luxi remained slower.

### Required qualification

These comparisons have been reproduced internally from a clean clone with raw measurements, engine-order reversal, and stability testing. They must remain labeled **internally reproduced** until TESTfort completes and signs the independent report.

---

## Energy measurement contract

For every official run:

```text
positions/s = completed prefill positions ÷ synchronized hot-window seconds

board J/position = cumulative NVML board-energy delta ÷ completed prefill positions

positions/J = 1 ÷ board J/position
```

Instantaneous power samples are secondary diagnostics. The authoritative energy value is the cumulative board-energy difference over completed useful work.

Always say **GPU-board energy**. Never silently convert this into:

- Wall-plug energy.
- Rack or cooling energy.
- Facility energy or PUE.
- Carbon reduction.
- Electricity-bill savings.

Those require a broader measurement boundary.

---

## Repository and replay identity

| Item | Value |
|---|---|
| Repository | `RegularJoe-CEO/attention-transformer-v2` |
| Branch | `version-99` |
| Frozen commit | `d9ad7e06ff727440a1237e9fbb709d8bad0ba452` |
| Recorded mechanism commit | `1356cf7` |
| Frozen NVML-arm binary SHA-256 | `cce7928038daa9248ffc989e0a4dbe04a2c5b93c0cc1829260ff278219003480` |
| Primary third-party harness | `scripts/peer_nvml_harness.py` |
| Matched comparison harness | `scripts/matched_vllm_prefill_compare.py` |
| Attention receipt | `scripts/flash_determinism_receipt.py` |
| Full-stack receipt | `examples/cuda_qwen7b_trade.rs --stack-fingerprint` |
| Frozen scoreboard | `evidence/peer-win-2026-07-22/SCOREBOARD.md` |
| Third-party instructions | `evidence/peer-win-2026-07-22/REPLAY_THIRD_PARTY.md` |

Build command:

```bash
cargo build --release --features cuda,gpt2,flash-bridge --example cuda_qwen7b_trade
```

Reproduction requires system Python with the pinned FlashAttention environment. Placing the separate serving-engine virtual environment first on `PATH` previously caused the wrong backend environment and must be treated as an invalid setup.

---

## Current product maturity

| Capability | Status |
|---|---|
| Qwen2-7B-class packed prefill on H100 | Working |
| Device-resident 28-layer stack | Working |
| Packed B16/S128 execution | Working and repeatedly measured |
| Full-stack deterministic receipt | Working in tested scope |
| Serial attention oracle | Working |
| Cross-sequence isolation receipt | Working |
| Backend identity and fallback audit | Working |
| Cumulative board-energy harness | Working |
| Clean-clone reproduction package | Working |
| Longer run and soak stability | Working internally |
| Independent TESTfort report | Pending |
| Complete online serving | Not established |
| Decode and latency competitiveness | Not established |
| Request-order Luxi serving receipt | Not yet exposed by current API |
| Multi-GPU operation | Not established |
| Training or backward pass | Not established |
| Production security, operations, and SLA | Not established |

The correct maturity description is:

> A working deep-technology GPU executor with a frozen reproducibility package, scoped deterministic receipts, and internally reproduced absolute performance and energy measurements.

---

## Current engineering position and next order

The current `version-99` source and evidence package are frozen for independent validation. Do not modify the frozen commit or mix later experiments into its evidence.

After independent validation, the engineering order should be:

1. Preserve the deterministic receipt and B16 energy result.
2. Close the remaining throughput gap without increasing board J/position.
3. Expose Luxi request-order and batch-composition controls and add direct invariance receipts.
4. Add locally rehearsed decode, latency, and serving measurements.
5. Expand one variable at a time: shape, model, GPU, and then serving topology.
6. Re-run cumulative-energy evidence for every meaningful mechanism change.

Avoid optimizing merely to improve a single selected benchmark. The goal remains a simpler execution path that generalizes.

---

## TESTfort status

The current independent protocol is prepared for the frozen `version-99` commit. The planned package includes:

- Immutable repository, model, binary, dependency, and GPU identity.
- Clean build and replay.
- B16 sustained runs with engine-order control.
- Cumulative NVML board-energy arithmetic from raw fields.
- Ten-minute stability soaks.
- Luxi full-stack fingerprints and serial-oracle receipt.
- Backend identity and fallback count.
- A separately labeled determinism-aligned peer arm.
- Explicit exclusions for B32 peer energy, serving/decode, training, equal-power, and facility-energy claims.

Until the signed report exists, say:

> Internally reproduced; independent TESTfort validation pending.

After the report arrives, replace the internal headline only with the exact result TESTfort independently confirms. Do not silently keep a stronger internal number if the third-party result differs.

---

## Related Luxi Quant Engine

LuxiEdge also has a separate Rust numerical and risk-computation prototype. It applies the same receipt-driven philosophy to deterministic-oriented numerical work.

Current working areas include:

- Compensated CPU reductions.
- Canonical SHA-256 result receipts.
- Numerical functions such as softmax, normal CDF, GELU, exponential, error function, gamma, LayerNorm, RMSNorm, SiLU, square root, and logarithm.
- Online statistics and covariance.
- Synthetic portfolio variance, volatility, and delta/gamma-style risk.
- Portable GPU experiments and separate research CUDA kernels.

It is a working prototype and integration candidate, not yet a production regulated-risk platform. Keep it separate from the transformer benchmark unless explaining the broader deterministic-compute platform.

---

## Licensing, patent, and dependency posture

- Treat the current product as proprietary/private unless the actual public release state says otherwise.
- If AGPLv3 plus commercial dual licensing remains the chosen direction, describe it as being prepared until release.
- Reconcile any repository metadata that says MIT with the intended company licensing strategy before publication.
- Preserve FlashAttention and all other third-party licenses and notices.
- Say “patent pending” only when the filing and current status are confirmed by the actual filing record or counsel.
- Never say patented, granted, or that LuxiEdge owns all transformer inference unless an issued claim supports that statement.

---

## Honesty rules

### Safe statements

- LuxiEdge is an engine beneath an AI model, not the model itself.
- The current system is a working packed-prefill executor on one H100.
- Weights and working activations remain device-resident across the measured stack.
- Independent sequences are packed for heavy matrix work and isolated during attention.
- Fixed-input full-stack fingerprints repeat in the pinned tested environment.
- Cross-sequence contamination is zero in the tested receipt.
- Flash is the recorded backend and fallback is zero in the frozen runs.
- The frozen B16 package is approximately 27,824 positions/s at 0.018692 GPU-board J/position.
- Longer clean-recreation runs are approximately 27,100–27,200 positions/s at approximately 0.01894–0.01898 board J/position.
- Current peer comparisons are internally reproduced and independent validation is pending.
- FlashAttention is a third-party accelerator behind a Luxi-owned execution boundary.

### Never say without additional evidence

- LuxiEdge is faster than the mature serving engines tested.
- LuxiEdge is a complete production inference server.
- Prefill positions are generated serving tokens.
- LuxiEdge is universally bitwise deterministic.
- A fixed-environment receipt proves arbitrary request-order invariance.
- GPU-board energy equals wall-plug or facility savings.
- The B32 peer-energy result passed.
- TESTfort validated `version-99` before its report is complete.
- FlashAttention is LuxiEdge-owned technology.
- LuxiEdge is already publicly open source unless that release has actually occurred.
- A patent is granted unless an issued patent exists.

---

## One-sentence north star

**LuxiEdge keeps AI work resident, packs the heavy computation efficiently, preserves strict information boundaries, eliminates unnecessary steps and movement, and produces a deterministic receipt for the path that actually ran.**

---

*When creating another artifact from this brief, preserve the snapshot date, frozen commit, work-unit definition, current maturity, scoped determinism language, third-party Flash attribution, and distinction between internal reproduction and independent validation. If a request would turn prefill positions into serving tokens, board energy into facility savings, scoped determinism into a universal claim, or an internal result into a third-party result, produce the strongest accurate version instead.*
