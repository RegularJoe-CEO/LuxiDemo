# version-100 H100 pack (public)

**Hardware:** NVIDIA H100 80GB HBM3  
**Model class:** Qwen2-7B-Instruct (FP16)  
**Workload:** Prefill-heavy · sequence 128 · batch 16 / 32  

This folder retains raw run files from the version-100 short-prompt (S=128) multi-run campaign on the TRADE energy path.

**Measured results, including Luxi vs vLLM 0.25.1 (Qwen2-7B, prefill + decode):** see [`BENCHMARKS.md`](../../BENCHMARKS.md)

## Files

| File | Role |
|------|------|
| `MULTI_RUN_LOCK_SLIM.json` | 5×15s multi-run medians (raw) |
| `MULTI_RUN_LOCK.json` | Full multi-run detail (raw) |
| `H2H_ANSWER.json` | Run medians (raw) |
| `luxi_results.json` / `vllm_results.json` | Per-run raw files |
| `DETERMINISM_FORMAL.md` | Det definition |
| `PUBLIC_GTM_ONE_PAGER.md` | Buyer one-pager |

## Demo (binary, no source)

Serve binaries: [`../../downloads/`](../../downloads/) · catalog: [`../../DEMOS.md`](../../DEMOS.md)  
Marketing site (Replit, not this repo): https://luxiedge.com

## Method footnotes

1. Board joules (NVML), not wall-plug.  
2. Prefill positions = iters × batch × seq.  
3. TRADE energy path ≠ AUDIT bit-exact gold.  
4. Not a multi-tenant full-server claim.

Contact: e@ewaller.com
