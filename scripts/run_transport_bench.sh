#!/usr/bin/env bash
set -euo pipefail
BS="${1:?batch_size}"
THR="${2:?threads}"
CONC="${3:?concurrency}"
DUR="${4:-20}"
UDS_PATH="${UDS_PATH:-/tmp/erock.sock}"

echo "sudo may prompt for your password..."
sudo -v

opsj() {
  python3 - "$1" "$2" <<'PY'
import sys,csv,re
csvp, txtp = sys.argv[1], sys.argv[2]
times=[]; samples=0
with open(csvp) as f:
    r=csv.DictReader(f)
    for row in r:
        times.append(float(row["elapsed_s"])); samples+=int(row["samples"])
total=sum(times)
cpu=gpu=soc=0.0; ncpu=ngpu=nsoc=0
rx=re.compile(r'(CPU|GPU|SoC).*?[Pp]ower[^:]*:\s*([0-9]*\.?[0-9]+)\s*([mM]?W)')
for line in open(txtp, errors="ignore"):
    m=rx.search(line)
    if not m: continue
    kind,val,unit=m.group(1).upper(),float(m.group(2)),m.group(3).lower()
    w = val/1000.0 if unit=="mw" else val
    if kind=="CPU": cpu+=w; ncpu+=1
    elif kind=="GPU": gpu+=w; ngpu+=1
    elif kind=="SOC": soc+=w; nsoc+=1
cpu = cpu/ncpu if ncpu else 0.0
gpu = gpu/ngpu if ngpu else 0.0
tot = cpu+gpu if (cpu+gpu)>0 else (soc/nsoc if nsoc else 0.0)
eng = tot*total if tot>0 else 0.0
opsj = (samples/eng) if eng>0 else 0.0
print(f"{opsj:.6f}")
PY
}

mkdir -p docs/benchmarks

# TCP
export LUXI_URL="http://127.0.0.1:8080"
CSV_TCP="docs/benchmarks/torch_luxi_tcp_power.csv"
PM_TCP="docs/benchmarks/torch_luxi_tcp_power.txt"
scripts/power_macos.sh "$DUR" "$PM_TCP" &
PM_PID=$!
python3 benchmarks/torch_pipeline.py --mode luxi --batch-size "$BS" --threads "$THR" --concurrency "$CONC" --duration-s "$DUR" --csv "$CSV_TCP"
wait "$PM_PID"

# UDS proxy
python3 scripts/uds_tcp_proxy.py --uds "$UDS_PATH" --tcp 127.0.0.1:8080 > docs/benchmarks/uds_proxy.log 2>&1 &
PROXY_PID=$!
for i in $(seq 1 50); do [ -S "$UDS_PATH" ] && break; sleep 0.1; done
[ -S "$UDS_PATH" ] || { echo "UDS socket did not appear"; kill "$PROXY_PID" 2>/dev/null || true; exit 1; }

# UDS
ENC_UDS=$(printf "%s" "$UDS_PATH" | sed 's|^/||; s|/|%2F|g')
export LUXI_URL="http+unix://%2F${ENC_UDS}"
CSV_UDS="docs/benchmarks/torch_luxi_uds_power.csv"
PM_UDS="docs/benchmarks/torch_luxi_uds_power.txt"
scripts/power_macos.sh "$DUR" "$PM_UDS" &
PM_PID=$!
python3 benchmarks/torch_pipeline.py --mode luxi --batch-size "$BS" --threads "$THR" --concurrency "$CONC" --duration-s "$DUR" --csv "$CSV_UDS"
wait "$PM_PID"
kill "$PROXY_PID" 2>/dev/null || true
sleep 0.3; [ -S "$UDS_PATH" ] && rm -f "$UDS_PATH" || true

# Decide winner by Ops/J
OPSJ_TCP="$(opsj "$CSV_TCP" "$PM_TCP")"
OPSJ_UDS="$(opsj "$CSV_UDS" "$PM_UDS")"
echo "Ops/J TCP: $OPSJ_TCP"
echo "Ops/J UDS: $OPSJ_UDS"
WIN="tcp"; awk -v t="$OPSJ_TCP" -v u="$OPSJ_UDS" 'BEGIN{exit !(u>t)}' && WIN="uds"
echo "Adopting $WIN as canonical torch_luxi_*"

# Adopt winner -> canonical
if [ "$WIN" = "uds" ]; then
  cp -f "$CSV_UDS" docs/benchmarks/torch_luxi_power.csv
  cp -f "$PM_UDS"  docs/benchmarks/torch_luxi_power.txt
  export LUXI_URL="http+unix://%2F${ENC_UDS}" # metadata hint
else
  cp -f "$CSV_TCP" docs/benchmarks/torch_luxi_power.csv
  cp -f "$PM_TCP"  docs/benchmarks/torch_luxi_power.txt
  export LUXI_URL="http://127.0.0.1:8080"
fi

# Refresh latency-only canonical at same tuned settings
python3 benchmarks/torch_pipeline.py --mode luxi --batch-size "$BS" --threads "$THR" --concurrency "$CONC" --batches 200 --csv docs/benchmarks/torch_luxi.csv

# Rewrite report
python3 benchmarks/summarize_bench.py
echo "Done. Canonical torch_luxi_* set to $WIN."
ls -l docs/benchmarks/torch_luxi_*_power.csv docs/benchmarks/torch_luxi_*_power.txt docs/benchmarks/torch_luxi_power.csv docs/benchmarks/torch_luxi_power.txt docs/benchmarks/torch_luxi.csv docs/benchmarks/xai_integration.md
