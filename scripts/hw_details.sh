#!/usr/bin/env bash
set -euo pipefail
OUT="${1:-hw_details.txt}"

iso_ts() {
  # Portable ISO-8601 UTC timestamp
  if date --version >/dev/null 2>&1; then
    date -u +"%Y-%m-%dT%H:%M:%SZ"
  else
    date -u +"%Y-%m-%dT%H:%M:%SZ"
  fi
}

{
  echo "=== Timestamp ==="
  iso_ts
  echo
  echo "=== OS & Kernel ==="
  uname -a || true
  sw_vers 2>/dev/null || true
  lsb_release -a 2>/dev/null || true
  cat /etc/os-release 2>/dev/null || true
  echo
  echo "=== CPU ==="
  sysctl -n machdep.cpu.brand_string 2>/dev/null || true
  lscpu 2>/dev/null || true
  grep 'model name' /proc/cpuinfo 2>/dev/null | head -n1 || true
  echo
  echo "=== Memory ==="
  vm_stat 2>/dev/null || true
  free -h 2>/dev/null || true
  echo
  echo "=== Python/Torch/TF ==="
  python3 -c "import sys; print(sys.version)" || true
  python3 -c "import torch; print('torch', torch.__version__)" 2>/dev/null || echo "torch: not installed"
  python3 -c "import tensorflow as tf; print('tensorflow', tf.__version__)" 2>/dev/null || echo "tensorflow: not installed"
  echo
  echo "=== Listeners (8080/50051/8081/50052) ==="
  if command -v lsof >/dev/null; then
    lsof -nP -iTCP -sTCP:LISTEN | egrep '50051|8080|8081|50052' || true
  fi
} > "$OUT"
echo "Wrote $OUT"
