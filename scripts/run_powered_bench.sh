#!/usr/bin/env bash
set -euo pipefail
# Usage: run_powered_bench.sh <framework: torch|tf> <mode: baseline|luxi> <duration_s>
FW="${1:?torch|tf}"
MODE="${2:?baseline|luxi}"
DUR="${3:?seconds}"

CSV="docs/benchmarks/${FW}_${MODE}_power.csv"
PM="docs/benchmarks/${FW}_${MODE}_power.txt"

echo "sudo may prompt for your password..."
sudo -v

# Start powermetrics in background
scripts/power_macos.sh "$DUR" "$PM" &
PM_PID=$!

# Run the bench for exactly DUR seconds (steady-state mode)
if [ "$FW" = "torch" ]; then
  python3 benchmarks/torch_pipeline.py --mode "$MODE" --batch-size 8192 --threads 1 --duration-s "$DUR" --csv "$CSV"
elif [ "$FW" = "tf" ]; then
  if python3 - <<'PY' >/dev/null 2>&1
import tensorflow as tf
PY
  then
    python3 benchmarks/tf_pipeline.py --mode "$MODE" --batch-size 8192 --threads 1 --duration-s "$DUR" --csv "$CSV"
  else
    echo "TensorFlow not available; skipping $FW $MODE"
    kill "$PM_PID" 2>/dev/null || true
    wait "$PM_PID" 2>/dev/null || true
    exit 0
  fi
else
  echo "Unknown framework '$FW'"; exit 2
fi

# Wait for powermetrics to finish its fixed samples
wait "$PM_PID"

echo "Wrote $CSV and $PM"
