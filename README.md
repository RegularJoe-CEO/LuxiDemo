# LuxiEdge

LuxiEdge is deterministic numerics: same bits on every chip, with receipts. A receipt is the SHA-256 of the bytes from the run that produced them. The proof sections are the measurements. Stock position gaps on the catalogs below are sub-millimeter, and some Luxi f64 sweeps do not share one hash.

## Try it in 30 seconds

[Art of Fugue](https://github.com/RegularJoe-CEO/Art-of-Fugue) runs 200,000 floating-point operations and hashes the exact results. Its README records platform libm hashes `b02237875da129b90b6355d5918b38657e4bb2e697e413124c661acd0899eb19` on macOS ARM64 and `c060c5031b1bdb7b730ad6de7639ca87fd3257009a3e3580b71b4f116158c9d6` on Linux x86_64 with an NVIDIA GPU. The LuxiEdge example in that README is `781ae6b19ca58f36bfaaedde6e605f6fc8a766701d69175cbbcd39fe20efd610` on both of those rows. Download `art_of_fugue.py` and run `python3 art_of_fugue.py`.

## LuxiQuant Risk State Engine

LuxiQuant Risk State Engine (Risk Runtime v0.4.0) is the paid institutional product of LuxiQuant, LuxiEdge's primary commercial division. It is proprietary and evaluation-only, with no public download: [request a confidential evaluation at luxiedge.com](https://luxiedge.com/luxiquant). LuxiBook is the free public receipt and determinism demonstration, under Proof.

## LuxiRisk v0.2 (legacy demo calculator)

The LuxiRisk v0.2 CLI is a separate, free legacy demo calculator, not the Risk State Engine. It computes a liquidation price, a position size from a risk percent, and the max dollar loss at a stop. Each result can carry an Ed25519 receipt (`lxr1_`).

Try it from the [luxirisk-v0.2 release](https://github.com/RegularJoe-CEO/LuxiDemo/releases/tag/luxirisk-v0.2) or from [`luxirisk/`](luxirisk/). The v0.2 binaries are not code-signed. Formulas and test vectors are in that folder.

```bash
chmod +x luxirisk-macos-arm64
shasum -a 256 -c luxirisk-macos-arm64.sha256
./luxirisk-macos-arm64 liq --side long --entry 65000 --leverage 10
```

## Proof

### Space and orbital

Luxi f64 SGP4 is byte-identical on macOS arm64, Linux arm64, qemu Linux x86_64, and an H100 for jax100 (`26eafb2b244fc1d4e24a7f561378774cf934f3976364fd7cc402853975d7175e`) and starlink-382a419 (`42d0ca2ca598dae1553ecfae12ca8204b3dae5ceb67e9afa76e36dcb40a75c09`). A 2026-10-11 H100 relaunch matched those hashes. Whole-process medians were 0.451879 s and 0.505075 s. The GPU image is near-earth, and the OrbVeil catalog was not run on it. See [`proofs/luxi-sgp4`](proofs/luxi-sgp4/) and [`proofs/citra-gpu-h100`](proofs/citra-gpu-h100/).

On the OrbVeil catalog, 15,940 objects and 1,441 epochs, Luxi hash `a14f625475a05892277a1a198aacc79aaf18b0192c97326bbe50fcafd31f8fa3` is the same on macOS arm64, native Linux arm64, and qemu x86_64. One-core speed is a tie: Luxi median 5.333366 s, python-sgp4 2.27 median 5.332472 s. Stock hashes differ by platform. The largest stock position gap is `4.22175690645485688e-08` km. `screen()` on ISS (NORAD 25544) stored 6,054 events, all at 0.0 km, from six secondaries whose element columns match Zarya. The machine is an M1 Pro. See [`proofs/orbveil-edge`](proofs/orbveil-edge/).

Citra keplemon `67d6497` on that H100 printed max position error 1.649 m on ISS at 24 h, and the parity test passed. The same printed 1.649 m was recorded on 2026-10-10 at commit `14114699`.

The Space Data Network proxy is the first 2,000 catalog objects. Luxi is one hash on three CPUs. Stock's largest cross-platform gap on that proxy is `4.22175690645485688e-08` km, and 1,999,000 pairs had 0 class flips at 10 cm, 75 m, 1 km, and 10 km. The TraCSS proxy is 53 public CARA CDMs. Luxi is one hash on three CPUs, and mode 64 stock versus Luxi on macOS has max absolute Pc `1.734723e-18`. That set is not the paper's 72,356 operational CDMs. Apple powermetrics CPU joules for these windows, labeled as estimates, are in [`proofs/job3-powermetrics`](proofs/job3-powermetrics/). The 2,000-object Luxi 1-thread median is 2.961125 J.

Stock jaxsgp4 commit `382a419`, same code on the H100 and on CPU, is not bit-identical. Float64 on 100 satellites and 29 times: 2701 of 2900 states differ, max position `1.5853778190904915e-09` km. Float32 on that grid: max position `2.4645502245160555` km. The published figure near 1e-9 km was jaxsgp4 versus C++ on CPU. See [`proofs/sgp4-gpu-cpu`](proofs/sgp4-gpu-cpu/).

SAAL v9.6 (US Space Force Sgp4Prop) black-box cross-platform comparison on the same catalog: see [`proofs/saal-compare/SAAL_COMPARISON.md`](proofs/saal-compare/SAAL_COMPARISON.md).

NCAR LAFT Kessler warm-rain microphysics (Linck, Dennis, Schreck, Stengel & Gagne 2026, Apache-2.0): each LLM JAX translation gives a different float64 hash on Intel Xeon, Apple M1 Pro and NVIDIA A100, while Luxi gives one hash on all three. On the same A100, Luxi median 0.074 ms vs Gemini JAX 0.83 ms at 50 columns and 82.52 ms vs 154.56 ms at 1M (1M verified GPU-only). See [`proofs/kessler-laft/KESSLER.md`](proofs/kessler-laft/KESSLER.md).

### GPU sums

Stock GPU float32 sums are repeatable but wrong: on 10,000,000 dollar values (exact total $499,894,306,221.96), cuBLAS, CuPy and TensorFlow on an NVIDIA L4 each returned the same answer every run, +$14,930.04 off. Luxi's fixed-order Kahan sum was exact ($0.00) with hash `b952a970e4ad4440` on the L4, x86 and Mac arm64. Luxi chunked GPU reduce was +$379.60 at 0.03 to 0.05 ms, with chunk totals combined in float64 on the host. Synthetic data, one GPU model. See [`proofs/gpu-sum`](proofs/gpu-sum/).

### Genomics

On 5,819,596,800 cells of public GIAB HG002 MHC reads, an unordered CPU reduction of PairHMM haplotype scores changed fingerprint with thread count. The canonical policy kept one fingerprint, `8296effda654807942e0f0121e8bdf32d4c6585b0ebe77c86151c19b7f66c150`, at every tested count. No throughput penalty was observed in the recorded runs. On an H200 the canonical GPU path finished that cell count in 80.3 ms at block size 128 and kept one fingerprint, `e678bd4eb00b96e7a6976f88055c21f511ff321703632017ad3d458fce011e94`, across block sizes 128, 256, 512, and 1024.

This is not a HaplotypeCaller replacement, not a demonstration of GATK-equivalent likelihoods, and not a claim of identical bits across CPU and GPU. It is not a genotype caller: no VCF is emitted, and production GATK per-read gap vectors were not used. It is not open source, and it is not the LuxiEdge engine. Paper: [10.5281/zenodo.22152053](https://doi.org/10.5281/zenodo.22152053). Repository: [Deterministic-PairHMM](https://github.com/RegularJoe-CEO/Deterministic-PairHMM). Upstream banding fix: [rust-bio PR 688](https://github.com/rust-bio/rust-bio/pull/688), merged 2026-09-16.

### Inference

On one H100, Qwen2-7B-Instruct fp16, 20 prompts and 256 greedy tokens, vLLM 0.25.1 default changed the visible answer in 12 of 140 conditions and SGLang 0.5.19 default changed it in 11 of 140. Luxi c8 matched tokens and logprobs in 80 of 80 batch conditions. The long prompt's Luxi rows are from a fresh process. Evidence: [`evidence/h100-qwen2-7b-drift-demo-2026-09-26`](evidence/h100-qwen2-7b-drift-demo-2026-09-26/).

Prefill versus vLLM 0.25.1 at 32767 tokens by batch 1: Luxi FA3 32,190 tok/s and 0.02162 J/token, vLLM default 29,980 tok/s and 0.02269 J/token. Evidence: [`evidence/h100-qwen2-7b-vs-vllm-0.25.1-2026-09-25`](evidence/h100-qwen2-7b-vs-vllm-0.25.1-2026-09-25/).

Prefill versus SGLang 0.5.19 on a separate H100, same shape: Luxi c8 median `3.178e+04` tok/s and `0.02201` J/token, SGLang deterministic fp16 `2.622e+04` tok/s and `0.02652` J/token. Evidence: [`evidence/h100-qwen2-7b-vs-sglang-0.5.19-2026-09-26`](evidence/h100-qwen2-7b-vs-sglang-0.5.19-2026-09-26/).

Compute efficiency is the frozen H100 and H200 set in [attention-v2-benchmarks](https://github.com/RegularJoe-CEO/attention-v2-benchmarks), checked by [`run_bench.sh`](https://github.com/RegularJoe-CEO/attention-v2-benchmarks/blob/main/run_bench.sh). On an H200, morph versus Flash at 16k `rag_tokenized` is 9.84 times faster and uses 16.78 times fewer joules per pass. The production-layer morph auto path on that fixture is 2.04 times faster and uses 2.06 times fewer joules. Those ratios are scoped to compressible KV. On the short-sequence 12-layer H100 same-shape comparison, PyTorch Flash fp16 wins by `19.169052480042783` times on joules per token. Records: [H200 morph](https://github.com/RegularJoe-CEO/attention-v2-benchmarks/blob/main/frozen/h200_morph_rag_tokenized_20260620.json) and [H100 12-layer](https://github.com/RegularJoe-CEO/attention-v2-benchmarks/blob/main/frozen/h100_stack12_h2h_20260711.json). The same H100 ratio is already in [`evidence/h100-stack12-H2H`](evidence/h100-stack12-H2H/).

### Luxi Book

The public 10-row `example_book.csv` hash is `4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a` on the measured CPU and GPU boxes. That book is not the 1,000,000-option book. After the erfc fix, the million-option book hashed to `821adc80126a76dc3c244344a109fefd8cb1e3ebbfe56ee0458048b71280d5d9` on macOS arm64, Linux arm64, and qemu x86_64, with book total minus reference `-0.0005505851893818692`. A later row-accuracy pass on those three CPUs hashed to `3aa738c3fd9369d2c262a547dd0ba342c6e2386ab2dfcb0086256bc7d2a3ff0f`, and the max absolute row error was `3.8385300817345264e-10`. An AWS sample of 1,000,000 European options, C++ on x86 versus Graviton ARM, had 45,867 prices differ. The million-option logs are not a public binary here. Run the 10-row book from [`downloads/luxibook/`](downloads/luxibook/).

## Full table

Every headline above is a row in [`RESULTS.md`](RESULTS.md). The chronology that used to open this page is in [`HISTORY.md`](HISTORY.md).

## Contact

Eric Waller, e@ewaller.com

Copyright 2026 Eric Waller. Public binaries are evaluation builds.
