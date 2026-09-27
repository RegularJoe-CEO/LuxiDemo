#!/usr/bin/env python3
"""Rebuild <block>.json files from the per-run CSV rows in this folder.

usage: python3 csv_to_json.py [folder]
Each CSV holds the measured columns for one block. The derived columns are
added back with the same formulas the benchmark harness used:
  prompt_tok_per_s  = prompt_tokens / wall_s     (prefill rows)
  J_per_prompt_tok  = energy_J / prompt_tokens   (prefill rows)
  out_tok_per_s_e2e = gen_tokens / wall_s        (end-to-end generation rows)
  J_per_out_tok_e2e = energy_J / gen_tokens      (end-to-end generation rows)
  J_per_call        = energy_J / calls
  s_per_call        = wall_s / calls
Engine metadata per block comes from meta.json."""
import csv, glob, json, os, sys

d = sys.argv[1] if len(sys.argv) > 1 else "."
meta = json.load(open(os.path.join(d, "meta.json")))
TEXT = {"engine", "kind", "mode"}

def val(k, s):
    if s == "":
        return None
    if k in TEXT:
        return s
    return float(s) if any(c in s for c in ".eEn") else int(s)

for f in sorted(glob.glob(os.path.join(d, "*.csv"))):
    block = os.path.splitext(os.path.basename(f))[0]
    rows = []
    for raw in csv.DictReader(open(f, newline="")):
        r = {k: val(k, v) for k, v in raw.items() if v != ""}
        if r["mode"] == "prefill":
            r["prompt_tok_per_s"] = r["prompt_tokens"] / r["wall_s"]
            r["J_per_prompt_tok"] = r["energy_J"] / r["prompt_tokens"]
        if r["mode"] == "e2e":
            r["out_tok_per_s_e2e"] = r["gen_tokens"] / r["wall_s"]
            r["J_per_out_tok_e2e"] = r["energy_J"] / r["gen_tokens"]
        r["J_per_call"] = r["energy_J"] / r["calls"]
        r["s_per_call"] = r["wall_s"] / r["calls"]
        rows.append(r)
    json.dump({"meta": meta.get(block), "rows": rows}, open(os.path.join(d, block + ".json"), "w"), indent=1)
    print(block, len(rows), "rows")
