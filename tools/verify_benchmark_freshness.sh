#!/usr/bin/env bash
set -euo pipefail

if ! command -v git >/dev/null 2>&1; then
  echo "git is required but was not found in PATH" >&2
  exit 1
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

echo "🔎 Checking Luxi Edge benchmark freshness" 

echo "• Current branch: $(git rev-parse --abbrev-ref HEAD)"

git fetch --quiet origin main || echo "(warning) Unable to reach origin; continuing with local data"

upstream="$(git rev-parse --verify origin/main 2>/dev/null || echo '')"
if [ -n "$upstream" ]; then
  echo "• Latest origin/main commit: $upstream"
else
  echo "• origin/main not available locally; run 'git fetch origin main' when you have network access"
fi

echo "• Local HEAD: $(git rev-parse HEAD)"

echo
if git status --short | grep -q BENCHMARK_DATA.md; then
  echo "⚠️  BENCHMARK_DATA.md has local modifications. Run 'git checkout -- BENCHMARK_DATA.md' if you want the committed version."
else
  echo "✅ BENCHMARK_DATA.md is clean in the working tree."
fi

echo
if git log -1 --stat -- "BENCHMARK_DATA.md" >/tmp/bench_stat.$$; then
  echo "Last commit touching BENCHMARK_DATA.md:"
  cat /tmp/bench_stat.$$
  rm /tmp/bench_stat.$$
else
  echo "No history found for BENCHMARK_DATA.md."
fi

echo
mapfile -t files < <(find "$repo_root" -name 'BENCHMARK_DATA*.md' -not -path '*/target/*')
if [ "${#files[@]}" -gt 1 ]; then
  echo "⚠️  Multiple benchmark summary files detected:"
  printf '   - %s\n' "${files[@]}"
  echo "    Remove any stale copies you do not need."
elif [ "${#files[@]}" -eq 1 ]; then
  echo "✅ Single benchmark summary file present at ${files[0]}"
else
  echo "⚠️  No BENCHMARK_DATA.md file found in the repository root."
fi

echo
cat <<'INSTRUCTIONS'
Next steps if the dates do not match the January 2025 revision:
1. git checkout main
2. git pull --ff-only origin main
3. Run this script again
4. If GitHub still shows an older date, hard refresh the browser or clear the cached blob
INSTRUCTIONS
