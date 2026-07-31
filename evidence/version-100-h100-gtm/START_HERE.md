# version-100 H100 GTM pack (public)

**Status:** Authoritative multi-run thr + board energy + determinism lock  
**Hardware:** NVIDIA H100 80GB HBM3  
**Model class:** Qwen2-7B-Instruct (FP16)  
**Workload:** Prefill-heavy · sequence 128 · batch 16 / 32  

## Headline

| Batch | Thr median (pos/s) | J/pos median | Det |
|------:|-------------------:|-------------:|----:|
| 16 | ~39,865 | ~0.0168 | 1.0 |
| 32 | ~42,967 | ~0.0160 | 1.0 |

**vs vLLM (matched prefill, same day):** thr **~1.17–1.18×** · board J/pos **~10–14% lower**

## Files

| File | Role |
|------|------|
| `MULTI_RUN_LOCK_SLIM.json` | Authoritative 5×15s medians |
| `MULTI_RUN_LOCK.json` | Full multi-run detail |
| `PUBLIC_GTM_ONE_PAGER.md` | Buyer one-pager |
| `PUBLIC_H2H_PREFILL_ENERGY_BRIEF.md` | Shareable H2H brief |
| `DETERMINISM_FORMAL.md` | Det definition |
| `luxi_results.json` / `vllm_results.json` | H2H raw |

## Demo (binary, no source)

https://luxiedge.com/demo.html

## Method footnotes

1. Board joules (NVML), not wall-plug.  
2. Prefill positions = iters × batch × seq.  
3. TRADE energy path ≠ AUDIT bit-exact gold.  
4. Not a multi-tenant full-server claim.

Contact: e@ewaller.com
