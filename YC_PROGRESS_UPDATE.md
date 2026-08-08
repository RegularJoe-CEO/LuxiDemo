# Y Combinator — LuxiEdge progress update (copy-paste)

**Use this to replace / supersede the TESTfort-era note.**  
TESTfort remains valid independent validation of the earlier stack. Report **two separate facts**: absolute Luxi prefill peak (dual_gemm B72) and matched vLLM H2H (B16/B32 only).

---

## Short update (YC application / founder updates — recommended)

LuxiEdge is a working deterministic GPU inference engine, not a research concept. I built and measured the Rust/CUDA execution path, full-model transformer prefill, dual-lane AUDIT vs TRADE, and board-level energy metering myself.

**Progress since the independent TESTfort evaluation:** TESTfort validated an earlier configuration on H100 with strong determinism and a modest board-energy edge while still trailing vLLM on throughput. Since then Flash-class attention, device-resident stack, FP16 TRADE residency, and dual-stream GEMM at high batch raised the **absolute** prefill operating point.

**Two facts (do not blend batches):**

1. **Best measured Luxi absolute prefill:** multi-run **~44.9k pos/s** and **~0.0153 board J/pos** at **B72 dual_gemm** (flash + device-resident + FP16, S=128, H100 NVML). About **9% more thr and ~9% less energy per position** than the prior Luxi B16 lock (~41.2k / 0.0169).  
2. **Matched vLLM H2H (B16 only, separate pack):** **~1.18× thr** and **~12% lower board J/pos** vs default vLLM. Not yet matched at B72 — so the 44.9k absolute number is **not** claimed as 1.18× vLLM.

| Fact | Operating point | Thr median | Board J/pos |
|------|-----------------|----------:|------------:|
| Absolute champion | B72 dual_gemm | ~44,860 | ~0.0153 |
| Matched H2H | B16 vs vLLM | ~41,221 | ~0.0169 (~1.18× thr / ~12% lower J) |

This remains a **measured prefill executor** advantage on a defined workload—not multi-tenant serving leadership, decode crown, or wall-plug power. Dual lanes: **TRADE** = thr + joules; **AUDIT** = trust/receipts.

**Demo:** https://luxiedge.com/demo.html  
**Absolute pack:** https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/prefill_accel_lock_20260807T233111Z  
**Matched vLLM pack:** https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/prefill_freeze_matched_20260807T210749Z  

Contact: e@ewaller.com

---

## One-paragraph “what changed vs TESTfort”

TESTfort independently measured an earlier LuxiEdge stack at ~28.4k prefill positions/s and ~0.0187 J/pos on H100 batch-16, with energy slightly better than vLLM but throughput still below vLLM. Matched H2H later showed ~1.18× thr and ~12% lower board J at B16. The absolute product operating point is now dual_gemm at B72 (~44.9k pos/s, ~0.0153 J/pos multi-run)—higher thr and lower energy than the prior Luxi B16 freeze—without claiming that B72 figure is 1.18× vLLM until a matched B72 vLLM arm exists.

---

## Links checklist for the form

| Item | URL |
|------|-----|
| **Evidence (absolute champion)** | https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/prefill_accel_lock_20260807T233111Z |
| **Evidence (matched vLLM)** | https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/prefill_freeze_matched_20260807T210749Z |
| **Demo package (binary, no source)** | https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/demo/luxiedge-yc-demo |
| **macOS binary (raw)** | https://github.com/RegularJoe-CEO/LuxiDemo/raw/main/site/downloads/luxiedge-serve-macos-arm64 |
| **Linux binary (raw)** | https://github.com/RegularJoe-CEO/LuxiDemo/raw/main/site/downloads/luxiedge-serve-linux-x86_64 |
| **Release assets** | https://github.com/RegularJoe-CEO/LuxiDemo/releases/tag/version-100-serve |
| **Site demo page** | https://luxiedge.com/demo.html *(after you publish `site/` to Replit)* |
| Contact | e@ewaller.com |

*YC can use the GitHub evidence + raw binary links immediately. Replit is only needed for the pretty luxiedge.com/demo.html page.*

---

## What not to claim in the YC box

- Full OpenAI multi-tenant server thr crown vs every vLLM recipe  
- Decode-only leadership  
- Wall-plug / PUE  
- That the laptop demo thr equals H100 TRADE thr (demo embeds the lock; thr was measured on GPU)

---

## Optional founder note (tone)

Independent third-party TESTfort established we were real. Version-100 establishes we can win the comparison that matters for prefill economics: more useful work per second *and* fewer board joules per unit of that work, with determinism held—while still being honest about scope.
