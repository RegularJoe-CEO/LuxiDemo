# GPU swap test, November 9, 2026 (original evidence)

Internal measurements by LuxiEdge on one cloud provider (RunPod), with fixed tested configurations and single
performance runs. Published on November 10, 2026 from the private record. No result was changed, re-run or
reconstructed for this folder. The per-GPU signed records, recovered later on November 10, are in
[`records/`](records/). What is still missing is listed under [Completeness](#completeness).

## What was tested

| Item | Value |
|---|---|
| Engine | LuxiEdge exact inference engine (Attention Transformer v2), engine v27 final. Engine source sha256 `795c71736e22e2f66935e4acc91b8040120d40528b04d84ea9daedd5a5692f36`. The source is private. On November 9 each machine compiled this same frozen source for its own GPU |
| Model | `RedHatAI/DeepSeek-R1-Distill-Qwen-7B-quantized.w8a8`, revision `925259de674c80799e9455bfcd2bf50f999022cb` (the publisher's INT8 W8A8 quantization, unchanged) |
| Workload | 17 problems (13 from MATH-500, 4 from AIME 2024) with 8 sampled answers each: 136 sampled answers, not 136 distinct questions. Temperature 0.6, top-p 0.95, up to 16,384 new tokens per answer, 32 answers in flight. 546,114 generated tokens in total |
| Reference | The signed H100 NVL records for those 17 jobs (every 8th job of the 130-job November R1 evaluation), in [`reference/`](reference/) |
| Check | Every generated token id and every recorded per-step logit fingerprint, compared with the H100 NVL reference |
| Separate gate | 92 recorded outputs of `RedHatAI/Qwen2-7B-Instruct-quantized.w8a8` (128 tokens each, six run conditions: alone, in batches of 16 and 64, and other load conditions) |

## Results

- **Nine GPU types:** H100 NVL, H200, A100 SXM 80GB, RTX PRO 6000 Blackwell Server Edition, RTX A6000, A40, RTX 5090,
  RTX 4090 and RTX 3090.
- Each matched **136/136 sampled answers and 546,114 generated tokens**, with matching recorded per-step fingerprints,
  against the signed H100 reference, plus a separate **92/92 Qwen gate**. This is 17 problems with eight samples each,
  not 136 distinct questions.
- **Seven CPU models:** AMD EPYC 9654, 7742, 9355, 7352 and 7452; Intel Xeon Platinum 8568Y+ and Xeon Gold 6342. Each
  matched **2/2 small Qwen checks and the short signed R1 replay capped at 32 tokens**. These CPUs did not run the full
  GPU workload in this evidence.
- **A40 soak:** 1,502 calls, 1,239 checks, zero mismatches, in one process (the Qwen2-7B gate model; this repeats the
  call pattern of the long-running-process fault published on September 26).
- Runs with more answers in flight (64 or 128) also matched 136/136 on every card where they ran. The RTX A6000 ran
  out of memory at 128 and has no larger-batch result. On the RTX 4090 and RTX 3090 the engine paused and resumed
  answers under memory pressure (61 times at 32 in flight, 259 to 264 times at 64) and still matched.
- **vLLM 0.32**, normal mode, same prompts and sampling settings: faster and cheaper per token on every GPU where the
  tested int8 configuration ran. Its sequence matches below are against its own H100 run, not against Luxi. On the
  same H100 NVL, capping vLLM at 32 answers in flight changed 77 of its 136 answers (59/136 unchanged). Different
  sequences do not establish accuracy errors. The tested vLLM int8 configuration did not run on compute capability
  12.0 (RTX PRO 6000, RTX 5090).
- **Rental choice, Luxi versus Luxi:** the RTX 5090 delivered about 80% of Luxi's H100 NVL throughput (2,348 tokens/s
  at 64 in flight versus 2,946 at 128) at about one-third of its calculated rental cost per output token ($0.082 versus
  $0.244 per million tokens, from the November 9 rates of $0.69/h and $2.59/h). The rates exclude licensing and other
  operating costs. They are not API prices or current rental offers.

| GPU | Nov 9 $/h | 136 answers vs H100 | Qwen gate | Host CPU | Host CPU checks | Luxi tok/s, 32 in flight | Luxi tok/s, more in flight | Luxi J/token (32 / more) | vLLM 0.32 tok/s | vLLM J/token | vLLM vs its own H100 run |
|---|---|---|---|---|---|---|---|---|---|---|---|
| H100 NVL | 2.59 | 136/136 | 92/92 | AMD EPYC 9654 | 2/2, 32-token replay same | 2,243 | 2,946 (128) | 0.136 / 0.104 | 4,137 | 0.074 | 135/136 |
| H200 | 5.29 | 136/136 | 92/92 | Intel Xeon Platinum 8568Y+ | 2/2, 32-token replay same | 1,885 | not run | 0.213 | 5,443 | 0.093 | 135/136 |
| A100 SXM 80GB | 1.39 | 136/136 | 92/92 | AMD EPYC 7742 | 2/2, 32-token replay same | 1,035 | 1,947 (128) | 0.259 / 0.167 | 2,468 | 0.132 | 0/136 |
| RTX PRO 6000 Blackwell Server Edition | 2.49 | 136/136 | 92/92 | AMD EPYC 9355 | 2/2, 32-token replay same | 1,450 | not run | 0.216 | did not run | - | - |
| RTX A6000 | 0.59 | 136/136 | 92/92 | Intel Xeon Gold 6342 | 2/2, 32-token replay same | 745 | out of memory at 128 | 0.386 | 1,271 | 0.230 | 0/136 |
| A40 | 0.59 | 136/136 | 92/92 | Intel Xeon Gold 6342 | 2/2, 32-token replay same | 631 | 842 (64) | 0.441 / 0.345 | 1,082 | 0.261 | 0/136 |
| RTX 5090 | 0.69 | 136/136 | 92/92 | AMD EPYC 9654 | 2/2, 32-token replay same | 1,499 | 2,348 (64) | 0.281 / 0.198 | did not run | - | - |
| RTX 4090 | 0.89 | 136/136 | 92/92 | AMD EPYC 7352 | 2/2, 32-token replay same | 998 | 1,102 (64) | 0.282 / 0.264 | 1,693 | 0.180 | 0/136 |
| RTX 3090 | 0.22 | 136/136 | 92/92 | AMD EPYC 7452 | 2/2, 32-token replay same | 812 | 953 (64) | 0.418 / 0.365 | 1,268 | 0.269 | 0/136 |

Speed rows are single runs. The first 32-in-flight run on each machine includes one-time compilation (its prefill
time is in the run summaries). Energy is NVML GPU-board energy, not facility electricity. All numbers in this table
come from [`grid.json`](grid.json) and the files in [`results/`](results/).

## Files

| Path | What it is |
|---|---|
| [`grid.json`](grid.json) | Every number in the table, one row per GPU (original file) |
| `results/<GPU>__cmp_p17_vs_h100.json` | The 136-answer comparison computed on that machine on November 9 (also `cmp_p17s64`, `cmp_p17s128` for larger batches, `cmp_b1` for one answer run alone) |
| `results/<GPU>__gate_q2_<GPU>.json` | That GPU's 92 gate outputs: token ids and per-step fingerprints |
| `results/<GPU>__gsmall_cpu_<GPU>.json` | That host CPU's two small Qwen check outputs |
| `results/<GPU>__cmp_tiny_cpu.json` | The 32-token R1 replay on that host CPU compared with the H100 NVL replay |
| `results/<GPU>__luxi_*_summary.json` | Luxi run summaries: tokens, wall time, board joules, tokens/s (public extracts, see [`results/REDACTIONS.json`](results/REDACTIONS.json)) |
| `results/<GPU>__vllm*_summary.json` | vLLM 0.32 run summaries (`vllm32` = capped at 32 in flight) |
| `results/A40__soak_A40.json` | The A40 soak log: 1,502 calls, 1,239 checks |
| `results/<GPU>__gpu.txt`, `__lscpu.txt` | `nvidia-smi` and `lscpu` from that machine |
| [`reference/`](reference/) | The 17 signed H100 NVL records and the benchmark public key |
| [`records/`](records/) | The per-GPU signed records and the 32-token replay records, recovered November 10 and added unchanged, with their own `SHA256SUMS` and README |
| [`verify/`](verify/) | The standalone offline verifier and its November 10 outputs on these records |
| [`SHA256SUMS`](SHA256SUMS) | sha256 of every file in this folder |

## Check it yourself (offline, no GPU, no model)

The benchmark public key is
`42bb6bccd2bc7d0570b44d5e068971707a704a26e8cb0294b9df12f23fdfe418` ([`reference/luxi_benchmark_key_2026-11.pub`](reference/luxi_benchmark_key_2026-11.pub)).
The same key ships in the public replay package. The private key is not published.

```bash
pip install pynacl                       # Python 3.9 or newer
cd evidence/r1-distill-7b-gpu-swap-2026-11-09
sha256sum -c SHA256SUMS
python3 verify/luxi_verify.py verify reference/bundle_a4-math500-090.json \
    --trusted-key reference/luxi_benchmark_key_2026-11.pub          # exit 0
python3 verify/luxi_verify.py verify reference/bundle_a4-math500-090.json \
    --trusted-key 0000000000000000000000000000000000000000000000000000000000000001   # wrong key: exit 1
python3 verify/luxi_verify.py tamper-test reference/bundle_a4-math500-090.json \
    --trusted-key reference/luxi_benchmark_key_2026-11.pub          # changed token, fingerprint, prompt, seed: all rejected
```

On November 10 all 17 reference records verified with this key, the wrong key was rejected, and every tampered copy
was rejected ([`verify/`](verify/)). The nine gate files in `results/` can be compared with each other directly: all nine
carry identical token ids, per-step fingerprints and logits hashes on all 92 checks (re-checked November 10).

A signature authenticates a record relative to a key. It does not independently prove which machine executed it, or
that its answer is correct. Reproduction is the separate check: identical token ids and per-step fingerprints from an
independent run. The [public replay package](../../downloads/luxi-replay-demo/) lets anyone run that check on the
short replay and, with a 24 GB or larger GPU, on all 136 answers.

## Completeness

Included unchanged: `grid.json`; every per-GPU comparison, gate, CPU-check, soak, vLLM-summary and machine-information
file from the November 9 archive; the 17 signed H100 NVL reference records and the public key.

Changed for publication: the Luxi run summaries are public extracts. Internal memory-layout and engine-mode fields were
removed; nothing was changed or added. [`results/REDACTIONS.json`](results/REDACTIONS.json) lists the removed field
names and the sha256 of each original.

Added on November 10, 2026 after recovery, unchanged: [`records/`](records/) holds the per-GPU signed records (the 136
answers from each of the nine GPUs at 32 in flight, the 64 and 128 in-flight runs that were kept, the one-job checks),
the nine host-CPU 32-token replay records and the H100 NVL 32-token record. They were found in the working storage of
the session that ran the test. Every one verifies with the benchmark key and matches the reference; the check and its
commands are in [`records/README.md`](records/README.md).

Not in this folder:

1. **vLLM token rows.** Recovered on November 10 into the private record; only the vLLM run summaries are published here.
2. **Run logs, run summaries in full, and kit scripts.** They name internal build settings and are kept in the private
   record. Public extracts of the run summaries are in [`results/`](results/).

A separately dated, complete demonstration with every record retained is in
[`../r1-replay-package-2026-11-10/`](../r1-replay-package-2026-11-10/). It tests the public package build, which is a
different build identity from the per-machine builds tested here.

## Limits

- Internal measurements, one cloud provider, fixed tested configurations, single performance runs. No third party has
  run or checked these results.
- These results belong to the November 9 per-machine builds of engine source `795c7173…`. A new binary does not
  inherit this nine-GPU validation; the public package's own results are published separately.
- Identical answers say nothing about answer accuracy.
- The September 2026 Qwen2-7B results on H100 SXM are a different engine version, model and workload. Do not transfer
  their speed or quality claims to this test, or the reverse.

Source: private repository `attention-transformer-v2`, branch `xhw-inference-2026-11-07`, folder
`xhw_portability_2026-11-09` (commits `8fc39174` and `50fadd99`, November 9, 2026); reference records from
`xhw_inference_2026-11-09_v27/data/a4/`.
