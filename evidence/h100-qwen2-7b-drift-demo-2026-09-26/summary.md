# Drift demo summary (Qwen2-7B-Instruct fp16, greedy, H100)

Each row compares the target prompt's output under a load condition with the SAME engine's output for the SAME prompt alone (batch conditions vs offline B=1; server conditions vs server alone). Tokens = what a client receives (up to the first end-of-turn token).

## Per engine (all non-reference load conditions; the repeat-alone controls are excluded here)

| engine | prompt-conditions | identical tokens | identical logprob hash | first divergence median / max (token index) | visible text changed |
|---|---|---|---|---|---|
| Luxi c8 (batch-invariant engine) | 80 | 80/80 (100.0%) | 80/80 (100.0%) | none | 0/80 |
| vLLM 0.25.1 default | 140 | 128/140 (91.4%) | 0/140 (0.0%) | 33 / 47 | 12/140 |
| SGLang 0.5.19 default | 140 | 129/140 (92.1%) | 1/140 (0.7%) | 153 / 162 | 11/140 |
| vLLM 0.25.1 VLLM_BATCH_INVARIANT=1 | 140 | 140/140 (100.0%) | 140/140 (100.0%) | none | 0/140 |
| SGLang 0.5.19 deterministic mode | 140 | 140/140 (100.0%) | 140/140 (100.0%) | none | 0/140 |

## Per engine and condition

| engine | condition | n | identical tokens | identical logprob hash | first div. median / max | text changed | max abs logprob diff before divergence |
|---|---|---|---|---|---|---|---|
| Luxi c8 (batch-invariant engine) | b16_f1 | 20 | 100.0% | 100.0% | none | 0 | 0 |
| Luxi c8 (batch-invariant engine) | b16_f2 | 20 | 100.0% | 100.0% | none | 0 | 0 |
| Luxi c8 (batch-invariant engine) | b1_rep | 20 | 100.0% | 100.0% | none | 0 | 0 |
| Luxi c8 (batch-invariant engine) | b64_f1 | 20 | 100.0% | 100.0% | none | 0 | 0 |
| Luxi c8 (batch-invariant engine) | b64_f2 | 20 | 100.0% | 100.0% | none | 0 | 0 |
| vLLM 0.25.1 default | b16_f1 | 20 | 80.0% | 0.0% | 37.5 / 47 | 4 | 0.0177 |
| vLLM 0.25.1 default | b16_f2 | 20 | 95.0% | 0.0% | 33 / 33 | 1 | 0.0192 |
| vLLM 0.25.1 default | b16_mix | 20 | 95.0% | 0.0% | 33 / 33 | 1 | 0.0192 |
| vLLM 0.25.1 default | b1_rep | 20 | 100.0% | 100.0% | none | 0 | 0 |
| vLLM 0.25.1 default | b64_f1 | 20 | 90.0% | 0.0% | 21.5 / 40 | 2 | 0.0157 |
| vLLM 0.25.1 default | b64_f2 | 20 | 95.0% | 0.0% | 40 / 40 | 1 | 0.0174 |
| vLLM 0.25.1 default | srv_b1_rep | 20 | 100.0% | 100.0% | none | 0 | 0 |
| vLLM 0.25.1 default | srv_b1_vs_offline_b1 | 20 | 100.0% | 0.0% | none | 0 | 0.00945 |
| vLLM 0.25.1 default | srv_stagA | 20 | 90.0% | 0.0% | 18 / 33 | 2 | 0.0159 |
| vLLM 0.25.1 default | srv_stagB | 20 | 95.0% | 0.0% | 40 / 40 | 1 | 0.0155 |
| SGLang 0.5.19 default | b16_f1 | 20 | 90.0% | 0.0% | 157.5 / 162 | 2 | 0.0163 |
| SGLang 0.5.19 default | b16_f2 | 20 | 90.0% | 0.0% | 157.5 / 162 | 2 | 0.0163 |
| SGLang 0.5.19 default | b16_mix | 20 | 90.0% | 0.0% | 157.5 / 162 | 2 | 0.0163 |
| SGLang 0.5.19 default | b1_rep | 20 | 100.0% | 100.0% | none | 0 | 0 |
| SGLang 0.5.19 default | b64_f1 | 20 | 95.0% | 0.0% | 40 / 40 | 1 | 0.0208 |
| SGLang 0.5.19 default | b64_f2 | 20 | 95.0% | 0.0% | 40 / 40 | 1 | 0.0208 |
| SGLang 0.5.19 default | srv_b1_rep | 20 | 100.0% | 100.0% | none | 0 | 0 |
| SGLang 0.5.19 default | srv_b1_vs_offline_b1 | 20 | 100.0% | 100.0% | none | 0 | 0 |
| SGLang 0.5.19 default | srv_stagA | 20 | 90.0% | 0.0% | 100 / 153 | 2 | 0.0156 |
| SGLang 0.5.19 default | srv_stagB | 20 | 95.0% | 5.0% | 140 / 140 | 1 | 0.0172 |
| vLLM 0.25.1 VLLM_BATCH_INVARIANT=1 | b16_f1 | 20 | 100.0% | 100.0% | none | 0 | 0 |
| vLLM 0.25.1 VLLM_BATCH_INVARIANT=1 | b16_f2 | 20 | 100.0% | 100.0% | none | 0 | 0 |
| vLLM 0.25.1 VLLM_BATCH_INVARIANT=1 | b16_mix | 20 | 100.0% | 100.0% | none | 0 | 0 |
| vLLM 0.25.1 VLLM_BATCH_INVARIANT=1 | b1_rep | 20 | 100.0% | 100.0% | none | 0 | 0 |
| vLLM 0.25.1 VLLM_BATCH_INVARIANT=1 | b64_f1 | 20 | 100.0% | 100.0% | none | 0 | 0 |
| vLLM 0.25.1 VLLM_BATCH_INVARIANT=1 | b64_f2 | 20 | 100.0% | 100.0% | none | 0 | 0 |
| vLLM 0.25.1 VLLM_BATCH_INVARIANT=1 | srv_b1_rep | 20 | 100.0% | 100.0% | none | 0 | 0 |
| vLLM 0.25.1 VLLM_BATCH_INVARIANT=1 | srv_b1_vs_offline_b1 | 20 | 100.0% | 100.0% | none | 0 | 0 |
| vLLM 0.25.1 VLLM_BATCH_INVARIANT=1 | srv_stagA | 20 | 100.0% | 100.0% | none | 0 | 0 |
| vLLM 0.25.1 VLLM_BATCH_INVARIANT=1 | srv_stagB | 20 | 100.0% | 100.0% | none | 0 | 0 |
| SGLang 0.5.19 deterministic mode | b16_f1 | 20 | 100.0% | 100.0% | none | 0 | 0 |
| SGLang 0.5.19 deterministic mode | b16_f2 | 20 | 100.0% | 100.0% | none | 0 | 0 |
| SGLang 0.5.19 deterministic mode | b16_mix | 20 | 100.0% | 100.0% | none | 0 | 0 |
| SGLang 0.5.19 deterministic mode | b1_rep | 20 | 100.0% | 100.0% | none | 0 | 0 |
| SGLang 0.5.19 deterministic mode | b64_f1 | 20 | 100.0% | 100.0% | none | 0 | 0 |
| SGLang 0.5.19 deterministic mode | b64_f2 | 20 | 100.0% | 100.0% | none | 0 | 0 |
| SGLang 0.5.19 deterministic mode | srv_b1_rep | 20 | 100.0% | 100.0% | none | 0 | 0 |
| SGLang 0.5.19 deterministic mode | srv_b1_vs_offline_b1 | 20 | 100.0% | 100.0% | none | 0 | 0 |
| SGLang 0.5.19 deterministic mode | srv_stagA | 20 | 100.0% | 100.0% | none | 0 | 0 |
| SGLang 0.5.19 deterministic mode | srv_stagB | 20 | 100.0% | 100.0% | none | 0 | 0 |

## Notes

- Luxi logprobs are sampled at checkpoint positions 0,1,3,7,15,31,63,127,255 (its benchmarked batch API returns final-step logits only; each checkpoint is a separate generate call of the same batch). At those positions the full 152,064-entry logits vector is also hashed. vLLM/SGLang logprobs cover every generated position.
- Luxi has no concurrent-arrival server for the benchmarked engine, so it has batch conditions only (no srv_* rows).
- Luxi's batch API needs equal-length rows, so equal-length batch conditions use identical token rows for all engines; the mixed-length batch (b16_mix) is vLLM/SGLang only.
- For the ~1.7k-token prompt (p20) the '64' conditions use 32 rows (Luxi's benchmarked prefill buffer holds 65,536 tokens); this applies to every engine.
- Staggered-arrival timing is not reproducible run to run by design; those rows show what a server can do under live traffic, not a fixed schedule.
- Luxi full-vocab logits bit-identical to its B=1 reference at every checkpoint: 100/100 prompt-conditions.
- Luxi sanity lines reproduce the 2026-09-26 race logits hashes bit for bit: True.
