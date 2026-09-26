# Prefill freeze - matched rerun (replacement pack)

**Status:** AUTHORITATIVE for website replacement of withdrawn prefill headlines  
**Pack:** `prefill_freeze_matched_20260807T210749Z`  
**Timestamp:** 2026-08-07T21:30:38.592378+00:00  
**GPU:** H100 80GB HBM3 · **Recipe:** flash + device-resident + FP16  

## Rules
- Internal multi-run was run **on this pod session**.
- **Not blended** with TESTfort v99 independent numbers.
- Token def: **prefill positions = iters × batch × seq** (S=128).
- Energy: **board NVML joules**, not wall-plug.

## Luxi multi-run thr (5×15s)

| Batch | Median pos/s | Min to Max | Stdev | Backend |
|------:|-------------:|--------:|------:|:--------|
| 16 | **41,221** | 40,805 to 41,389 | 247 | flash |
| 32 | **43,464** | 43,119 to 43,721 | 241 | flash |

## Luxi board energy (NVML, 3×15s window)

| Batch | J/position | Median W under load |
|------:|-----------:|--------------------:|
| 16 | **0.0169** | 681.3 |
| 32 | **0.0158** | 688.8 |

## Comparison against vLLM

For the current Qwen2-7B comparison against vLLM 0.25.1 on H100, see
[`../../BENCHMARKS.md`](../../BENCHMARKS.md).

## Website replacement copy (delete the “withdrawn / conflicted” paragraph)

**Suggested hero metrics (B16 primary):**
- Prefill pos/s: **~41.2k**
- Board J/pos: **0.0169**

**Method footnote:** H100 multi-run lock (5×15s thr · NVML energy · flash TRADE path · Qwen2-7B · S=128 · B16 primary). Board ≠ wall-plug. Not blended with prior independent TESTfort pack.

## Non-claims
- Not decode thr · not multi-tenant serving crown · not wall-plug PUE · not chat quality.

## Absolute peak moved (2026-08-07)

This pack remains authoritative for the **B16/B32 multi-run freeze** only.

Luxi **absolute** best prefill thr+J is now **dual_gemm @ B72** (~44,860 pos/s · 0.01532 board J/pos) in:

[`../prefill_accel_lock_20260807T233111Z/`](../prefill_accel_lock_20260807T233111Z/) · `CHAMPION_LOCK.json`
