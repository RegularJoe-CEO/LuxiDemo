# Claims Register — luxiedge.com rebuild

**Date:** 2026-07-11  
**Sources of truth:** live crawl of https://luxiedge.com + public [RegularJoe-CEO/LuxiDemo](https://github.com/RegularJoe-CEO/LuxiDemo) `evidence/**`  
**Rule:** No unlinked hero metric. Prefer public pack `START_HERE` over private NDA.

Legend: **KEEP** · **REFRAME** · **DELETE** · **ADD**

---

## Live site → action

| # | Live claim (approx) | Location | Action | Replacement / notes |
|---|---------------------|----------|--------|---------------------|
| 1 | Hero: “Deterministic, receipt-attested **quant risk engine**” | `/` | **REFRAME** | Deterministic, receipt-attested, **energy-aware AI compute** (quant AI + data centers) |
| 2 | GAE “in development, not production”; ATE early testing | `/` | **DELETE** as product status | **Measured** TRADE/WNSM/long-ctx stacks are public; label as measured paths + honest non-claims |
| 3 | 286.94B ops/sec aggregate (H100 / TestFort) | `/benchmarks`, `/hyperscale`, `/how-it-works` | **DELETE** from hero | Optional labeled **microbench appendix only** if ever reintroduced with scope |
| 4 | 2.35B ops/J | `/hyperscale`, `/how-it-works` | **DELETE** from hero | Same as above — not product J/token |
| 5 | Raw H100 FP64 85.8 TFLOPS as product proof | `/benchmarks`, `/hyperscale` | **DELETE** | Silicon capability ≠ LuxiEdge product thr/energy |
| 6 | L4 FMA 30.7B ops/sec @ 72W / Gold Master 880M ops/J | `/benchmarks` | **DELETE** / archive | Microkernel FMA loop, not inference stack |
| 7 | Competitors: Rhai, NumExpr, TFLite, xsimd | `/competitors` | **DELETE** page | Wrong set; compare open **attention/serve** tech (FlashAttention, SDPA) with honesty |
| 8 | Cross-platform bit-exact for all workloads (incl. GPU risk) | `/`, `/how-it-works` | **REFRAME** | Scope: f64 CPU gold path + disclosed GPU oracle/drift; not “all workloads bit-exact” |
| 9 | Bit-exact on H100 “across all 80GB HBM3” | `/hyperscale` | **DELETE** | Overclaim; not pack-backed as product statement |
| 10 | Patent “issuance expected early July 2026” | `/` | **REFRAME** | “Non-provisional filed …” only if date verified; **no speculative issuance date** on public pages |
| 11 | Private NDA as primary proof | `/` | **REFRAME** | **Public packs first**; NDA for full source/commercial |
| 12 | TestFort / OpenBenchmarking links | `/benchmarks` | **KEEP** demoted | Supporting validation only; not thr/J/tok heroes |
| 13 | REST `/evaluate` quant expression API | `/` | **KEEP** secondary | Demo binaries still ship; not home hero |
| 14 | luxi-jit 5/12 bit-exact, rms_norm 0 ULP A100/H200 | `/` | **KEEP** optional deep | Link private or labeled artifacts only if public; else omit from public site |
| 15 | >10× faster / >5× efficiency marketing | `/benchmarks` | **DELETE** | Unscoped vs what |
| 16 | 1.6ms / 100K ARM latency | `/benchmarks` | **DELETE** or appendix | Not product energy story |
| 17 | Sub-1ms API latency | `/benchmarks` | **DELETE** | Unscoped |
| 18 | RISC-V / WASM “planned” as platform guarantee | `/benchmarks` | **DELETE** | Roadmap noise on buyer pages |

---

## Public measured truth → site heroes (**ADD**)

### Absolute prefill champion (**current lead thr+J** — 2026-08-07)

| Claim | Numbers | Public pack | Site placement |
|-------|---------|-------------|----------------|
| **Best Luxi absolute prefill thr** | **~44.9k pos/s @ B72** dual_gemm multi-run | `evidence/prefill_accel_lock_20260807T233111Z` | Home hero, proof#absolute, demo |
| **Best Luxi absolute board J/pos** | **~0.0153** @ B72 dual_gemm | same | Home, proof, demo |
| Recipe | `LUXI_GEMM_DUAL_STREAM=1` · flash+device_resident+FP16 · S=128 | `CHAMPION_LOCK.json` | Method footnotes |
| **Do not** crown dual_fuse / single-shot 45,986 | thr edge +0.1% / engineering peak only | same | Internal only |

**Rule:** Absolute B72 numbers are **not** a matched vLLM claim. Never write “44.9k = 1.18× vLLM.”

### Matched vLLM H2H (**separate fact** — B16/B32 only)

| Claim | Numbers | Public pack | Site placement |
|-------|---------|-------------|----------------|
| vs vLLM thr / J @ **B16** | **~1.18× thr · ~12% lower J** | `evidence/prefill_freeze_matched_20260807T210749Z` | Home secondary, proof#matched-vllm |
| vs vLLM thr / J @ **B32** | **~1.19× thr · ~14% lower J** | same | proof#matched-vllm |
| B72 vLLM matched | **false** until re-run | — | Non-claim |

### version-100 GTM lock (lineage — 2026-07)

| Claim | Numbers | Public pack | Site placement |
|-------|---------|-------------|----------------|
| Prefill thr B16 / B32 | **~39.9k / ~43.0k pos/s** | `evidence/version-100-h100-gtm` | proof lineage |
| Board J/pos B16 / B32 | **~0.0168 / ~0.0160** | same | proof lineage |
| vs vLLM thr / J | **~1.17–1.18×** thr · **~10–14%** lower J | same + H2H JSON | superseded by matched freeze pack for H2H |
| Multi-run det | **1.0** | same | lineage; B72 det pending receipt pack |

### Prior packs (lineage — do not mix protocols without labels)

| Claim | Numbers | Public pack | Site placement |
|-------|---------|-------------|----------------|
| 7B-class TRADE prefill thr (prior) | **~403 tok/s** | `evidence/h100-7b-class-TRADE` seq≥128 | proof lineage only |
| 7B-class J/token (prior) | **0.630 ± 0.002** | same | proof lineage only |
| Median board W (prior) | **~254 W** | same | proof lineage only |
| Longer ctx | **~464 tok/s**, **~0.60 J/tok** @ seq=256 | same | Benchmarks |
| Seq=5 microbench | ~44 tok/s, **~3.56 J/tok** | root multirun | **Labeled microbench only** — never hero |
| Stack12 TRADE energy | ~0.013 J/tok prefill, ~177 W | `h100-stack12-TRADE-cuda` | Evidence index |
| Flash H2H honesty | PT+Flash **~19×** thr/J on 12L shape | `h100-stack12-H2H` | Product + benchmarks honesty |
| WNSM free-ride | ~1% vs side channel; null residual ~1e-8 | `h100-WNSM-free-ride` | Product |
| O(N) memory | slope ~1.0 vs dense ~2.0; 256× @ 32k | `h100-LONGCTX-scaling` | Product, data-centers |
| Board power method | pynvml sustain-only | all energy packs | Footnotes everywhere |

---

## Hard non-claims (must appear)

1. TRADE 7B path is **not** HF chat-quality (architecture map into TRADE kernels).  
2. Flash-class kernels may **win short-seq thr/J/tok**; differentiated axes: AUDIT/free-ride, O(N) memory, measured board energy on real 7B-scale weights.  
3. Power = **GPU board (pynvml)**, not wall-plug AC.  
4. Small-stack residual-MLP numbers ≠ 7B J/tok without clear labels.  
5. **No company diligence codenames** on public pages or pack titles.  
6. **Absolute B72 thr/J is not matched vLLM H2H** — different batch/shape unless a matched B72 vLLM arm is published.  
7. **Do not headline single-shot thr** (e.g. 45,986); multi-run median only.  
8. **B72 dual_gemm receipt equality** required before bit-exact / full det hero on that recipe.

---

## Public evidence packs (neutral names only)

| Folder | Role |
|--------|------|
| `h100-7b-class-TRADE` | Primary 7B-class thr + J/token |
| `h100-stack12-TRADE-cuda` | 12L device-resident energy |
| `h100-stack12-H2H` | TRADE vs Flash honesty |
| `h100-WNSM-free-ride` | Free-ride under load |
| `h100-LONGCTX-scaling` | O(N) memory scaling |
| `h100-BASELINE-vs-geo` | Single-layer baseline wedges |
| `h100-serve-sustain-2026-07-11` | Serve sustain traces |

GitHub URLs: `https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/<name>`

---

## Page disposition

| Old route | Disposition |
|-----------|-------------|
| `/` | Full rewrite |
| `/how-it-works` | Replace with `/product` |
| `/benchmarks` | Rewrite; pack-backed only |
| `/hyperscale` | **Archive/remove** from nav (raw TFLOPS) |
| `/competitors` | **Remove** |
| `/thermal` | Optional later; not required |
| `/math`, `/whitepaper` | Optional archive if claims allow |
| `/download` | Keep purpose; LuxiDemo releases |
| New `/data-centers` | Buyer page |
| New `/energy` | Public rally |
| New `/evidence` | Pack index |
| New `/contact` | Commercial / NDA |

---

## Verification checklist

- [ ] Every hero number links to a public pack  
- [ ] Zero company diligence codenames on site + pack titles  
- [ ] Seq=5 ~3.56 J/tok never leads  
- [ ] Flash honesty visible on product/benchmarks  
- [ ] Board ≠ wall plug stated  
- [ ] Local preview works without Replit  
