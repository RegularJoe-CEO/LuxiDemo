# LuxiEdge

LuxiEdge is deterministic numerics with receipts. Every result here is scoped to a named workload, environment, comparator, and evidence artifact; there is no universal determinism or accuracy guarantee. Output hashes identify computation bytes. Signed receipts bind a payload to a key. Reference checks test numerical agreement. None is a substitute for the others. The proof sections are the measurements. Stock position gaps on the catalogs below are sub-millimeter, and some Luxi f64 sweeps do not share one hash.

## Try it in 30 seconds

[Art of Fugue](https://github.com/RegularJoe-CEO/Art-of-Fugue) runs 200,000 floating-point operations and hashes the exact results. Its README records platform libm hashes `b02237875da129b90b6355d5918b38657e4bb2e697e413124c661acd0899eb19` on macOS ARM64 and `c060c5031b1bdb7b730ad6de7639ca87fd3257009a3e3580b71b4f116158c9d6` on Linux x86_64 with an NVIDIA GPU. The LuxiEdge example in that README is `781ae6b19ca58f36bfaaedde6e605f6fc8a766701d69175cbbcd39fe20efd610` on both of those rows. Download `art_of_fugue.py` and run `python3 art_of_fugue.py`.

## What is public here

LuxiEdge is one core engine (patent allowed, grant pending) with seven commercial products: Inference, Haplotastic, LuxiQuant Risk State Engine, Orbital, Catastrophe, Edge and LuxiEnergy ([luxiedge.com](https://luxiedge.com)). They are not equally validated, and most have no public binary. This repository holds only what the public can download, run or inspect:

| Product | Public in this repository | Not public / how to get more |
|---|---|---|
| Inference | [Luxi replay demo 1.0.1](downloads/luxi-replay-demo/): closed-binary package for Linux x86_64 (NVIDIA GPU or CPU) that replays a frozen DeepSeek-R1-Distill-Qwen-7B job, verifies signed records offline and compares records across machines. Evidence: [November 9 GPU swap test](evidence/r1-distill-7b-gpu-swap-2026-11-09/) with its per-GPU signed records, and the November 10 package tests ([1.0.1](evidence/r1-replay-package-1.0.1-2026-11-10/), [1.0.0](evidence/r1-replay-package-2026-11-10/)) | Engine source is private. Your own model or workload: [private GPU-swap evaluation](https://luxiedge.com/contact#gpu-swap-evaluation) |
| Haplotastic (GPU germline variant caller) | Nothing. The Dorado and PairHMM studies below are separate research, not Haplotastic benchmarks | [luxiedge.com/haplotastic](https://luxiedge.com/haplotastic) |
| LuxiQuant Risk State Engine (Risk Runtime v0.4.2) | Nothing. No public runtime download or public prices. LuxiBook below is a separate free pricer, not the Risk State Engine | [Confidential evaluation](https://luxiedge.com/pilot) |
| Orbital, Catastrophe, Edge | Proof studies only (SGP4, Oasis, GPU sums, Art of Fugue below); no product binary | [luxiedge.com/contact](https://luxiedge.com/contact) |
| LuxiEnergy | Nothing beyond historical research notes | [luxiedge.com/energy](https://luxiedge.com/energy) |
| LuxiBook v0.2.1 (free demo, includes LuxiSize) | Closed binaries, example book, signed receipts | - |

The Risk State Engine maintains deterministic, authenticated risk state as a scenario-by-position matrix changes. The proofs below do not validate it.

## Inference replay demo (public download)

[`downloads/luxi-replay-demo/`](downloads/luxi-replay-demo/) has a closed-binary build of the Luxi inference engine (Attention Transformer v2, engine v27 final, `795c71736e22e2f6`) with DeepSeek-R1-Distill-Qwen-7B w8a8. Install it on Linux x86_64, download the model from Hugging Face, and run a 32-token frozen replay on an NVIDIA GPU or a CPU that is checked against the signed November 9 H100 NVL record, offline verification and tamper tests, a record-to-record compare, and on a GPU the full 136-answer suite against the signed November 9 H100 NVL records. Current release 1.0.1, package sha256 `c2114aa7bd6c47822bdf94de2f7d77f1aa773a32dcc3268072cfb979c2e5c8b3`; release 1.0.0 (`b1f6cf0b…39c9`, the same engine files) stays available. Evaluation license, no open-source license on the engine.

November 10, 2026 package tests (internal measurement), each from a clean install of the tarball with no private source or credentials. **Release 1.0.1** on a RunPod RTX A6000 pod and on a separate x86_64 sandbox VM: the 32-token replay produced the same token ids and per-step logit fingerprints as the signed November 9 H100 NVL short record (recovered on November 10 and shipped in the package; step root `158ee5ae768cb633…`) on the RTX A6000 twice, its host CPU (Intel Xeon Gold 6342) once and the sandbox CPU twice. The full suite on the RTX A6000 matched all 136 answers and 546,114 tokens of the 17 signed November 9 H100 NVL records (662.7 s at 32 slots, 824.1 output tokens/s, 0.362 GPU-board joules per token). **Release 1.0.0**, which ships the same engine files, on a RunPod RTX 5090 pod and a RunPod A40 pod (GPU twice, host CPU once on each: AMD EPYC 7543, Intel Xeon Gold 6342) and the sandbox CPU (twice): identical 32-token records, which also compare identical to the 1.0.1 records; full suite 136/136 answers and 546,114 tokens on the RTX 5090 (301.6 s, 1,810.5 tokens/s, 0.230 J per token) and the A40 (799.0 s, 683.5 tokens/s, 0.424 J per token), and the two GPUs matched each other on all 17 jobs. On every machine each record verified offline against that installation's public key and was rejected under a wrong key, and the tamper test rejected every tested edit (changed token, per-step fingerprint, prompt token, seed or temperature; a dropped token; a re-signed forgery). Full-suite timings are single runs; NVML GPU-board energy is not wall power. Speed is not the claim: on November 9, vLLM 0.32 normal was faster and cheaper per token wherever its int8 configuration ran. Raw outputs: [`evidence/r1-replay-package-1.0.1-2026-11-10/`](evidence/r1-replay-package-1.0.1-2026-11-10/) (1.0.1) and [`evidence/r1-replay-package-2026-11-10/`](evidence/r1-replay-package-2026-11-10/) (1.0.0).

November 9, 2026 GPU swap test (frozen v27 build, internal measurement, earlier build identity than the package above): nine GPU types (H100 NVL, H200, A100 SXM 80GB, RTX PRO 6000 Blackwell Server Edition, RTX A6000, A40, RTX 5090, RTX 4090 and RTX 3090) each matched 136/136 sampled answers and 546,114 generated tokens, with matching recorded per-step fingerprints against the signed H100 reference, plus a separate 92/92 Qwen gate. This is 17 problems with eight samples each, not 136 distinct questions. Seven CPU models (AMD EPYC 9654, 7742, 9355, 7352 and 7452; Intel Xeon Platinum 8568Y+ and Xeon Gold 6342) each matched 2/2 small Qwen checks and the short signed R1 replay capped at 32 tokens; these CPUs did not run the full GPU workload in this evidence. A40 soak: 1,502 calls, 1,239 checks, zero mismatches. vLLM 0.32 normal was faster and cheaper per token wherever the tested int8 configuration ran; its sequence matches are against its own H100 reference, not Luxi, and different sequences do not establish accuracy errors. The rental-choice comparison is Luxi versus Luxi: RTX 5090 delivered about 80% of Luxi H100 throughput at about one-third of its calculated rental cost per output token. November rental rates exclude licensing and other operating costs; they are not API prices or current rental offers. Internal measurements, one cloud, fixed tested configurations, single performance runs; initial 32-slot timings include one-time compilation; NVML GPU-board energy is not facility electricity. The per-GPU signed records (282 records: each GPU's 136 answers, the second runs that were kept, one-job checks and the host-CPU 32-token replays, plus the H100 NVL 32-token record) were recovered on November 10 and added unchanged in [`records/`](evidence/r1-distill-7b-gpu-swap-2026-11-09/records/); every one verifies with the benchmark key, and each GPU's 136 answers re-compare identical to the H100 NVL reference with the folder's offline verifier. Evidence, including what was not retained: [`evidence/r1-distill-7b-gpu-swap-2026-11-09/`](evidence/r1-distill-7b-gpu-swap-2026-11-09/).

## LuxiBook free download (includes LuxiSize)

LuxiBook is the free download: the option-book pricer described under [LuxiBook (free demo)](#luxibook-free-demo) plus LuxiSize, the sizing calculator. Neither is the Risk State Engine. LuxiSize is a small offline CLI. It computes a liquidation price, a position size from a risk percent, and the max dollar loss at a stop. Each result can carry an Ed25519 receipt (`lxr1_`).

Get the option-book pricer from [`downloads/luxibook/`](downloads/luxibook/). Get LuxiSize from the [luxirisk-v0.2 release](https://github.com/RegularJoe-CEO/LuxiDemo/releases/tag/luxirisk-v0.2) or from [`luxirisk/`](luxirisk/). The v0.2 binaries are not code-signed. Formulas and test vectors are in that folder.

```bash
chmod +x luxirisk-macos-arm64
shasum -a 256 -c luxirisk-macos-arm64.sha256
./luxirisk-macos-arm64 liq --side long --entry 65000 --leverage 10
```

## Proof

### Financial / catastrophe risk

Oasis PiWind (oasislmf 2.5.8): stock defaults produced 10 distinct hashes from 10 runs on 2 machines; Luxi produced 1 hash. On a synthetic extreme-TIV stress case, stock differed from an exact correctly rounded reference on 5,851 of 6,006 ground-up totals; Luxi differed on none. No speed or energy claim. See [`proofs/oasis-drift/OASIS_DRIFT.md`](proofs/oasis-drift/OASIS_DRIFT.md).

### Genomics

These are research studies on specific workloads. They are not a Haplotastic variant-calling benchmark, and there is no public Haplotastic package in this repository.

Oxford Nanopore Dorado 2.1.2 (A100, models v5.0.0, 40 reads): stock Dorado output md5 differed on all 12 runs; Luxi output was identical on every run and matches a CPU reference. Fast model GPU kernel time incl. decode (CUPTI, mean of 3): Luxi 246.1 vs Dorado 295.7 ms (b864), 323.7 vs 351.4 ms (b512). Hac: exact, but slower on GPU kernel time (943.5 vs 838.4 ms). See [`proofs/dorado-drift/DORADO_DRIFT.md`](proofs/dorado-drift/DORADO_DRIFT.md).

PairHMM score stability (GIAB HG002): see the [Deterministic-PairHMM paper](https://github.com/RegularJoe-CEO/Deterministic-PairHMM).

### Space and orbital

Luxi f64 SGP4 is byte-identical on macOS arm64, Linux arm64, qemu Linux x86_64, and an H100 for jax100 (`26eafb2b244fc1d4e24a7f561378774cf934f3976364fd7cc402853975d7175e`) and starlink-382a419 (`42d0ca2ca598dae1553ecfae12ca8204b3dae5ceb67e9afa76e36dcb40a75c09`). A 2026-10-11 H100 relaunch matched those hashes. Whole-process medians were 0.451879 s and 0.505075 s. The GPU image is near-earth, and the OrbVeil catalog was not run on it. See [`proofs/luxi-sgp4`](proofs/luxi-sgp4/) and [`proofs/citra-gpu-h100`](proofs/citra-gpu-h100/).

### GPU sums

Stock GPU float32 sums are repeatable but wrong: on 10,000,000 dollar values (exact total $499,894,306,221.96), cuBLAS, CuPy and TensorFlow on an NVIDIA L4 each returned the same answer every run, +$14,930.04 off. Luxi's fixed-order Kahan sum was exact ($0.00) with hash `b952a970e4ad4440` on the L4, x86 and Mac arm64. Luxi chunked GPU reduce was +$379.60 at 0.03 to 0.05 ms, with chunk totals combined in float64 on the host. Synthetic data, one GPU model. See [`proofs/gpu-sum`](proofs/gpu-sum/).

### Inference (historical, September 2026)

These September results concern Qwen2-7B-Instruct and earlier engine builds. They do not transfer to the November v27 build or the replay demo above.

On one H100, Qwen2-7B-Instruct fp16, 20 prompts and 256 greedy tokens, vLLM 0.25.1 default changed the visible answer in 12 of 140 conditions and SGLang 0.5.19 default changed it in 11 of 140. Luxi c8 matched tokens and logprobs in 80 of 80 batch conditions. The long prompt's Luxi rows are from a fresh process. Evidence: [`evidence/h100-qwen2-7b-drift-demo-2026-09-26`](evidence/h100-qwen2-7b-drift-demo-2026-09-26/).

Generation versus the named comparators (Qwen2-7B-Instruct, H100 SXM, 1,024-token prompt, 256 greedy tokens): against vLLM 0.25.1 batch-invariant mode, Luxi was 1.5 to 2.7 times faster with 16 to 50% less GPU energy per token; against vLLM 0.25.1 default, the same speed with 2 to 7% more GPU energy per token; against SGLang 0.5.19 deterministic fp16/bf16, 1.21 to 2.86 times faster; against SGLang normal fp16, about the same speed with 1 to 7% more GPU energy per token. Energy is NVML GPU-board energy, not facility energy. Offline fixed batches, not production serving, and not third-party validated.

Prefill versus vLLM 0.25.1 at 32767 tokens by batch 1: Luxi FA3 32,190 tok/s and 0.02162 J/token, vLLM default 29,980 tok/s and 0.02269 J/token. Evidence: [`evidence/h100-qwen2-7b-vs-vllm-0.25.1-2026-09-25`](evidence/h100-qwen2-7b-vs-vllm-0.25.1-2026-09-25/).

Prefill versus SGLang 0.5.19 on a separate H100, same shape: Luxi c8 median `3.178e+04` tok/s and `0.02201` J/token, SGLang deterministic fp16 `2.622e+04` tok/s and `0.02652` J/token. Evidence: [`evidence/h100-qwen2-7b-vs-sglang-0.5.19-2026-09-26`](evidence/h100-qwen2-7b-vs-sglang-0.5.19-2026-09-26/).

Versus NVIDIA's Nemotron-H-8B Mamba-2 hybrid on vLLM 0.25.1 (one H100 SXM, 8 WikiText prompts, greedy, batch 1, 16 and 64): Luxi (Qwen2-7B-Instruct fp16) gave identical output at every batch size on 8 of 8 prompts; the Mamba-2 hybrid did on 2 of 8 (3 of 8 with a cuTile prefill). At batch 64 end to end, Luxi was 1.36x faster with 23.7% less GPU energy per token. Different models (7B Instruct vs 8B Base, fp16 vs bf16). See [`proofs/mamba2-comparison/RESULTS.md`](proofs/mamba2-comparison/RESULTS.md).

### LuxiBook (free demo)

The public 10-row `example_book.csv` hash is `4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a` on the measured CPU and GPU boxes. That book is not the 1,000,000-option book. After the erfc fix, the million-option book hashed to `821adc80126a76dc3c244344a109fefd8cb1e3ebbfe56ee0458048b71280d5d9` on macOS arm64, Linux arm64, and qemu x86_64, with book total minus reference `-0.0005505851893818692`. A later row-accuracy pass on those three CPUs hashed to `3aa738c3fd9369d2c262a547dd0ba342c6e2386ab2dfcb0086256bc7d2a3ff0f`, and the max absolute row error was `3.8385300817345264e-10`. An AWS sample of 1,000,000 European options, C++ on x86 versus Graviton ARM, had 45,867 prices differ. The million-option logs are not a public binary here. Run the 10-row book from [`downloads/luxibook/`](downloads/luxibook/).

## Additional evidence

These studies show research breadth. They are not additional commercial products and do not validate the current Risk State Engine.

### More orbital studies

On the OrbVeil catalog, 15,940 objects and 1,441 epochs, Luxi hash `a14f625475a05892277a1a198aacc79aaf18b0192c97326bbe50fcafd31f8fa3` is the same on macOS arm64, native Linux arm64, and qemu x86_64. One-core speed is a tie: Luxi median 5.333366 s, python-sgp4 2.27 median 5.332472 s. Stock hashes differ by platform. The largest stock position gap is `4.22175690645485688e-08` km. `screen()` on ISS (NORAD 25544) stored 6,054 events, all at 0.0 km, from six secondaries whose element columns match Zarya. The machine is an M1 Pro. See [`proofs/orbveil-edge`](proofs/orbveil-edge/).

Citra keplemon `67d6497` on that H100 printed max position error 1.649 m on ISS at 24 h, and the parity test passed. The same printed 1.649 m was recorded on 2026-10-10 at commit `14114699`.

The Space Data Network proxy is the first 2,000 catalog objects. Luxi is one hash on three CPUs. Stock's largest cross-platform gap on that proxy is `4.22175690645485688e-08` km, and 1,999,000 pairs had 0 class flips at 10 cm, 75 m, 1 km, and 10 km. The TraCSS proxy is 53 public CARA CDMs. Luxi is one hash on three CPUs, and mode 64 stock versus Luxi on macOS has max absolute Pc `1.734723e-18`. That set is not the paper's 72,356 operational CDMs. Apple powermetrics CPU joules for these windows, labeled as estimates, are in [`proofs/job3-powermetrics`](proofs/job3-powermetrics/). The 2,000-object Luxi 1-thread median is 2.961125 J.

Stock jaxsgp4 commit `382a419`, same code on the H100 and on CPU, is not bit-identical. Float64 on 100 satellites and 29 times: 2701 of 2900 states differ, max position `1.5853778190904915e-09` km. Float32 on that grid: max position `2.4645502245160555` km. The published figure near 1e-9 km was jaxsgp4 versus C++ on CPU. See [`proofs/sgp4-gpu-cpu`](proofs/sgp4-gpu-cpu/).

SAAL v9.6 (US Space Force Sgp4Prop) black-box cross-platform comparison on the same catalog: see [`proofs/saal-compare/SAAL_COMPARISON.md`](proofs/saal-compare/SAAL_COMPARISON.md).

### Atmospheric science

NCAR LAFT Kessler warm-rain microphysics (Linck, Dennis, Schreck, Stengel & Gagne 2026, Apache-2.0): each LLM JAX translation gives a different float64 hash on Intel Xeon, Apple M1 Pro and NVIDIA A100, while Luxi gives one hash on all three. On the same A100, Luxi median 0.074 ms vs Gemini JAX 0.83 ms at 50 columns and 82.52 ms vs 154.56 ms at 1M (1M verified GPU-only). See [`proofs/kessler-laft/KESSLER.md`](proofs/kessler-laft/KESSLER.md).

### Genomics (separate research, not the LuxiEdge engine)

On 5,819,596,800 cells of public GIAB HG002 MHC reads, an unordered CPU reduction of PairHMM haplotype scores changed fingerprint with thread count. The canonical policy kept one fingerprint, `8296effda654807942e0f0121e8bdf32d4c6585b0ebe77c86151c19b7f66c150`, at every tested count. No throughput penalty was observed in the recorded runs. On an H200 the canonical GPU path finished that cell count in 80.3 ms at block size 128 and kept one fingerprint, `e678bd4eb00b96e7a6976f88055c21f511ff321703632017ad3d458fce011e94`, across block sizes 128, 256, 512, and 1024.

This is not a HaplotypeCaller replacement, not a demonstration of GATK-equivalent likelihoods, and not a claim of identical bits across CPU and GPU. It is not a genotype caller: no VCF is emitted, and production GATK per-read gap vectors were not used. It is not open source, and it is not the LuxiEdge engine. Paper: [10.5281/zenodo.22152053](https://doi.org/10.5281/zenodo.22152053). Repository: [Deterministic-PairHMM](https://github.com/RegularJoe-CEO/Deterministic-PairHMM). Upstream banding fix: [rust-bio PR 688](https://github.com/rust-bio/rust-bio/pull/688), merged 2026-09-16.

### Memory movement (historical research)

Compute efficiency is the frozen H100 and H200 set in [attention-v2-benchmarks](https://github.com/RegularJoe-CEO/attention-v2-benchmarks), checked by [`run_bench.sh`](https://github.com/RegularJoe-CEO/attention-v2-benchmarks/blob/main/run_bench.sh). On an H200, morph versus Flash at 16k `rag_tokenized` is 9.84 times faster and uses 16.78 times fewer joules per pass. The production-layer morph auto path on that fixture is 2.04 times faster and uses 2.06 times fewer joules. Those ratios are scoped to compressible KV. On the short-sequence 12-layer H100 same-shape comparison, PyTorch Flash fp16 wins by `19.169052480042783` times on joules per token. Records: [H200 morph](https://github.com/RegularJoe-CEO/attention-v2-benchmarks/blob/main/frozen/h200_morph_rag_tokenized_20260620.json) and [H100 12-layer](https://github.com/RegularJoe-CEO/attention-v2-benchmarks/blob/main/frozen/h100_stack12_h2h_20260711.json). The same H100 ratio is already in [`evidence/h100-stack12-H2H`](evidence/h100-stack12-H2H/).

## Full table

Every headline above is a row in [`RESULTS.md`](RESULTS.md). The chronology that used to open this page is in [`HISTORY.md`](HISTORY.md).

## Contact

Eric Waller, eric@luxiedge.com

Copyright 2026 Eric Waller. Public binaries are evaluation builds.
