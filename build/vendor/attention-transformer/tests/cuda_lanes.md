# CUDA lane tests (manual — requires NVIDIA + `--features cuda`)

**Locked benchmarks, gates, and env reference:** [`docs/QUANT_TRADE_LOCKED.md`](../docs/QUANT_TRADE_LOCKED.md)

## Quick regression (RunPod)

```bash
export CUDA_ARCH=90 PATH="/usr/local/cuda/bin:$HOME/.cargo/bin:$PATH"
bash scripts/runpod_quant_gate.sh
```

## AUDIT receipt gate

```bash
LUXI_RECEIPT_AUDIT=1 cargo run --release --features cuda --example cuda_verify
```

Expect: `FULL DECODER CUDA PATH VERIFIED`, receipt `0ae659948eabc3fa1212b84d9a2006c707c28ba4209ce28410df676d38d37ada`, `max_diff 0.00e0`.

## TRADE throughput benches

```bash
cargo run --release --features cuda --example cuda_layer_bench -- 20 1024 1024 16 256
cargo run --release --features cuda --example cuda_quant_bench -- 20 1024 12
cargo run --release --features cuda --example cuda_bench -- 50 1024 1024 16
```

Phase breakdown: `LUXI_CUDA_PHASE_TIMING=1` with `cuda_layer_bench`.

## NPOW scaling witness (CPU — any machine)

```bash
LUXI_NPOW_FAST=1 cargo run --release --example npow_scaling_proof
```

Expect: `NPOW memory scaling proof PASS`, WNSM `max_diff 0.00e0`. Included in `runpod_quant_gate.sh`.

## Extended regression

```bash
bash scripts/runpod_substantial_test.sh
bash scripts/runpod_geodesic_bench.sh
```

Performance history: [`docs/verification/MILESTONE_H100_CUDA.md`](../docs/verification/MILESTONE_H100_CUDA.md).