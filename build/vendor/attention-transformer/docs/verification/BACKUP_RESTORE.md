# CUDA optimization backup and restore

> Narrow git-revert instructions. Current ops: [`QUANT_TRADE_LOCKED.md`](../QUANT_TRADE_LOCKED.md).

## Backup tag (before aggressive Phase 3)

**Tag:** `backup/pre-aggressive-cuda-20260602`  
**Points at:** `d5a75c0` era (4.16 ms median bench, decoder bit-exact)

### Restore entire tree to backup

```bash
cd /workspace/attention-transformer
git fetch origin
git checkout backup/pre-aggressive-cuda-20260602
cargo build --release --features cuda
cargo run --release --features cuda --example cuda_verify
```

### Restore only CUDA files from backup (stay on current branch)

```bash
git checkout backup/pre-aggressive-cuda-20260602 -- cuda_src/waller_operator.cu src/gpu/cuda.rs examples/cuda_bench.rs
```

## Phase 3 regression gate

```bash
cargo build --release --features cuda
cargo run --release --features cuda --example cuda_verify
cargo run --release --features cuda --example cuda_bench -- 500 1024 1024 16
```

Decoder receipt must remain:

`0ae659948eabc3fa1212b84d9a2006c707c28ba4209ce28410df676d38d37ada`