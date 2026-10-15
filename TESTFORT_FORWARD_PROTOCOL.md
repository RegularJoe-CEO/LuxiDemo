# Proposed go-forward TestFort evaluation protocol

Status: proposed protocol, not a completed evaluation.

Purpose: independently test the current public Luxi claims across quant, numerical primitives, orbital propagation, and inference. The protocol must preserve Luxi's proprietary implementation boundary. TestFort receives signed closed binaries, public or agreed test inputs, and verification tools, not source code, private methods, model weights, credentials, or pod access details.

## 1. Freeze the test article

Before execution, TestFort records:

- binary SHA-256, version, build identifier, and signing status,
- model or mathematical workload and input hashes,
- hardware SKU, CPU architecture, GPU, driver, runtime, and operating system,
- comparator versions and configuration,
- the accepted work unit for throughput and energy,
- the start and end date for every campaign,
- every exclusion, patch, retry, crash, and invalidated run.

Luxi may stage binaries. TestFort controls the final command order, retains the raw outputs, and signs the final manifest.

## 2. Independent test lanes

| Lane | Required workload | Required checks | Required limits |
|---|---|---|---|
| Risk State Engine | Risk Runtime v0.4.0 under the paid confidential evaluation and NDA, using an agreed scenario-by-position matrix | receipt stability, authoritative rebuild comparison, repeatability, throughput, latency, failure handling | confidential inputs and methodology remain customer-controlled; public report must not imply a public download or disclose customer data |
| LuxiBook | LuxiBook v0.2.1 with the public 10-row example book and an agreed larger book | Ed25519 receipt verification, output-vector hash, CPU and CUDA repeatability, reference-price comparison | report each book and device separately; do not generalize to every book or GPU |
| LuxiRisk | LuxiRisk v0.2 public legacy calculator vectors | receipt verification, formula-vector correctness, repeatability on each published binary | free legacy demo only; not the Risk State Engine |
| GPU numerical primitive | Published 10,000,000-value float32 sum workload on NVIDIA L4, plus an agreed second GPU | exact-reference error, repeatability, throughput, energy only where sampling resolution supports it | synthetic values, not a customer portfolio; report correctness and speed paths separately |
| Orbital propagation | Published jax100 and starlink-382a419 grids, plus the public 15,940-object catalog where supported | output hashes, rejected states, reference deltas, throughput, endurance | identify native versus qemu or Rosetta; H100 SGP4 is near-earth only unless a new build is explicitly tested |
| Qwen2-7B inference | Qwen2-7B-Instruct fp16 on H100, compared with vLLM 0.25.1 normal and batch-invariant modes and SGLang 0.5.19 normal and deterministic modes | tokens, selected logits or logprobs, determinism across load, throughput, latency, and GPU-board energy | one model and tested GPU; current SGLang version boundary; no full-server claim unless HTTP serving is actually tested |
| Additional evidence | Kessler, SAAL, OrbVeil, genomics, and attention-v2 only when their exact test articles are included in the signed manifest | reproduce the claim named in each evidence page | report these as separate families, not as proof of the Risk State Engine or current inference claims |

## 3. Determinism and correctness matrix

For every lane:

1. Run identical accepted inputs repeatedly on the same process.
2. Restart the process and repeat.
3. Change only the tested concurrency, batch, worker count, or request order.
4. Run on every declared hardware target.
5. Compare exact output bytes or the published canonical hash.
6. Compare against the lane's external reference or exact oracle.
7. Keep all mismatches and faults in the evidence pack.

A determinism pass requires zero unexplained hash mismatches inside the declared matrix. Correctness is a separate gate. Matching the same wrong answer is not a correctness pass.

For inference, include batch 1, 16, and 64 generation conditions and the published 2k, 8k, and 32k prompt shapes. Include mixed-length batches and staggered arrivals only for engines that expose those interfaces. Report token identity, visible-answer identity, and logit or logprob identity separately. Do not describe top-token or top-five agreement as full-sequence equivalence.

## 4. Performance and energy

- Alternate Luxi and each comparator in matched blocks on the same hardware.
- Preserve the current six-run median design for every reported cell.
- Publish every per-run row, not only the median.
- Report throughput, latency, and energy with the same accepted work unit.
- Use NVML GPU total energy for GPU-board measurements and state that boundary in every table.
- If facility energy is tested, meter it separately and never merge it with GPU-board energy.
- Report wins, ties, and losses. Do not create a composite score unless its formula is fixed before the run.

Generation must appear before prefill in the inference executive summary because normal-mode generation is the current energy tradeoff.

## 5. Endurance and fault handling

Each current binary intended for sustained use must complete a 24-hour soak and at least 100,000 accepted calls. The workload mix, concurrency schedule, restart policy, and memory telemetry are fixed before execution.

Pass conditions:

- zero unexplained output-hash changes for identical accepted inputs,
- zero silent fallbacks,
- zero lost or duplicated accepted requests,
- zero unreported process or GPU faults,
- stable post-soak correctness and receipt verification.

The published long-running Qwen2-7B fault is a required regression case. A fresh-process rerun does not erase an endurance failure. The final report must show the failure, diagnosis status, and rerun separately.

## 6. Reporting package

TestFort should deliver:

- signed executive report,
- signed manifest of binaries, inputs, comparators, and hardware,
- raw per-run rows and timestamps,
- exact commands or operator runbook that do not expose proprietary implementation,
- failure and retry log,
- correctness and determinism matrices,
- separate throughput, latency, GPU-board energy, and any facility-energy tables,
- limitations and non-claims,
- public redacted pack plus a confidential appendix when NDA material is involved.

The executive conclusion must name the workload, hardware, date, and limits. It must state that the December 2025 seven-function report and July 2026 packed-prefill report concern different builds and workloads and do not validate the current September inference comparisons.

## 7. Release gate

No new "independently evaluated" claim should be attached to a current Luxi surface until the signed manifest, signed report, raw rows, and public limitations are available. The original December 2025 report remains valid only for its stated seven-function numerical workload. The July 2026 packed-prefill pack remains historical lineage and is not required in current top-line marketing.
