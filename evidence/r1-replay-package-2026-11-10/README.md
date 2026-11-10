# Luxi replay demo 1.0.0: clean-install test, November 10, 2026

Internal measurement. This folder holds every replay record, summary, comparison and log the public package
[`downloads/luxi-replay-demo/`](../../downloads/luxi-replay-demo/) wrote when it was installed from the tarball on
three machines with no private source, compiler or credentials. Not included: each installation's private signing key
(its public key is in every record) and the downloaded model files (identified by sha256 below). Nothing in it was edited after the run. It is a
separate, dated demonstration from the [November 9 GPU swap test](../r1-distill-7b-gpu-swap-2026-11-09/), and it
tests a different build identity.

## What was tested

| Item | Value |
|---|---|
| Package | `luxi-replay-demo-1.0.0-linux-x86_64.tar.gz`, 25,473,516 bytes, sha256 `b1f6cf0b024f0270223807662ebc15d72191db4838c408bf6727400c5ec539c9` ([`package/`](package/) has its `BUILD_INFO.json` and per-file `SHA256SUMS`) |
| Build | `build_sha256` `9f7931d9d5457e2f9e48241836ae741eff6100a7140d9863967f8c42417a6694`, compiled from engine source `795c71736e22e2f66935e4acc91b8040120d40528b04d84ea9daedd5a5692f36` (v27 final, the value in every November 9 record) |
| What differs from November 9 | New binaries with a different GPU matrix-multiply library than the November 9 runs. Model, quantization, sampler and the reference outputs are unchanged. Because the binaries are new, this build gets its own machine list (below) and does not inherit the November 9 nine-GPU result |
| Model | `RedHatAI/DeepSeek-R1-Distill-Qwen-7B-quantized.w8a8`, revision `925259de674c80799e9455bfcd2bf50f999022cb`, downloaded on each machine by `./luxi fetch-model` and checked by sha256 |
| Short replay | `tiny-math500-090`: first sample of MATH-500 `test/prealgebra/1761.json`, 63-token prompt, 32 sampled tokens, temperature 0.6, top_p 0.95, seed 2026. Expected step Merkle root prefix from the November 9 H100 NVL run: `158ee5ae768cb633` (only these 16 hex digits survive; the full November 9 short record was not recovered, so "matches November 9" below means this prefix) |
| Full suite | 17 problems (4 AIME 2024, 13 MATH-500) x 8 samples = 136 answers, up to 16,384 new tokens each, 32 answers in flight, compared with the 17 signed November 9 H100 NVL records shipped in the package |

## Results

| Machine | Short replay (32 tokens) | Full suite |
|---|---|---|
| **RTX 5090** (RunPod; compute capability 12.0, 32 GB, driver 595.91.07; Ubuntu 24.04, glibc 2.39) | GPU run 1 and run 2: identical, root matches the November 9 prefix. Execution 0.65 s and 0.71 s after a 29 s model load | **136/136 answers, 546,114 tokens identical** to the November 9 H100 NVL records. 301.6 s run, 1,810.5 output tokens/s at 32 slots, 0.230 NVML GPU-board J per token |
| **AMD EPYC 7543** (host CPU of the RTX 5090 pod, 14 threads from the container's CPU quota) | Identical, root matches the November 9 prefix. Execution 76.7 s; the shared host had a load average near 68 at the time | Not run (CPU path is short replay only) |
| **A40** (RunPod; compute capability 8.6, 48 GB, driver 595.91.07; Ubuntu 24.04, glibc 2.39) | GPU run 1 and run 2: identical, root matches the November 9 prefix. Execution 0.78 s both times, after a 16 to 17 s model load | **136/136 answers, 546,114 tokens identical** to the November 9 H100 NVL records. 799.0 s run, 683.5 output tokens/s at 32 slots, 0.424 NVML GPU-board J per token |
| **Intel Xeon Gold 6342** (host CPU of the A40 pod, 7 threads from the container's CPU quota) | Identical, root matches the November 9 prefix. Execution 10.2 s | Not run |
| **Sandbox VM** (not RunPod; x86_64, CPU model not exposed by the VM, 16 threads; Ubuntu 22.04, glibc 2.35; no GPU) | Run 1 and run 2: identical, root matches the November 9 prefix. Execution 6.5 s and 6.2 s | `full-suite --device cpu` refused with exit 2, as designed. `demo --device gpu` exited 3 (no GPU, no CPU fallback) |

Direct cross-machine comparisons, run with the package's own `./luxi compare` ([`cross-machine/`](cross-machine/)):

- **Short replay:** all 10 pairs among the five records (RTX 5090, A40, EPYC 7543, Xeon Gold 6342, sandbox CPU) are
  identical in every token id and every per-step logit fingerprint.
- **Full suite:** RTX 5090 versus A40, job by job: 17/17 jobs, 136/136 answers and 546,114 tokens identical.

Checks that passed on every machine ([`rtx5090/test_log.txt`](rtx5090/test_log.txt),
[`a40/test_log.txt`](a40/test_log.txt), [`sandbox-cpu/log.txt`](sandbox-cpu/log.txt)):

| Check | Command | Result |
|---|---|---|
| Downloaded tarball checksum | `sha256sum -c luxi-replay-demo-1.0.0-linux-x86_64.tar.gz.sha256` | OK |
| Every shipped file | `sha256sum -c SHA256SUMS` (sandbox: 59 files) | OK |
| Install | `./install.sh --gpu` (pods) or `--cpu` (sandbox) | exit 0 (44 s, 173 s, 42 s) |
| Model download and check | `./luxi fetch-model` | exit 0 on both pods (31 s, 21 s), every file sha256-checked. The sandbox used a copy of the same files downloaded and sha256-checked earlier; this run's check used the package's size-and-time checksum cache |
| Offline verify, own key | `./luxi verify <record> --trusted-key <this install's public key>` | exit 0 |
| Wrong trusted key | `./luxi verify <record> --trusted-key <a different key>` (pods: the benchmark key; sandbox: `00…01`) | rejected, exit 1 |
| Tamper test | `./luxi tamper-test <record> --trusted-key <own key>` | every tested edit rejected: changed token, changed per-step fingerprint, changed prompt token, changed seed (signed and job), changed temperature, dropped token, re-signed forgery with a new key, wrong key |
| November 9 records | `./luxi verify` and `tamper-test` on `reference/nov09_h100_nvl_signed/` with the benchmark key (sandbox) | verified; every tested edit rejected |
| Missing model | `./luxi demo` with an empty model directory | exit 4 |

Timings are single runs, printed by the package, with model load reported separately from execution. They are not
benchmark results. Full-suite figures are from one run at 32 slots on each GPU; November 9 measured other slot counts
and is the place for throughput, rental cost and comparator figures.

## Files

| Folder | Contents |
|---|---|
| [`package/`](package/) | `BUILD_INFO.json`, the package's `SHA256SUMS`, and the tarball `.sha256` |
| [`rtx5090/`](rtx5090/), [`a40/`](a40/) | Everything the test script captured on each pod, flattened as `<subfolder>__<file>`: `test_log.txt` (the sequence above), install, fetch and inspect output, `demo_*` logs, signed replay records (`*.json`) and their summaries, `cmp_*.json`, `verify_*.json`, `tamper_own.json`, `install_public_key.txt` (that installation's public key), `full__full_suite_result.json`, `full__run.log`, `full__records.tar.gz` (the 17 signed full-suite records plus the run summary), GPU, CPU, OS and memory information. `pip_freeze_uv.txt` (A40) lists the installed Python packages; the RTX 5090 install used the same pinned requirements |
| [`sandbox-cpu/`](sandbox-cpu/) | The sandbox run: `run.sh` (the exact commands), `log.txt`, records, compares, verify and tamper outputs, exit-code checks |
| [`cross-machine/`](cross-machine/) | The five short-replay records, `short_replay_all_pairs.json` (10 pairs) and `full_suite_rtx5090_vs_a40.json` (17 jobs) |
| [`superseded-rc1/`](superseded-rc1/) | The release candidate tested first (see below) |
| `summary.json` | Machine-readable extract of the results above |
| `SHA256SUMS` | sha256 of every file in this folder. Check with `sha256sum -c SHA256SUMS` from inside the folder |

To check any record yourself, install the package (CPU is enough; no model needed) and run
`./luxi verify <record> --trusted-key <the key in that machine's install_public_key.txt>` or
`./luxi compare <record A> <record B>`.

## Release candidate rc1 (superseded, never published)

An earlier tarball (sha256 `49e768e31a3609786996052e6e61ec02c777d36c2e0fbfa54a43968be43c48ec`) had the same engine
binaries and `build_sha256`. On the RTX 5090 pod it passed every check above, including the full suite (136/136 in
300.5 s). On the A40 pod its CPU replay never finished: the CLI read only the newer cgroup v2 CPU quota, so under that
pod's cgroup v1 quota of about 7.7 CPUs it started 96 threads. The run was stopped after more than 13 minutes. Version
1.0.0 reads both cgroup versions and sets the thread count before PyTorch starts; that and README wording are the only
differences. The rc1 logs are kept in [`superseded-rc1/`](superseded-rc1/).

## Limits

- Internal measurement on rented machines from one cloud plus one sandbox VM. No third party has run or checked it.
- Two GPU types ran this build: RTX 5090 and A40. The package is also compiled for compute capability 8.0, 8.9 and 9.0
  (A100, RTX 40, L4, L40S, H100, H200) and for the RTX PRO 6000 Blackwell; those were not run with this build.
- The CPU result is the 32-token replay only. No CPU full-suite result is claimed.
- A signature binds a record to the key that signed it. Each installation made its own key, so these records show what
  each installation signed, not which physical machine ran it. The reproduction claim rests on the compares.
- Identical outputs say nothing about whether an answer is correct.
