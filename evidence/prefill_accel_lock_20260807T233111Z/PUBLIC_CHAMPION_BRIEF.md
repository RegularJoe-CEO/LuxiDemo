# Prefill champion — dual_gemm @ B72

**Status:** AUTHORITATIVE for Luxi **absolute** prefill thr + board energy  
**Pack:** `prefill_accel_lock_20260807T233111Z`  
**Champion:** `LUXI_GEMM_DUAL_STREAM=1` · flash + device-resident + FP16 · **B72** · S=128  

## Two facts (never blend)

### Fact A — Best measured Luxi operating point (this pack)

| Metric | Value |
|--------|------:|
| Multi-run thr median (5×15s) | **44,860 pos/s** |
| Board J/position (NVML) | **0.01532** |
| Median W under load | **687.5** |
| Thr runs | 45,116 · 44,704 · 44,860 · 44,921 · 44,295 |
| Backend | flash (fallback=0) |

**vs prior Luxi B16 freeze (41,221 / 0.0169):** ~**1.09× thr**, ~**9% lower J/pos**.

### Fact B — Matched vLLM comparison (separate pack)

Authority: `prefill_freeze_matched_20260807T210749Z` (B16/B32 only).

| Batch | Luxi thr | vLLM thr | thr ratio | Luxi J savings |
|------:|---------:|---------:|----------:|---------------:|
| 16 | 41,221 | 35,043 | **1.18×** | **~12% lower J/pos** |
| 32 | 43,464 | 36,557 | **1.19×** | **~14% lower J/pos** |

**`b72_vllm_matched = false`.** Do **not** say the B72 absolute result is 1.18× vLLM until a matched B72 vLLM arm is measured.

## Why dual_gemm (not dual_fuse)

| | dual_gemm B72 | dual_fuse B72 |
|--|--------------:|--------------:|
| Multi thr | **44,860** | 44,907 (+47, ~0.1%) |
| J/pos | **0.01532** | 0.01548 (worse) |
| Stability | five-run tight | outlier 36,783 |
| Product | **champion** | demoted |

Dual_gemm is simpler, more efficient, and more defensible.

## Physics (copy-safe)

High-batch prefill gives dual GEMM streams enough work to overlap and amortize weight movement. Dual-stream failing at M=1 decode is a different execution regime — not a contradiction.

## Receipt equality (publication check)

**Status: PASS** — `B72_DUAL_GEMM_RECEIPT.json`

- Contract: Door-B stack fingerprint under dual_gemm env, **independent-process hash equality**
- B1 / B16 / B32: `hash_stable=true`, `max_abs_vs_run0=0`, same hashes across processes
- B72 thr dual-run: flash fb=0, thr relative spread ~0.6% (performance stability; thr loop does not emit hashes)
- **Not** Door-A AUDIT bit-exact hero — declared Door-B TRADE train-lane contract

## Non-claims

- Not decode thr · not multi-tenant serving · not wall-plug  
- Not single-shot 45,986 as product thr  
- Not B72 vs vLLM until matched arm exists  
- Not AUDIT bit-exact on B72 thr loop (Door-B fingerprint only)

## Website hero (suggested)

- Absolute: **~44.9k** pos/s · **0.0153** board J/pos · B72 dual_gemm multi-run  
- Matched H2H (B16): **~1.18× thr** · **~12% lower J** vs vLLM  
- Method: H100 · flash TRADE · Qwen2-7B · S=128 · board NVML. Absolute batch ≠ H2H batch.
