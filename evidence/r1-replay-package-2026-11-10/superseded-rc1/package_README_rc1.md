# Luxi replay demo 1.0.0 (Linux x86_64)

A closed-binary build of the Luxi exact inference engine (Attention Transformer v2, engine v27 final, frozen on
November 9, 2026) running DeepSeek-R1-Distill-Qwen-7B in the published RedHatAI INT8 (W8A8) quantization.

With it you can:

1. **inspect** the build identity, model checksums, the selected device and the fixtures;
2. run the **short frozen replay** (a 63-token MATH-500 prompt, one sample, capped at 32 generated tokens) on an NVIDIA
   GPU or on the CPU, and see whether it reproduces the November 9 H100 NVL result;
3. **verify** a signed record offline against a public key you choose to trust (no GPU, no model);
4. **compare** two records token by token and step by step, wherever they were made;
5. run the **full GPU suite**: 17 problems x 8 samples = 136 sampled answers, up to 16,384 tokens each, and compare
   them with the 17 signed November 9 H100 NVL records shipped in `reference/`.

What it is not: a chat app or API server, or a speed benchmark. On November 9, vLLM 0.32 (normal mode) was faster and
cheaper per token than Luxi on every GPU where its int8 configuration ran. Luxi's claim is narrower: the same token ids
and per-step logit fingerprints on different supported machines.

## Requirements

- Linux x86_64 with glibc 2.34 or newer (Ubuntu 22.04+, Debian 12+, RHEL/Rocky/Alma 9+, Amazon Linux 2023).
  macOS, Windows, ARM and musl/Alpine are not supported.
- Internet access for the install (Python packages; the GPU PyTorch build is about 3 GB) and for one 8.7 GB model
  download from Hugging Face (no account needed). Disk: about 13 GB for GPU, 10 GB for CPU.
- No compiler, no private repository and no credentials. `install.sh` gets CPython 3.12 through uv (or use your own
  CPython 3.12 with `LUXI_PYTHON=/path/to/python3.12 ./install.sh ...`).
- **CPU replay:** any x86_64 CPU; 32 GB RAM recommended.
- **GPU:** an NVIDIA GPU with compute capability 8.0, 8.6, 8.9, 9.0 or 12.0 and driver 570 or newer.

| Compute capability | Examples | Short replay | Full suite |
|---|---|---|---|
| 8.0 | A100 | yes | yes (40/80 GB) |
| 8.6 | RTX 3090, RTX A6000, A40 | yes | yes (24 GB or more) |
| 8.9 | RTX 4090, L40S, L4 (24 GB) | yes | yes (24 GB or more) |
| 9.0 | H100, H200 | yes | yes |
| 12.0 | RTX 5090, RTX PRO 6000 Blackwell | yes | yes |

  Minimum GPU memory: 16 GB for the short replay, 22 GB for the full suite. Other GPUs (for example B200, compute
  capability 10.0, or anything before 8.0) are refused with exit code 3. **A GPU request never falls back to the CPU.**
  Which exact GPUs and CPUs this build was tested on is published with the release, separately from the
  November 9 results (see "Identity" below).

## Install (copy and paste)

```bash
tar xzf luxi-replay-demo-1.0.0-linux-x86_64.tar.gz
cd luxi-replay-demo-1.0.0-linux-x86_64
sha256sum -c SHA256SUMS          # every shipped file
./install.sh --gpu               # or: ./install.sh --cpu
./luxi fetch-model               # 8.7 GB once, to ~/.cache/luxi/models/ ; every file is checked by sha256
```

Use `--model DIR` (or `LUXI_MODEL_DIR=DIR`) on any command to keep the model elsewhere.

## The five things to run

```bash
# 1. inspect: build identity, model checksums, device, fixtures
./luxi inspect --device gpu --check-weights          # or --device cpu

# 2. short frozen replay (32 tokens), GPU or CPU
./luxi demo --device gpu
./luxi demo --device cpu
#    prints the 32 generated token ids and their text, the step Merkle root, and timing with
#    weights checksum, model load and execution reported separately. Expected last lines:
#    RESULT: SAME BYTES as the Nov 9 H100 NVL reference ...

# 3. verify offline (no GPU, no model). Your own records: use the public key the demo printed.
./luxi verify luxi_out/bundle_tiny-math500-090_gpu-*.json --trusted-key <public key printed by the demo>
#    The November 9 records, against the benchmark public key:
./luxi verify reference/nov09_h100_nvl_signed/full_suite/bundle_a4-math500-090.json \
     --trusted-key reference/keys/luxi_benchmark_key_2026-11.pub
#    The wrong key is rejected (exit 1):
./luxi verify reference/nov09_h100_nvl_signed/full_suite/bundle_a4-math500-090.json \
     --trusted-key 0000000000000000000000000000000000000000000000000000000000000001
#    Changed token, changed fingerprint, changed seed, re-signed forgery, wrong key: all rejected:
./luxi tamper-test reference/nov09_h100_nvl_signed/full_suite/bundle_a4-math500-090.json \
     --trusted-key reference/keys/luxi_benchmark_key_2026-11.pub

# 4. compare two runs (for example GPU vs CPU, or your machine vs a colleague's). Exit 0 only if every
#    token id and every per-step fingerprint is identical; otherwise prints the first mismatch and exits 1.
./luxi compare luxi_out/bundle_tiny-math500-090_gpu-*.json luxi_out/bundle_tiny-math500-090_cpu-*.json

# 5. full GPU suite: 136 answers, compared with the signed November 9 H100 NVL records
./luxi full-suite --device gpu
#    RESULT: 136/136 answers and 546,114 tokens identical to the signed Nov 9 H100 NVL records
```

Both records are always kept: `compare` never edits or deletes either file.

## What the short replay is

The fixture is the first sample of the MATH-500 problem `test/prealgebra/1761.json` from the November 2026 R1
evaluation (job `a4-math500-090`), capped at 32 generated tokens: temperature 0.6, top_p 95%, top_k off, seed 2026,
request id `0f8e95dc95735a58`. It is built exactly as the November 9 GPU swap test built its short replay. On November 9 the
H100 NVL produced step Merkle root `158ee5ae768cb633...`; only those first 16 hex digits survive in the run log, and
the demo checks them. The same short replay matched on the host CPUs of the nine November 9 GPU machines.

## Identity

- `engine_source_sha256` `795c71736e22e2f66935e4acc91b8040120d40528b04d84ea9daedd5a5692f36` identifies the frozen engine
  source the binaries were compiled from (the source is not shipped). Every November 9 record carries the same value.
- `build_sha256` identifies the shipped binaries. `./luxi inspect` prints it and every record this package makes signs
  it, together with the label `luxi-replay-demo-1.0.0 gpu-gemm=cublas`.
- This build is a new build identity. Its GPU integer matrix products run through cuBLAS instead of the Triton kernels
  used on November 9. Both compute exact int32 sums. The model, quantization, sampler, CUDA kernels and reference
  outputs are unchanged. This build's cross-machine results are published separately; it does not inherit the
  November 9 nine-GPU result.
- Comparing a record from this package with a November 9 record shows `outputs_identical: true` with
  `engine_or_build_field_diffs` listing `build` and `build_sha256`. That shows which build made each record.

## Keys and trust

- `reference/keys/luxi_benchmark_key_2026-11.pub` is the public key that signed the November 2026 benchmark records:
  `42bb6bccd2bc7d0570b44d5e068971707a704a26e8cb0294b9df12f23fdfe418`. Check that it matches the copy published in the
  LuxiDemo repository evidence folder before you trust it.
- This package does **not** contain the benchmark signing private key. On first use, each installation creates its own
  key at `.luxi_state/demo_signing_key.hex` (mode 0600, never shipped) and signs its own records with it.
- A signature shows that the holder of a key signed a record. It does not show which machine ran the job, or that an
  answer is correct. Reproduction is what `compare` checks: identical token ids and per-step logit fingerprints from an
  independent run.

## Exit codes

`0` pass; `1` outputs differ or a record is rejected; `2` usage or input error; `3` the requested device cannot run
(no NVIDIA driver, unsupported GPU, too little memory, CUDA failure; never a silent CPU fallback); `4` model files missing
or checksum mismatch.

## Model files

`./luxi fetch-model` downloads `RedHatAI/DeepSeek-R1-Distill-Qwen-7B-quantized.w8a8` at revision
`925259de674c80799e9455bfcd2bf50f999022cb` (MIT) and checks every file. Every command refuses other files.

| File | Bytes | sha256 |
|---|---|---|
| model-00001-of-00002.safetensors | 4,985,986,104 | 2fff4b44f0f6480741177898ad875d8cc2d3e61f1f324ed5bc4ff93a6e31e571 |
| model-00002-of-00002.safetensors | 3,722,801,480 | e615cb42da0232d0dc4782addcb9eae60d4cf1c0f06238a94e7d9a40b0c2ceab |
| config.json | 1,831 | 8bed7317068d2d914e477a6c8a37324f9f9231d6b05e88b44f2c436c05867177 |
| tokenizer.json | 7,031,660 | 88145e3c3249adc2546ede277e9819d6e405e19072456e4b521cbc724bd60773 |

These are the publisher's original files, the same bytes recorded in the November 9 signed model manifest. The
quantization is the publisher's: int8 weights, symmetric per output channel; int8 activations, symmetric per token,
dynamic; lm_head not quantized. This package neither creates nor modifies a quantization.

Manual download instead of `fetch-model`:
`hf download RedHatAI/DeepSeek-R1-Distill-Qwen-7B-quantized.w8a8 --revision 925259de674c80799e9455bfcd2bf50f999022cb --local-dir DIR`,
then `./luxi inspect --model DIR --check-weights`.

## Limits

- Internal benchmark evidence from one cloud provider, fixed configurations and single runs. First-run timings include
  one-time setup. Energy figures elsewhere are NVML GPU-board energy, not facility electricity.
- The CPU path covers the short replay only. No CPU full-suite result is claimed.
- Identical outputs say nothing about answer accuracy; they show the same computation was reproduced.

## License and private evaluation

Copyright 2026 Eric Waller. Public binaries are evaluation builds; see `LICENSE.txt` and `THIRD_PARTY_NOTICES.md`.
To evaluate the engine on your own workload, request a private evaluation: https://luxiedge.com/pilot
