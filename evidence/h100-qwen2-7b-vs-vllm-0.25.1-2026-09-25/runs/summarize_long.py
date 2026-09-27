#!/usr/bin/env python3
"""usage: summarize_long.py LABEL=file.json [LABEL=file.json ...]  -> markdown table (mean±sd, median) per cell"""
import json, sys, statistics as st
from collections import defaultdict
data = defaultdict(lambda: defaultdict(list))
labels = []
for a in sys.argv[1:]:
    lab, f = a.split("=", 1); labels.append(lab)
    for r in json.load(open(f))["rows"]:
        key = (r["kind"], r["mode"], r["S"], r["B"], r["gen"])
        data[key][lab].append(r)
def ms(v):
    if not v: return "-"
    m = st.mean(v); sd = st.stdev(v) if len(v) > 1 else 0.0
    return f"{m:.4g}±{sd:.2g} (med {st.median(v):.4g}, n={len(v)})"
for key in sorted(data, key=lambda k: (k[0] != "p", k[1], k[3], k[2])):
    kind, mode, S, B, G = key
    pre = kind == "p" or mode == "prefill"
    print(f"\n### {'prefill' if kind=='p' else 'decode-cell '+mode} S={S} B={B} gen={G}")
    if pre: print("| engine | prompt tok/s | J/prompt tok | latency s/call | median W | SM MHz |")
    else: print("| engine | out tok/s (e2e) | J/out tok (e2e) | latency s/call | per-token ms (B-seq step) | median W |")
    print("|---|---|---|---|---|---|")
    for lab in labels:
        R = data[key].get(lab, [])
        if not R: continue
        if pre:
            print(f"| {lab} | {ms([r['prompt_tok_per_s'] for r in R])} | {ms([r['J_per_prompt_tok'] for r in R])} | {ms([r['call_lat_mean_s'] for r in R])} | {st.median([r['median_W'] for r in R]):.0f} | {st.median([r['sm_clock_med'] for r in R]):.0f} |")
        else:
            print(f"| {lab} | {ms([r['out_tok_per_s_e2e'] for r in R])} | {ms([r['J_per_out_tok_e2e'] for r in R])} | {ms([r['call_lat_mean_s'] for r in R])} | {st.mean([r['call_lat_mean_s'] for r in R])/G*1000:.3f} | {st.median([r['median_W'] for r in R]):.0f} |")
