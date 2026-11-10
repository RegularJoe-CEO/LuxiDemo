# Luxi replay demo 1.0.1: clean-install test, November 10, 2026

Internal measurement. This folder holds every replay record, comparison and log that the public package
[`downloads/luxi-replay-demo/`](../../downloads/luxi-replay-demo/) wrote when release 1.0.1 was installed from its
tarball on two machines with no private source, compiler or credentials. Not included: each installation's private
signing key (its public key is in every record) and the downloaded model files (identified by sha256 in the package).
Nothing in it was edited after the run.

Release 1.0.0 was tested separately on the same day: [`../r1-replay-package-2026-11-10/`](../r1-replay-package-2026-11-10/).

## What changed from 1.0.0

1.0.1 changes the wrapper, the documentation and one reference file. The engine files did not change.
[`package/CHANGES_FROM_1.0.0.txt`](package/CHANGES_FROM_1.0.0.txt) lists every shipped file with its sha256 in both
releases:

- **54 files identical**, including all 11 engine files, the fixtures, the 17 signed full-suite reference records and the
  public key. `build_sha256` `9f7931d9d5457e2f9e48241836ae741eff6100a7140d9863967f8c42417a6694` and the build label are
  unchanged. They come from the engine files, so records made by 1.0.1 still name `luxi-replay-demo-1.0.0` in their
  signed `build` and `package` fields.
- **5 files changed:** `cli/luxi_cli.py`, `verifier/luxi_verify.py`, `README.md`, `LICENSE.txt`, `BUILD_INFO.json`.
- **1 file added:** `reference/nov09_h100_nvl_signed/short_replay/bundle_tiny_h100.json`, the signed November 9 H100
  NVL record of the 32-token replay (sha256 `a2d3cf40119916334126076c606bfe71ca269f27691b8e4e281bf77d4f2036df`). It was
  recovered on November 10 ([`../r1-distill-7b-gpu-swap-2026-11-09/records/`](../r1-distill-7b-gpu-swap-2026-11-09/records/))
  and copied unchanged.

What that does for a user:

1. `./luxi demo` now checks the run against that whole November 9 record. The record's signature must check against the
   benchmark key, and all 32 token ids and per-step fingerprints must match. 1.0.0 had only the first 16 hex digits of
   that record's step root to check against.
2. The demo's readable summary is now `luxi_out/summary_<job>_<device>.json`. In 1.0.0 it was named
   `bundle_<job>_<device>.summary.json`. The `bundle_...-*.json` patterns in the documented verify and compare commands
   then also matched it, and those two commands stopped with a usage error (exit 2). In 1.0.1 they run as written.
3. `verify`, `compare` and `tamper-test` reject a file that is not a signed record with a clear message (exit 2), and
   `compare-dirs` skips 1.0.0 summary files.

An earlier 1.0.1 candidate (tarball sha256 `d9cd5c0a…`, branch commit `4e2c5fc`) had changes 2 and 3 but still checked
the 16-digit prefix. It passed the same tests on the sandbox CPU and the RTX A6000. It was replaced before release, when
the full November 9 record was recovered, and is not the published 1.0.1.

## What was tested

| Item | Value |
|---|---|
| Package | `luxi-replay-demo-1.0.1-linux-x86_64.tar.gz`, 25,476,758 bytes, sha256 `c2114aa7bd6c47822bdf94de2f7d77f1aa773a32dcc3268072cfb979c2e5c8b3` ([`package/`](package/) has its `BUILD_INFO.json`, per-file `SHA256SUMS` and the tarball `.sha256`) |
| Build | `build_sha256` `9f7931d9…6694`, engine source `795c71736e22e2f66935e4acc91b8040120d40528b04d84ea9daedd5a5692f36` (v27 final), the same engine files as 1.0.0 |
| Model | `RedHatAI/DeepSeek-R1-Distill-Qwen-7B-quantized.w8a8`, revision `925259de674c80799e9455bfcd2bf50f999022cb`, checked by sha256 |
| Short replay | `tiny-math500-090`: 63-token prompt, 32 sampled tokens, temperature 0.6, top_p 0.95, seed 2026, compared with the signed November 9 H100 NVL record |
| Full suite | 17 problems (4 AIME 2024, 13 MATH-500) x 8 samples = 136 answers, up to 16,384 new tokens each, 32 answers in flight, compared with the 17 signed November 9 H100 NVL records |

## Results

| Machine | Short replay (32 tokens) | Full suite |
|---|---|---|
| **RTX A6000** (RunPod; compute capability 8.6, 48 GB, driver 595.91.07; Ubuntu 24.04, glibc 2.39) | GPU run 1 and run 2: identical to each other and to the signed November 9 H100 NVL record. Execution 0.69 s and 0.67 s after a 21 to 24 s model load | **136/136 answers, 546,114 tokens identical** to the November 9 H100 NVL records. 662.7 s at 32 slots (695 s with model load): 824.1 output tokens/s, 0.362 GPU-board J per token. Also identical, job by job, to the RTX A6000's own November 9 records from its per-machine build |
| **Intel Xeon Gold 6342** (host CPU of the RTX A6000 pod, 7 threads from the container's CPU quota) | Identical to the November 9 H100 NVL record. Execution 16.3 s | Not run (CPU path is short replay only) |
| **Sandbox VM** (not RunPod; x86_64, CPU model not exposed by the VM, 16 threads; Ubuntu 22.04, glibc 2.35; no GPU) | Run 1 and run 2: identical to each other and to the November 9 H100 NVL record. Execution 5.9 s and 5.8 s | `full-suite --device cpu` refused with exit 2, as designed. `demo --device gpu` exited 3 with "No CPU fallback" |

Comparisons with records made elsewhere, run with the package's own `./luxi compare`:

- **Release 1.0.0 records:** the RTX 5090, A40, AMD EPYC 7543, Intel Xeon Gold 6342 and sandbox CPU short-replay records
  from [`../r1-replay-package-2026-11-10/`](../r1-replay-package-2026-11-10/) are identical to the 1.0.1 records from
  both machines here.
- **November 9 records:** the nine host-CPU short-replay records from the November 9 test (seven CPU models) are identical
  to the 1.0.1 records from both machines here.

Checks that passed on both machines ([`rtxa6000/test_log.txt`](rtxa6000/test_log.txt),
[`sandbox-cpu/test_log.txt`](sandbox-cpu/test_log.txt)):

| Check | Command | Result |
|---|---|---|
| Downloaded tarball checksum | `sha256sum -c luxi-replay-demo-1.0.1-linux-x86_64.tar.gz.sha256` | OK |
| Every shipped file | `sha256sum -c SHA256SUMS` | 60/60 OK |
| Install | `./install.sh --gpu` (pod) or `--cpu` (sandbox) | exit 0 (45 s, 8 s) |
| Model download and check | `./luxi fetch-model` | exit 0 on the pod (24 s), every file sha256-checked. The sandbox used a copy of the same files checked earlier |
| Documented verify and compare commands, patterns as written | `./luxi verify luxi_out/bundle_tiny-math500-090_gpu-*.json ...`, `./luxi compare luxi_out/bundle_..._gpu-*.json luxi_out/bundle_..._cpu-*.json` | exit 0 |
| November 9 short record | `./luxi verify reference/nov09_h100_nvl_signed/short_replay/bundle_tiny_h100.json --trusted-key 42bb6bcc…` and `./luxi compare` against this run | verified; identical |
| Wrong trusted key | `./luxi verify <record> --trusted-key <a different key>` | rejected, exit 1 |
| Tamper test | `./luxi tamper-test <record> --trusted-key <own key>` | every tested edit rejected |
| Altered November 9 reference (sandbox) | one fingerprint in the shipped November 9 record changed, then `./luxi demo --device cpu` | exit 1, "NO MATCH", first difference at step 5; file restored and `SHA256SUMS` passed again |
| A summary file passed as a record (sandbox) | `./luxi compare <1.0.0 summary file> <record>` | exit 2 with a clear message |
| A 1.0.0 summary file left in a compared folder (sandbox) | `./luxi compare-dirs` | skipped; exit 0 |
| Missing model (sandbox) | `./luxi demo --model <a folder that does not exist>` | exit 4 with the fetch command to run |

Timings are single runs, printed by the package, with model load reported separately from execution. They are not
benchmark results. On November 9, vLLM 0.32 (normal mode) was faster and cheaper per output token than Luxi on every GPU
where its tested int8 configuration ran.

## Files

| Folder | Contents |
|---|---|
| [`package/`](package/) | `BUILD_INFO.json`, the package's `SHA256SUMS`, the tarball `.sha256`, and `CHANGES_FROM_1.0.0.txt` |
| [`rtxa6000/`](rtxa6000/) | Everything the RTX A6000 pod test wrote: machine information, install and model logs, `inspect` output, both GPU replay records and the host CPU record, every comparison, verify and tamper output, the full-suite result, log and records (`full__records.tar.gz`), and the test script |
| [`sandbox-cpu/`](sandbox-cpu/) | The same for the sandbox CPU, plus the negative tests |
| [`SHA256SUMS`](SHA256SUMS) | sha256 of every file in this folder |

File names that start with `luxi_out__`, `demo_gpu_2__`, `demo_cpu_2__`, `demo_cpu_badref__` or `full__` were in that
subfolder of the test's output; they were flattened into one folder for upload, unchanged.

## Limits

- Internal measurements: one RunPod pod and one sandbox VM, single runs. Not independent validation.
- 1.0.1 ran on one GPU type here. Its engine files are byte-identical to 1.0.0, which matched 136/136 on the RTX 5090
  and the A40, so the package's tested GPUs are those three; other GPU types it is compiled for were not run with
  either release.
- Identical outputs show the same computation was reproduced; they say nothing about whether an answer is correct.
- A signature shows which key signed a record, not which machine ran it.
