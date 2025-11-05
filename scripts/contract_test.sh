#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-http://localhost:8080}"

say() { printf '%s\n' "$*"; }
hdr() { printf '\n=== %s ===\n' "$*"; }

json_field() { python3 - "$1" "$2" <<'PY'
import sys, json
doc=json.loads(sys.argv[1])
key=sys.argv[2]
print(doc.get(key))
PY
}

status_and_body() {
  # prints "<code>\n<body>"
  URL="$1"; BODY="$2"
  CODE=$(curl -sS -o /tmp/luxi_tmp.json -w '%{http_code}' \
    -H 'Content-Type: application/json' -d "$BODY" "$URL") || true
  printf '%s\n' "$CODE"
  cat /tmp/luxi_tmp.json 2>/dev/null || true
}

hdr "Health"
curl -fsS "$BASE/health" >/dev/null
say "OK"

hdr "Evaluate"
EVAL_BODY='{"expression":"x*x + 2*x + 1","x":3.0}'
read -r CODE BODY < <(status_and_body "$BASE/evaluate" "$EVAL_BODY")
[ "$CODE" = "200" ] || { say "Evaluate failed: $CODE"; echo "$BODY"; exit 1; }
RES=$(json_field "$BODY" "result")
say "result=$RES (expected ~16)"

hdr "Bisect"
BI_BODY='{"expression":"x*x - x - 1","a":0.0,"b":2.0,"tolerance":1e-9}'
read -r CODE BODY < <(status_and_body "$BASE/bisect" "$BI_BODY")
[ "$CODE" = "200" ] || { say "Bisect failed: $CODE"; echo "$BODY"; exit 1; }
BOK=$(json_field "$BODY" "bracket_ok")
say "bracket_ok=$BOK"

hdr "Bisect Auto"
BA_BODY='{"expression":"x*x - 2","tolerance":1e-9}'
read -r CODE BODY < <(status_and_body "$BASE/bisect_auto" "$BA_BODY")
[ "$CODE" = "200" ] || { say "Bisect Auto failed: $CODE"; echo "$BODY"; exit 1; }
BOK=$(json_field "$BODY" "bracket_ok")
say "bracket_ok=$BOK"

hdr "Precision (optional, should be ignored gracefully by older servers)"
for EP in evaluate bisect bisect_auto; do
  case "$EP" in
    evaluate) BODY="$EVAL_BODY" ;;
    bisect) BODY="$BI_BODY" ;;
    bisect_auto) BODY="$BA_BODY" ;;
  esac
  read -r CODE BODYOUT < <(status_and_body "$BASE/$EP?precision=f32" "$BODY")
  if [ "$CODE" = "200" ]; then
    say "$EP with ?precision=f32 -> 200 OK (server accepted or ignored)"
  else
    say "$EP with ?precision=f32 -> $CODE (non-blocking; server likely blocks unknown query)"
  fi
done

say "All core checks passed."
