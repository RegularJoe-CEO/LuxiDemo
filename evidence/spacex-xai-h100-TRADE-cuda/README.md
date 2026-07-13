#TRADE CUDA H100 energy pack (2026-07-11)

**Public:** https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/spacex-xai-h100-TRADE-cuda

## Headline (pynvml while CUDA is loaded)

| Stage | Median board W | J/token (NVML) | Tokens in sustain |
|-------|---------------:|---------------:|------------------:|
| Prefill 12L @1024 device-resident | **177.0 W** | **0.0131** | 135168 |
| Decode gen_resident_batch (2048 + 64) | **169.0 W** | **0.0077** | 218624 |
| Job estimate 1024 prefill + 64 decode | — | — | **13.93 J** |

| Comparison | Value |
|------------|------:|
| Phase-1 city-block prefill / decode | 0.0125 / 0.0075 J/tok |
| Phase-1 full-stack job | 13.27 J |
| **This TRADE re-measure job** | **13.93 J** |
| CPU AUDIT serve (prior pack) median W | ~63.7 W (GPU idle) |
| **TRADE prefill median W** | **177.0 W** (~2.7× idle) |

## Exact commands

```bash
export PATH=$HOME/.cargo/bin:/usr/local/cuda/bin:$PATH
export LUXI_KERNEL_MORPH=0 LUXI_CUDA_QUANT_STACK=1 LUXI_CUDA_GRAPH=1
cargo build --release --features cuda \
  --example cuda_stack_device_resident_audit --example cuda_decode_kv_bench

python3 scripts/trade_cuda_joules_capture.py \
  --prefill-sec 10 --decode-sec 10 --out-dir trade_cuda_joules
```

```bash
LUXI_STACK_SUSTAIN_SEC=10 LUXI_STACK_JOULES_ONLY=1 \
  ./target/release/examples/cuda_stack_device_resident_audit 12 1024 768 12 3072 3

LUXI_DECODE_SUSTAIN=gen_resident_batch LUXI_SUSTAIN_SEC=10 \
  ./target/release/examples/cuda_decode_kv_bench 2048 64 768 12 3072 3
```

## Files

| File | Role |
|------|------|
| TRADE_SUMMARY.json | Numbers |
| power_trace_prefill_12L_1024.csv | Power during prefill CUDA |
| power_trace_decode_resident_batch.csv | Power during decode CUDA |
| PHASE1_CITY_BLOCK_EXCERPT.md | Prior Phase-1 tables |
| run.log | Capture stdout |

## Honest bounds

- GPT-2-small **width** residual stack (12/768/12/3072), not full HF GPT-2 124M.
- Power = single GPU board power.draw (pynvml), not whole-pod AC wall.
- J/token = median_W × wall_s / tokens in sustain loop.
- CPU AUDIT pack (GPU idle): `evidence/spacex-xai-h100-2026-07-11/`

Contact: Eric Waller · e@ewaller.com
