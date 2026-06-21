#!/usr/bin/env bash
# Wipe local repo copy on RunPod and clone fresh from GitHub main, then build + verify + bench.
#
# Run on the pod (one paste):
#   curl -fsSL https://raw.githubusercontent.com/RegularJoe-CEO/attention-transformer/main/scripts/runpod_fresh_deploy.sh | bash
#
# Or if you already have any clone:
#   bash scripts/runpod_fresh_deploy.sh

set -euo pipefail

REPO_URL="${REPO_URL:-https://github.com/RegularJoe-CEO/attention-transformer.git}"
INSTALL_DIR="${INSTALL_DIR:-/workspace/attention-transformer}"
BRANCH="${BRANCH:-main}"

export PATH="/usr/local/cuda/bin:${PATH:-}"
export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
export RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}"

LOGFILE="${INSTALL_DIR}/runpod_fresh_deploy_$(date +%Y%m%d_%H%M%S).log"
mkdir -p "$(dirname "$INSTALL_DIR")"

exec > >(tee -a "$LOGFILE") 2>&1

echo "════════════════════════════════════════════════════════════════════════"
echo " attention-transformer — RunPod FRESH DEPLOY"
echo " $(date -u)"
echo " repo=$REPO_URL branch=$BRANCH dir=$INSTALL_DIR"
echo "════════════════════════════════════════════════════════════════════════"

echo "=== GPU ==="
nvidia-smi

echo "=== CUDA ==="
command -v nvcc && nvcc --version || echo "WARN: nvcc not in PATH (try export PATH=/usr/local/cuda/bin:\$PATH)"

echo "=== Rust ==="
if ! command -v cargo >/dev/null 2>&1; then
  echo "Installing rustup..."
  apt-get update -y || true
  apt-get install -y build-essential curl pkg-config libssl-dev git || true
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal
fi
# shellcheck source=/dev/null
source "$CARGO_HOME/env" 2>/dev/null || true
export PATH="$CARGO_HOME/bin:$PATH"
rustc --version
cargo --version

echo "=== Wipe old tree ==="
if [[ -d "$INSTALL_DIR" ]]; then
  BACKUP="${INSTALL_DIR}.bak.$(date +%Y%m%d_%H%M%S)"
  echo "Moving $INSTALL_DIR -> $BACKUP"
  mv "$INSTALL_DIR" "$BACKUP"
fi

echo "=== Fresh clone ==="
git clone --depth 1 --branch "$BRANCH" "$REPO_URL" "$INSTALL_DIR"
cd "$INSTALL_DIR"
echo "HEAD=$(git rev-parse HEAD)"
git log -1 --oneline

echo "=== Build (CUDA) ==="
cargo build --release --features cuda

echo "=== cuda_verify ==="
cargo run --release --features cuda --example cuda_verify

echo "=== cuda_bench ==="
cargo run --release --features cuda --example cuda_bench -- 500 1024 1024 16

echo "=== Done ==="
echo "Log: $LOGFILE"
echo "Expected decoder receipt: 0ae659948eabc3fa1212b84d9a2006c707c28ba4209ce28410df676d38d37ada"
echo "════════════════════════════════════════════════════════════════════════"