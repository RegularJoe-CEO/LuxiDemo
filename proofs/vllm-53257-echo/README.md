# vLLM #53257 echo test

Qwen2-7B-Instruct (fp16) on one H100 SXM 80GB. This reproduces the method of vLLM #53257, not its setup: it is not DeepSeek-V4-Flash, not NVFP4, not B300, and says nothing about those.

Temperature 0. vLLM used max_tokens 2000. One run of each vLLM cell. Three runs of each Luxi cell.
Every completion was the eight identifiers, one per line, prefixed with `ECHO: `. No second completion appeared, so there is no differing character.
Luxi rows are L-H: batch-harness equivalent, not a live server. The logits gate matched the 2026-09-26 hashes before those rows were measured.
Identical batch 32 has 512 echo rows per run because 32 times ceil(500/32) is 512. The requested count was 500. Filler batch 1 keeps both emits at that single row, 100 per run.

Engine | mode | conc | n | runs | DISTINCT per run | minority % per run
---|---|---:|---:|---:|---|---
vLLM 0.27.1 | standard | 1 | 100 | 1 | 1 | 0.00
vLLM 0.27.1 | standard | 8 | 400 | 1 | 1 | 0.00
vLLM 0.27.1 | standard | 16 | 400 | 1 | 1 | 0.00
vLLM 0.27.1 | standard | 32 | 500 | 1 | 1 | 0.00
vLLM 0.27.1 | batch-invariant | 1 | 100 | 1 | 1 | 0.00
vLLM 0.27.1 | batch-invariant | 8 | 400 | 1 | 1 | 0.00
vLLM 0.27.1 | batch-invariant | 16 | 400 | 1 | 1 | 0.00
vLLM 0.27.1 | batch-invariant | 32 | 500 | 1 | 1 | 0.00
Luxi c8 | L-H identical | 1 | 100 | 3 | 1,1,1 | 0.00,0.00,0.00
Luxi c8 | L-H identical | 8 | 400 | 3 | 1,1,1 | 0.00,0.00,0.00
Luxi c8 | L-H identical | 16 | 400 | 3 | 1,1,1 | 0.00,0.00,0.00
Luxi c8 | L-H identical | 32 | 512 | 3 | 1,1,1 | 0.00,0.00,0.00
Luxi c8 | L-H filler | 1 | 100 | 3 | 1,1,1 | 0.00,0.00,0.00
Luxi c8 | L-H filler | 8 | 400 | 3 | 1,1,1 | 0.00,0.00,0.00
Luxi c8 | L-H filler | 16 | 400 | 3 | 1,1,1 | 0.00,0.00,0.00
Luxi c8 | L-H filler | 32 | 500 | 3 | 1,1,1 | 0.00,0.00,0.00

Not in this folder: vLLM 0.25.1, a Luxi live server, and the stretch measurements (logprobs, higher concurrency, throughput, mixed lengths, Qwen2.5-7B-Instruct).
