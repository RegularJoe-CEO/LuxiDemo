# November 9 per-GPU signed records (recovered November 10, 2026)

These are the signed answer records that the nine GPUs and their host CPUs produced in the
[November 9, 2026 GPU swap test](../README.md). They were not available when this evidence folder was first published.
They were recovered on November 10 from the working storage of the session that ran the test, placed in the private
record (repository `attention-transformer-v2`, branch `xhw-inference-2026-11-07`, folder
`xhw_portability_2026-11-09/raw/`, commit `b18e2ae`), and added here unchanged. No record was regenerated, re-signed
or edited: every record inside the archives is byte-identical to the recovered original, and [`SHA256SUMS`](SHA256SUMS)
lists each one.

## What is here

| File | Contents |
|---|---|
| `nov09_<GPU>_records.tar.gz` (9 files) | `p17/`: the 17 signed records (136 answers) at 32 answers in flight. `p17s64/` or `p17s128/`: the same 136 answers at 64 or 128 in flight, where that run was kept. `b1/` and `b1w/`: one-job checks. `bundle_tiny_cpu_<GPU>.json`: the host CPU's 32-token replay |
| [`bundle_tiny_h100.json`](bundle_tiny_h100.json) | The H100 NVL 32-token short-replay record, signed. Step Merkle root `158ee5ae768cb633afeb89d288642651d0bedada58c0603a57abfd1620a66e6d`; before this recovery only its first 16 hex digits were available |
| `bundle_tiny_cpu_<GPU>.json` (9 files) | The host CPU 32-token replay records, also inside each archive, as plain files |
| [`independent_check_2026-11-10.json`](independent_check_2026-11-10.json) | Output of the check below |
| [`SHA256SUMS`](SHA256SUMS) | Every file in this folder and every record inside each archive |

Archive names follow the [`../results/`](../results/) prefixes: `NVL` H100 NVL, `H200` H200, `A100SXM` A100 SXM 80GB,
`PRO6000` RTX PRO 6000 Blackwell Server Edition, `RTX5090`, `A6000` RTX A6000, `A40`, `RTX4090`, `RTX3090`.

Not included: the run summaries that sat next to these records (their public extracts are in [`../results/`](../results/),
with internal fields removed as listed in `../results/REDACTIONS.json`), run logs, the test kit, and vLLM output rows.
Those stay in the private record.

## Check them yourself

From the root of this evidence folder, with Python 3.9+ and PyNaCl (no GPU, no model):

```bash
python3 -m pip install pynacl
cd records && for f in nov09_*_records.tar.gz; do tar xzf "$f"; done && sha256sum -c SHA256SUMS && cd ..

# one GPU's 136 answers against the signed H100 NVL reference (every token id and per-step fingerprint)
python3 verify/luxi_verify.py compare-dirs reference records/A40/p17
# a record's signature against the benchmark public key
python3 verify/luxi_verify.py verify records/A40/p17/bundle_a4-math500-090.json \
     --trusted-key reference/luxi_benchmark_key_2026-11.pub
# a host CPU's 32-token replay against the H100 NVL 32-token record
python3 verify/luxi_verify.py compare records/bundle_tiny_h100.json records/bundle_tiny_cpu_A40.json
```

## Result of that check (November 10, 2026)

Run with the standalone verifier in [`../verify/`](../verify/) on the extracted archives. Full output:
[`independent_check_2026-11-10.json`](independent_check_2026-11-10.json).

| GPU | Records, all verified with the benchmark key | 32 in flight vs H100 NVL reference | Second run vs reference | Host CPU 32-token replay vs H100 NVL record |
|---|---|---|---|---|
| H100 NVL | 37 | 136/136 answers, 546,114 tokens identical | 128 in flight: 136/136, 546,114 | AMD EPYC 9654: identical |
| H200 | 20 | 136/136, 546,114 | not run | Intel Xeon Platinum 8568Y+: identical |
| A100 SXM 80GB | 37 | 136/136, 546,114 | 128 in flight: 136/136, 546,114 | AMD EPYC 7742: identical |
| RTX PRO 6000 Blackwell Server Edition | 20 | 136/136, 546,114 | not run | AMD EPYC 9355: identical |
| RTX 5090 | 37 | 136/136, 546,114 | 64 in flight: 136/136, 546,114 | AMD EPYC 9654: identical |
| RTX A6000 | 20 | 136/136, 546,114 | none: ran out of memory at 128 (its comparison file shows 0 jobs) | Intel Xeon Gold 6342: identical |
| A40 | 37 | 136/136, 546,114 | 64 in flight: 136/136, 546,114 | Intel Xeon Gold 6342: identical |
| RTX 4090 | 37 | 136/136, 546,114 | 64 in flight: 136/136, 546,114 | AMD EPYC 7352: identical |
| RTX 3090 | 37 | 136/136, 546,114 | 64 in flight: 136/136, 546,114 | AMD EPYC 7452: identical |

All 282 records verified. Each GPU's one-job checks (`b1`, `b1w`) were identical to the H100 NVL's. The H100 NVL
32-token record verified with the benchmark key, and the tamper test rejected every tested edit. Seven CPU models are
covered (two pairs of pods shared a CPU model).

These raw records confirm the per-GPU comparison files in [`../results/`](../results/) that the original claims rested on.

## Limits

- Internal measurements on one cloud provider (RunPod), November 9, 2026. No third party ran these tests.
- Every record was signed with the same benchmark key on each test machine. A signature shows which key signed a record,
  not which machine ran it; the device names in the records are what each machine reported.
- These records belong to the November 9 per-machine builds of engine source `795c7173…`. The public replay package is a
  different build identity with its own tested machines.
- Identical answers show the same computation was reproduced. They say nothing about whether an answer is correct.
