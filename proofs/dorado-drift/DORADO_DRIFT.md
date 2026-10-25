# LuxiDemo: Oxford Nanopore Dorado basecalling that gives the same answer every run

Every figure below is copied from a run log listed under "Sources".

**Stock Dorado on an NVIDIA A100 wrote a different output file on every one of 12 runs. Luxi wrote byte-identical output on every run. On Dorado's fast model, Luxi is also faster on GPU kernel time, like for like. On the high-accuracy (hac) model, Luxi is exact but still slower on GPU kernel time.**

Dorado is Oxford Nanopore Technologies' open-source basecaller. It turns the raw electrical signal from a nanopore sequencer into DNA letters (A, C, G, T) with quality scores, using a neural network on the GPU followed by a CRF beam-search decode. Luxi runs the same Dorado v5.0.0 models with exact integer arithmetic (int8 tensor-core and dp4a products, integer accumulation), so the sum does not depend on thread order, and the result does not depend on batch size or scheduling.

## Results at a glance

| | Stock Dorado 2.1.2 (GPU) | Luxi |
|---|---|---|
| Output file md5 across 12 A100 runs | Different on all 12 | Identical on every run (15 of 15 in the decode series) |
| Fast model, batch 864, GPU kernel time incl. decode (mean of 3) | 295.7 ms | **246.1 ms** |
| Fast model, batch 512, GPU kernel time incl. decode (mean of 3) | 351.4 ms | **323.7 ms** |
| Hac model, batch 512, GPU kernel time incl. decode (final, mean of 3) | **838.4 ms** | 943.5 ms |
| Hac model, first to last kernel span (mean of 3) | 1043.4 ms | **1001.1 ms** |
| Hac base edit distance vs Dorado's own CPU output | 17.89% (Dorado GPU) | **7.82%** |

## Drift finding

- Dorado 2.1.2, models v5.0.0, input `sub40.pod5` (40 reads), NVIDIA A100.
- Dorado's FASTQ md5 was different on all 12 runs (REPORT.md, Line H). The fast-model fairness series alone showed a different md5 on each of its 6 runs.
- Changing batch size changes Dorado's basecalls, not just metadata. On the A100, fast at batch 64 vs auto: 26,052 of 561,279 bases differ (4.64%). Hac at batch 64, 256, 512 and 1024 vs auto: 16,398 of 356,818 bases differ (4.60%), with only 1 of 41 output reads identical (Dorado split one of the 40 input reads into two, giving 41 output reads).
- Same reads, A100 vs H100: fast 5.63% of bases differ, hac 4.60%. GPU vs CPU: fast about 13.6% to 13.9%, hac about 16.5%.

## Luxi hashes (FASTQ md5, first 8 hex)

| Model / config | Hash | Where it matched |
|---|---|---|
| Hac, final int8 config (QA=8, QW=8 linear, QCW=16 convs), timed in this page | `df7cd39a` | A100 GPU on every run, and the CPU reference |
| Fast, 1 pass int8, GPU decode | `e690b44a` (scorehash `d94df072`) | b864 and b512, matches CPU reference |
| Fast, 24-bit int8, GPU decode | `eb47c496` (scorehash `eec7e710`) | b864 and b512, matches CPU reference |

A CPU reference implementation of the same integer math produces the same hash, so each GPU result is checked against an independent run, not only against itself.

## Speed, fast model (like for like)

CUPTI all-kernel trace, A100, same session, 3 runs each. Both totals include the decode: Luxi's `k_decode` and Dorado's 4 decode kernels.

| Batch | Dorado total / decode / encoder | Luxi total / decode / encoder |
|---|---|---|
| 864 | 295.7 / 107.2 / 188.5 ms | **246.1** / 84.6 / 161.5 ms |
| 512 | 351.4 / 163.5 / 187.9 ms | **323.7** / 106.9 / 216.8 ms |

At batch 864, Luxi is faster on both the encoder and the decode. At batch 512, the whole win comes from the decode, and Luxi's encoder is 28.9 ms slower. Luxi's first to last kernel span is 284.5 vs 541.2 ms (b864) and 373.0 vs 562.8 ms (b512). Every timed Luxi run wrote `e690b44a`; every Dorado run wrote a different md5.

## Speed, high-accuracy model (honest current state)

Final configuration, 08:40 CDT Oct 25 2026, same session, hac batch 512, 3 runs each, hash `df7cd39a` on all 3 (matches the CPU reference):

| Run | Dorado GPU kernel total | Luxi GPU kernel total |
|---|---|---|
| 1 | 857.7 ms | 945.4 ms |
| 2 | 827.5 ms | 952.7 ms |
| 3 | 830.0 ms | 932.5 ms |
| Mean | **838.4 ms** | 943.5 ms |

Luxi is about 12.5% slower on GPU kernel time on hac. Split (mean of 3): Dorado decode 209.4 / encoder 629.0 ms; Luxi decode 238.2 / encoder 705.3 ms. First to last kernel span: Dorado 1043.4 ms, Luxi **1001.1 ms**.


## Accuracy

Edit distance over 40 reads against Dorado's own CPU output.

| Model | Dorado GPU vs Dorado CPU | Luxi vs Dorado CPU |
|---|---|---|
| Hac (`df7cd39a`, final) | 17.89% | **7.82%** |
| Fast | 13.65% | **9.15%** (1 pass), **2.03%** (24-bit) |

Luxi tracks Dorado's CPU answer more closely than Dorado's GPU does. The `sub40` reads are low quality (median qscore 4.8), so these numbers compare implementations, not sequencing accuracy.

## Method

- **Hardware:** one NVIDIA A100 (RunPod), driver 580.
- **Metric:** CUPTI all-kernel trace injected with `CUDA_INJECTION64_PATH`. The total is the sum of every GPU kernel's duration for the whole basecall, both tools measured the same way, in the same session.
- **Like for like:** the decode (scores to DNA letters) is counted on both sides. Batch sizes match: Luxi was run with `LUXI_MINB=512` for the b512 rows, no padding.
- **Exactness:** each config's FASTQ md5 is compared with a CPU reference of the same integer math.
- **Runs serialized:** a GPU lock ensured only one job used the GPU at a time.

## Reproduce the stock drift

All inputs are public. Dorado 2.1.2 (linux-x64), models `dna_r10.4.1_e8.2_400bps_fast@v5.0.0` and `..._hac@v5.0.0`, any recent NVIDIA GPU:

```bash
for i in 1 2 3; do
  dorado basecaller hac@v5.0.0 sub40.pod5 --emit-fastq > run$i.fq
done
md5sum run*.fq
dorado basecaller hac@v5.0.0 sub40.pod5 --emit-fastq -b 64 > b64.fq
# compare read sequences by read id (edit distance) to see real basecall changes
```

## Closed engine, open evidence

We publish the inputs, the output hashes, the measurement method and per-run timings. The Luxi kernel source is proprietary and is not in this repo.

## Non-claims

- Hac: Luxi is not yet faster than Dorado on GPU kernel time (943.5 vs 838.4 ms, about 12.5% slower).
- One GPU model (A100), one 40-read input, one Dorado version.
- Results are GPU kernel time measured with CUPTI. Process wall time is not compared on this page.

## Sources

- Run logs for every figure (per-run CUPTI totals, hashes, wall times) are kept by LuxiEdge and available on request.
