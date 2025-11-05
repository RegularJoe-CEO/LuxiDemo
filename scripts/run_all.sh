#!/usr/bin/env bash
set -euo pipefail

REPO="${REPO:-$HOME/src/LuxiEdge}"
REPO="${REPO/#\~/$HOME}"
REPO="$(cd "$REPO" && pwd)"
PORT="${PORT:-8080}"
BASE="http://127.0.0.1:${PORT}"

pids="$(lsof -nP -iTCP:$PORT -sTCP:LISTEN | awk 'NR>1{print $2}' || true)"
if [ -n "${pids:-}" ]; then kill -9 $pids || true; fi
sleep 0.5

cargo build --manifest-path "$REPO/edge/Cargo.toml" --release

nohup "$REPO/edge/target/release/erock_edge" >/tmp/luxi_edge.log 2>&1 &
PID=$!
echo "$PID" > /tmp/luxi_edge.pid
cleanup() { kill -9 "$PID" >/dev/null 2>&1 || true; }
trap cleanup EXIT
sleep 0.5

for i in $(seq 1 50); do
  if curl -fsS "$BASE/health" >/dev/null 2>&1; then break; fi
  sleep 0.1
  if [ "$i" -eq 50 ]; then echo "Server did not become ready on $BASE"; exit 1; fi
done

echo "GET /health"
curl -fsS "$BASE/health" | python3 -m json.tool

echo "POST /evaluate"
curl -fsS -H 'content-type: application/json' -d '{"expr":"x*x + 2*x + 1","x":[0.0,1.0,2.0,3.0]}' "$BASE/evaluate" | python3 -m json.tool

echo "POST /bisect"
curl -fsS -H 'content-type: application/json' -d '{"expr":"x*x - 2","lo":1.0,"hi":2.0,"tol":1e-9,"max_iter":60}' "$BASE/bisect" | python3 -m json.tool

echo "POST /bisect_auto"
curl -fsS -H 'content-type: application/json' -d '{"expr":"x*x - 2","guess":1.0,"step":0.5,"max_expand":20,"tol":1e-9,"max_iter":60}' "$BASE/bisect_auto" | python3 -m json.tool

echo "OK"
