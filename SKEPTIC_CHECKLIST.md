# Skeptic checklist — site rebuild

Run this with **only** public LuxiDemo (no private repo access).

## Headlines

| Site claim | Verify |
|------------|--------|
| ~403 tok/s @ seq=128 | `evidence/h100-7b-class-TRADE/START_HERE.md` + `thr_sustain_seq128/AGGREGATE.json` / `LADDER.json` |
| 0.630 ± 0.002 J/tok | same |
| ~254 W median | same |
| ~464 tok/s / ~0.60 J/tok @ 256 | `thr_sustain_seq256/` |
| Seq=5 ~3.56 J/tok is not hero | LADDER + site energy page labels microbench |
| Flash ~19× win on 12L | `evidence/h100-stack12-H2H/START_HERE.md` |
| O(N) memory ladder | `evidence/h100-LONGCTX-scaling/START_HERE.md` |

## Hard constraints

- [ ] No SpaceX / xAI / diligence company names on site pages  
- [ ] Evidence folder names are neutral (`h100-…`)  
- [ ] No raw H100 TFLOPS / 286B ops/sec as product heroes  
- [ ] Board power ≠ wall plug stated on home + energy  
- [ ] Every hero metric has a public pack link  
- [ ] Local preview: `cd site && python3 -m http.server 8080` works without Replit  

## Persona walks (≤2 minutes)

1. **Data-center buyer:** Home → Data centers → Evidence → Contact  
2. **Public:** Home → Why energy → Benchmarks ladder → Evidence pack  

## Result (2026-07-11 rebuild)

- Claims register written: `CLAIMS_REGISTER.md`  
- Site routes: home, product, data-centers, energy, evidence, benchmarks, download, contact  
- Evidence renamed + docs neutralized  
- Publish deferred: `PUBLISH.md`  
