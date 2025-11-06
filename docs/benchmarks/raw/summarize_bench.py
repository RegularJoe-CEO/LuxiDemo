import os, csv, json, re, glob, statistics as stats, pathlib

OUT_MD = pathlib.Path("docs/benchmarks/xai_integration.md")

def summarize_latency(csv_path):
    times = []
    samples = 0
    with open(csv_path) as f:
        r = csv.DictReader(f)
        for row in r:
            try:
                t = float(row["elapsed_s"])
                n = int(row["samples"])
            except Exception:
                continue
            times.append(t)
            samples += n
    if not times:
        return None
    total = sum(times)
    med_ms = stats.median(times) * 1000.0
    try:
        p95_ms = stats.quantiles(times, n=20)[18] * 1000.0 if len(times) >= 20 else max(times) * 1000.0
    except Exception:
        p95_ms = max(times) * 1000.0
    meta = {}
    meta_path = csv_path.replace(".csv", ".meta.json")
    if os.path.exists(meta_path):
        try:
            with open(meta_path) as mf:
                meta = json.load(mf)
        except Exception:
            meta = {}
    thr_sps = samples / total if total > 0 else 0.0
    return {
        "file": csv_path,
        "samples": samples,
        "bench_time_s": total,
        "median_ms": med_ms,
        "p95_ms": p95_ms,
        "throughput_sps": thr_sps,
        "meta": meta,
    }

def parse_power(pm_txt_path):
    # Accept Apple powermetrics and Linux NVML/rapl formats like "CPU Power: 10.2 W", "GPU Power: 35.6 W"
    rx = re.compile(r'(CPU|GPU|SoC)\s*Power[^:]*:\s*([0-9]*\.?[0-9]+)\s*([mM]?W)')
    cpu_ws, gpu_ws, soc_ws = [], [], []
    with open(pm_txt_path, "r", errors="ignore") as f:
        for line in f:
            m = rx.search(line)
            if not m:
                continue
            kind, val, unit = m.group(1).upper(), float(m.group(2)), m.group(3).lower()
            w = val / 1000.0 if unit == "mw" else val
            if kind == "CPU":
                cpu_ws.append(w)
            elif kind == "GPU":
                gpu_ws.append(w)
            elif kind == "SOC":
                soc_ws.append(w)
    pm_samples = max(len(cpu_ws), len(gpu_ws), len(soc_ws), 0)
    cpu_avg = sum(cpu_ws) / len(cpu_ws) if cpu_ws else 0.0
    gpu_avg = sum(gpu_ws) / len(gpu_ws) if gpu_ws else 0.0
    total_w = cpu_avg + gpu_avg
    if total_w == 0.0 and soc_ws:
        total_w = sum(soc_ws) / len(soc_ws)
    return {"duration_pm_s": float(pm_samples), "cpu_w": cpu_avg, "gpu_w": gpu_avg, "total_w": total_w}

def sec(w, title):
    w.write(f"## {title}\n\n")

def write_latency(w, s):
    meta, name = s["meta"], pathlib.Path(s["file"]).name
    w.write(f"### {name}\n")
    w.write(f"- Framework: {meta.get('framework','?')}  Mode: {meta.get('mode','?')}\n")
    if "device" in meta:
        w.write(f"- Device: {meta.get('device')}\n")
    w.write(f"- Batch size: {meta.get('batch_size','?')}  Batches: {meta.get('batches','?')}  Threads: {meta.get('threads','?')}\n")
    if "concurrency" in meta:
        w.write(f"- Concurrency: {meta.get('concurrency')}\n")
    if meta.get("mode") == "luxi":
        w.write(f"- Transport: {meta.get('transport','tcp')}\n")
    w.write(f"- Median batch latency: {s['median_ms']:.3f} ms  p95: {s['p95_ms']:.3f} ms\n")
    w.write(f"- Throughput (samples/s, compute-time): {s['throughput_sps']:.1f}\n\n")

def write_energy(w, power_csv, s_lat):
    pm_txt = power_csv.replace(".csv", ".txt")
    if not os.path.exists(pm_txt):
        return
    pm = parse_power(pm_txt)
    dur_compute = s_lat["bench_time_s"]
    dur_pm = pm["duration_pm_s"]
    dur_used = dur_compute if dur_compute > 0 else dur_pm
    samples = s_lat["samples"]
    energy_j = pm["total_w"] * dur_used
    ops_per_j = (samples / energy_j) if energy_j > 0 else 0.0
    meta = s_lat["meta"]
    name = pathlib.Path(power_csv).name
    w.write(f"### {name} (energy)\n")
    if meta.get("mode") == "luxi":
        w.write(f"- Transport: {meta.get('transport','tcp')}\n")
    if "device" in meta:
        w.write(f"- Device: {meta.get('device')}\n")
    w.write(f"- Duration (compute-time used): {dur_used:.1f} s  [compute={dur_compute:.1f}s, pm={dur_pm:.1f}s]\n")
    w.write(f"- Avg CPU: {pm['cpu_w']:.2f} W  Avg GPU: {pm['gpu_w']:.2f} W  Total: {pm['total_w']:.2f} W\n")
    w.write(f"- Samples processed: {samples}  Energy: {energy_j:.2f} J\n")
    w.write(f"- Ops/J (operation = one expression evaluation per x): {ops_per_j:.2f}\n\n")

def load_hw_block():
    buf = []
    for hp in ("docs/benchmarks/hw_details.txt", "docs/benchmarks/hw_details_gpu.txt"):
        if os.path.exists(hp):
            try:
                with open(hp) as f:
                    buf.append(f.read())
            except Exception:
                pass
    return "\n".join(buf).strip()

def load_algorithm_block():
    for ap in ("ALGORITHM_DETAILS.md", "docs/ALGORITHM_DETAILS.md", "docs/benchmarks/ALGORITHM_DETAILS.md"):
        if os.path.exists(ap):
            try:
                with open(ap) as f:
                    return f.read().strip()
            except Exception:
                pass
    return ""

# Collect CSVs
csvs = []
for f in (
    "docs/benchmarks/torch_baseline.csv",
    "docs/benchmarks/torch_luxi.csv",
    "docs/benchmarks/tf_baseline.csv",
    "docs/benchmarks/tf_luxi.csv",
    "docs/benchmarks/torch_gpu_baseline.csv",
):
    if os.path.exists(f):
        csvs.append(f)
for f in sorted(glob.glob("docs/benchmarks/*_power.csv")):
    csvs.append(f)

summaries = []
for f in csvs:
    try:
        s = summarize_latency(f)
        if s:
            summaries.append(s)
    except Exception:
        continue

with open(OUT_MD, "w") as w:
    w.write("# Luxi Edge xAI Pipeline Integration Benchmark\n\n")
    w.write("_Canonical metric: 20 s steady-state ops/J from powermetrics/NVML (CPU+GPU). Compute-only throughput is de-emphasized._\n\n")

    hw = load_hw_block()
    if hw:
        sec(w, "Host details")
        w.write("```\n" + hw + "\n```\n\n")

    alg = load_algorithm_block()
    if alg:
        sec(w, "Algorithm details (from ALGORITHM_DETAILS.md)")
        w.write(alg + "\n\n")

    sec(w, "Latency and throughput")
    if not summaries:
        w.write("_No benchmark CSVs found._\n\n")
    else:
        for s in summaries:
            write_latency(w, s)

    sec(w, "Energy and ops/J (steady-state runs)")
    for s in summaries:
        if s["file"].endswith("_power.csv"):
            write_energy(w, s["file"], s)

    # Optional tuning section
    tune = "docs/benchmarks/tuning_results.csv"
    if os.path.exists(tune):
        try:
            rows = []
            with open(tune) as f:
                r = csv.DictReader(f)
                for row in r:
                    try:
                        row["_ops_per_j"] = float(row["ops_per_j"])
                        row["_thr"] = float(row["throughput_sps"])
                        rows.append(row)
                    except Exception:
                        pass
            if rows:
                sec(w, "Tuning sweeps (PyTorch + Luxi)")
                top_opsj = sorted(rows, key=lambda r: r["_ops_per_j"], reverse=True)[:5]
                top_thr = sorted(rows, key=lambda r: r["_thr"], reverse=True)[:5]
                w.write("Top 5 by Ops/J:\n\n")
                for r in top_opsj:
                    w.write(f"- bs={r['batch_size']}  thr={r['threads']}  conc={r['concurrency']}  ops/J={r['ops_per_j']}  thr_sps={r['throughput_sps']}\n")
                w.write("\nTop 5 by Throughput:\n\n")
                for r in top_thr:
                    w.write(f"- bs={r['batch_size']}  thr={r['threads']}  conc={r['concurrency']}  thr_sps={r['throughput_sps']}  ops/J={r['ops_per_j']}\n")
                w.write("\n")
        except Exception:
            pass
