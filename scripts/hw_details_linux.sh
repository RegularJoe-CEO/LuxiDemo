#!/usr/bin/env bash
set -euo pipefail
OUT="${1:?out_path}"
{
  echo "=== uname -a ==="; uname -a
  echo; echo "=== lscpu ==="; lscpu || true
  echo; echo "=== free -h ==="; free -h || true
  echo; echo "=== nvidia-smi -L ==="; nvidia-smi -L || true
  echo; echo "=== nvidia-smi ==="; nvidia-smi || true
} > "$OUT"
echo "Wrote $OUT"
