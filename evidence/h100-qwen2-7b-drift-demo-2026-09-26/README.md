# H100 · Qwen2-7B-Instruct · output drift under batching and live traffic (2026-09-26)

**Question (from a buyer):** hold the model and each prompt fixed, and change only what else is running on the GPU (batch size,
which other prompts share the batch, where the prompt sits in the batch, when it arrives at a live server). Do the output
tokens, logprobs or hashes change?

**Engines.** All run fp16 with greedy decoding (temperature 0, repetition penalty 1.0) on one H100 SXM 80GB, same weights (Qwen/Qwen2-7B-Instruct).
| label | configuration |
|---|---|
| Luxi c8 | LuxiEdge's batch-invariant engine (the build benchmarked on 2026-09-26), driven through its batch generate API |
| vLLM 0.25.1 default | stock settings, prefix caching off |
| vLLM 0.25.1 batch-invariant | `VLLM_BATCH_INVARIANT=1` (control) |
| SGLang 0.5.19 default | stock settings, radix cache off, FA3 attention |
| SGLang 0.5.19 deterministic | `--enable-deterministic-inference` (control) |

Prefix and radix caching are off for every engine so each run recomputes the prompt. Model default sampling settings from generation_config were disabled (`--generation-config vllm`, `--sampling-defaults openai`), so decoding is pure argmax.

**Prompts.** 20 fixed prompts, pre-tokenized once with the Qwen2 chat template, so every engine receives the same token ids (`data/prompts.json`).
They cover 5 factual, 5 short-reasoning, 5 code and 4 writing prompts, plus 1 long-context prompt (the Declaration of Independence, Project Gutenberg #1, followed by a question, 1,698 tokens).
Each prompt generates 256 tokens (ignore_eos). Comparisons use what a client would receive: tokens up to and including the first end-of-turn token.

**Conditions (per prompt, per engine).**
- `b1`: the prompt alone, batch 1. This is the reference. `b1_rep` repeats it (run-to-run control).
- `b16_f1`, `b16_f2`, `b64_f1`, `b64_f2`: the prompt inside a batch of 16 or 64. Filler set 1 puts the prompt in row 0; filler set 2 uses different fillers and puts it in the middle row. The filler rows are real chat text cut to the prompt's length. Every engine receives identical token rows, because Luxi's batch API takes equal-length rows. For the 1,698-token prompt the "64" conditions use 32 rows, for every engine.
- `b16_mix` (vLLM/SGLang only): batch of 16 with mixed-length fillers (16–1,200 tokens).
- `srv_stagA`, `srv_stagB` (vLLM/SGLang only, through the OpenAI-compatible `/v1/completions` server): the prompt is sent 0.6 s or 1.5 s after 24 or 56 background requests start arriving. The background requests are 16–1,500 tokens long, generate 32–320 tokens, and arrive with exponential gaps averaging 80 ms or 40 ms. The server results are compared with `srv_b1`, the prompt alone on the same server. `srv_b1_rep` is the server run-to-run control, and `srv_b1_vs_offline_b1` cross-checks the server against the offline engine.

**What is recorded** (`raw/*.jsonl`, `drift_results_*.json`): generated token ids and the logprob of each chosen token, the SHA-256 of the token ids and of the logprobs (float64 little-endian), the first token index that differs from the reference, the max |logprob difference| before that index, and whether the decoded visible text differs.

**Luxi specifics (stated plainly).**
- The benchmarked Luxi engine has no concurrent-arrival server, so Luxi has **batch conditions only**. There are no server/staggered rows for Luxi.
- Luxi's batch API returns only the final-step logits of a generate call. So Luxi logprobs are measured at checkpoint positions 0, 1, 3, 7, 15, 31, 63, 127 and 255: the same batch is re-generated with 1, 2, 4, …, 256 new tokens. At each checkpoint the full 152,064-entry logits vector is SHA-256 hashed as well. vLLM and SGLang logprobs cover every position.
- Luxi sanity check: before the drift run, Luxi re-ran 11 gate lines from the 2026-09-26 race. They must reproduce that race's logits hashes bit for bit (`raw/luxi_sanity.json`).
- **Open Luxi fault (long prompt, reported as found).** The 19 shorter prompts and the sanity lines ran in one long-running Luxi process. The 1,698-token prompt (p20) came next: its first call was call 1,044 in that process, after 1,043 earlier calls. (The `note` field in the raw fault records says 1,062; the count here is from the process log.) In that process p20 did not behave consistently. Its first batch-1 run matched the fresh-process run below at positions 0 and 1 and differed from position 3 on, and its 256 generated tokens differed from the fresh-process run from the first token. The batch-1 repeat reproduced those tokens but gave different logits at all 9 checkpoints. Two batch-16 calls then completed, and the next one (call 1,064) stopped with a CUDA illegal-memory-access error. The two affected batch-1 records are kept in `raw/luxi_main_process_p20_fault.jsonl`.
  All 6 conditions of p20 were then re-run in a fresh Luxi process. There, every logits hash and every token matched across batch 1, the repeat, batch 16 (two filler sets) and batch 32 (two filler sets), and a separate fresh-process diagnostic matched too. The Luxi rows for p20 in `summary.md` come from that fresh process (`raw/results_luxi_c8_p20_separate_process.jsonl`), so the Luxi totals (80 of 80 load conditions identical) combine p01–p19 from the long-running process (run before p20) with p20 from the fresh process.
  All Luxi drift runs used a decode context of 2,048 tokens (p20 needs it), above the 1,280 used in the 2026-09-26 race. The state-dependent fault is an open issue in the engine under investigation, not a measured property of batch invariance.
- Luxi engine source, patches and builds are not part of this folder.

**Honesty notes.**
- The rates in `summary.md` are the measured rates for this prompt set, including cases that did not change. Most prompt-conditions may be unchanged even for default engines. `examples.md` shows real cases only, and says so if none occurred.
- Staggered-arrival timing is not reproducible run to run by design. Those rows show what a live server can do, not a fixed schedule.
- One GPU, one run, one model and one dtype. Other models, dtypes, GPUs or engine versions can behave differently.

**Files in this folder.**
- `summary.md` / `summary.json`: rates per engine and condition.
- `per_prompt.csv`: one row per engine and prompt, one column per condition. `same` = tokens and logprob hash identical to the reference, `lp` = tokens identical but logprob hash different, `text@N` = visible text changed with the first differing token at index N, `n/a` = condition not run for that engine. Built from the drift results by `scripts/per_prompt.py`.
- `drift_chart.svg`: % identical tokens and logprob hashes by engine and condition group (the same chart as the `drift_chart.png` drawn by `scripts/analyze.py`).
- `examples.md`: side-by-side diffs of real cases.
- `scripts/`: the vLLM/SGLang drivers, the analysis, `per_prompt.py`, `rebuild.sh`, and `drift_view.py` (e.g. `python scripts/drift_view.py . vllm_normal p07 srv_stagB`).
- `data/prompts.json`: the 20 prompts with their token ids (stored in compact JSON; `build_data.py` writes the same content with indentation). `data/build_data.py`: builds the prompts and fillers. `data/declaration_of_independence_gutenberg1.txt`: the Project Gutenberg #1 text used for the long prompt.
- `raw/results_<engine>.partNN.jsonl`: every generated token and logprob for each engine (`luxi_c8`, `vllm_normal`, `vllm_det`, `sglang_normal`, `sglang_det`), split on line boundaries into parts of about 300 KB. Each part is valid JSONL.
- `raw/`: also the Luxi sanity check, the Luxi p20 records (fault and fresh process) and the vLLM/SGLang run metadata.
- `MANIFEST.sha256`: SHA-256 of every file in this folder, including the rebuilt files listed below.

**Rebuilding the large files.** The joined `raw/results_<engine>.jsonl` files, `data/filler_pool.json`, the `drift_results_<engine>.json` files and `drift_chart.png` are not committed because of their size. All of them rebuild byte for byte from the committed files, and the same steps reproduce the committed `summary.md`, `summary.json`, `examples.md` and `per_prompt.csv`:
- `raw/results_<engine>.jsonl`: join the parts, e.g. `cat raw/results_vllm_normal.part*.jsonl > raw/results_vllm_normal.jsonl`.
- `data/filler_pool.json` (2.8 MB): `python data/build_data.py TOKENIZER_DIR` (fixed seed). Run it on a copy of `data/`, because it also rewrites `data/prompts.json` with indentation.
- `drift_results_<engine>.json`, `drift_chart.png`, `summary.md`, `summary.json` and `examples.md`: `python scripts/analyze.py RAW_DIR OUT_DIR --tokenizer TOKENIZER_DIR`, where RAW_DIR holds the five joined `results_<engine>.jsonl` files, `results_luxi_c8_p20_separate_process.jsonl` and `luxi_sanity.json`.
- `per_prompt.csv`: `python scripts/per_prompt.py OUT_DIR`.

`bash scripts/rebuild.sh TOKENIZER_DIR` runs all of these and then checks every file against `MANIFEST.sha256`. TOKENIZER_DIR holds `tokenizer.json` from Qwen/Qwen2-7B-Instruct (SHA-256 `f7c9b2dba4a296b1aa76c16a34b8225c0c118978400d4bb66bff0902d702f5b8`). The rebuild used Python 3.13, tokenizers 0.23.2 and matplotlib 3.11.2. A different matplotlib version can draw the same PNG chart with different bytes.
