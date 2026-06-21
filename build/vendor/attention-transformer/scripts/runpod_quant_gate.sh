#!/usr/bin/env bash
# Quant TRADE regression gate (H100 NVL). See docs/QUANT_TRADE_LOCKED.md
set -euo pipefail
cd "${INSTALL_DIR:-/workspace/attention-transformer}"
export PATH="/usr/local/cuda/bin:${HOME}/.cargo/bin:${PATH:-}"
export CUDA_ARCH="${CUDA_ARCH:-90}"
source "${HOME}/.cargo/env" 2>/dev/null || true

echo "════════════════════════════════════════════════════════════════════════"
echo " Quant TRADE gate — $(git rev-parse --short HEAD 2>/dev/null || echo unknown)"
echo " $(date -u)"
echo "════════════════════════════════════════════════════════════════════════"

cargo build --release --features cuda

echo ""
echo "=== AUDIT receipt ==="
LUXI_RECEIPT_AUDIT=1 cargo run --release --features cuda --example cuda_verify 2>&1 | tee /tmp/cuda_verify_gate.txt
grep -q "FULL DECODER CUDA PATH VERIFIED" /tmp/cuda_verify_gate.txt
grep -q "Receipts match: true" /tmp/cuda_verify_gate.txt
grep -q "Max diff:      0.00e0" /tmp/cuda_verify_gate.txt
echo "AUDIT: PASS"

echo ""
echo "=== 1 layer TRADE ==="
LUXI_CUDA_PHASE_TIMING=1 cargo run --release --features cuda --example cuda_layer_bench -- 20 1024 1024 16 256 2>&1 | tee /tmp/cuda_layer_gate.txt
grep -q "TOTAL (full layer)" /tmp/cuda_layer_gate.txt
echo "layer_bench: OK (expect median ~6–8 ms)"

echo ""
echo "=== 12 layer quant stack ==="
cargo run --release --features cuda --example cuda_quant_bench -- 20 1024 12 2>&1 | tee /tmp/cuda_quant_gate.txt
grep -q "12 layers (quant stack)" /tmp/cuda_quant_gate.txt
if grep -q "WARN:" /tmp/cuda_quant_gate.txt; then
  echo "quant_bench: FAIL (stack faster than N×1-layer — broken stack)"
  exit 1
fi
echo "quant_bench: OK (expect median ~65–85 ms)"

echo ""
echo "=== Waller-only reference ==="
cargo run --release --features cuda --example cuda_bench -- 50 1024 1024 16 2>&1 | tee /tmp/cuda_bench_gate.txt
grep -q "DEVICE-RESIDENT" /tmp/cuda_bench_gate.txt
echo "cuda_bench: OK (expect median ~3–4 ms)"

echo ""
echo "=== NPOW scaling witness (CPU, fast smoke) ==="
LUXI_NPOW_FAST=1 cargo run --release --example npow_scaling_proof 2>&1 | tee /tmp/npow_gate.txt
grep -q "NPOW memory scaling proof PASS" /tmp/npow_gate.txt
grep -q "NPOW scaling proof complete" /tmp/npow_gate.txt
echo "npow_scaling_proof: OK"

echo ""
echo "════════════════════════════════════════════════════════════════════════"
echo " QUANT TRADE GATE: PASS"
echo " See docs/QUANT_TRADE_LOCKED.md for locked numbers."
echo "════════════════════════════════════════════════════════════════════════"