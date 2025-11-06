import os, csv, json, statistics as stats, re, sys

def summarize_latency(csv_path):
    times, samples = [], 0
    with open(csv_path) as f:
        r = csv.DictReader(f)
        for row in r:
            t = float(row["elapsed_s"]); n = int(row["samples"])
            times.append(t); samples += n
    if not times: return None
    total_time = sum(times)
    med = stats.median(times)*1000.0
    p95 = (stats.quantiles(times, n=20)[18]*1000.0) if len(times) >= 20 else (max(times)*1000.0)
    thr = samples / total_time if total_time > 0 else 0.0
    return samples, total_time, med, p95, thr

def parse_pm(pm_path):
    rx = re.compile(r'\b(CPU|GPU|SoC)\b.*?\b[Pp]ower\b[^:]*:\s*([0-9]*\.?[0-9]+)\s*([mM]?W)\b')
    cpu_w, gpu_w, soc_w = [], [], []
    with open(pm_path, "r", errors="ignore") as f:
        for line in f:
            m = rx.search(line)
            if not m: continue
            kind, val, unit = m.group(1).upper(), float(m.group(2)), m.group(3).lower()
            watts = val/1000.0 if unit == "mw" else val
            if kind == "CPU": cpu_w.append(watts)
            elif kind == "GPU": gpu_w.append(watts)
            elif kind == "SOC": soc_w.append(watts)
    cpu_avg = sum(cpu_w)/len(cpu_w) if cpu_w else 0.0
    gpu_avg = sum(gpu_w)/len(gpu_w) if gpu_w else 0.0
    total_w = cpu_avg + gpu_avg
    if total_w == 0.0 and soc_w:
        total_w = sum(soc_w)/len(soc_w)
    return 0.0, cpu_avg, gpu_avg, total_w  # duration 0 -> caller uses bench time

def append_row(tune_csv, row, header):
    write_header = not os.path.exists(tune_csv)
    with open(tune_csv, "a", newline="") as f:
        w = csv.DictWriter(f, fieldnames=header)
        if write_header: w.writeheader()
        w.writerow(row)

def main():
    if len(sys.argv) < 3:
        print("usage: aggregate_tuning.py <bench_csv> <power_txt> [tune_out_csv]", file=sys.stderr)
        sys.exit(2)
    bench_csv, pm_txt = sys.argv[1], sys.argv[2]
    tune_out = sys.argv[3] if len(sys.argv) > 3 else "docs/benchmarks/tuning_results.csv"

    lat = summarize_latency(bench_csv)
    if not lat: sys.exit("no latency rows")
    samples, bench_time_s, med_ms, p95_ms, thr_sps = lat
    dur_s, cpu_w, gpu_w, tot_w = parse_pm(pm_txt)
    energy_j = tot_w * bench_time_s
    ops_j = (samples / energy_j) if energy_j > 0 else 0.0

    meta_path = bench_csv.replace(".csv",".meta.json")
    meta = {}
    if os.path.exists(meta_path):
        with open(meta_path) as mf: meta = json.load(mf)
    row = {
        "framework": meta.get("framework","pytorch"),
        "mode": meta.get("mode","luxi"),
        "endpoint": meta.get("endpoint",""),
        "expr": meta.get("expr",""),
        "a": meta.get("a",""),
        "batch_size": meta.get("batch_size",""),
        "threads": meta.get("threads",""),
        "concurrency": meta.get("concurrency",""),
        "duration_s": bench_time_s,
        "samples": samples,
        "median_ms": f"{med_ms:.3f}",
        "p95_ms": f"{p95_ms:.3f}",
        "throughput_sps": f"{thr_sps:.1f}",
        "cpu_w": f"{cpu_w:.2f}",
        "gpu_w": f"{gpu_w:.2f}",
        "total_w": f"{tot_w:.2f}",
        "energy_j": f"{energy_j:.2f}",
        "ops_per_j": f"{ops_j:.2f}",
        "bench_csv": bench_csv,
        "power_txt": pm_txt,
    }
    header = list(row.keys())
    append_row(tune_out, row, header)
    print(f"Appended tuning row to {tune_out}")

if __name__ == "__main__":
    main()
