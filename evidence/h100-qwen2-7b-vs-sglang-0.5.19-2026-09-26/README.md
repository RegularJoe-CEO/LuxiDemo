# Qwen2-7B on H100: Luxi vs SGLang 0.5.19 (2026-09-26)

Raw files behind the SGLang section of [`BENCHMARKS.md`](../../BENCHMARKS.md). One H100 SXM 80GB on RunPod (US-NE-1).

| File | What it is |
|---|---|
| [`summary.md`](summary.md) | Per-test medians (n=6): throughput, J per token, seconds per call, median board power, median SM clock, plus ratios vs Luxi and engine settings |
| [`runs/R1_blocks.log`](runs/R1_blocks.log) | Block order and times (UTC) for Luxi vs SGLang deterministic fp16 (D) and bf16 (E) |
| [`runs/R2_blocks.log`](runs/R2_blocks.log) | Block order and times (UTC) for normal SGLang (N) and vLLM 0.25.1 default (V) |
| [`gates/gates.out`](gates/gates.out) | Luxi gate result (determinism, correctness, same hashes as the 2026-09-26 final gate) and the first SGLang gate attempts |
| [`gates/gates2.out`](gates/gates2.out), [`gates3.out`](gates/gates3.out), [`gates4.out`](gates/gates4.out), [`chain2.out`](gates/chain2.out) | Later SGLang gate attempts and the passing runs (rc=0) |
| [`gates/luxi_gate.json`](gates/luxi_gate.json) | Luxi per-test logits hashes and checks |
| [`gates/luxi_short_lgate.log`](gates/luxi_short_lgate.log) | Luxi five-prompt Hugging Face check, raw output |
| [`gates/sglang_gate_summary.md`](gates/sglang_gate_summary.md) | SGLang determinism and correctness probe results for deterministic fp16, deterministic bf16 and normal fp16 |
| [`setup/`](setup/) | Pod environment, nvidia-smi report, install and build logs, SGLang package versions, the CUDA 12.9 setting for bf16 runs, and the one-line SGLang logprob patch used by the correctness probe |
| [`RUNBOOK.md`](RUNBOOK.md) | Run plan written before the pod run |

Notes:

- `setup/env_pod.txt` and `setup/nvidia_smi_q_pre.txt` have terminal color codes, tabs and trailing spaces removed. The data is unchanged.
- GPU serial number and UUID, other pod names and an SSH key path are redacted.
- The import check at the end of `setup/sglang.log` failed on a torchvision mismatch. That was fixed on the pod before the gates and speed runs (the fix itself was not logged); the run metadata in `summary.md` shows SGLang 0.5.19 loaded and running.
- Per-run JSON rows, per-block engine logs, the full SGLang probe JSON files (with generated token lists) and the raw logits dumps are not included here.
