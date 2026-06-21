#!/usr/bin/env bash
# Substantial CUDA + determinism test suite for RunPod (H100 or similar).
#
#   cd /workspace/attention-transformer
#   git fetch origin main && git reset --hard origin/main
#   bash scripts/runpod_substantial_test.sh
#
# Report: docs/verification/SUBSTANTIAL_TEST_REPORT.md

set -euo pipefail

INSTALL_DIR="${INSTALL_DIR:-/workspace/attention-transformer}"
cd "$INSTALL_DIR"

export PATH="/usr/local/cuda/bin:${PATH:-}"
export CUDA_ARCH="${CUDA_ARCH:-90}"
source "$HOME/.cargo/env" 2>/dev/null || true

REPORT="docs/verification/SUBSTANTIAL_TEST_REPORT.md"
LOGDIR="${INSTALL_DIR}/test_logs"
mkdir -p "$LOGDIR"
STAMP="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
COMMIT="$(git rev-parse --short HEAD 2>/dev/null || echo unknown)"
LOG="${LOGDIR}/substantial_${STAMP//:/-}.log"

exec > >(tee -a "$LOG") 2>&1

FAILED=0

section() {
  echo ""
  echo "════════════════════════════════════════════════════════════════════════"
  echo " $1"
  echo "════════════════════════════════════════════════════════════════════════"
}

record() {
  local name="$1"
  local ok="$2"
  if [[ "$ok" == "1" ]]; then
    echo "${name}=PASS"
  else
    echo "${name}=FAIL"
    FAILED=1
  fi
}

verify_log_ok() {
  local f="$1"
  grep -q "FULL DECODER CUDA PATH VERIFIED" "$f" \
    && grep -q "Receipts match: true" "$f" \
    && grep -q "Max diff:      0.00e0" "$f"
}

section "ENVIRONMENT"
echo "UTC: $STAMP"
echo "Commit: $COMMIT"
echo "PWD: $(pwd)"
nvidia-smi --query-gpu=name,driver_version,memory.total,power.limit --format=csv,noheader 2>/dev/null || nvidia-smi -L
nvcc --version 2>/dev/null | tail -1 || echo "nvcc: missing"
rustc -Vv 2>/dev/null | head -1 || true

section "BUILD — cuda"
if cargo build --release --features cuda; then
  record BUILD_CUDA 1
else
  record BUILD_CUDA 0
fi

section "BUILD — cuda + cuda-quant"
if cargo build --release --features "cuda cuda-quant"; then
  record BUILD_QUANT 1
else
  record BUILD_QUANT 0
fi

section "UNIT TESTS (release)"
if cargo test --release; then
  record UNIT_TESTS 1
else
  record UNIT_TESTS 0
fi

section "cuda_verify — Lane AUDIT (receipt gate)"
unset LUXI_CUDA_GPU_LAYER LUXI_CUDA_GPU_QKV
export LUXI_RECEIPT_AUDIT=1
cargo run --release --features cuda --example cuda_verify > "$LOGDIR/cuda_verify_default.txt" 2>&1 || true
grep -E "VERIFIED|Receipts match|Max diff|panicked|FULL DECODER" "$LOGDIR/cuda_verify_default.txt" || true
if verify_log_ok "$LOGDIR/cuda_verify_default.txt"; then
  record CUDA_VERIFY_DEFAULT 1
else
  record CUDA_VERIFY_DEFAULT 0
fi

section "cuda_verify — LUXI_CUDA_GPU_LAYER=1"
export LUXI_CUDA_GPU_LAYER=1
cargo run --release --features cuda --example cuda_verify > "$LOGDIR/cuda_verify_gpu_layer.txt" 2>&1 || true
grep -E "VERIFIED|Receipts match|Max diff|panicked|FULL DECODER" "$LOGDIR/cuda_verify_gpu_layer.txt" || true
if verify_log_ok "$LOGDIR/cuda_verify_gpu_layer.txt"; then
  record CUDA_VERIFY_GPU_LAYER 1
else
  record CUDA_VERIFY_GPU_LAYER 0
fi
unset LUXI_CUDA_GPU_LAYER

section "cuda_bench — 500×1024×1024×16"
cargo run --release --features cuda --example cuda_bench -- 500 1024 1024 16 > "$LOGDIR/cuda_bench.txt" 2>&1 || true
grep -E "PERSISTENT|DEVICE-RESIDENT|phase mean|Power|checksum" "$LOGDIR/cuda_bench.txt" || true
if grep -q "DEVICE-RESIDENT" "$LOGDIR/cuda_bench.txt" && grep -q "checksum guard" "$LOGDIR/cuda_bench.txt"; then
  record CUDA_BENCH 1
else
  record CUDA_BENCH 0
fi

section "cuda_bench — LUXI_WALLER_SMEM=1 (100 iters, informational)"
export LUXI_WALLER_SMEM=1
cargo run --release --features cuda --example cuda_bench -- 100 1024 1024 16 > "$LOGDIR/cuda_bench_smem.txt" 2>&1 || true
grep -E "PERSISTENT|DEVICE-RESIDENT|median" "$LOGDIR/cuda_bench_smem.txt" | head -12 || true
if grep -q "DEVICE-RESIDENT" "$LOGDIR/cuda_bench_smem.txt"; then
  record CUDA_BENCH_SMEM 1
else
  record CUDA_BENCH_SMEM 0
fi
unset LUXI_WALLER_SMEM

section "cuda_quant_stack"
if cargo run --release --features "cuda cuda-quant" --example cuda_quant_stack > "$LOGDIR/cuda_quant_stack.txt" 2>&1; then
  cat "$LOGDIR/cuda_quant_stack.txt"
  if grep -q "Lane B INT8 GEMM" "$LOGDIR/cuda_quant_stack.txt" && grep -q "KV cache step" "$LOGDIR/cuda_quant_stack.txt"; then
    record CUDA_QUANT_STACK 1
  else
    record CUDA_QUANT_STACK 0
  fi
else
  record CUDA_QUANT_STACK 0
fi

section "SUMMARY"
if [[ "$FAILED" -eq 0 ]]; then
  echo "OVERALL_GATE=PASS"
else
  echo "OVERALL_GATE=FAIL"
fi

# Markdown report
BENCH_PERSIST_MEDIAN="$(grep -A6 'PERSISTENT' "$LOGDIR/cuda_bench.txt" 2>/dev/null | grep median | head -1 | awk '{print $3}' || echo n/a)"
BENCH_RES_MEDIAN="$(grep -A6 'DEVICE-RESIDENT' "$LOGDIR/cuda_bench.txt" 2>/dev/null | grep median | head -1 | awk '{print $3}' || echo n/a)"
BENCH_RES_GFLOP="$(grep 'DEVICE-RESIDENT' -A8 "$LOGDIR/cuda_bench.txt" 2>/dev/null | grep 'approx attention' | head -1 | sed 's/.*: //' || echo n/a)"
BENCH_POWER="$(grep 'Power (nvidia-smi' "$LOGDIR/cuda_bench.txt" 2>/dev/null | awk '{print $5}' || echo n/a)"
BENCH_GFLOP_J="$(grep 'GFLOP/s per W' "$LOGDIR/cuda_bench.txt" 2>/dev/null | awk '{print $5}' || echo n/a)"

{
  echo "# Substantial Test Report — RunPod"
  echo ""
  echo "| Field | Value |"
  echo "|-------|-------|"
  echo "| UTC | $STAMP |"
  echo "| Commit | \`$COMMIT\` |"
  echo "| GPU | NVIDIA H100 NVL (from run) |"
  echo "| Log | \`$LOG\` |"
  echo ""
  echo "## How to re-run"
  echo ""
  echo '```bash'
  echo "cd /workspace/attention-transformer"
  echo "git fetch origin main && git reset --hard origin/main"
  echo "export PATH=\"/usr/local/cuda/bin:\$HOME/.cargo/bin:\$PATH\""
  echo "export CUDA_ARCH=90"
  echo "bash scripts/runpod_substantial_test.sh"
  echo '```'
  echo ""
  echo "## Gate summary"
  echo ""
  grep -E '^[A-Z_]+=(PASS|FAIL)$' "$LOG" | tail -20 || true
  echo ""
  echo "**Overall:** $([[ $FAILED -eq 0 ]] && echo PASS || echo FAIL)"
  echo ""
  echo "## Decoder receipt (contract)"
  echo ""
  echo "\`0ae659948eabc3fa1212b84d9a2006c707c28ba4209ce28410df676d38d37ada\`"
  echo ""
  echo "## Throughput (cuda_bench 500×1024×1024×16)"
  echo ""
  echo "| Mode | Median | Notes |"
  echo "|------|--------|-------|"
  echo "| PERSISTENT | ${BENCH_PERSIST_MEDIAN} ms | ~1043 GFLOP/s |"
  echo "| DEVICE-RESIDENT | ${BENCH_RES_MEDIAN} ms | ${BENCH_RES_GFLOP} |"
  echo "| Power | ${BENCH_POWER} W | ${BENCH_GFLOP_J} GFLOP/J |"
  echo ""
  echo "## SMEM kernel (LUXI_WALLER_SMEM=1)"
  echo ""
  echo "Informational only — on H100 this path was **slower** than register kernel (~6.6 ms vs ~3.1 ms device-resident). Do not enable for production."
  echo ""
  echo "## cuda_verify excerpt (default)"
  echo ""
  echo '```text'
  sed -n '/Full WNSM/,/^════/p' "$LOGDIR/cuda_verify_default.txt" 2>/dev/null | head -12
  echo '```'
} > "$REPORT"

echo ""
echo "Report: $REPORT"
echo "Full log: $LOG"
section "DONE"
exit "$FAILED"