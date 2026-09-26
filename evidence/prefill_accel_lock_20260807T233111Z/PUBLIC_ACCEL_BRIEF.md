# August 2026 short-prompt prefill sweep

**Pack:** `prefill_accel_lock_20260807T233111Z`  
**GPU:** H100 80GB HBM3 · flash + device-resident + FP16  
**Model class:** Qwen2-7B-Instruct · sequence length 128

This folder retains raw run files from an August 2026 short-prompt (S=128) prefill campaign: a recipe sweep (quantization, TF32, fusion, dual-stream GEMM, pair fuse, CUDA graphs, batch ladder up to B72) and 5×15 s multi-run sustains with NVML board energy.

For measured Luxi results, including the comparison against vLLM 0.25.1, see [`BENCHMARKS.md`](../../BENCHMARKS.md).

## Engineering finding: activation-buffer allocation

`CudaWallerBuffers::ensure_capacity` used `next_power_of_two`. Crossing 2²⁵ floats (B72×128×3584) doubles every activation buffer, so B76 ran out of memory while the exact need was only slightly larger. The `grow_act_cap` change (need + 12.5% headroom, 256-float aligned) removes that cliff; status is in `PHASE3_GROW_ACT_CAP_NOTE.md`.

## Files

| File | Contents |
|------|----------|
| `sweep_all.json` | Recipe sweep (raw) |
| `B_multirun_*.json` | Per-recipe 5×15 s multi-run (raw) |
| `CHAMPION_LOCK.json` · `FREEZE_PACK.json` · `B72_DUAL_GEMM_RECEIPT.json` | Campaign summary and receipt files (raw) |
| `PHASE3_GROW_ACT_CAP_NOTE.md` | Engineering note on activation-buffer allocation |

Contact: e@ewaller.com
