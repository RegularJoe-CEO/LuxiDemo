#!/usr/bin/env bash
set -euo pipefail

DUR="${1:-10}"          # seconds per test combo
SELECT="${2:-opsj}"     # 'opsj' or 'throughput'
BATCHES="${BATCHES:-2048 4096 8192 16384}"
THREADS="${THREADS:-1 2 4}"
CONCS="${CONCURRENCY:-1 2 4}"

echo "sudo may prompt for your password..."
sudo -v

TUNE_OUT="docs/docs/benchmarks/raw/tuning_results.csv"
rm -f "$TUNE_OUT"

for bs in $BATCHES; do
  for t in $THREADS; do
    for c in $CONCS; do
      base="docs/docs/benchmarks/raw/torch_luxi_tune_bs${bs}_t${t}_c${c}"
      csv="${base}.csv"
      pm="${base}.txt"
      echo "=== bs=${bs} threads=${t} conc=${c} dur=${DUR}s ==="
      scripts/power_macos.sh "$DUR" "$pm" &
      PM_PID=$!
      python3 docs/benchmarks/raw/torch_pipeline.py --mode luxi --batch-size "$bs" --threads "$t" --concurrency "$c" --duration-s "$DUR" --csv "$csv"
      wait "$PM_PID"
      python3 docs/benchmarks/raw/aggregate_tuning.py "$csv" "$pm" "$TUNE_OUT"
    done
  done
done

# Pick best combo
python3 - "$TUNE_OUT" "$SELECT" <<'PY'
import csv, json, sys
path, key = sys.argv[1], sys.argv[2]
rows=[]
with open(path) as f:
    r=csv.DictReader(f)
    for row in r:
        try:
            row["_ops_per_j"]=float(row["ops_per_j"])
            row["_thr"]=float(row["throughput_sps"])
            rows.append(row)
        except: pass
if not rows:
    print("NO_ROWS"); sys.exit(0)
if key=="throughput":
    rows.sort(key=lambda r: r["_thr"], reverse=True)
else:
    rows.sort(key=lambda r: r["_ops_per_j"], reverse=True)
best=rows[0]
print(json.dumps({"batch_size": int(best["batch_size"]), "threads": int(best["threads"]), "concurrency": int(best["concurrency"])}))
PY
