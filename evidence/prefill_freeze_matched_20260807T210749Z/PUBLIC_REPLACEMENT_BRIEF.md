# August 2026 short-prompt B16/B32 multi-run

**Pack:** `prefill_freeze_matched_20260807T210749Z`  
**GPU:** H100 80GB HBM3 · **Recipe:** flash + device-resident + FP16  
**Model class:** Qwen2-7B-Instruct · sequence length 128 · batch 16 / 32

This folder retains raw run files from an August 2026 short-prompt (S=128) B16/B32 multi-run (5×15 s sustains, NVML board energy, prefill positions = iters × batch × seq).

For measured Luxi results, including the comparison against vLLM 0.25.1, see [`BENCHMARKS.md`](../../BENCHMARKS.md).

## Files

| File | Contents |
|------|----------|
| `MULTI_RUN_LOCK_SLIM.json` | 5×15 s multi-run medians (raw) |
| `FREEZE_PACK.json` | Campaign summary (raw) |
| `LUXI_ENERGY.json` | NVML energy (raw) |

Related sweep from the same period: [`../prefill_accel_lock_20260807T233111Z/`](../prefill_accel_lock_20260807T233111Z/)

Contact: e@ewaller.com
