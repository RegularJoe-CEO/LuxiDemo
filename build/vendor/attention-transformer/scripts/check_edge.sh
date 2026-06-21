#!/usr/bin/env bash
set -euo pipefail

# Edge / portable target verification script
# Runs the core library with the absolute minimum feature set (no rayon, no simd, no std marker)
# and verifies it still builds and the essential tests pass.
# Intended for CI and for ARM / embedded-style targets.

echo "=== Edge target check: --no-default-features + release ==="

cargo check --no-default-features
cargo test --release --no-default-features -- --quiet

echo "=== Cross-compilation smoke test for aarch64 (Apple Silicon native or linux-gnu via cross) ==="

# On Apple Silicon Macs this target is native.
if rustup target list --installed | grep -q aarch64-apple-darwin; then
    echo "Building for aarch64-apple-darwin (native on this Mac)..."
    cargo check --target aarch64-apple-darwin --no-default-features --release
else
    echo "aarch64-apple-darwin not installed; skipping native cross check."
    echo "To enable: rustup target add aarch64-apple-darwin"
fi

# For true embedded Linux ARM (requires 'cross' tool):
# cross build --target aarch64-unknown-linux-gnu --no-default-features --release

echo "=== Edge check PASSED ==="
echo "The pure-Rust math path compiles and tests pass with no rayon/simd/std features."