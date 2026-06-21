# Instructions — attention-transformer

Short production reference. **CUDA / RunPod / locked benchmarks:** [`docs/QUANT_TRADE_LOCKED.md`](docs/QUANT_TRADE_LOCKED.md).

## Building & running (CPU)

```bash
cargo build --release
cargo run --release --example production_demo
cargo test --release
cargo test --release --no-default-features
cargo clippy -- -D warnings
```

## Sacred check

`production_demo` must show identical normal/WNSM receipts (`e1980a6f…37628`) and `0.00e0` max diff. If either changes, revert the optimization.

## LuxiEdge gates (CPU)

```bash
LUXI_NPOW_FAST=1 cargo run --release --example npow_scaling_proof
cargo run --release --example fp8_parity_gate
cargo run --release --example receipt_backtest
```

Roadmap: [`docs/LUXIEDGE_BUILD_ROADMAP.md`](docs/LUXIEDGE_BUILD_ROADMAP.md).

## GPT-2 (124M)

```bash
cargo run --release --features gpt2 --example gpt2_demo -- "Once upon a time"
```

Weights: `gpt2-124M-snapshot/` or HF cache. Proves deterministic real-model execution.

## Performance (bit-exact)

- `rayon` — parallel matmul, fixed order  
- `simd` — ARM NEON, no FMA reorder  
- Recurrent KV — recommended generation path (`gpt2_demo`); ~7.6× vs full recompute  
- Speculative decoding — research artifact only; slower than KV on CPU

## Receipt contract (do not change)

`sha256_of_f32_slice`: each `f32` → `to_bits()` → LE bytes → SHA-256.

## Determinism discipline

Fixed accumulation order only. No atomic float adds, no `par_iter().sum()`, no FMA in SIMD vs scalar baseline.

## CUDA / RunPod

See [`docs/QUANT_TRADE_LOCKED.md`](docs/QUANT_TRADE_LOCKED.md) §4:

```bash
export CUDA_ARCH=90
bash scripts/runpod_quant_gate.sh          # full gate
./scripts/runpod_cuda_check.sh             # first-time AUDIT
```

AUDIT receipt: `0ae659948eabc3fa…d37ada` · TRADE ~6.8 ms/layer @ seq=1024.

Manual checklist: [`tests/cuda_lanes.md`](tests/cuda_lanes.md). History: [`docs/verification/MILESTONE_H100_CUDA.md`](docs/verification/MILESTONE_H100_CUDA.md).

## Contributing

- `cargo test --release` + `cargo clippy -- -D warnings` before push  
- NVIDIA: `bash scripts/runpod_quant_gate.sh` after CUDA changes  
- Update `README.md`, this file, `TECHNICAL_DEEP_DIVE.md`, `QUANT_TRADE_LOCKED.md` when behavior or benchmarks change  
- Agents: [`docs/AGENT_TECHNOLOGY_BRIEF.md`](docs/AGENT_TECHNOLOGY_BRIEF.md) — no separate handoff docs