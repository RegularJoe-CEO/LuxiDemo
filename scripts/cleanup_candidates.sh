#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0

# cleanup_candidates.sh
# Non-destructive repository cleanup automation script
# Implements the file moves described in the repository restructure plan

set -euo pipefail

echo "================================================"
echo "LuxiEdge Repository Cleanup Script"
echo "================================================"
echo ""
echo "This script performs non-destructive moves to:"
echo "  - Create holding/ for legacy/backup files"
echo "  - Reorganize benchmark documentation"
echo "  - Consolidate agent integration docs"
echo ""

# Change to repository root
cd "$(dirname "$0")/.."
REPO_ROOT="$(pwd)"
echo "Repository root: $REPO_ROOT"
echo ""

# Create holding directory if it doesn't exist
if [ ! -d "holding" ]; then
    echo "✓ Creating holding/ directory..."
    mkdir -p holding
else
    echo "✓ holding/ directory already exists"
fi

# Move backup Cargo.toml files
echo ""
echo "Moving backup files..."
for file in Cargo.toml.bak.*; do
    if [ -f "$file" ]; then
        echo "  - Moving $file to holding/"
        git mv "$file" holding/ || mv "$file" holding/
    fi
done

# Move documentation files
for file in README_DOCKER_PUBLISH.md TECHNICAL_OVERVIEW.md; do
    if [ -f "$file" ]; then
        echo "  - Moving $file to holding/"
        git mv "$file" holding/ || mv "$file" holding/
    fi
done

# Move load_test.lua if load_test.rs exists (keep Rust version)
if [ -f "load_test.lua" ] && [ -f "load_test.rs" ]; then
    echo "  - Moving load_test.lua to holding/ (keeping load_test.rs)"
    git mv load_test.lua holding/ || mv load_test.lua holding/
fi

# Move directories
echo ""
echo "Moving directories..."
for dir in marketing-site products .reuse LICENSES; do
    if [ -d "$dir" ]; then
        echo "  - Moving $dir/ to holding/"
        git mv "$dir" holding/ || mv "$dir" holding/
    fi
done

# Move benchmarks directory to docs/benchmarks/raw if not already done
echo ""
echo "Organizing benchmark resources..."
if [ -d "benchmarks" ] && [ ! -d "docs/benchmarks/raw" ]; then
    echo "  - Moving benchmarks/ to docs/benchmarks/raw/"
    mkdir -p docs/benchmarks
    git mv benchmarks docs/benchmarks/raw || mv benchmarks docs/benchmarks/raw
elif [ -d "docs/benchmarks/raw" ]; then
    echo "  ✓ docs/benchmarks/raw/ already exists"
fi

# Move BENCHMARK_DATA.md if not already moved
if [ -f "BENCHMARK_DATA.md" ] && [ ! -f "docs/benchmarks/BENCHMARK_DATA.md" ]; then
    echo "  - Moving BENCHMARK_DATA.md to docs/benchmarks/"
    mkdir -p docs/benchmarks
    git mv BENCHMARK_DATA.md docs/benchmarks/ || mv BENCHMARK_DATA.md docs/benchmarks/
elif [ -f "docs/benchmarks/BENCHMARK_DATA.md" ]; then
    echo "  ✓ BENCHMARK_DATA.md already in docs/benchmarks/"
fi

echo ""
echo "================================================"
echo "Cleanup complete!"
echo "================================================"
echo ""
echo "Summary of changes:"
echo "  - Legacy/backup files moved to holding/"
echo "  - Benchmark documentation centralized in docs/benchmarks/"
echo "  - Repository structure streamlined"
echo ""
echo "Next steps:"
echo "  1. Review changes with: git status"
echo "  2. Commit changes with: git commit -m 'Repository cleanup and restructure'"
echo "  3. Verify links in README.md are correct"
echo ""
