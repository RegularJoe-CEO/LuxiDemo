#!/bin/bash
set -euo pipefail  # Strict mode: exit on error, undefined vars, pipe fail

MODE="${1:-luxi}"  # Default to luxi if not provided
DURATION=20  # Steady-state seconds
OUTPUT_DIR="docs/benchmarks"
mkdir -p "$OUTPUT_DIR"

case "$MODE" in
  "baseline")
    echo "Running baseline (in-process PyTorch) for $DURATION s..."
    # Run baseline load (adjust path if needed)
    python3 benchmarks/torch_baseline_bench.py --duration "$DURATION" > "${OUTPUT_DIR}/torch_baseline_${DURATION}s.txt" 2>&1 &
    BASE_PID=$!
    ;;
  "luxi")
    echo "Running Luxi mode for $DURATION s..."
    # Ensure server running (kill/restart if needed)
    lsof -ti:8080 | xargs kill -9 2>/dev/null || true
    cargo run -p erock_edge --release > /dev/null 2>&1 &
    SERVER_PID=$!
    sleep 5  # Wait for server startup
    # Run Luxi load (via torch_luxi or similar—adjust if path differs)
    python3 benchmarks/torch_luxi_bench.py --duration "$DURATION" --mode luxi > "${OUTPUT_DIR}/torch_luxi_${DURATION}s.txt" 2>&1 &
    LUXI_PID=$!
    ;;
  *)
    echo "Error: Invalid mode '$MODE'. Use 'baseline' or 'luxi'."
    exit 1
    ;;
esac

# Capture power metrics (requires sudo for powermetrics)
echo "Capturing power metrics for $DURATION s (sudo required)..."
sudo ./scripts/power_macos.sh "${MODE}" "$DURATION" > "${OUTPUT_DIR}/${MODE}_${DURATION}s_power.txt" 2>&1

# Wait for load to finish
wait $LUXI_PID 2>/dev/null || wait $BASE_PID 2>/dev/null || true
# Kill server if Luxi mode
if [ "$MODE" = "luxi" ]; then
  kill $SERVER_PID 2>/dev/null || true
fi

echo "Benchmark complete. Outputs in $OUTPUT_DIR/ (*_${DURATION}s.txt, *_power.txt)."