#!/usr/bin/env bash
set -euo pipefail

# -------------------------------
# Find a free port
# -------------------------------
PORT="$(python3 - <<'PY'
import socket
s = socket.socket()
s.bind(("",0))
print(s.getsockname()[1])
s.close()
PY
)"

echo "🔌 Using port: $PORT"

# -------------------------------
# Build
# -------------------------------
echo "🚧 Building Luxi…"
cargo build --release >/dev/null

# -------------------------------
# Launch
# -------------------------------
echo "🚀 Starting Luxi…"
RUST_LOG=info PORT=$PORT target/release/luxi & APP_PID=$!
trap "kill $APP_PID 2>/dev/null || true" EXIT

# Wait until healthy (max 5s)
for i in {1..20}; do
  if curl -sf "http://127.0.0.1:$PORT/ping" >/dev/null; then
    break
  fi
  sleep 0.25
done

echo "✅ Health check:"
curl -s "http://127.0.0.1:$PORT/ping" && echo

# -------------------------------
# Evaluate test
# -------------------------------
EVAL_OUT="$(curl -s -X POST "http://127.0.0.1:$PORT/evaluate" \
  -H 'content-type: application/json' \
  -d '{"expr":"2*x+sin(x)","x":3.14}')"

echo "✅ /evaluate → $EVAL_OUT"

# Basic correctness check
if [[ "$EVAL_OUT" != *"6.28"* ]]; then
  echo "❌ Unexpected /evaluate result"
  exit 1
fi

# -------------------------------
# Bisect test
# -------------------------------
BI_OUT="$(curl -s -X POST "http://127.0.0.1:$PORT/bisect" \
  -H 'content-type: application/json' \
  -d '{"func":"x*x-4","a":0,"b":5,"tol":1e-6}')"

echo "✅ /bisect → $BI_OUT"

if [[ "$BI_OUT" != *"2.0"* ]]; then
  echo "❌ Unexpected /bisect result"
  exit 1
fi

# -------------------------------
# Auto-bisect test
# -------------------------------
AUTO_OUT="$(curl -s -X POST "http://127.0.0.1:$PORT/bisect_auto" \
  -H 'content-type: application/json' \
  -d '{"func":"x*x-4","a":0,"b":5}')"

echo "✅ /bisect_auto → $AUTO_OUT"

echo "🎉 All smoke tests passed successfully."

