# Y Combinator — LuxiEdge progress update (copy-paste)

**Use this to replace / supersede the TESTfort-era note.**  
TESTfort remains valid independent validation of the earlier stack; this update reports the **newer version-100 configuration** that **wins on both throughput and board energy** vs vLLM on the matched prefill protocol.

---

## Short update (YC application / founder updates — recommended)

LuxiEdge is a working deterministic GPU inference engine, not a research concept. I built and measured the Rust/CUDA execution path, full-model transformer prefill, dual-lane AUDIT vs TRADE, and board-level energy metering myself.

**Progress since the independent TESTfort evaluation:** TESTfort validated an earlier configuration on H100 (packed batch-16 Qwen2-7B prefill) with strong determinism, soak stability, and a modest board-energy edge while still trailing vLLM on throughput. Since then I shipped a new energy/throughput configuration (**version-100**): Flash-class attention control, device-resident multi-layer stack, and FP16 weight residency on the TRADE path.

On a matched **prefill-heavy** protocol (Qwen2-7B-Instruct class, sequence 128, batches 16 and 32, single NVIDIA H100 80GB, NVML board joules, same token definition for both arms), LuxiEdge is now:

- **~1.17–1.18× faster** than default vLLM on prefill throughput, and  
- **~10–14% lower** board joules per prompt position, with  
- **multi-run determinism score 1.0** (5×15 s campaign; token agreement).

Authoritative multi-run lock (5×15 s sustains):

| Batch | Thr median (pos/s) | Board J/pos | Det |
|------:|-------------------:|------------:|----:|
| 16 | ~39,865 | ~0.0168 | 1.0 |
| 32 | ~42,967 | ~0.0160 | 1.0 |

This remains a **measured prefill executor** advantage on a defined workload—not a claim of overall multi-tenant serving leadership, decode crown, or facility wall-plug power. Dual product lanes are explicit: **TRADE** = thr + joules; **AUDIT** = trust/receipts (separate contract).

**Demo (binary only, no source):** https://luxiedge.com/demo.html  
**Public evidence pack:** https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/version-100-h100-gtm  
**Technical one-pager:** in that pack (`PUBLIC_GTM_ONE_PAGER.md`)

Commercial serve surface is live (OpenAI-shaped HTTP + locked scoreboard on `GET /v1/gtm` + dual-run audit). Next: continuous batching, decode, multi-GPU, and energy against useful work per GPU and per unit of facility power with infrastructure partners.

Contact: e@ewaller.com

---

## One-paragraph “what changed vs TESTfort”

TESTfort independently measured an earlier LuxiEdge stack at ~28.4k prefill positions/s and ~0.0187 J/pos on H100 batch-16, with energy slightly better than vLLM but throughput still below vLLM. The version-100 TRADE configuration reverses the throughput story while widening the energy win: multi-run medians ~39.9k pos/s at ~0.0168 J/pos (B16) and ~43.0k at ~0.0160 J/pos (B32), with matched H2H ~1.17–1.18× thr and ~10–14% lower board J/pos vs vLLM, det=1.0. Demo and public JSON/lock files are linked above without releasing engine source.

---

## Links checklist for the form

| Item | URL |
|------|-----|
| **Demo (primary)** | https://luxiedge.com/demo.html |
| macOS binary | https://luxiedge.com/downloads/luxiedge-serve-macos-arm64 |
| Linux binary | https://luxiedge.com/downloads/luxiedge-serve-linux-x86_64 |
| Evidence | https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/version-100-h100-gtm |
| Site proof | https://luxiedge.com/proof.html#v100 |
| Contact | e@ewaller.com |

*If Replit deploy lags GitHub, use the GitHub evidence URL immediately and the demo.html URL after you publish `site/`.*

---

## What not to claim in the YC box

- Full OpenAI multi-tenant server thr crown vs every vLLM recipe  
- Decode-only leadership  
- Wall-plug / PUE  
- That the laptop demo thr equals H100 TRADE thr (demo embeds the lock; thr was measured on GPU)

---

## Optional founder note (tone)

Independent third-party TESTfort established we were real. Version-100 establishes we can win the comparison that matters for prefill economics: more useful work per second *and* fewer board joules per unit of that work, with determinism held—while still being honest about scope.
