#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0

# Agent smoke test script - validates Luxi Edge API correctness
# Expected to be run after server is already running on port 8080

set -euo pipefail

BASE="${1:-http://localhost:8080}"
PASS=0
FAIL=0

say() { printf '%s\n' "$*"; }
hdr() { printf '\n=== %s ===\n' "$*"; }
pass() { say "✅ PASS: $*"; PASS=$((PASS+1)); }
fail() { say "❌ FAIL: $*"; FAIL=$((FAIL+1)); }

# Test /ping endpoint
hdr "Test 1: /ping endpoint"
RESULT=$(curl -s http://localhost:8080/ping)
if [ "$RESULT" = "pong" ]; then
    pass "/ping returned 'pong'"
else
    fail "/ping returned '$RESULT' (expected 'pong')"
fi

# Test /health endpoint
hdr "Test 2: /health endpoint"
HEALTH=$(curl -s http://localhost:8080/health)
if echo "$HEALTH" | grep -q '"status":"ok"'; then
    pass "/health returned status ok"
else
    fail "/health did not return status ok: $HEALTH"
fi

# Test /evaluate with 2*x+sin(x) at x=3.14
hdr "Test 3: /evaluate endpoint"
EVAL_RESULT=$(curl -s -X POST http://localhost:8080/evaluate \
    -H "Content-Type: application/json" \
    -d '{"expr":"2*x+sin(x)", "x":[3.14]}')

# Extract the y value from response {"y":[6.281592652916487]}
Y_VALUE=$(echo "$EVAL_RESULT" | python3 -c "import sys, json; print(json.load(sys.stdin)['y'][0])" 2>/dev/null || echo "ERROR")

if [ "$Y_VALUE" != "ERROR" ]; then
    # Expected result: 2*3.14 + sin(3.14) ≈ 6.28 + 0.00159... ≈ 6.28159
    # Allow tolerance of 0.01
    python3 <<EOF
import sys
y = $Y_VALUE
expected = 6.28159
if abs(y - expected) < 0.01:
    sys.exit(0)
else:
    sys.exit(1)
EOF
    if [ $? -eq 0 ]; then
        pass "/evaluate(2*x+sin(x)) @ x=3.14 returned ~6.28159 (got $Y_VALUE)"
    else
        fail "/evaluate returned $Y_VALUE (expected ~6.28159)"
    fi
else
    fail "/evaluate endpoint error: $EVAL_RESULT"
fi

# Test /bisect with x*x - 4 (root should be near 2.0)
hdr "Test 4: /bisect endpoint"
BISECT_RESULT=$(curl -s -X POST http://localhost:8080/bisect \
    -H "Content-Type: application/json" \
    -d '{"expr":"x*x - 4", "lo":0, "hi":3}')

ROOT=$(echo "$BISECT_RESULT" | python3 -c "import sys, json; print(json.load(sys.stdin).get('root', 'null'))" 2>/dev/null || echo "ERROR")
BRACKET_OK=$(echo "$BISECT_RESULT" | python3 -c "import sys, json; print(json.load(sys.stdin).get('bracket_ok', False))" 2>/dev/null || echo "ERROR")

if [ "$ROOT" != "ERROR" ] && [ "$BRACKET_OK" = "True" ]; then
    python3 <<EOF
import sys
root = float('$ROOT')
expected = 2.0
if abs(root - expected) < 0.001:
    sys.exit(0)
else:
    sys.exit(1)
EOF
    if [ $? -eq 0 ]; then
        pass "/bisect(x*x - 4) found root near 2.0 (got $ROOT)"
    else
        fail "/bisect found root $ROOT (expected ~2.0)"
    fi
else
    fail "/bisect endpoint error: $BISECT_RESULT"
fi

# Test /bisect_auto with x*x - 4
hdr "Test 5: /bisect_auto endpoint"
BISECT_AUTO_RESULT=$(curl -s -X POST http://localhost:8080/bisect_auto \
    -H "Content-Type: application/json" \
    -d '{"expr":"x*x - 4", "guess":2.0}')

ROOT_AUTO=$(echo "$BISECT_AUTO_RESULT" | python3 -c "import sys, json; print(json.load(sys.stdin).get('root', 'null'))" 2>/dev/null || echo "ERROR")
ITERS=$(echo "$BISECT_AUTO_RESULT" | python3 -c "import sys, json; print(json.load(sys.stdin).get('iters', 999))" 2>/dev/null || echo "999")
BRACKET_OK_AUTO=$(echo "$BISECT_AUTO_RESULT" | python3 -c "import sys, json; print(json.load(sys.stdin).get('bracket_ok', False))" 2>/dev/null || echo "ERROR")

if [ "$ROOT_AUTO" != "ERROR" ] && [ "$BRACKET_OK_AUTO" = "True" ]; then
    python3 <<EOF
import sys
root = float('$ROOT_AUTO')
iters = int('$ITERS')
expected = 2.0
if abs(root - expected) < 0.001 and iters <= 35:
    sys.exit(0)
else:
    sys.exit(1)
EOF
    if [ $? -eq 0 ]; then
        pass "/bisect_auto(x*x - 4) found root ~2.0 in $ITERS iterations (≤35)"
    else
        fail "/bisect_auto found root $ROOT_AUTO in $ITERS iterations (expected ~2.0, ≤35 iters)"
    fi
else
    fail "/bisect_auto endpoint error: $BISECT_AUTO_RESULT"
fi

# Summary
hdr "Test Summary"
say "Passed: $PASS"
say "Failed: $FAIL"

if [ $FAIL -eq 0 ]; then
    say "✅ All smoke tests PASSED"
    exit 0
else
    say "❌ Some tests FAILED"
    exit 1
fi
