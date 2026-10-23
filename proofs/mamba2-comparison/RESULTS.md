# Luxi vs NVIDIA Nemotron-H Mamba-2 hybrid: results

Measured October 22, 2026. Numbers only, taken from the run result files.

## Setup

- Hardware: one NVIDIA H100 80GB HBM3 (SXM, 700 W power limit).
- Luxi: Qwen2-7B-Instruct, fp16.
- Baseline: NVIDIA Nemotron-H-8B-Base-8K (Mamba-2 hybrid), bf16, on vLLM 0.25.1.
- Variant: the same Nemotron-H model on vLLM with a cuTile kernel for the Mamba-2 prefill (chunked scan). Decode is unchanged.
- Workloads: prompts of 1024 tokens at batch 1, 16 and 64, both prefill-only and end to end with 256 generated tokens. Each engine ran in alternating blocks, 2 runs x 2 blocks, so n = 4. Every table value is the median.
- Energy is from the GPU energy counter (NVML). Idle-subtracted values remove the measured idle draw of 73.2 W.

## Quality gate

| model | WikiText bpb (lower is better) | LAMBADA accuracy |
|---|---:|---:|
| Qwen2-7B-Instruct (Luxi) | 0.7415 | 0.708 |
| Nemotron-H-8B-Base-8K (vLLM) | 0.6734 | 0.744 |

The bpb ratio is 0.908 and the LAMBADA gap is -0.036. Verdict: FAIR. The two models are close enough in quality for a speed and energy comparison. The Mamba-2 model scores somewhat better on both measures.

cuTile variant check against vLLM-Mamba:
- bpb 0.67370, which is +0.043% from the reference.
- First-token agreement 0.979.
- Token agreement 0.892.
- PASS.

## Speed and energy

### End to end (1024-token prompt, 256 generated tokens)

| batch | engine | out tok/s | J/out tok raw | J/out tok idle-sub | median W |
|---:|---|---:|---:|---:|---:|
| 1 | Luxi (Qwen2-7B) | 169.6 | 2.94107 | 2.50948 | 500 |
| 1 | Mamba-2 hybrid, vLLM | 156.1 | 2.97702 | 2.50824 | 466 |
| 1 | Mamba-2 hybrid, cuTile prefill | 159.6 | 2.96709 | 2.50826 | 472 |
| 16 | Luxi (Qwen2-7B) | 2150.3 | 0.27428 | 0.24024 | 595 |
| 16 | Mamba-2 hybrid, vLLM | 1779.1 | 0.31445 | 0.27332 | 549 |
| 16 | Mamba-2 hybrid, cuTile prefill | 1802.8 | 0.31411 | 0.27350 | 554 |
| 64 | Luxi (Qwen2-7B) | 4933.4 | 0.13722 | 0.12238 | 678 |
| 64 | Mamba-2 hybrid, vLLM | 3632.6 | 0.17983 | 0.15964 | 631 |
| 64 | Mamba-2 hybrid, cuTile prefill | 3665.9 | 0.17860 | 0.15863 | 635 |

Luxi compared with vLLM-Mamba:

| batch | speed | energy per token, raw | energy per token, idle-sub |
|---:|---:|---:|---:|
| 1 | 1.09x | -1.2% | +0.0% |
| 16 | 1.21x | -12.8% | -12.1% |
| 64 | 1.36x | -23.7% | -23.3% |

### Prefill only (1024-token prompt)

| batch | engine | prompt tok/s | J/prompt tok raw | J/prompt tok idle-sub | median W |
|---:|---|---:|---:|---:|---:|
| 1 | Luxi (Qwen2-7B) | 37884.8 | 0.01833 | 0.01640 | 691 |
| 1 | Mamba-2 hybrid, vLLM | 32082.0 | 0.02079 | 0.01851 | 668 |
| 1 | Mamba-2 hybrid, cuTile prefill | 26956.0 | 0.02231 | 0.01959 | 602 |
| 16 | Luxi (Qwen2-7B) | 45281.6 | 0.01545 | 0.01383 | 698 |
| 16 | Mamba-2 hybrid, vLLM | 37933.0 | 0.01806 | 0.01613 | 688 |
| 16 | Mamba-2 hybrid, cuTile prefill | 29747.9 | 0.02043 | 0.01775 | 556 |
| 64 | Luxi (Qwen2-7B) | 46017.2 | 0.01519 | 0.01360 | 698 |
| 64 | Mamba-2 hybrid, vLLM | 39210.1 | 0.01750 | 0.01564 | 694 |
| 64 | Mamba-2 hybrid, cuTile prefill | 12223.6 | 0.03876 | 0.03277 | 425 |

Luxi compared with vLLM-Mamba:

| batch | speed | energy per token, raw | energy per token, idle-sub |
|---:|---:|---:|---:|
| 1 | 1.18x | -11.9% | -11.4% |
| 16 | 1.19x | -14.4% | -14.2% |
| 64 | 1.17x | -13.2% | -13.1% |

## Drift: does the answer change with batch size?

How the test worked:
- Each of 8 WikiText articles was cut to a 512-token prompt and decoded greedily for 32 tokens.
- The prompt was always row 0. At batch 16 and 64, the other rows held filler prompts.
- Each configuration ran twice, and outputs were compared bit for bit.

| engine | prompts | repeats identical | tokens identical across batch 1/16/64 |
|---|---:|---:|---:|
| Luxi (Qwen2-7B) | 8 | 24/24 | 8/8 (logit hashes also identical, 8/8) |
| Mamba-2 hybrid, vLLM | 8 | 24/24 | 2/8 |
| Mamba-2 hybrid, cuTile prefill | 8 | 24/24 | 3/8 |

All three engines repeat exactly at a fixed batch size. Only Luxi gives the same output at every batch size. The cuTile and vLLM-Mamba outputs matched each other on 18/48 runs.

### Output hashes (first 16 hex digits of SHA-256 over the 32 generated token ids, repeat 0)

Luxi and Nemotron-H use different tokenizers, so hashes can only be compared within an engine.

| prompt | WikiText article | Luxi B1/B16/B64 | vLLM-Mamba B1 / B16 / B64 | cuTile-Mamba B1 / B16 / B64 |
|---:|---|---|---|---|
| 0 | Little Gidding ( poem ) | adc6023b3b0257d5 (all three) | ebc742469311a128 / ebc742469311a128 / dc9c2be18d7807c5 | 5624d29c37c70d6c / ebc742469311a128 / eb0f59872e3eacdd |
| 1 | The Portage to San Cristobal of A.H. | b2ea7ceb8e303f01 (all three) | 903537b145b8673d / da44dcfc1957d5dc / da44dcfc1957d5dc | 903537b145b8673d / 903537b145b8673d / 903537b145b8673d |
| 2 | Temnospondyli | e38680f1e5f6b58e (all three) | 22c501669576ab33 / 22c501669576ab33 / 476d90d5cf723b01 | 476d90d5cf723b01 / 476d90d5cf723b01 / 476d90d5cf723b01 |
| 3 | Osbert de Bayeux | 150d377495b29e14 (all three) | 6059f947b875152a / 6059f947b875152a / 8b15cb3fc3cb583f | 6059f947b875152a / 6059f947b875152a / 6059f947b875152a |
| 4 | Dvorak technique | 7f06d7838c00546b (all three) | 028ea91824cfc2d4 / 028ea91824cfc2d4 / a3d7079b8ec404c9 | a50e8dba991cebc7 / a50e8dba991cebc7 / a3d7079b8ec404c9 |
| 5 | Ben Amos | d4d6201a8e4c48d4 (all three) | c86447e5788c3aa2 / c86447e5788c3aa2 / b9d5b16de6cbb89f | f05094bdf814398b / b9d5b16de6cbb89f / 0042adc9ba069a07 |
| 6 | Clayton Kershaw | 2cc646fe2ae38907 (all three) | d6798691fdbd11b0 / d6798691fdbd11b0 / d6798691fdbd11b0 | d6798691fdbd11b0 / 3aa92c2d1bd88d21 / d6798691fdbd11b0 |
| 7 | Josepha Petrick Kemarre | 3260bfbbf1a1eda3 (all three) | 915f9f85704a87df / 915f9f85704a87df / 915f9f85704a87df | 915f9f85704a87df / f22ca93edc5ea273 / f22ca93edc5ea273 |

Luxi logit hashes (FNV-64 of the final logits) were also identical at batch 1, 16 and 64 for all 8 prompts.

Inputs: the 8 drift prompts are the WikiText articles named in the table above.

## Caveats

- The models differ: Qwen2-7B (7B, Instruct) vs Nemotron-H-8B (8B, Base). The quality gate above covers this gap, but they are not the same model.
- Precision differs: Luxi runs fp16 and the Mamba-2 hybrid runs bf16.
- One GPU type (H100 SXM), one machine, short runs (n = 4 per cell).
- The cuTile variant changes only the Mamba-2 prefill. Decode is plain vLLM.
- At batch 1, end-to-end idle-subtracted energy per token is the same for all three engines (2.508 to 2.509 J).
