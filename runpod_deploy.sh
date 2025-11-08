#!/bin/bash
# RunPod GPU Deployment Script for Luxi Edge Benchmark
# Upload this to your RunPod instance and run it

set -e

echo "🚀 Luxi Edge RunPod GPU Benchmark Setup"
echo "========================================"

# Install dependencies
echo "📦 Installing dependencies..."
pip3 install pynvml requests numpy -q

# Check GPU
echo "🎮 Checking GPU..."
nvidia-smi || echo "⚠️  GPU check failed"

# Build Luxi Edge
echo "🔨 Building Luxi Edge..."
cd /workspace
cargo build --release --bin l4_benchmark

# Start server in background
echo "🌐 Starting Luxi Edge server..."
pkill -f l4_benchmark || true
nohup ./target/release/l4_benchmark > /tmp/l4_benchmark.log 2>&1 &
SERVER_PID=$!
echo "   Server PID: $SERVER_PID"

# Wait for server to start
echo "⏳ Waiting for server startup..."
sleep 5

# Check if server is running
if curl -s http://127.0.0.1:8080/health > /dev/null; then
    echo "✅ Server is running"
else
    echo "❌ Server failed to start"
    cat /tmp/l4_benchmark.log
    exit 1
fi

# Run benchmark
echo ""
echo "🔥 Running GPU Benchmark..."
echo "========================================"
python3 benchmark_4m.py

echo ""
echo "✅ Benchmark Complete!"
echo "   Server logs: /tmp/l4_benchmark.log"
echo "   Server PID: $SERVER_PID"
echo ""
echo "To stop server: kill $SERVER_PID"
