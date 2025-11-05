#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-http://127.0.0.1:8080}"

echo "GET /health"
curl -fsS "$BASE/health" | python3 -m json.tool

echo "POST /evaluate"
curl -fsS -H 'content-type: application/json' -d '{
  "expr": "x*x + 2*x + 1",
  "x": [0.0, 1.0, 2.0, 3.0]
}' "$BASE/evaluate" | python3 -m json.tool

echo "POST /bisect"
curl -fsS -H 'content-type: application/json' -d '{
  "expr": "x*x - 2",
  "lo": 1.0,
  "hi": 2.0,
  "tol": 1e-9,
  "max_iter": 60
}' "$BASE/bisect" | python3 -m json.tool

echo "POST /bisect_auto"
curl -fsS -H 'content-type: application/json' -d '{
  "expr": "x*x - 2",
  "guess": 1.0,
  "step": 0.5,
  "max_expand": 20,
  "tol": 1e-9,
  "max_iter": 60
}' "$BASE/bisect_auto" | python3 -m json.tool

echo "OK"
