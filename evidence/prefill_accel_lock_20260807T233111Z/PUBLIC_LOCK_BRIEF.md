# August 2026 short-prompt prefill sweep

**Pack:** `prefill_accel_lock_20260807T233111Z`  
**GPU:** H100 80GB HBM3 · flash + device-resident + FP16  
**Model class:** Qwen2-7B-Instruct · sequence length 128

This folder retains raw run files from an August 2026 short-prompt (S=128) prefill campaign: a recipe sweep (quantization, TF32, fusion, dual-stream GEMM, pair fuse, CUDA graphs, batch ladder up to B72) and 5×15 s multi-run sustains with NVML board energy.

For measured Luxi results, including the comparison against vLLM 0.25.1, see [`BENCHMARKS.md`](../../BENCHMARKS.md).

## Files

| File | Contents |
|------|----------|
| `sweep_all.json` | Recipe sweep (raw) |
| `B_multirun_*.json` | Per-recipe 5×15 s multi-run (raw) |
| `CHAMPION_LOCK.json` · `FREEZE_PACK.json` · `B72_DUAL_GEMM_RECEIPT.json` | Campaign summary and receipt files (raw) |
| `PHASE3_GROW_ACT_CAP_NOTE.md` | Engineering note on activation-buffer allocation |

Contact: e@ewaller.com
