#!/usr/bin/env python3
"""usage: summarize_race.py REF_LABEL LABEL=file1.json,file2.json [LABEL=...]  -> markdown: per-cell medians (n) + ratios vs REF_LABEL
Only measured values; median over all runs of all blocks for that engine; n = number of metered runs."""
import json, sys, statistics as st
from collections import defaultdict
ref = sys.argv[1]; data = defaultdict(lambda: defaultdict(list)); labels = []; meta = {}
for a in sys.argv[2:]:
    lab, fs = a.split("=", 1); labels.append(lab)
    for f in fs.split(","):
        j = json.load(open(f)); meta.setdefault(lab, j.get("meta"))
        for r in j["rows"]: data[(r["kind"], r["mode"], r["S"], r["B"], r["gen"])][lab].append(r)
def med(R, k): v = [r[k] for r in R if r.get(k) is not None]; return (st.median(v), len(v)) if v else (None, 0)
order = sorted(data, key=lambda k: (k[0] != "p", k[1] != "prefill", -k[2] if k[0] == "p" else 0, k[3]))
print("## Medians per test (n = metered runs)\n")
for key in order:
    kind, mode, S, B, G = key; pre = mode == "prefill"
    name = f"prefill S={S} B={B}" if kind == "p" else (f"decode-cell prefill-only S={S} B={B} (gen=1)" if pre else f"generation S={S} B={B} gen={G} (end-to-end)")
    tk, ek = ("prompt_tok_per_s", "J_per_prompt_tok") if pre else ("out_tok_per_s_e2e", "J_per_out_tok_e2e")
    print(f"### {name}\n| engine | {'prompt' if pre else 'output'} tok/s (median) | J/token (median) | s/call (median) | board W (median) | SM MHz (median) | n |\n|---|---|---|---|---|---|---|")
    base = data[key].get(ref)
    for lab in labels:
        R = data[key].get(lab)
        if not R: print(f"| {lab} | not run | | | | | 0 |"); continue
        t, n = med(R, tk); e, _ = med(R, ek); s, _ = med(R, "s_per_call"); w, _ = med(R, "median_W"); c, _ = med(R, "sm_clock_med")
        print(f"| {lab} | {t:.4g} | {e:.4g} | {s:.4g} | {w:.0f} | {c:.0f} | {n} |")
    if base:
        print(f"\n| ratio vs {ref} | throughput ({ref} / other) | energy per token ({ref} / other) |\n|---|---|---|")
        bt, _ = med(base, tk); be, _ = med(base, ek)
        for lab in labels:
            if lab == ref or not data[key].get(lab): continue
            t, _ = med(data[key][lab], tk); e, _ = med(data[key][lab], ek)
            print(f"| {lab} | {bt / t:.3f}x | {be / e:.3f}x |")
    print()
print("## Engine metadata\n```")
for lab, m in meta.items(): print(lab, json.dumps(m))
print("```")
