#!/usr/bin/env python3
"""Build per_prompt.csv from the drift_results_<engine>.json files written by analyze.py.
usage: per_prompt.py DIR_WITH_drift_results [OUT_CSV]   (default OUT_CSV: DIR/per_prompt.csv)
Cell values: same = tokens and logprob hash identical to the reference; lp = tokens identical, logprob hash different;
text@N = visible text changed, first differing token at index N; n/a = condition not run for that engine."""
import os, sys, json

ORDER = ["luxi_c8", "vllm_normal", "sglang_normal", "vllm_det", "sglang_det"]
CONDS = ["b1_rep", "b16_f1", "b16_f2", "b16_mix", "b64_f1", "b64_f2", "srv_b1_rep", "srv_stagA", "srv_stagB", "srv_b1_vs_offline_b1"]

def cell(c):
    if c["tokens_identical"]: return "same" if c["logprob_hash_identical"] else "lp"
    return f"text@{c['first_divergence']}"

def main():
    d = sys.argv[1]; out = sys.argv[2] if len(sys.argv) > 2 else os.path.join(d, "per_prompt.csv")
    lines = ["engine,prompt," + ",".join(CONDS)]
    for eng in ORDER:
        f = os.path.join(d, f"drift_results_{eng}.json")
        if not os.path.exists(f): continue
        cmp = {(c["prompt_id"], c["condition"]): c for c in json.load(open(f))["comparisons"]}
        for pid in sorted({p for p, _ in cmp}):
            lines.append(",".join([eng, pid] + [cell(cmp[(pid, k)]) if (pid, k) in cmp else "n/a" for k in CONDS]))
    open(out, "w").write("\n".join(lines) + "\n")

if __name__ == "__main__":
    main()
