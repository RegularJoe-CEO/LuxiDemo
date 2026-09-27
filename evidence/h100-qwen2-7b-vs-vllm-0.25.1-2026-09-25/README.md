# Qwen2-7B on H100: Luxi vs vLLM 0.25.1 (2026-09-25/26)

Raw files behind the vLLM section of [`BENCHMARKS.md`](../../BENCHMARKS.md). One H100 SXM on RunPod, a different pod from the [SGLang pack](../h100-qwen2-7b-vs-sglang-0.5.19-2026-09-26/). What was and wasn't tested: [`TEST_SCOPE.md`](../../TEST_SCOPE.md).

| File | What it is |
|---|---|
| [`summary.md`](summary.md) | Per-test mean ± standard deviation, median and n, latency, median board power and SM clock. The vLLM tables in BENCHMARKS.md use these means |
| [`runs/`](runs/) `*.csv` | Per-run rows, one file per block: `V1`, `V5` = vLLM default · `B3`, `B7` = vLLM batch-invariant mode · `L2`, `L6` = Luxi (FA3) · `O4`, `O8` = Luxi own attention kernel |
| [`runs/blocks.log`](runs/blocks.log) | Block order and start/end times (UTC) |
| [`runs/meta.json`](runs/meta.json) | Engine settings recorded by each block |
| [`runs/csv_to_json.py`](runs/csv_to_json.py) | Rebuilds each block's JSON from its CSV |
| [`runs/summarize_long.py`](runs/summarize_long.py) | The script that produced `summary.md` |
| [`gates/luxi_gate_rows.csv`](gates/luxi_gate_rows.csv) | Luxi determinism gate (2026-09-26 final gate): one row per test, batch size and repeat, with the logits hash, first token and a SHA-256 prefix of the generated token list (token lists omitted) |
| [`gates/luxi_long_prompts_vs_hf.json`](gates/luxi_long_prompts_vs_hf.json) | Luxi vs Hugging Face fp16 on the 2,048, 8,192 and 32,767-token prompts: top token, top-5 order, max and mean absolute logit difference |
| [`gates/luxi_greedy_vs_hf.json`](gates/luxi_greedy_vs_hf.json) | First token where Luxi's greedy output differs from the Hugging Face greedy reference, with Hugging Face's top-2 margin at that step |
| [`gates/vllm_greedy_vs_hf.json`](gates/vllm_greedy_vs_hf.json) | The same comparison for vLLM default and batch-invariant mode |
| [`gates/vllm_default_batch_probe.json`](gates/vllm_default_batch_probe.json), [`gates/vllm_batch_invariant_probe.json`](gates/vllm_batch_invariant_probe.json) | vLLM alone vs in-batch probes: steps (of 256) where per-step top-5 logprobs differ |

## Recompute the summary

Each block ran 3 runs of 12 seconds per test, and each engine had 2 blocks, so n = 6 per engine per test. From `runs/`:

```bash
python3 csv_to_json.py
python3 summarize_long.py vllm_default=V1.json vllm_default=V5.json vllm_BI=B3.json vllm_BI=B7.json \
  luxi_c8_FA3=L2.json luxi_c8_FA3=L6.json luxi_c8_ownattn=O4.json luxi_c8_ownattn=O8.json \
  | awk '!seen[$0]++' > summary_check.md
cmp summary_check.md ../summary.md
```

`awk` drops repeated header lines. The CSV files hold the measured columns: wall time (`wall_s`), NVML energy (`energy_J`), token counts, calls, median board power, SM clock, temperature and call latency. `csv_to_json.py` adds throughput and energy per token as tokens / `wall_s` and `energy_J` / tokens. `wall_s_bin` (Luxi only) is the time measured inside the Luxi binary; the tables use `wall_s`.

`gates/luxi_gate_rows.csv` is byte-identical to the one in the SGLang pack, which came from a separate gate process on a different H100.

Not included: per-block engine logs, generated token lists and raw logits dumps.
