# LuxiEdge Build Roadmap

**Engine:** deterministic WNSM/GAE (`attention-transformer`)  
**Ops & locked numbers:** [`QUANT_TRADE_LOCKED.md`](QUANT_TRADE_LOCKED.md)  
**Agent context (no repo):** [`AGENT_TECHNOLOGY_BRIEF.md`](AGENT_TECHNOLOGY_BRIEF.md)

Non-negotiables: CPU f32 golden · TRADE geodesic CUDA · Waller O(N) mem · `sha256_of_f32_slice` · AUDIT bit-exact to CPU.

---

## Delivery order

| # | Pillar | Status | Entry |
|---|--------|--------|--------|
| 1 | Receipt backtest + **NPOW** | **NPOW done** · backtest scaffold | `receipt_backtest`, `npow_scaling_proof`, `src/quant/`, `src/npow/` |
| 2 | 70B+ identical AUDIT receipts | Profile + contract | `src/scaling/`, `scaling_audit_contract` test |
| 3 | FP8 deterministic | Shadow path + gate | `fp8_parity_gate`, `src/fp8/` |
| 4 | PyTorch | FFI + `luxiedge_ops.py` | `src/ffi/`, `integrations/torch/` |

---

## 1 — P&L + NPOW (pillar 1)

**Done:** NPOW memory slopes (1.0 / 2.0), WNSM **0.00e0**, witness receipt on bus. Gate: `LUXI_NPOW_FAST=1` in `runpod_quant_gate.sh`.

**Next:** live desk CSV, real alpha head, persist `AuditReport` per bar.

```bash
LUXI_NPOW_FAST=1 cargo run --release --example npow_scaling_proof
cargo run --release --example receipt_backtest
```

---

## 2 — Scale (pillar 2)

Same AUDIT receipt at 124M → 70B; only weights/sharding change. Profiles: `gpt2_124m`, `llama2_7b`, `llama2_70b`. Sharded load TBD.

---

## 3 — FP8 (pillar 3)

Shadow E4M3 → decode f32 in fixed order → **0.00e0** and same SHA-256 as CPU. TRADE may use HBM FP8 later; AUDIT always vs f32 golden.

```bash
cargo run --release --example fp8_parity_gate
```

---

## 4 — PyTorch (pillar 4)

C ABI receipts + Python wrappers; `pyo3` handoff when stable.

---

## Regression

All commands: [`QUANT_TRADE_LOCKED.md`](QUANT_TRADE_LOCKED.md) §3–4.

```bash
cargo test && cargo clippy -- -D warnings
bash scripts/runpod_quant_gate.sh   # NVIDIA pods
```