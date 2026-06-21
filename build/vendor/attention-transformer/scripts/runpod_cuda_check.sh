#!/usr/bin/env bash
# attention-transformer — RunPod CUDA Verification Check (robust version)
# This script is designed to be as self-contained as possible on minimal
# RunPod CUDA templates (which often have no Rust pre-installed).

set -euo pipefail

LOGFILE="runpod_cuda_check_$(date +%Y%m%d_%H%M%S).log"
exec > >(tee -a "$LOGFILE") 2>&1

echo "════════════════════════════════════════════════════════════════════════"
echo "   attention-transformer — RunPod CUDA Verification Check"
echo "   $(date -u)"
echo "════════════════════════════════════════════════════════════════════════"
echo
echo "=== 0. Purpose ==="
echo "First-time RunPod AUDIT gate: prove CUDA decoder matches CPU receipts."
echo "The pure-Rust f32 CPU path is the canonical source of truth."
echo "Pass criteria: FULL DECODER CUDA PATH VERIFIED, max_diff 0.00e0."
echo "Current ops and locked TRADE benchmarks: docs/QUANT_TRADE_LOCKED.md"
echo

echo "=== 1. System Information ==="
uname -a || true
cat /etc/os-release 2>/dev/null || true
echo

echo "=== 2. NVIDIA GPU ==="
nvidia-smi || {
    echo "ERROR: nvidia-smi failed. This pod does not have a usable NVIDIA GPU."
    exit 1
}
echo

echo "=== 3. CUDA Toolkit ==="
if command -v nvcc >/dev/null 2>&1; then
    nvcc --version
else
    echo "WARNING: nvcc not found in PATH."
fi
echo "CUDA_VERSION=${CUDA_VERSION:-unknown}"
echo

# --- Robust Rust installation (the main fix for real pods) ---
echo "=== 4. Ensuring Rust Toolchain is Present ==="
export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
export RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}"

if ! command -v cargo >/dev/null 2>&1; then
    echo "Rust not found — installing via rustup (this can take 1-3 minutes)..."
    apt-get update -y || true
    apt-get install -y build-essential curl pkg-config libssl-dev || true

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal
fi

# Make sure cargo/rustc are on PATH for the rest of this script and the build
source "$CARGO_HOME/env" 2>/dev/null || true
export PATH="$CARGO_HOME/bin:$PATH"

if command -v rustc >/dev/null 2>&1 && command -v cargo >/dev/null 2>&1; then
    echo "Rust toolchain ready:"
    rustc --version
    cargo --version
else
    echo "ERROR: Still cannot find rustc/cargo after installation attempt."
    echo "Manual recovery (run these on the pod):"
    echo '  curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y'
    echo '  source "$HOME/.cargo/env"'
    echo '  export PATH="$HOME/.cargo/bin:$PATH"'
    exit 1
fi
echo

echo "=== 5. Git State ==="
git status --short || true
git rev-parse --abbrev-ref HEAD || true
git rev-parse HEAD || true
echo

echo "=== 6. Build with CUDA Feature ==="
cargo build --release --features cuda
echo

echo "=== 7. Run CUDA Verification Harness (Lane AUDIT) ==="
export LUXI_RECEIPT_AUDIT=1
cargo run --release --features cuda --example cuda_verify
echo
echo "=== 8. Layer bench snapshot (Lane TRADE, optional) ==="
cargo run --release --features cuda --example cuda_layer_bench -- 5 128 1024 16 256 || true
echo

echo "=== Done ==="
echo "Full log saved to: $LOGFILE"
echo "Please copy the entire log (especially the receipts and diff numbers) back to the owner."
echo "════════════════════════════════════════════════════════════════════════"