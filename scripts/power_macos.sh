#!/usr/bin/env bash
set -euo pipefail
DUR="${1:-20}"     # seconds (1 Hz samples)
OUT="${2:-powermetrics.txt}"
echo "sudo may prompt for your password..."
sudo -v
sudo powermetrics --samplers cpu_power,gpu_power --show-initial-usage -n "$DUR" -i 1000 > "$OUT"
echo "Wrote $OUT"
