#!/usr/bin/env python3
"""Replay a drift comparison from the published JSON.
usage: drift_view.py EVIDENCE_DIR [ENGINE] [PROMPT_ID] [CONDITION]
  no ENGINE: list engines and headline rates; no PROMPT_ID: list that engine's diverging cases;
  otherwise print reference vs condition side by side (text, first divergence, logprobs around it)."""
import sys, os, json, glob, textwrap
d = sys.argv[1]; a = sys.argv[2:]
files = {json.load(open(f))["engine"]: f for f in glob.glob(os.path.join(d, "drift_results_*.json"))}
if not a:
    for e, f in sorted(files.items()):
        c = [r for r in json.load(open(f))["comparisons"] if r["condition"] not in ("b1_rep", "srv_b1_rep", "srv_b1_vs_offline_b1")]
        print(f"{e:15s} n={len(c):3d} identical tokens={sum(r['tokens_identical'] for r in c):3d} identical logprobs={sum(r['logprob_hash_identical'] for r in c):3d} text changed={sum(r['text_changed'] for r in c):3d}")
    sys.exit()
J = json.load(open(files[a[0]]))
if len(a) == 1:
    for r in J["comparisons"]:
        if not r["tokens_identical"] or not r["logprob_hash_identical"]:
            print(f"{r['prompt_id']} {r['condition']:22s} first_div={r['first_divergence']} text_changed={r['text_changed']} max|dlp|={r['max_abs_logprob_diff_before_divergence']:.3g}")
    sys.exit()
pid, cond = a[1], a[2] if len(a) > 2 else "b64_f1"
r = next(x for x in J["comparisons"] if x["prompt_id"] == pid and x["condition"] == cond)
rec = {(x["prompt_id"], x["condition"]): x for x in J["records"]}
print(f"{J['label']} | prompt {pid} | {cond} vs {r['reference']}")
print(f"tokens identical: {r['tokens_identical']}  first divergence: {r['first_divergence']}  logprob hash identical: {r['logprob_hash_identical']}  text changed: {r['text_changed']}")
print(f"token sha256 ref {r['ref_tokens_sha256'][:16]}  cand {r['cand_tokens_sha256'][:16]}")
fd = r["first_divergence"]
if fd is not None:
    R, C = rec[(pid, r["reference"])], rec[(pid, cond)]
    lr = dict(zip(R["lp_positions"] or range(len(R["logprobs"])), R["logprobs"])); lc = dict(zip(C["lp_positions"] or range(len(C["logprobs"])), C["logprobs"]))
    print("pos  ref_tok  ref_lp        cand_tok cand_lp")
    for p in range(max(0, fd - 3), fd + 3):
        if p < len(R["tokens"]) and p < len(C["tokens"]):
            print(f"{p:4d} {R['tokens'][p]:8d} {lr.get(p, float('nan')):+.6f}  {C['tokens'][p]:8d} {lc.get(p, float('nan')):+.6f}")
W = 60
A = textwrap.wrap(r.get("ref_text", ""), W) or [""]; B = textwrap.wrap(r.get("cand_text", ""), W) or [""]
print("\n" + "ALONE (reference)".ljust(W) + " | " + "UNDER LOAD")
for i in range(max(len(A), len(B))):
    x = A[i] if i < len(A) else ""; y = B[i] if i < len(B) else ""
    print(x.ljust(W) + (" | " if x == y else " # ") + y)
