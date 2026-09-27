# Qwen2-7B on H100: Luxi vs SGLang 0.5.19 (2026-09-26)

Raw files behind the SGLang section of [`BENCHMARKS.md`](../../BENCHMARKS.md). One H100 SXM 80GB on RunPod (US-NE-1), a different pod from the [vLLM pack](../h100-qwen2-7b-vs-vllm-0.25.1-2026-09-25/). What was and wasn't tested: [`TEST_SCOPE.md`](../../TEST_SCOPE.md).

| File | What it is |
|---|---|
| [`summary.md`](summary.md) | Per-test medians (n=6): throughput, J per token, seconds per call, median board power, median SM clock, plus ratios vs Luxi and engine settings |
| [`runs/`](runs/) `*.csv` | Per-run rows, one file per block: `D1`, `D4` = SGLang deterministic fp16 · `L2`, `L5` = Luxi (FA3) · `E3`, `E6` = SGLang deterministic bf16 (round 1) · `N1`, `N3` = SGLang normal fp16 · `V2`, `V4` = vLLM 0.25.1 default (round 2) |
| [`runs/meta.json`](runs/meta.json) | Engine settings recorded by each block (also shown at the end of `summary.md`) |
| [`runs/csv_to_json.py`](runs/csv_to_json.py) | Rebuilds each block's JSON from its CSV |
| [`runs/summarize_race.py`](runs/summarize_race.py) | The script that produced `summary.md` |
| [`runs/R1_blocks.log`](runs/R1_blocks.log) | Block order and times (UTC) for Luxi vs SGLang deterministic fp16 (D) and bf16 (E) |
| [`runs/R2_blocks.log`](runs/R2_blocks.log) | Block order and times (UTC) for normal SGLang (N) and vLLM 0.25.1 default (V) |
| [`gates/gates.out`](gates/gates.out) | Luxi gate result (determinism, correctness, same hashes as the 2026-09-26 final gate) and the first SGLang gate attempts |
| [`gates/gates2.out`](gates/gates2.out), [`gates3.out`](gates/gates3.out), [`gates4.out`](gates/gates4.out), [`chain2.out`](gates/chain2.out) | Later SGLang gate attempts and the passing runs (rc=0) |
| [`gates/luxi_gate.json`](gates/luxi_gate.json) | Luxi per-test logits hashes and checks |
| [`gates/luxi_gate_rows.csv`](gates/luxi_gate_rows.csv) | Luxi determinism gate: one row per test, batch size and repeat, with the logits hash, first token and a SHA-256 prefix of the generated token list (token lists omitted). Byte-identical to the vLLM pack's copy, which came from a separate gate process on the other pod |
| [`gates/luxi_short_lgate.log`](gates/luxi_short_lgate.log) | Luxi five-prompt Hugging Face check, raw output |
| [`gates/sglang_gate_summary.md`](gates/sglang_gate_summary.md) | SGLang determinism and correctness probe results for deterministic fp16, deterministic bf16 and normal fp16 |
| [`setup/`](setup/) | Pod environment, nvidia-smi report, install and build logs, SGLang package versions, the CUDA 12.9 setting for bf16 runs, and the one-line SGLang logprob patch used by the correctness probe |
| [`RUNBOOK.md`](RUNBOOK.md) | Run plan written before the pod run |

Notes:

- `setup/env_pod.txt` and `setup/nvidia_smi_q_pre.txt` have terminal color codes, tabs and trailing spaces removed. The data is unchanged.
- GPU serial number and UUID, other pod names and an SSH key path are redacted.
- The import check at the end of `setup/sglang.log` failed on a torchvision mismatch. That was fixed on the pod before the gates and speed runs (the fix itself was not logged); the run metadata in `summary.md` shows SGLang 0.5.19 loaded and running.
- Per-block engine logs, the full SGLang probe JSON files (with generated token lists) and the raw logits dumps are not included here.

## Recompute the medians

Each block ran 3 runs of 12 seconds per test, and each engine had 2 blocks, so n = 6 per engine per test. From `runs/`:

```bash
python3 csv_to_json.py
python3 summarize_race.py luxi_c8 luxi_c8=L2.json,L5.json sglang_det_fp16=D1.json,D4.json \
  sglang_det_bf16=E3.json,E6.json sglang_normal_fp16=N1.json,N3.json \
  vllm_0.25.1_default=V2.json,V4.json > summary_check.md
cmp summary_check.md ../summary.md
```

The CSV files hold the measured columns: wall time (`wall_s`), NVML energy (`energy_J`), token counts, calls, median board power, SM clock, temperature and call latency. `csv_to_json.py` adds throughput and energy per token as tokens / `wall_s` and `energy_J` / tokens. `summarize_race.py` takes the median over the 6 runs for each engine and test. `wall_s_bin` (Luxi only) is the time measured inside the Luxi binary; the tables use `wall_s`.
