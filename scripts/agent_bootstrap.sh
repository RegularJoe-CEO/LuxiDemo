#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0

# Agent bootstrap script - one command to build, run, and test Luxi Edge
# Usage: bash scripts/agent_bootstrap.sh

set -euo pipefail

say() { printf '%s\n' "$*"; }
hdr() { printf '\n=== %s ===\n' "$*"; }
err() { say "ERROR: $*" >&2; exit 1; }

# Change to repository root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

hdr "Luxi Edge Agent Bootstrap"
say "Repository: $REPO_ROOT"

# Check prerequisites
hdr "Checking Prerequisites"
command -v cargo >/dev/null 2>&1 || err "cargo not found. Install Rust from https://rustup.rs/"
command -v curl >/dev/null 2>&1 || err "curl not found. Please install curl."
say "✅ cargo found: $(cargo --version)"
say "✅ curl found: $(curl --version | head -1)"

# Build release binary
hdr "Building Luxi Edge (release mode)"
cd "$REPO_ROOT/edge"
cargo build --release || err "Build failed"
say "✅ Build successful"

# Start server in background
hdr "Starting Luxi Edge Server"
EDGE_BIN="$REPO_ROOT/edge/target/release/erock_edge"
if [ ! -x "$EDGE_BIN" ]; then
    err "Edge binary not found at $EDGE_BIN"
fi

# Kill any existing server
pkill -f erock_edge 2>/dev/null || true
sleep 1

# Start new server
"$EDGE_BIN" > /tmp/luxi_edge.log 2>&1 &
SERVER_PID=$!
say "Server started with PID: $SERVER_PID"
say "Log file: /tmp/luxi_edge.log"

# Wait for server to be ready
say "Waiting for server to start..."
for i in {1..10}; do
    if curl -s http://localhost:8080/ping >/dev/null 2>&1; then
        say "✅ Server is ready"
        break
    fi
    if [ $i -eq 10 ]; then
        kill $SERVER_PID 2>/dev/null || true
        cat /tmp/luxi_edge.log
        err "Server failed to start after 10 seconds"
    fi
    sleep 1
done

# Run smoke tests
hdr "Running Smoke Tests"
"$REPO_ROOT/scripts/agent_smoke.sh" http://localhost:8080

SMOKE_EXIT=$?

# Cleanup
hdr "Cleanup"
kill $SERVER_PID 2>/dev/null || true
say "Server stopped"

if [ $SMOKE_EXIT -eq 0 ]; then
    hdr "🎉 SUCCESS"
    say "Luxi Edge is built, running, and all tests passed!"
    say ""
    say "To start the server manually:"
    say "  cd $REPO_ROOT/edge"
    say "  ./target/release/erock_edge"
    say ""
    say "To run tests again:"
    say "  bash $REPO_ROOT/scripts/agent_smoke.sh"
    exit 0
else
    hdr "❌ FAILURE"
    say "Some smoke tests failed. Check logs above."
    say "Server log: /tmp/luxi_edge.log"
    exit 1
fi
