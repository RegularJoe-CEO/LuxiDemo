#!/usr/bin/env bash
set -euo pipefail

EXPECTED_DATE="2025-01-18"
FILE="BENCHMARK_DATA.md"

if ! command -v git >/dev/null 2>&1; then
  echo "[error] git is required to verify benchmark freshness" >&2
  exit 1
fi

if [ ! -f "$FILE" ]; then
  echo "[error] $FILE is missing in $(pwd)" >&2
  exit 1
fi

branch=$(git rev-parse --abbrev-ref HEAD)
if [ "$branch" != "main" ]; then
  echo "[warn] You are on branch '$branch'. Switch to 'main' for the canonical benchmark data." >&2
else
  echo "[ok] On main branch"
fi

git fetch --quiet --all --tags || echo "[warn] Unable to contact remotes; continuing with local data only"

latest_subject=$(git log -1 --pretty=%s -- "$FILE")
latest_date=$(git log -1 --pretty=%cs -- "$FILE")

if [[ "$latest_date" < "$EXPECTED_DATE" ]]; then
  echo "[warn] $FILE is currently on '$latest_subject' dated $latest_date" >&2
  echo "[hint] Run 'git pull --rebase origin main' to sync the latest benchmark updates." >&2
else
  echo "[ok] $FILE commit date $latest_date (subject: $latest_subject) is at or newer than $EXPECTED_DATE"
fi

if ! grep -q "Last updated: $EXPECTED_DATE" "$FILE"; then
  echo "[warn] File header does not list $EXPECTED_DATE; browser/editor caching may be serving an older copy." >&2
else
  echo "[ok] Header confirms Last updated: $EXPECTED_DATE"
fi

missing_docs=0
for path in docs/benchmarks docs/benchmarks/COMPARATIVE_ANALYSIS.md docs/benchmarks/FINDING_DATA.md docs/benchmarks/README.md; do
  if [ ! -e "$path" ]; then
    echo "[warn] Missing benchmark companion artifact: $path" >&2
    missing_docs=1
  fi
done

if [ $missing_docs -eq 0 ]; then
  echo "[ok] Benchmark companion documents are present"
fi

echo "[info] To clear stale exports, run: git clean -fd docs/benchmarks/data_exports/" >&2

