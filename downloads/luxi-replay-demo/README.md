# Luxi replay demo 1.0.0 (Linux x86_64): public download

A closed-binary build of the Luxi exact inference engine (Attention Transformer v2, engine v27 final, frozen on
November 9, 2026) running DeepSeek-R1-Distill-Qwen-7B in the publisher's INT8 (W8A8) quantization. You can run a short
frozen replay on an NVIDIA GPU or on a CPU, check the signed record offline, compare records made on different
machines, and (on a GPU) rerun the full 136-answer suite against the signed November 9 H100 NVL records.

It is a reproducibility demo, not a chat app, an API server or a speed benchmark. The engine source is private and is
not in this package. To test the engine on your own workload, use the private evaluation route below.

## Download

| File | Bytes | sha256 |
|---|---|---|
| [`luxi-replay-demo-1.0.0-linux-x86_64.tar.gz`](luxi-replay-demo-1.0.0-linux-x86_64.tar.gz) | 25,473,516 | `b1f6cf0b024f0270223807662ebc15d72191db4838c408bf6727400c5ec539c9` |
| [`luxi-replay-demo-1.0.0-linux-x86_64.tar.gz.sha256`](luxi-replay-demo-1.0.0-linux-x86_64.tar.gz.sha256) | | checksum file for the line above |

Pinned direct link (repository commit `32d85ac`, the commit that added the tarball): https://github.com/RegularJoe-CEO/LuxiDemo/raw/32d85ac1891eba4d03b7dc365690acaea78b5aea/downloads/luxi-replay-demo/luxi-replay-demo-1.0.0-linux-x86_64.tar.gz

License and access: proprietary closed binary, free to download and use for evaluation; no open-source license on the
engine. Terms: [`LICENSE.txt`](LICENSE.txt). Third-party components: [`THIRD_PARTY_NOTICES.md`](THIRD_PARTY_NOTICES.md).
Both files are also inside the tarball.

Model weights are not in the package. `./luxi fetch-model` downloads them from Hugging Face (8.7 GB, MIT license) and
checks every file by sha256.

## Requirements

- Linux x86_64, glibc 2.34 or newer (Ubuntu 22.04+, Debian 12+, RHEL 9+, Amazon Linux 2023). Not macOS, Windows or ARM.
- Internet for the install and the one model download. Disk about 13 GB (GPU) or 10 GB (CPU).
- GPU path: NVIDIA compute capability 8.0, 8.6, 8.9, 9.0 or 12.0 (A100, RTX 30, RTX A6000, A40, RTX 40, L4, L40S, H100,
  H200, RTX 50, RTX PRO 6000 Blackwell), driver 570 or newer, 16 GB for the short replay, 22 GB for the full suite.
  A GPU request never falls back to the CPU: an unusable GPU exits with code 3.
- CPU path: Linux x86_64 CPU, 32 GB RAM recommended. Short replay only. Tested on AMD EPYC 7543, Intel Xeon Gold 6342
  and one sandbox CPU whose model the VM did not expose.

## Exact commands

```bash
# download and check
curl -LO https://github.com/RegularJoe-CEO/LuxiDemo/raw/32d85ac1891eba4d03b7dc365690acaea78b5aea/downloads/luxi-replay-demo/luxi-replay-demo-1.0.0-linux-x86_64.tar.gz
echo "b1f6cf0b024f0270223807662ebc15d72191db4838c408bf6727400c5ec539c9  luxi-replay-demo-1.0.0-linux-x86_64.tar.gz" | sha256sum -c

# install
tar xzf luxi-replay-demo-1.0.0-linux-x86_64.tar.gz
cd luxi-replay-demo-1.0.0-linux-x86_64
sha256sum -c SHA256SUMS          # every shipped file
./install.sh --gpu               # or: ./install.sh --cpu
./luxi fetch-model               # 8.7 GB once; every file checked by sha256

# 1. inspect: build identity, model checksums, device, fixtures
./luxi inspect --device gpu --check-weights      # or --device cpu

# 2. short frozen replay (63-token prompt, 32 sampled tokens)
./luxi demo --device gpu
./luxi demo --device cpu
#    last line on success: RESULT: SAME BYTES as the Nov 9 H100 NVL reference ...
#    (the demo checks the step root against the 16 hex digits that survive from the Nov 9 H100 NVL short
#    replay, 158ee5ae768cb633; the full Nov 9 short record was not recovered. Use ./luxi compare for a
#    full record-to-record comparison between two machines.)

# 3. verify offline (no GPU, no model needed)
./luxi verify luxi_out/bundle_tiny-math500-090_gpu-*.json --trusted-key <public key printed by the demo>
./luxi verify reference/nov09_h100_nvl_signed/full_suite/bundle_a4-math500-090.json \
     --trusted-key reference/keys/luxi_benchmark_key_2026-11.pub
#    a wrong trusted key is rejected (exit 1)
./luxi verify reference/nov09_h100_nvl_signed/full_suite/bundle_a4-math500-090.json \
     --trusted-key 0000000000000000000000000000000000000000000000000000000000000001
#    changed token, fingerprint, prompt, seed or temperature, a dropped token, a re-signed forgery
#    and a wrong key: all rejected (exit 0 means every tested edit was caught)
./luxi tamper-test reference/nov09_h100_nvl_signed/full_suite/bundle_a4-math500-090.json \
     --trusted-key reference/keys/luxi_benchmark_key_2026-11.pub

# 4. compare two records from any two machines (exit 0 only if every token id and per-step fingerprint matches)
./luxi compare luxi_out/bundle_tiny-math500-090_gpu-*.json luxi_out/bundle_tiny-math500-090_cpu-*.json

# 5. full GPU suite: 136 answers vs the signed November 9 H100 NVL records (about 5 to 10 minutes on the GPUs tested)
./luxi full-suite --device gpu
```

Exit codes: `0` pass; `1` outputs differ or a record is rejected; `2` usage error (for example `full-suite --device
cpu`); `3` the requested device cannot run (never a silent CPU fallback); `4` model files missing or wrong.

Check the benchmark public key before trusting it: `reference/keys/luxi_benchmark_key_2026-11.pub` must equal
`42bb6bccd2bc7d0570b44d5e068971707a704a26e8cb0294b9df12f23fdfe418`, the copy in
[`evidence/r1-distill-7b-gpu-swap-2026-11-09/reference/`](../../evidence/r1-distill-7b-gpu-swap-2026-11-09/reference/).
Each installation signs its own records with its own new key; the benchmark private key is not distributed.

## What this build was tested on (November 10, 2026)

Clean install from the tarball on each machine, no private source or credentials. Raw outputs:
[`evidence/r1-replay-package-2026-11-10/`](../../evidence/r1-replay-package-2026-11-10/).

| Machine | GPU | Host CPU (threads used) | 32-token replay | Full suite (136 answers) |
|---|---|---|---|---|
| RunPod RTX 5090 pod, Ubuntu 24.04, glibc 2.39 | NVIDIA GeForce RTX 5090, compute capability 12.0, 32 GB, driver 595.91.07 | AMD EPYC 7543 (14) | GPU twice and CPU once: identical records, root matches the Nov 9 prefix | 136/136 answers and 546,114 tokens identical; 301.6 s at 32 slots |
| RunPod A40 pod, Ubuntu 24.04, glibc 2.39 | NVIDIA A40, compute capability 8.6, 48 GB, driver 595.91.07 | Intel Xeon Gold 6342 (7) | GPU twice and CPU once: identical records, root matches the Nov 9 prefix | 136/136 answers and 546,114 tokens identical; 799.0 s at 32 slots |
| Sandbox VM (not RunPod), Ubuntu 22.04, glibc 2.35 | none: `demo --device gpu` exits 3, no CPU fallback | x86_64, model not exposed by the VM (16) | CPU twice: identical records, root matches the Nov 9 prefix | not run: `full-suite --device cpu` is refused (exit 2) |

On all three machines: tarball and per-file checksums passed; each record verified offline against that installation's
public key; a wrong trusted key was rejected (exit 1); the tamper test rejected every tested edit; a missing model exited 4.
Records from different machines compared identical (`./luxi compare`, exit 0).

What the cross-machine claim for this build covers: the 32-token replay produced the same token ids and per-step logit
fingerprints (same step Merkle root `158ee5ae768cb633…`) on the two GPU types and the CPUs listed, and that root
matches the 16 hex digits that survive from the November 9 H100 NVL short replay (its full record was not recovered); the full 136-answer suite matched the 17 signed November 9 H100 NVL records on the two GPU
types listed. Other GPU types listed under Requirements are compiled for but were not run with this build. The CPU path
covers the short replay only; no CPU full-suite result is claimed.

## Identity

| Item | Value |
|---|---|
| Package | `luxi-replay-demo-1.0.0-linux-x86_64.tar.gz`, sha256 `b1f6cf0b024f0270223807662ebc15d72191db4838c408bf6727400c5ec539c9` |
| Shipped binaries (`build_sha256`, signed into every record) | `9f7931d9d5457e2f9e48241836ae741eff6100a7140d9863967f8c42417a6694` |
| Engine source compiled (`engine_source_sha256`, not shipped) | `795c71736e22e2f66935e4acc91b8040120d40528b04d84ea9daedd5a5692f36` (v27 final, the same value as every November 9 record) |
| Model | `RedHatAI/DeepSeek-R1-Distill-Qwen-7B-quantized.w8a8`, revision `925259de674c80799e9455bfcd2bf50f999022cb` (MIT) |
| Short replay fixture | `tiny-math500-090`: first sample of MATH-500 `test/prealgebra/1761.json`, 63-token prompt, 32 tokens, temperature 0.6, top_p 0.95, seed 2026; November 9 H100 NVL step root prefix `158ee5ae768cb633` |
| Full suite | 17 problems (4 AIME 2024, 13 MATH-500) x 8 samples = 136 answers, 546,114 tokens, up to 16,384 new tokens each |

This is a new build identity, compiled from the same v27 source. Its GPU matrix-multiply library differs from the one
used on November 9; the model, quantization, sampler and reference outputs are unchanged. It does not inherit the November 9 nine-GPU result: its own tested machines are the
ones in the table above.

## Limits

- Internal measurements on rented machines from one cloud (RunPod) plus one sandbox VM. Full-suite timings are single
  runs. Not independent validation.
- Identical outputs show the same computation was reproduced; they say nothing about whether an answer is correct.
- A signature shows which key signed a record, not which machine ran it.
- Speed is not the claim. On November 9, vLLM 0.32 (normal mode) was faster and cheaper per output token than Luxi on
  every GPU where its tested int8 configuration ran.

## Private evaluation

To run the engine on your own model and workload, or across your own choice of GPUs, request a private GPU-swap
evaluation: https://luxiedge.com/contact#gpu-swap-evaluation
