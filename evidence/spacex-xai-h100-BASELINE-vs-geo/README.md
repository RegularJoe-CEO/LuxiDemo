# SpaceX / xAI — Baseline comparison pack (H100 NVL, 2026-07-11)

**Public:** https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/spacex-xai-h100-BASELINE-vs-geo

Same hardware, same width (h=768, heads=12, mlp=3072), seq=1024 — **single-layer** energy microbench via `benchmarks/benchmark_joules.py`.

## Head-to-head (layer forward, pynvml)

| Path | Median ms | Median board W | J/token |
|------|----------:|---------------:|--------:|
| **[A] Geodesic TRADE fused layer** | 5.304 | 72.5 | **3.753e-04** |
| **[B] PyTorch unfused full layer (fp16)** | 0.414 | 98.5 | **3.978e-05** |
| **[A]/[B] J/token ratio** | — | — | **0.106** |

**Interpretation:** ratio **0.106 < 1** ⇒ at this **single-layer** microbench, unfused PyTorch FP16 uses **less** board J/token than the current geodesic fused layer path. We publish that honestly.

Attention-only (not full layer): Flash SDPA 0.047 ms · math SDPA 0.155 ms.

## Where TRADE shows wins (different regime — same run)

| Wedge | Result |
|-------|--------|
| **[F] Full-layer morph @ seq=8192, h=64** | **2.09×** energy ratio (no-morph / morph) |
| **[E] Mesh void attn @ seq=8192** (edge-del ~95%) | **7.54×** vs Waller attn kernel |

## GPU util (nvidia-smi dmon @ 1 Hz during bench)

| Metric | Value |
|--------|------:|
| Power range (dmon) | 63–108 W |
| SM util max % (1 Hz) | 1.0 |
| Note | Short kernels under-sample at 1 Hz; prefer pynvml power samples in JSON |

## How this relates to the 12L TRADE pack

| Pack | What it measures |
|------|------------------|
| `spacex-xai-h100-TRADE-cuda` | **12-layer device-resident stack** prefill + decode sustain → ~0.0131 / 0.0077 J/tok, GPU ~170–177 W |
| **This pack** | **Single-layer** geodesic vs PyTorch/Flash head-to-head + morph wedges |

They answer different diligence questions. Both are needed.

## Exact command

```bash
python3 benchmarks/benchmark_joules.py \
  --seq 1024 --hidden 768 --heads 12 --mlp 3072 --iters 40 \
  --out baseline_vs_geo_1024.json
# concurrent:
nvidia-smi dmon -s pucm -d 1 -c 600 -o TD > baseline_dmon.csv
```

## Files

- BASELINE_SUMMARY.json
- baseline_vs_geo_1024.json (full RESULT_JSON)
- baseline_vs_geo_1024.log
- baseline_dmon.csv

Contact: Eric Waller · e@ewaller.com
