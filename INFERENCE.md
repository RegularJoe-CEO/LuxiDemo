# LuxiEdge inference

LuxiEdge Inference is built for reproducible inference across rented GPUs and CPUs. In the November 2026 tests below, frozen DeepSeek-R1-Distill-Qwen-7B jobs replayed on the listed GPUs and CPUs gave the same token ids and per-step logit fingerprints; other models, jobs and machines are outside this evidence. The engine source is private. What the public can do today:

| You want to | Where |
|---|---|
| Run a real replay yourself (Linux x86_64, NVIDIA GPU or CPU) | [`downloads/luxi-replay-demo/`](downloads/luxi-replay-demo/): closed-binary package, checksums, license, exact commands |
| Inspect the November 9 nine-GPU result | [`evidence/r1-distill-7b-gpu-swap-2026-11-09/`](evidence/r1-distill-7b-gpu-swap-2026-11-09/): per-GPU result files, the 17 signed H100 NVL records, the public key, an offline verifier, and a statement of what was not retained |
| Inspect how the public package was tested | [`evidence/r1-replay-package-2026-11-10/`](evidence/r1-replay-package-2026-11-10/) |
| Test the engine on your own model, workload or GPUs | Private GPU-swap evaluation: https://luxiedge.com/contact#gpu-swap-evaluation |

There is no browser inference demo, and the finance demos (LuxiBook, LuxiSize) are not this inference engine. The old `luxiedge-serve` binary in [`downloads/`](downloads/) is a historical API toy with a toy generate path, not the current reasoning engine.

## November 2026: DeepSeek-R1-Distill-Qwen-7B, frozen engine v27

### November 9 GPU swap test (internal measurement)

Frozen engine v27 (`795c71736e22e2f6`), DeepSeek-R1-Distill-Qwen-7B w8a8 (RedHatAI), RunPod.

- Nine GPU types: H100 NVL, H200, A100 SXM 80GB, RTX PRO 6000 Blackwell Server Edition, RTX A6000, A40, RTX 5090, RTX 4090 and RTX 3090.
- Each matched 136/136 sampled answers and 546,114 generated tokens, with matching recorded per-step fingerprints against the signed H100 reference, plus a separate 92/92 Qwen gate. This is 17 problems with eight samples each, not 136 distinct questions.
- Seven CPU models: AMD EPYC 9654, 7742, 9355, 7352 and 7452; Intel Xeon Platinum 8568Y+ and Xeon Gold 6342. Each matched 2/2 small Qwen checks and the short signed R1 replay capped at 32 tokens. These CPUs did not run the full GPU workload in this evidence.
- A40 soak: 1,502 calls, 1,239 checks, zero mismatches.
- vLLM 0.32 normal was faster and cheaper per token wherever the tested int8 configuration ran. Its sequence matches are against its own H100 reference, not Luxi. Different sequences do not establish accuracy errors.
- The rental-choice comparison is Luxi versus Luxi: RTX 5090 delivered about 80% of Luxi H100 throughput at about one-third of its calculated rental cost per output token. November rental rates exclude licensing and other operating costs; they are not API prices or current rental offers.
- Internal measurements, one cloud, fixed tested configurations. Single performance runs; initial 32-slot timings include one-time compilation. NVML GPU-board energy is not facility electricity.

Full table (throughput, rental cost, board energy, vLLM comparator): [`evidence/r1-distill-7b-gpu-swap-2026-11-09/README.md`](evidence/r1-distill-7b-gpu-swap-2026-11-09/README.md). That folder also states which November 9 files were retained and which were not.

### November 10 public replay package test

November 10, 2026 package test (internal measurement): from a clean install of this tarball, with no private source or credentials, the 32-token replay produced identical records (every token id and per-step logit fingerprint) on an RTX 5090 and an A40 (twice each), on their host CPUs (AMD EPYC 7543 and Intel Xeon Gold 6342, once each), and on a separate x86_64 sandbox CPU (twice): all 10 cross-machine pairs compared identical. Their step root begins `158ee5ae768cb633`, the 16 hex digits that survive from the November 9 H100 NVL short replay; the full November 9 short record was not recovered. The full suite matched all 136 answers and 546,114 tokens of the 17 signed November 9 H100 NVL records on the RTX 5090 (301.6 s at 32 slots) and on the A40 (799.0 s at 32 slots), and the two GPUs matched each other on all 17 jobs. On all three machines each record verified offline against that installation's public key and was rejected under a wrong key, and the tamper test rejected every tested edit (changed token, per-step fingerprint, prompt token, seed or temperature; a dropped token; a re-signed forgery). Machines: two RunPod pods and one sandbox VM. Full-suite timings are single runs. Raw outputs: [`evidence/r1-replay-package-2026-11-10/`](evidence/r1-replay-package-2026-11-10/).

The package is a new build identity (`build_sha256` `9f7931d9…6694`), compiled from the same v27 source. Its GPU matrix-multiply library differs from the one used on November 9; the model, quantization, sampler and reference outputs are unchanged. It does not inherit the November 9 nine-GPU result and was validated separately: its own tested machines are listed above.

## Historical: Qwen2-7B on H100 (September 2026)

The results in this section concern Qwen2-7B-Instruct and earlier engine builds, measured September 25-26, 2026. Their normal-speed, deterministic-mode and quality claims do not transfer to the November v27 build or the replay package.


**LuxiEdge keeps Qwen2-7B outputs bit-identical across every batch size we tested on an H100, runs at normal serving speed, and is faster and uses less energy than the deterministic modes of vLLM and SGLang in every test we ran.**

Identical outputs usually cost speed: vLLM's batch-invariant mode and SGLang's deterministic mode both run slower than their normal modes. LuxiEdge gives identical outputs at about the speed of normal vLLM 0.25.1 and SGLang 0.5.19.

"Every batch size we tested" means batch 1, 16 and 64 when generating tokens, and batch 1 against 16, 4 and 2 on 2k, 8k and 32k-token prompts, across repeated runs and separate processes. The [drift demo](#drift-demo) below adds batch 16, 32 and 64 on 20 more prompts, and describes one open issue seen in a long-running process. Full scope: [`TEST_SCOPE.md`](TEST_SCOPE.md).

### Results

Each ratio compares Luxi with one engine on the same GPU, in alternating blocks with 6 runs per engine per test. The vLLM and SGLang comparisons come from two different H100 pods, and each row uses only the Luxi runs from its own pod.

| Luxi compared with | Long prompts (2k–32k tokens) | Token generation (1k-token prompt, 256 tokens, batch 1, 16 and 64) |
|---|---|---|
| **Normal modes (not batch-invariant)** | | |
| vLLM 0.25.1 | 3–7% faster, 2–5% less energy per token | Same speed (within 2%), 2–7% more energy per token |
| SGLang 0.5.19 (fp16) | 1.3–4.6% faster, 1.0–4.1% less energy per token | From 1.5% slower (batch 1) to 4.3% faster, 1–7% more energy per token |
| **Deterministic modes** | | |
| vLLM 0.25.1 batch-invariant mode | 6–10% faster, 5–7% less energy per token | 1.5–2.7× faster, 16–50% less energy per token |
| SGLang 0.5.19 deterministic, bf16 | 1.18–1.20× faster, 14–17% less energy per token | 1.21–1.26× faster, 10–13% less energy per token |
| SGLang 0.5.19 deterministic, fp16 | 1.21–1.24× faster, 17–19% less energy per token | 1.72–2.86× faster, 24–51% less energy per token |

The tradeoff: when generating tokens, Luxi uses 1–7% more energy per token than normal vLLM and SGLang. To stay batch-invariant it keeps one matrix-multiply setup per shape at every batch size, while normal vLLM switches to a lower-power setup at batch 1. Details: [Why generation uses more energy](BENCHMARKS.md#why-generation-uses-more-energy).

Normal vLLM and SGLang are not batch-invariant: a prompt's per-step top-5 logprobs changed on 249–254 of 256 steps when it ran inside a batch of 16 or 64 instead of alone.

Energy is GPU board energy (NVML). One model, one GPU type, single-GPU harness; see [`TEST_SCOPE.md`](TEST_SCOPE.md) for what was not tested.

### Details and raw files

- Full tables, method, determinism and correctness checks: [`BENCHMARKS.md`](BENCHMARKS.md)
- What was and wasn't tested: [`TEST_SCOPE.md`](TEST_SCOPE.md)
- vLLM evidence, with per-run rows: [`evidence/h100-qwen2-7b-vs-vllm-0.25.1-2026-09-25/`](evidence/h100-qwen2-7b-vs-vllm-0.25.1-2026-09-25/)
- SGLang evidence, with per-run rows: [`evidence/h100-qwen2-7b-vs-sglang-0.5.19-2026-09-26/`](evidence/h100-qwen2-7b-vs-sglang-0.5.19-2026-09-26/)

### Drift demo

**Under batch load and live-traffic arrivals, normal vLLM and SGLang changed the visible answer in about 8% of prompt-conditions (12 of 140 and 11 of 140). LuxiEdge's output stayed bit-identical in every condition it ran (80 of 80, with the long prompt taken from a fresh-process re-run; see the open issue below), as did the deterministic modes of vLLM and SGLang, while LuxiEdge runs at about the speed of normal vLLM and SGLang (see [Results](#results)).**

What was tested: on one H100 on 2026-09-26, each engine ran Qwen2-7B-Instruct in fp16 with greedy decoding on 20 fixed prompts, 256 tokens each. Every prompt was first run alone. Then the same prompt was run with other work on the GPU: inside a batch of 16 or 64, with two different sets of filler prompts and two row positions, and, for vLLM and SGLang, in a batch with mixed-length fillers and on a live server while other requests were arriving. Each output under load was compared with the same engine's output for that prompt alone: the tokens, the logprobs, and the visible text. When a prompt was simply re-run alone, every engine gave identical output (for LuxiEdge, apart from the long-running-process issue described below). Differences appeared only when other prompts shared the batch or arrived at the same time.

| Engine | Load conditions | Identical tokens | Identical logprobs | Visible answer changed |
|---|---|---|---|---|
| LuxiEdge (Luxi c8) | 80 (batch only) | 80/80 | 80/80 | 0/80 |
| vLLM 0.25.1 default | 140 | 128/140 (91.4%) | 0/140 | 12/140 (8.6%) |
| SGLang 0.5.19 default | 140 | 129/140 (92.1%) | 1/140 | 11/140 (7.9%) |
| vLLM 0.25.1 batch-invariant (`VLLM_BATCH_INVARIANT=1`) | 140 | 140/140 | 140/140 | 0/140 |
| SGLang 0.5.19 deterministic | 140 | 140/140 | 140/140 | 0/140 |

The 80 LuxiEdge conditions are 20 prompts × 4 batch conditions (batch 16 and 64, two filler sets each). On those same 80 conditions, normal vLLM changed the visible answer in 8 and normal SGLang in 6. Normal vLLM's changes were in prompts p02, p07, p08 and p17, with the first different token at position 3 to 47 (median 33). Normal SGLang's were in p02, p04 and p17, at position 40 to 162 (median 153).

Example (normal vLLM, prompt p08, a short reasoning question). Alone, the answer begins "…The second youngest is Erin." In row 0 of a batch of 16 it begins "…The second youngest person is Erin." The first different token is at position 3, and a few more words differ later in the answer. The same change happened in the batch-of-64 run and in one live-server run, but not when the prompt sat in row 8 of the other batch of 16. LuxiEdge's output for this prompt was identical in every condition. More examples: [`examples.md`](evidence/h100-qwen2-7b-drift-demo-2026-09-26/examples.md).

![Share of identical tokens and logprobs by engine and condition](evidence/h100-qwen2-7b-drift-demo-2026-09-26/drift_chart.svg)

Scope:
- LuxiEdge ran batch conditions only. The benchmarked engine has no server for requests arriving at different times, and its batch API takes equal-length rows, so there are no live-server or mixed-length rows for LuxiEdge.
- For the one long prompt (p20, 1,698 tokens), the "64" conditions used 32 rows for every engine, because of a LuxiEdge buffer limit.
- LuxiEdge logprobs were checked at 9 positions (0, 1, 3, 7, 15, 31, 63, 127 and 255), with the full 152,064-entry logits vector hashed at each one. vLLM and SGLang logprobs cover every position.
- LuxiEdge's drift runs used a 2,048-token decode context, above the 1,280 used in the speed benchmarks. Before the drift runs, LuxiEdge reproduced 11 logits hashes from the speed benchmarks bit for bit.
- One GPU, one model, one dtype, one run.

Open issue: the 19 shorter prompts ran in one long-running LuxiEdge process. When p20 ran next in that same process, after 1,043 earlier calls, its first run alone did not match the fresh-process run described next, its repeat-alone run produced the same tokens as that first run but different logits at all 9 checkpoints, and a later p20 call stopped with a CUDA illegal-memory-access error. All 6 p20 conditions were then re-run in a fresh process, and every token and logits hash matched. The p20 numbers above come from that fresh process and p01–p19 from the long-running process (run before p20), and the faulty records are published in [`raw/luxi_main_process_p20_fault.jsonl`](evidence/h100-qwen2-7b-drift-demo-2026-09-26/raw/luxi_main_process_p20_fault.jsonl). The fault is under investigation.

Method, per-prompt results, scripts and raw records: [`evidence/h100-qwen2-7b-drift-demo-2026-09-26/`](evidence/h100-qwen2-7b-drift-demo-2026-09-26/).

Contact: Eric Waller, eric@luxiedge.com
