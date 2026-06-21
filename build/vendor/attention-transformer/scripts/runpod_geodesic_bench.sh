#!/usr/bin/env bash
# Geodesic P0+P1 bench on RunPod (H100: export CUDA_ARCH=90).
set -euo pipefail
cd "${INSTALL_DIR:-/workspace/attention-transformer}"
export PATH="/usr/local/cuda/bin:${HOME}/.cargo/bin:${PATH}"
export CUDA_ARCH="${CUDA_ARCH:-90}"
source "${HOME}/.cargo/env" 2>/dev/null || true

git pull origin main
cargo build --release --features cuda

echo "=== AUDIT receipt gate ==="
LUXI_RECEIPT_AUDIT=1 cargo run --release --features cuda --example cuda_verify

echo ""
echo "=== GEODESIC TRADE (default) seq=128 ==="
cargo run --release --features cuda --example cuda_layer_bench -- 20 128 1024 16 256

echo ""
echo "=== GEODESIC TRADE seq=1024 ==="
cargo run --release --features cuda --example cuda_layer_bench -- 20 1024 1024 16 256

echo ""
echo "=== QUANT TRADE (batched MLP + phase timing) ==="
LUXI_CUDA_PHASE_TIMING=1 cargo run --release --features cuda --example cuda_layer_bench -- 20 1024 1024 16 256

echo ""
echo "=== 12-layer quant stack ==="
cargo run --release --features cuda --example cuda_quant_bench -- 20 1024 12

echo ""
echo "Locked regression gate: bash scripts/runpod_quant_gate.sh"
echo ""
echo "=== CPU QKV fallback (compare) ==="
LUXI_CUDA_CPU_QKV=1 cargo run --release --features cuda --example cuda_layer_bench -- 10 1024 1024 16 256