#!/usr/bin/env bash
set -euo pipefail
DUR="${1:-20}"
BS="${2:-1048576}"
A="${3:-0.2}"
mkdir -p docs/benchmarks
scripts/hw_details_linux.sh docs/docs/benchmarks/raw/hw_details_gpu.txt || true
PM="docs/docs/benchmarks/raw/torch_gpu_baseline_power.txt"
CSV="docs/docs/benchmarks/raw/torch_gpu_baseline_power.csv"
python3 -m pip install -q --user pynvml >/dev/null 2>&1 || true
python3 scripts/power_linux.py "$DUR" "$PM" &
PM_PID=$!
python3 docs/benchmarks/raw/torch_pipeline_gpu.py --a "$A" --batch-size "$BS" --duration-s "$DUR" --csv "$CSV"
wait "$PM_PID"
# Optional latency CSV
python3 - <<'PY'
import json, time, numpy as np, torch
from pathlib import Path
csv="docs/docs/benchmarks/raw/torch_gpu_baseline.csv"
meta={"framework":"pytorch","device":"cuda","mode":"baseline","a":0.2,"batch_size":1048576,"batches":200,"threads":1,"concurrency":1,"transport":"inproc"}
Path(csv).parent.mkdir(parents=True, exist_ok=True)
with open(csv,"w") as f:
    f.write("elapsed_s,samples\n")
    rng=np.random.default_rng(1337)
    x=torch.from_numpy(rng.normal(0.0,5.0,size=(meta["batch_size"],)).astype(np.float32)).cuda()
    torch.cuda.synchronize()
    def phi(x,a):
        xa=torch.abs(x); neg=x<0; pos=~neg; y=torch.empty_like(x)
        y[neg]=torch.sin(x[neg])+a*x[neg]*x[neg]
        xc=torch.clamp(x[pos], min=-0.999999)
        y[pos]=torch.log1p(xc)-torch.sqrt(xa[pos])+0.1*x[pos]*x[pos]*x[pos]
        return y
    for _ in range(200):
        t0=time.perf_counter(); _=phi(x, meta["a"]); torch.cuda.synchronize(); t1=time.perf_counter()
        f.write(f"{t1-t0:.9f},{meta['batch_size']}\n")
with open(csv.replace(".csv",".meta.json"),"w") as mf: json.dump(meta, mf)
print("Wrote", csv)
PY
python3 docs/benchmarks/raw/summarize_bench.py || true
echo "GPU powered bench files written."
ls -l docs/docs/benchmarks/raw/torch_gpu_baseline*.csv docs/docs/benchmarks/raw/torch_gpu_baseline*.txt docs/docs/benchmarks/raw/xai_integration.md || true
