# LuxiEdge — YC current product, functionality, and evidence brief

**Snapshot:** July 22, 2026  
**Purpose:** Controlling technical handoff for the Y Combinator application, investor conversations, and the pending TESTfort independent report  
**Transformer repository:** `RegularJoe-CEO/attention-transformer-v2`  
**Frozen branch:** `version-99`  
**Immutable transformer test pin:** `d9ad7e06ff727440a1237e9fbb709d8bad0ba452`  
**Quant repository:** `RegularJoe-CEO/luxi-quant-engine`  
**Quant snapshot inspected:** `440a3382dffc1061aa077e4994c9016dafc71540`  

## Read this first

LuxiEdge has a working, internally reproduced deterministic GPU transformer executor—not merely a proposal. In its tested scope, it executes a packed Qwen2-7B prefill workload on one NVIDIA H100, produces stable full-stack fingerprints, records the attention backend and fallback status, isolates sequences correctly, and measures cumulative GPU-board energy.

The strongest accurate claim available **today** is:

> LuxiEdge built a deterministic GPU transformer executor that, in internally reproduced clean-clone tests on one NVIDIA H100, ran a Qwen2-7B B16/S128 packed-prefill workload at approximately 27,133 positions per second and 0.018975 GPU-board joules per prefill position. That retained 77.5% of default vLLM throughput while using 1.5% less board energy per position. In a separate confirmed comparison with vLLM's batch-invariant mode on the same workload and machine, LuxiEdge retained 88.4% of throughput and used 7.4% less board energy per position—about 8.0% more positions per board joule. Independent TESTfort validation is pending.

That wording is intentionally exact. It says what the current evidence proves without claiming that LuxiEdge is already a drop-in serving replacement, universally deterministic, faster than vLLM, or independently validated before the TESTfort report arrives.

Once TESTfort completes the protocol, change **“internally reproduced”** to **“independently validated by TESTfort”** only for the measurements and receipts that TESTfort actually confirms.

---

## 1. YC-ready company explanation

### One sentence

LuxiEdge is building deterministic, energy-aware execution infrastructure for AI: an engine that minimizes and audits the path data takes through a GPU so that repeated work is reproducible, measurable, and efficient.

### Short application answer

LuxiEdge has built a working deterministic transformer executor for NVIDIA H100 GPUs. It packs independent sequences into a device-resident execution path, records exactly which attention backend ran, fails evidence checks on silent fallback, and produces repeatable full-stack fingerprints. On a matched Qwen2-7B prefill workload, the current version used less GPU-board energy per position than both default vLLM and vLLM's batch-invariant mode, although it remains slower. We are commissioning independent TESTfort validation and then moving toward design-partner workloads.

### What is technically different

Most inference engines optimize first for maximum aggregate serving throughput and treat reproducibility as a configuration concern. LuxiEdge is designed around an explicit execution contract:

- Keep the model and working state on the GPU.
- Pack independent sequences so large matrix operations do useful work together.
- Eliminate redundant movement, conversions, launches, synchronization, and intermediate materialization.
- Make the attention backend and any fallback observable.
- Preserve deterministic reduction/order choices where tested.
- Produce numerical and energy receipts that can be replayed and audited.
- Separate faithful/audit execution from explicitly optimized trade execution so evidence from one path is not presented as evidence for the other.

### Recommended YC focus

Lead with the **deterministic transformer executor** as the current product wedge. Present the separate quant engine as evidence that the same deterministic-compute discipline applies beyond LLM inference, especially to financial and regulated numerical workloads. Do not pitch two unrelated startups.

The coherent platform story is:

> LuxiEdge is a deterministic compute platform. The transformer executor is the current GPU-scale product wedge; the quant engine is a working proof that receipt-driven numerical execution also applies to risk and audit workloads.

---

## 2. What the transformer engine does now

The current transformer system is a Rust/CUDA, Qwen2-7B-class, packed and device-resident **prefill executor**. It is a working research/product prototype, not yet a complete continuous-batching inference server.

### Implemented functionality

| Function | Current state |
|---|---|
| Model class | Qwen2-7B-class, 28-layer transformer path |
| Hardware tested | One NVIDIA H100 80GB HBM3 |
| Workload | Packed prefill at sequence length 128; peer headline is batch 16 |
| Packed layout | Independent `[B,S,H]` sequences are viewed as `[B×S,H]` for major GEMMs |
| Device residency | Main stack operates device-resident across the measured hot path |
| Attention boundary | Luxi-owned `attention_forward(...)` boundary with replaceable backend |
| Current fast attention | Third-party FlashAttention `2.8.3.post1` integration |
| Fallbacks | Batched Waller fallback plus serial oracle retained |
| Backend provenance | Backend used and fallback count are recorded; peer runs require `flash` and fallback `0` |
| Normalization | Parallel deterministic two-pass LayerNorm path used by the champion |
| MLP path | Half-precision intermediate and fused/packed operations used in the trade lane |
| Receipts | Full-stack fingerprints, numerical oracle comparison, contamination checks, backend identity, hashes |
| Energy | Cumulative NVML GPU-board energy over synchronized hot windows |
| Reproduction | Clean clone, immutable Git pin, binary hash, model identity, package identity, raw runs |

### Main mechanisms in the current champion

- Fused Flash half-precision QKV packing and reduced copy work.
- Flash half output feeding the half-precision output projection path.
- Half-precision MLP intermediate with half GELU path.
- Fused bias, residual, and second LayerNorm dataflow.
- Parallel two-pass LayerNorm.
- Packed GEMMs whose launch count does not grow with batch.
- A light device-to-host path for sustained measurement.

These are LuxiEdge integration and execution-path mechanisms. LuxiEdge does **not** claim ownership of FlashAttention; it is a third-party dependency behind LuxiEdge's backend boundary.

### What it does not do yet

- It is not a drop-in replacement for the full vLLM serving stack.
- It has not established continuous-batch serving parity, decode throughput, time to first token, or inter-token latency.
- It has not proven multi-GPU or tensor-parallel scaling.
- It has not proven training, backpropagation, convergence, or training-energy benefits.
- It has not proven universal behavior across GPU models, drivers, CUDA versions, or dependency versions.
- It has not beaten default or batch-invariant vLLM on throughput in the tested arm.
- It has not earned a B32 peer energy claim; the audited B32 energy result narrowly missed its pre-registered comparison ceiling.

---

## 3. Does LuxiEdge have a deterministic transformer?

**Yes, within the tested and pinned execution contract.** The word “deterministic” is supportable here, provided the scope is stated.

### What has been demonstrated

| Receipt | Result |
|---|---:|
| Full-stack fixed-input fingerprint, B1 | Stable across 3 repeats; all 28 layers/all rows; max repeat delta `0`; finite; Flash; fallback `0` |
| Full-stack fixed-input fingerprint, B16 | Stable across 3 repeats; all 28 layers/all rows; max repeat delta `0`; finite; Flash; fallback `0` |
| Full-stack fixed-input fingerprint, B32 | Stable across 3 repeats; all 28 layers/all rows; max repeat delta `0`; finite; Flash; fallback `0` |
| Overall stack fingerprint | `PASS` |
| Flash vs serial attention oracle | Maximum absolute difference approximately `2.8e-5` at B1 and B32 |
| Cross-sequence contamination | `0` in the tested receipt |
| Repeated generated completion | Identical in 3 repeats in the internal confirmation |
| Backend identity | `flash` |
| Silent fallback count | `0` |

This means that, for the pinned model, code, configuration, software environment, H100, shapes, and inputs tested, LuxiEdge follows a repeatable numerical execution path and returns the same tested full-stack result from run to run.

### Deterministic does not mean bit-for-bit identical to every implementation

LuxiEdge's Flash result is repeatable but differs slightly from the serial oracle—about `2.8e-5` maximum absolute error—because operation grouping and precision differ. That is compatible with a deterministic, explicitly non-bit-exact numerical contract.

### What is not yet proven

- Exact equality across different GPU architectures.
- Exact equality across driver, CUDA, PyTorch, or FlashAttention upgrades.
- Full online-serving determinism under arbitrary arrival timing.
- Direct Luxi request batch-order and batch-composition invariance; the current executor API did not expose the required shuffle test, so that receipt is `N/A`, not a pass.
- Equality with Hugging Face or another gold model for every token and end-to-end generation scenario.

### Correct comparative language about vLLM

Do not say that “vLLM is nondeterministic” as a universal statement. vLLM's default maximum-performance mode does not provide the same batch-invariant guarantee, but vLLM offers a documented batch-invariant mode. That mode was enabled successfully and passed the internal repeat, batch-composition, and batch-order screen.

The accurate comparison is:

> LuxiEdge is deterministic under its tested fixed execution contract. vLLM's default mode prioritizes performance, while vLLM's separately enabled batch-invariant mode provides a determinism-aligned comparison and incurs a measured performance and energy cost on this workload.

### Who benefits from this form of determinism

- **Regulated and audited deployments:** preserve a replayable record of what code path and backend produced a result.
- **Financial and scientific users:** separate numerical changes from scheduler or reduction-order noise.
- **Safety and model-validation teams:** make regression investigation easier because the same test should produce the same receipt.
- **Inference operators:** detect silent backend fallback or a changed execution path before it becomes an unexplained production variance.
- **Developers:** compare optimizations against a stable oracle and catch cross-sequence contamination.

Determinism does not automatically reduce the number of training passes or guarantee model correctness. Its immediate value is repeatability, isolation, debugging, validation, auditability, and control. Longer term, those properties can reduce repeated investigations, failed reproductions, and validation overhead, but LuxiEdge has not yet quantified those system-level savings.

---

## 4. Measurement protocol in plain English

Both engines were run sequentially on the same H100 and same staged Qwen2-7B model. Each iteration processed 16 independent prompts, each 128 tokens long. Prompts were unique and vLLM prefix caching was disabled so neither engine received free work from reusing cached prefixes.

vLLM generated one token only to force the prefill execution, but that generated token was not counted. The measured unit is therefore a **prefill position**, calculated as:

```text
positions = completed iterations × batch 16 × sequence length 128
```

Energy was read from the H100's cumulative NVML board-energy counter at the beginning and end of the synchronized hot window:

```text
GPU-board J/position = cumulative board joules during hot window / completed prefill positions
```

This is a defensible GPU-board energy measurement. It is **not** wall-plug, rack, cooling, facility, PUE, carbon, or electricity-bill energy.

### Matched conditions

| Item | Contract |
|---|---|
| GPU | Same single H100; engines run sequentially |
| Model | Same Qwen2-7B staged model path |
| Precision | FP16 execution contract |
| Shape | B16, S128 for the peer headline |
| Cache | Prefix cache off |
| Prompts | Unique, exactly 128 token IDs for vLLM |
| Parallelism | Tensor parallelism 1 |
| Counted work | Prefill positions only |
| vLLM generation | `max_new_tokens=1`, excluded from work count |
| Energy source | Cumulative NVML total-energy delta |
| Luxi attention | Flash backend, fallback `0` |
| Reproduction controls | Clean clone, immutable commit, package/GPU/model/binary receipts |

---

## 5. Current peer results

### 5.1 Preferred conservative YC headline: clean-clone 60-second testing

The preferred headline uses the longer clean-clone dress rehearsal and its reverse-order confirmation. It is slightly more conservative than the frozen 20-second package.

#### Primary arm: Luxi first, then default vLLM — five 60-second runs each

| Engine | Median positions/s | Median board J/position |
|---|---:|---:|
| LuxiEdge | 27,227 | 0.01894 |
| vLLM default | 35,194 | 0.01924 |

- Luxi throughput ratio: `0.774`, or **77.4%** of vLLM.
- Luxi board-energy advantage: approximately **1.6% lower J/position**.
- Luxi Flash fallback count: `0`.

#### Reverse arm: default vLLM first, then Luxi — three 60-second runs each

| Engine | Median positions/s | Median board J/position |
|---|---:|---:|
| vLLM default | 35,000.5 | 0.019265 |
| LuxiEdge | 27,132.8 | 0.018975 |

- Luxi throughput ratio: `0.775`, or **77.5%** of vLLM.
- Luxi board-energy advantage: **1.50% lower J/position**.
- Luxi Flash fallback count: `0`.

The forward and reverse arms agree at noise scale. The energy edge did not disappear when engine order was reversed.

### 5.2 Determinism-aligned comparison: vLLM batch-invariant mode

The confirmed vLLM batch-invariant screen used the same B16/S128 workload and machine, with three 30-second runs.

| Run | vLLM batch-invariant positions/s | Board J/position |
|---:|---:|---:|
| 1 | 30,622.8 | 0.020487 |
| 2 | 30,709.0 | 0.020482 |
| 3 | 30,686.6 | 0.020563 |
| Median | **30,686.6** | **0.020487** |

Relative to the reverse-order Luxi result:

| Comparison | Result |
|---|---:|
| Luxi throughput / batch-invariant vLLM throughput | **88.4%** |
| Luxi throughput gap | **11.6% slower** |
| Luxi board J/position advantage | **7.4% lower** |
| Luxi positions-per-board-joule advantage | **approximately 8.0% higher** |

Relative to default vLLM on the reverse-order arm, enabling vLLM batch invariance produced:

| vLLM mode cost on this workload | Result |
|---|---:|
| Throughput change | **12.3% lower** |
| Board J/position change | **6.3% higher** |

The vLLM batch-invariant mode also passed its internal functional screen: repeated hash stability, batch-composition stability, batch-order stability, and zero recorded mismatches.

This is an **energy win, not a double win**. LuxiEdge used less board energy per position but remained slower.

### 5.3 Frozen 20-second NVML evidence package

The immutable `version-99` evidence package contains a separate three-run, 20-second B16 result with raw cumulative NVML energy fields.

| Engine | Median positions/s | Median board J/position |
|---|---:|---:|
| LuxiEdge | 27,823.9 | 0.0186918 |
| Matched vLLM | 35,475.7 | 0.019114 |

- Luxi throughput ratio: approximately **78.4%**.
- Luxi board-energy advantage: approximately **2.2% lower J/position**.
- Raw Luxi runs retain `energy_J_hot`, completed position counts, hot-window duration, sample count, and fallback status.

The raw Luxi run medians were derived from:

| Run | Positions/s | Energy J in hot window | Positions | Board J/position |
|---:|---:|---:|---:|---:|
| 1 | 27,266 | 10,325.295 | 548,864 | 0.018812 |
| 2 | 28,400 | 10,586 | 569,344 | 0.018593 |
| 3 | 27,824 | 10,412 | 557,056 | 0.018692 |

This package is useful supporting evidence. For a public or YC headline before TESTfort reports, prefer the longer and more conservative **77.5% throughput / 1.5% lower energy** clean-clone result.

### 5.4 Ten-minute stability soak

Both LuxiEdge and default vLLM passed a ten-minute soak in the dress rehearsal.

| Engine | Throughput drift vs primary median | J/position drift vs primary median |
|---|---:|---:|
| LuxiEdge | +0.03% | +0.44% |
| vLLM | -0.26% | +0.17% |

All changes were well within the approximately 3% stability gate used in the rehearsal.

### 5.5 B32 status

B32 is not a current peer-energy claim. Audited Luxi throughput was roughly 28,860–29,008 positions/s, but median board energy of approximately 0.01819–0.01826 J/position narrowly missed the fresh comparison ceiling of 0.018176. Report the miss; do not round it into a win.

---

## 6. What the results mean—and do not mean

### What is legitimately impressive now

1. **The system works.** It is executing a real 28-layer Qwen2-7B-class packed prefill path on H100.
2. **The result is reproducible internally.** The peer outcome survived a clean clone, longer windows, reverse engine order, and a ten-minute soak.
3. **The deterministic execution contract has receipts.** Full-stack hashes are stable in the tested scope, sequences remain isolated, and backend/fallback identity is explicit.
4. **There is a measured energy advantage.** Luxi is not merely drawing less instantaneous power; it uses fewer cumulative GPU-board joules for each completed prefill position in the B16 comparison.
5. **The determinism-aligned energy difference is meaningful.** Against vLLM batch-invariant mode, the internal result is 7.4% lower board J/position, equivalent to roughly 8.0% more positions per board joule.
6. **The mechanism advanced quickly.** The engine moved from approximately 9.7–10.0k positions/s and roughly 0.030 J/position to about 27k positions/s and 0.019 J/position in the current champion line.

### What is not proven

1. Luxi is not faster than vLLM in the tested comparison.
2. Luxi is not yet a full serving product or a drop-in vLLM replacement.
3. Prefill positions/s is not the same metric as generated serving tokens/s.
4. Board energy is not facility or customer electricity consumption.
5. One model, one shape, one GPU, and one precision do not establish universal superiority.
6. The internal result is not independent validation until TESTfort completes and signs its report.
7. Deterministic execution does not prove model truth, safety, or accuracy.
8. The result does not establish training or fine-tuning savings.

---

## 7. Claim guide for the YC application agent

### Safe to say now

- “We have a working deterministic transformer executor running on an NVIDIA H100.”
- “In internally reproduced clean-clone Qwen2-7B prefill tests, LuxiEdge retained 77.5% of default vLLM throughput while using 1.5% less GPU-board energy per position.”
- “Against vLLM's batch-invariant mode, LuxiEdge retained 88.4% of throughput and used 7.4% less GPU-board energy per position in our internal matched screen.”
- “That is approximately 8% more prefill positions per GPU-board joule in the determinism-aligned arm.”
- “The output fingerprint was stable across repeated full-stack runs at B1, B16, and B32 in the pinned H100 environment.”
- “The attention backend was Flash and silent fallback was zero.”
- “We have rehearsed the independent protocol from a clean clone, reversed engine order, and completed ten-minute stability soaks.”
- “TESTfort independent validation is pending.”

### Say only with the qualifier shown

| Topic | Required qualifier |
|---|---|
| Deterministic | “Within the pinned H100/model/build/configuration and tested shapes.” |
| Energy | “Cumulative NVIDIA GPU-board energy per prefill position.” |
| Throughput | “Packed B16/S128 Qwen2-7B prefill positions per second.” |
| vLLM comparison | State default mode or batch-invariant mode; never merge the two baselines. |
| Third-party validation | “Pending” until the signed report exists. |
| Production | “Working prototype/executor,” not production-ready serving platform. |
| FlashAttention | Third-party dependency; Luxi owns the integration boundary and surrounding execution path, not FlashAttention. |

### Do not say

- “LuxiEdge is faster than vLLM.”
- “LuxiEdge beats vLLM overall.”
- “vLLM is nondeterministic.”
- “LuxiEdge is universally or cross-platform bitwise deterministic.”
- “LuxiEdge saves 7.4% on a customer's electricity bill.”
- “LuxiEdge is 7.4% lower wall-plug or facility energy.”
- “LuxiEdge does 27,000 generated tokens per second.”
- “LuxiEdge is production-ready.”
- “The B32 peer-energy result passed.”
- “TESTfort validated the result” before the report is complete.
- “FlashAttention is LuxiEdge proprietary technology.”

### Best short quantitative statement

> In a clean-clone internal H100 test on Qwen2-7B packed prefill, LuxiEdge delivered 27.1k positions/s at 0.01898 GPU-board J/position. That was 77.5% of default vLLM's speed with 1.5% lower board energy per position. Against vLLM batch-invariant mode, Luxi retained 88.4% of speed and used 7.4% less board energy.

### Best “what is your breakthrough?” statement

> We made deterministic GPU execution measurable rather than aspirational. The engine records the backend it actually used, rejects silent fallback in its evidence path, produces stable full-stack receipts, and has already crossed into lower energy per unit of useful prefill work than vLLM on our matched H100 test—even though throughput remains the next gap to close.

### Best honest founder-progress framing

> Eric Waller conceived the execution and product direction and has used AI coding agents as a high-speed engineering force multiplier. In a short development cycle, the system progressed from a slow serial-attention prototype to a packed, device-resident H100 executor with deterministic receipts and a measured peer-energy advantage. The code, benchmarks, failed gates, and frozen evidence are retained in the repositories.

Do not imply a conventional large engineering team if that is not true. The rapid AI-assisted build process is itself part of the story, but the evidence—not the amount of generated code—is the proof.

---

## 8. Current product maturity

| Area | Maturity today |
|---|---|
| Core transformer execution | Working H100 prototype |
| Packed B16/S128 prefill | Working and repeatedly measured |
| Scoped deterministic receipt | Working |
| Numerical serial oracle | Working for attention receipt |
| Backend/fallback audit | Working |
| Cumulative board-energy harness | Working |
| Clean third-party replay package | Prepared and rehearsed |
| Independent TESTfort report | Pending |
| Full continuous-batch serving | Not established |
| Decode/latency competitiveness | Not tested in current claim |
| Multi-model/general hardware support | Not established |
| Production operations/security/SLA | Not established |
| Design-partner deployment | Next commercial phase |

The correct maturity label is **working deep-tech prototype with a frozen, reproducible validation package**.

---

## 9. The separate Luxi Quant Engine

LuxiEdge also has a separate Rust repository implementing deterministic numerical and risk primitives. It is real software, but it should be described as a **working prototype/pilot integration candidate**, not a production-ready regulated risk system.

### Working functionality

- Deterministic-oriented CPU numerical path using `f64`, Kahan-compensated reductions, and explicit SIMD paths.
- Canonical SHA-256 receipts over little-endian `f64` result bytes.
- Scalar/vector functions including softmax, normal CDF, GELU, exponential, error function, gamma, LayerNorm, RMSNorm, SiLU, square root, and logarithm.
- Matrix and signal primitives including matrix multiplication, Cholesky, and one-dimensional convolution.
- Welford/Kahan online statistics, online covariance, and online sample covariance.
- A synthetic risk pipeline calculating covariance, portfolio variance/volatility, and delta/gamma-style risk with JSON output and receipts.
- A portable `wgpu` GPU path for covariance and portfolio variance.
- Research CUDA kernels accessible through the direct CUDA API.
- A small Axum REST integration surface with health and evaluation endpoints.

### Important current limitations

1. The risk pipeline uses synthetic observations and is not a live market-risk system.
2. The command-line `--benchmark N` path does not yet implement the full repeated benchmark loop it appears to promise.
3. Combining `--online-cov --gpu` does not currently prove that the GPU portfolio result consumed the host-computed online covariance; the high-level GPU path rebuilds its synthetic covariance inputs.
4. The high-level risk pipeline's `--gpu` path uses `wgpu`; the research CUDA kernels are a separate direct API, not that pipeline backend.
5. The REST server's `f32` expression dispatch is incomplete and should not be described as feature-equivalent to its `f64` dispatcher.
6. Cross-machine bit identity is an intended controlled-build contract but has not been independently established across every supported CPU/SIMD path.
7. This repository's licensing metadata is inconsistent: `Cargo.toml` says MIT while other project material describes proprietary/NDA or a planned AGPLv3 plus commercial strategy. Reconcile this with counsel before a public release or categorical YC licensing statement.

### YC-safe quant description

> LuxiEdge also has a working Rust quant engine that applies the same receipt-driven philosophy to numerical and risk computation: compensated reductions, deterministic-oriented CPU math, online covariance, synthetic portfolio risk, canonical output hashes, and portable GPU experiments. It is a prototype for audit-sensitive financial computation, not yet a production risk platform.

### How it supports the main story

The quant engine demonstrates that the core idea is broader than attention: define the numerical contract, remove ambiguous execution choices, retain a receipt, and optimize without losing provenance. It should support the transformer platform thesis, not distract from the independently testable H100 result.

---

## 10. Intellectual property, licensing, and dependencies

### Current safe posture

- Treat the repositories and product as proprietary/private unless and until the actual release and licensing state says otherwise.
- If the planned direction remains AGPLv3 plus a commercial license, describe it as **being prepared**, not already released.
- Reconcile the quant repository's MIT metadata with the intended company strategy before publication.
- Preserve FlashAttention and every other third-party dependency's license and notices.
- State “patent pending” only if the filed application and counsel-confirmed status can be produced. Never say patented or granted unless an issued patent exists.

### Dependency attribution

The transformer champion uses third-party FlashAttention `2.8.3.post1`. LuxiEdge's proprietary contribution is the larger execution architecture and implementation around it: packed/device-resident dataflow, the replaceable `attention_forward` boundary, integration and bridge work, normalization/MLP mechanisms, oracle/fallback structure, deterministic receipts, measurement harnesses, and evidence discipline.

---

## 11. Evidence and reproducibility map

### Transformer identity

| Item | Value |
|---|---|
| Repository | `https://github.com/RegularJoe-CEO/attention-transformer-v2` |
| Branch | `version-99` |
| Immutable pin | `d9ad7e06ff727440a1237e9fbb709d8bad0ba452` |
| Mechanism commit recorded in peer package | `1356cf7` |
| Publishable NVML-arm binary SHA-256 | `cce7928038daa9248ffc989e0a4dbe04a2c5b93c0cc1829260ff278219003480` |
| Primary third-party harness | `scripts/peer_nvml_harness.py` |
| Matched vLLM harness | `scripts/matched_vllm_prefill_compare.py` |
| Attention receipt | `scripts/flash_determinism_receipt.py` |
| Full-stack receipt entry | `examples/cuda_qwen7b_trade.rs --stack-fingerprint` |
| Frozen peer scoreboard | `evidence/peer-win-2026-07-22/SCOREBOARD.md` |
| Third-party replay guide | `evidence/peer-win-2026-07-22/REPLAY_THIRD_PARTY.md` |

### Rehearsed environment identity

| Item | Recorded value |
|---|---|
| GPU | NVIDIA H100 80GB HBM3 |
| Prior GPU UUID | `GPU-0fab7d65-c9c7-f2dd-0e7b-4a9efc010912` |
| NVIDIA driver | `580.126.09` |
| CUDA reported | `13.0` |
| H100 power limit | `700 W` |
| Maximum SM clock | `1980 MHz` |
| FlashAttention | `2.8.3.post1` |
| vLLM | `0.25.1` |

The independent report must record the environment it actually observes. Expected values are identity checks, not permission to substitute local historical metadata for third-party observations.

### Pending TESTfort package

The controlling test document is:

```text
TESTFORT_VERSION_99_H100_VALIDATION_EMAIL_2026-07-22.md
```

It specifies a four-hour independent run with:

1. Immutable identity and clean build.
2. B16 default-vLLM comparison with order control.
3. A vLLM batch-invariant comparison and separate determinism receipts.
4. Ten-minute stability soaks.
5. Independent arithmetic from raw cumulative NVML energy and completed work.
6. Explicit exclusions for B32, serving/decode, equal-power, training, and facility-energy claims.

---

## 12. What should change after TESTfort reports

Do not rewrite the story from memory. Apply the following rules:

1. Preserve every internal result as “internal” and every TESTfort result as “independent.”
2. Use TESTfort's raw run table and independently recomputed medians for the public headline.
3. If TESTfort reproduces the gates, use:

   > Independently validated by TESTfort on one NVIDIA H100: [TESTfort Luxi result] versus [TESTfort vLLM mode/result] on the defined Qwen2-7B B16/S128 prefill workload.

4. If its result differs, publish the third-party result and explain the variance; do not silently retain the stronger internal number as though it were the validation result.
5. State the exact vLLM mode: default or batch-invariant.
6. Keep `positions/s`, `J/prefill position`, and `GPU-board energy` in the wording.
7. Link or attach the report and immutable Git pin.
8. Do not promote any scope TESTfort did not run.

---

## 13. Near-term company plan

### Technical

1. Complete independent TESTfort validation on the frozen pin without modifying product source.
2. Preserve the B16 energy win while closing the remaining throughput gap.
3. Extend deterministic receipts to request-order and batch-composition behavior in Luxi's own public executor API.
4. Add serving/decode measurements only after local rehearsal.
5. Expand one variable at a time: shape, model, GPU, then serving topology.
6. Keep B32 honest until its cumulative-energy result clears a fresh peer ceiling.

### Commercial

1. Use the independent report as the first external evidence anchor.
2. Recruit design partners that value reproducibility, audit, or GPU-board energy—not generic chatbot hosting first.
3. Define one paid pilot around an actual customer workload and success metric.
4. Decide whether the first wedge is audit-sensitive inference, regulated/financial compute, or an engine/OEM integration.
5. Convert prefill evidence into buyer-relevant workload and total-cost evidence before making broad savings claims.

### YC narrative

The credible seed-stage story is not “we already replaced vLLM.” It is:

> We found an execution architecture that makes deterministic GPU computation measurable and has already produced a reproducible peer-energy win on a real H100 workload. The current system is slower than the mature incumbent, but the gap shrank dramatically in hours, the evidence survived clean recreation and order reversal, and independent validation is underway. We are building the platform and looking for design partners whose workloads value both efficiency and reproducibility.

---

## 14. Open questions the YC application agent must ask Eric—not invent

- Exact founder biography and the most relevant prior accomplishments.
- Company formation, ownership, and current team structure.
- Customers, pilots, letters of intent, revenue, or user count.
- Whether a patent application was filed and the exact confirmed status.
- Current ownership of every code contribution produced with collaborators or AI tools.
- The final licensing strategy and timing.
- Fundraising amount and use of funds.
- Which initial customer wedge Eric wants to prioritize.
- Whether TESTfort has completed, and the exact signed report results.

No technical benchmark should be converted into traction, customer savings, or production readiness without separate evidence.

---

## 15. Final source-of-truth summary

**What exists:** A working Rust/CUDA deterministic packed-prefill executor for a Qwen2-7B-class model on H100, plus a separate deterministic-oriented Rust quant engine prototype.

**What is proven internally:** Stable full-stack fingerprints in the pinned execution scope; numerical oracle tolerance; zero tested cross-sequence contamination; explicit Flash backend with zero fallback; roughly 27.1k B16/S128 prefill positions/s; 1.5% lower GPU-board J/position than default vLLM in longer order-controlled runs; 7.4% lower GPU-board J/position than vLLM batch-invariant mode while retaining 88.4% of its throughput; stable ten-minute soaks.

**What remains weaker:** Throughput—Luxi retains 77.5% of default vLLM and 88.4% of batch-invariant vLLM in the stated comparisons.

**What is pending:** Independent TESTfort validation and expansion beyond this narrow prefill workload.

**How to describe maturity:** Working deep-tech prototype with a frozen reproducibility package and an internally reproduced energy advantage; not yet a production serving replacement.

**Why it may matter:** It combines energy-aware execution with explicit deterministic receipts, a combination that can matter for audit-sensitive, financial, scientific, safety, and infrastructure users—provided the result generalizes beyond the current validated wedge.

