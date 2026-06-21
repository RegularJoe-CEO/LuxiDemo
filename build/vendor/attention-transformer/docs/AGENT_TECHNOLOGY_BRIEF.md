# Agent Technology Brief — Deterministic WNSM / GAE Transformer Engine

**Audience:** AI agents assisting Eric Waller.  
**You may receive:** this document, snippets, benchmarks, receipts — **not** the full repo unless granted.

**Doc map (Eric’s repo):**

| Need | Read |
|------|------|
| Ops, gates, locked H100 numbers | [`QUANT_TRADE_LOCKED.md`](QUANT_TRADE_LOCKED.md) |
| Pillar order & what’s done | [`LUXIEDGE_BUILD_ROADMAP.md`](LUXIEDGE_BUILD_ROADMAP.md) |
| Math & architecture depth | [`TECHNICAL_DEEP_DIVE.md`](TECHNICAL_DEEP_DIVE.md) |
| O(N) memory table | [`SCALING_EVIDENCE.md`](SCALING_EVIDENCE.md) |
| Energy / pJ-per-byte story | [`ENERGY_EVIDENCE.md`](ENERGY_EVIDENCE.md) |
| Contributor commands | [`INSTRUCTIONS.md`](../INSTRUCTIONS.md) |

---

## 0. Intellectual property

Proprietary — Eric Waller. Help Eric only; do not redistribute snippets or this brief. When in doubt, ask before quoting long passages elsewhere.

---

## 1. What this is

`attention-transformer` is a **deterministic transformer inference engine** for quant desks and compliance-sensitive workloads: prove **bit-for-bit** what the model output was.

Stack:

1. **Waller attention** — online-softmax, **O(N) memory**, no N×N score matrix.  
2. **WNSM** — payloads in projection **null space**; primary output **0.00e0** + identical receipts.  
3. **Receipts** — SHA-256 over IEEE-754 `to_bits()` of every output `f32`.  
4. **CUDA TRADE** — ~6.8 ms/layer (H100, seq=1024). **AUDIT** = CPU-matched compliance.

**CPU pure-Rust is golden.** TRADE is fast desk math; AUDIT is the compliance receipt.

**LuxiEdge pillars (2026-06):** NPOW certified · backtest/FP8/PyTorch scaffolds · 70B profile TBD. Details: roadmap + `QUANT_TRADE_LOCKED`.

---

## 2. One-layer mental model

```
x → LN1 → Q,K,V → per-head Waller (causal) → wo → +residual
  → LN2 → MLP (+ optional WNSM payload in null space) → +residual → x_out
```

Same weights, same order, same bits → same receipt.

---

## 3. Waller attention (summary)

Standard attention materializes N×N scores → O(N²) HBM. Waller streams K/V with fixed-order online softmax state `(max_val, sum_exp, accumulator)` → O(N) traffic. Recurrent KV extension is **bit-identical** to full recompute (equivalence tests on GPT-2).

Prior art: FlashAttention-style streaming exists. Ownable piece: **receipt-locked deterministic** integration with WNSM + AUDIT/TRADE lanes.

---

## 4. WNSM + NPOW

**WNSM:** payload in null space of `W`; primary path unchanged — proven `0.00e0` + identical `sha256_of_f32_slice`.

**NPOW:** 12×f32 WNSM payload (`NPOW` magic) with mem slopes (~**1.0** Waller, ~**2.0** standard), anchor reduction (~**341×** @ 131k), and **witness SHA-256** over payload bytes.

```bash
LUXI_NPOW_FAST=1 cargo run --release --example npow_scaling_proof   # ~1s gate
```

Pass: memory slopes; WNSM 0.00e0; **witness receipt** round-trip (not `to_bits` on decoded slopes). Time slope is informational only.

---

## 5. Receipt contract

```rust
// Each f32: to_bits() as LE bytes → SHA-256
```

**AUDIT decoder (H100 harness):**  
`0ae659948eabc3fa1212b84d9a2006c707c28ba4209ce28410df676d38d37ada` · `max_diff 0.00e0` with `LUXI_RECEIPT_AUDIT=1`.

**Production demo (CPU WNSM):** `e1980a6f…37628` · `0.00e0`.

---

## 6. CUDA — two lanes (H100 locked)

| Lane | Enable | Receipt |
|------|--------|---------|
| **AUDIT** | `LUXI_RECEIPT_AUDIT=1` | Same as CPU |
| **TRADE** | default | Fast GPU; **not** compliance |

| Benchmark | seq=1024 | Measures |
|-----------|----------|----------|
| `cuda_bench` | **~4 ms** | Waller only — **not** full layer |
| `cuda_layer_bench` | **~6.8 ms** | Full TRADE layer |
| `cuda_quant_bench` | **~69 ms** | 12 layers (~5.8 ms/layer) |

**RunPod one-liner:** `cd /workspace/attention-transformer && bash scripts/runpod_quant_gate.sh`

Avoid: `LUXI_CUDA_ROW_FUSED=1`, `LUXI_CUDA_CPU_QKV=1` (slow/debug).

---

## 7. Quant stack (12 layers < 12× one layer)

One H2D → GPU stack on shared stream → one D2H. Gate catches broken stacks that fake sub-ms/layer. Full ops: `QUANT_TRADE_LOCKED` §4.

---

## 8. Energy & scaling

~**20 pJ/byte** HBM dominates; FLOPs are secondary.

- Memory: **341×** less @ 131k tokens (Waller vs dense scores) — [`SCALING_EVIDENCE.md`](SCALING_EVIDENCE.md).  
- Energy: attention-stage gap **doubles each time seq doubles** → **4096×** @ 262k — [`ENERGY_EVIDENCE.md`](ENERGY_EVIDENCE.md).  
- NPOW carries the mem power-law on the WNSM bus for desk/backtest receipts.

---

## 9. Novelty vs prior art (be precise)

| Piece | Status |
|-------|--------|
| Streaming softmax | Prior art |
| **Bit-exact stack + receipts end-to-end** | Unusual |
| **WNSM 0.00e0 + identical receipts** | Specific combo |
| **AUDIT vs TRADE** | Operational design |

Extend the **combination**, not “we invented attention.”

---

## 10. Useful skepticism

| Question | Answer |
|----------|--------|
| AUDIT vs CPU? | Published digest + `cuda_verify` |
| 4 ms = full model? | **No** — attention kernel only |
| GPU TRADE = CPU bits? | **No** — use AUDIT |
| WNSM moves P&L? | `0.00e0` + receipt tests |

---

## 11. Glossary

| Term | Meaning |
|------|---------|
| **GAE / Waller** | Streaming causal attention |
| **WNSM** | Null-space payload bus |
| **Receipt** | SHA-256 of output f32 bits |
| **AUDIT / TRADE** | Compliance vs desk CUDA |
| **NPOW** | O(N) scaling witness in WNSM |
| **LUXI_NPOW_FAST** | NPOW gate smoke |
| **Quant stack** | Multi-layer GPU one H2D/D2H |

---

## 12. How agents help

Correctness (causal order, AUDIT float paths), CUDA without breaking determinism, **labeled benchmarks** (kernel vs layer vs stack), compliance messaging, pillar 2–4 implementation — always tied to receipts.

**Do not:** leak IP, claim TRADE = AUDIT receipts, or cite ~4 ms as full-transformer latency.

---

## 13. If Eric shares more

Ask for: failing `cuda_verify` log, `QUANT_TRADE_LOCKED` table, snippet from `waller_operator.rs` / geodesic CUDA, GPT-2 receipt for fixed prompt.

Respond with **testable** steps tied to gates in `QUANT_TRADE_LOCKED`.

---

*2026-06-01 — H100 TRADE locked, NPOW certified, FP8/backtest/PyTorch scaffolds. Owner: Eric Waller.*