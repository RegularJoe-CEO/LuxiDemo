#!/bin/bash
set -e  # Exit on any error

echo "=== Setting up L4 Benchmark (CPU-only) ==="

# 1. Clean previous build artifacts
echo "Cleaning build artifacts..."
rm -f Cargo.lock
cargo clean

# 2. Remove GPU requirement from bin target (if it exists)
echo "Configuring Cargo.toml for CPU-only build..."
if grep -q "required-features = \[\"gpu\"\]" Cargo.toml; then
    sed -i '/l4_benchmark/,/required-features/d' Cargo.toml
fi

# 3. Verify/ensure dependencies are properly formatted
echo "Verifying dependencies..."
if ! grep -q "\[dependencies.reqwest\]" Cargo.toml; then
    cat >> Cargo.toml << 'EOD'

[dependencies.reqwest]
version = "0.11"
features = ["json"]

[dependencies.rand]
version = "0.8"
features = ["std_rng"]
EOD
fi

# 4. Ensure [[bin]] target exists (add if missing)
if ! grep -q "[[bin]]" Cargo.toml; then
    cat >> Cargo.toml << 'EOD'

[[bin]]
name = "l4_benchmark"
