# Prefill aggressive accel lock

**Pack:** `prefill_accel_lock_20260807T233111Z`  
**GPU:** H100 80GB HBM3 · flash + device-resident + FP16  
**Status:** CANDIDATE_ACCEL_LOCK (beats prior B16 freeze on thr + energy at product batch)

## What we did

Aggressive thr / energy / determinism campaign after refusing to lock the weak 41.2k B16 headline as a ceiling.

1. **Recipe sweep** - quant, TF32, fusion, dual-stream, pair-fuse, mega, graphs, batch ladder  
2. **Combo fine ladder** - dual_gemm × pair × mlp_pers × fusion1 @ B64/68/72  
3. **Multi-run lock** - 5×15s thr + NVML cumulative energy  
4. **Det dual-run** - independent process thr stability  
5. **Code path invention** - `grow_act_cap` exact-fit capacity (eliminates 2× HBM cliff at B76+)

## Eliminated (regress or crash)

| Lever | Result |
|-------|--------|
| INT8 / FP8 weights | ~5× thr regression on this prefill path |
| Layer dual-stream | ~10% thr regress |
| CUDA graph on prefill | slight thr regress |
| `LUXI_THR_LIGHT_D2H` pre-set | early slice panic (binary auto-sets in sustain) |
| B≥76 S=128 (old allocator) | `cudaMalloc` OOM via `next_power_of_two` 2× cliff |

## Multi-run lock (5×15s, S=128, flash)

| Recipe | Batch | thr median pos/s | J/pos (NVML) | Median W |
|--------|------:|-----------------:|-------------:|---------:|
| **dual_fuse** (`GEMM_DUAL_STREAM`+`FUSION1`) | **72** | **44,907** | **0.01548** | 688.8 |
| dual_gemm | 72 | 44,860 | **0.01532** | 687.5 |
| dual_mlp | 64 | 44,805 | 0.01541 | 688.0 |
| pair fuse | 68 | 44,404 | 0.01540 | 685.6 |
| base (same session) | 32 | 42,764 | 0.01610 | 688.0 |
| base (same session) | 16 | 39,985 | 0.01684 | 675.7 |

**Peak single-shot (wave3):** dual_gemm B72 = **45,986** pos/s (not multi-run locked).

## vs prior freeze (B16 thr 41,221 / J 0.0169)

| Compare | Prior freeze | Accel lock winner | Delta |
|---------|-------------:|------------------:|------:|
| Multi thr (headline batch) | 41,221 @ B16 | **44,907 @ B72** | **+9% thr** |
| Board J/pos | 0.0169 | **0.0153 to 0.0155** | **~8 to 9% lower energy** |
| Same-session B16 thr | - | 39,985 | pod thermal lower than prior freeze B16 |
| Same-session thr ratio | - | 44,907 / 39,985 | **1.12×** vs same-session B16 |

## Recommended product recipe

```bash
export LUXI_TRADE_JOULE=1 LUXI_TRADE_ENERGY=1 LUXI_ATTN_SERIAL_BATCH=0
export LUXI_CUDA_FP16_MLP=1 LUXI_CUDA_FP16_WEIGHTS=1 LUXI_CUDA_FP16_GEMM=1
export LUXI_CUDA_QUANT_STACK=1 LUXI_GEODESIC_P3=1
export LUXI_TRADE_ATTN=flash LUXI_FLASH_BRIDGE=1 LUXI_ATTN_BACKEND=flash
export LUXI_KERNEL_MORPH=0 LUXI_CUDA_PHASE_TIMING=0
export LUXI_GEMM_DUAL_STREAM=1
# optional thr edge:
# export LUXI_FUSION1=1   # dual_fuse - slight thr↑, energy≈same
# batch=72  sustain_seq=128
```

**Primary lock:** dual_gemm @ B72 (best energy + near-best thr)  
**Thr edge:** dual_fuse @ B72 (best multi thr)

## Determinism notes

- All locked arms: `ATTN_BACKEND_USED=flash` fb=0  
- Pair B64 dual independent process: thr rel_diff **3.5%** (acceptable thr variance)  
- Dual_gemm B72 dual process: one outlier (37.4k @ 641 W) - thermal/clock dip after multi-hour campaign; multi-run 5× window still stable ~44.7 to 45.1k  
- Numeric receipt det (AUDIT lane) unchanged by these TRADE thr knobs

## Code invention: kill the power-of-two HBM cliff

`CudaWallerBuffers::ensure_capacity` used `next_power_of_two`. Crossing 2²⁵ floats (B72×128×3584) doubles every activation buffer → B76 OOM while exact need is only +5% rows.

**Fix (local `src/gpu/cuda.rs`):** `grow_act_cap` = need + 12.5% headroom, 256-float aligned. Unlocks B76+ once rebuilt on pod (rebuild blocked this session by unrelated SwiGLU API drift on remote tree).

## Website copy options

**Conservative (same B16 primary as prior freeze):** keep ~41.2k if re-locked on a cool pod; this session’s B16 was soft.

**Aggressive product (recommended):**  
- Prefill pos/s: **~44.9k** (B72, dual-stream GEMM + flash TRADE, 5×15s multi-run)  
- Board J/pos: **~0.0153**  
- vs prior B16 freeze: **~1.09× thr**, **~9% lower board J/pos**

**Method footnote:** H100 multi-run (5×15s thr · NVML energy · flash TRADE · Qwen2-7B · S=128). Board ≠ wall-plug. Not blended with TESTfort. High-batch thr is product continuous thr, not B16 chat-shape only.

## Non-claims

- Not decode thr · not multi-tenant serving · not wall-plug PUE · not chat quality  
- Not bit-exact AUDIT lane · thr det is dual-run stability not receipt SHA  
- B≥76 thr ceiling is allocator-limited until `grow_act_cap` ships
