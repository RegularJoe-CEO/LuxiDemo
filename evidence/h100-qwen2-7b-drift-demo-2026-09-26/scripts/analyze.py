#!/usr/bin/env python3
"""Drift demo analysis. Reads results_*.jsonl (one line per engine/prompt/condition), compares every condition with the
same engine's reference for the same prompt, and writes:
  drift_results_<engine>.json   per-record comparison (tokens, logprobs, hashes, first divergence, text)
  summary.md                    per-engine and per-condition tables
  examples.md                   1-3 real cases where default vLLM / SGLang visible text changed (only if any occurred)
  drift_chart.png               % identical tokens / logprob hashes by engine and condition group (needs matplotlib)
usage: analyze.py RESULTS_DIR OUT_DIR [--tokenizer DIR_WITH_tokenizer.json]
Reference: batch conditions (b*) -> "b1" (target alone, B=1, same engine, offline); server conditions (srv_*) -> "srv_b1".
Comparisons use the tokens a client receives (up to and including the first <|im_end|>/<|endoftext|>); all runs
generated a fixed 256 tokens with ignore_eos."""
import os, sys, json, glob, statistics, difflib, collections, hashlib, struct

STOP = (151645, 151643)
LABEL = {"luxi_c8": "Luxi c8 (batch-invariant engine)", "vllm_normal": "vLLM 0.25.1 default", "vllm_det": "vLLM 0.25.1 VLLM_BATCH_INVARIANT=1",
         "sglang_normal": "SGLang 0.5.19 default", "sglang_det": "SGLang 0.5.19 deterministic mode"}
ORDER = ["luxi_c8", "vllm_normal", "sglang_normal", "vllm_det", "sglang_det"]
CGROUP = {"b1_rep": "repeat alone (B=1)", "b16_f1": "batch 16", "b16_f2": "batch 16", "b64_f1": "batch 64", "b64_f2": "batch 64",
          "b16_mix": "batch 16 mixed lengths", "srv_b1_rep": "server repeat alone", "srv_stagA": "server staggered arrivals",
          "srv_stagB": "server staggered arrivals"}

def served(t):
    for i, x in enumerate(t):
        if x in STOP: return i + 1
    return len(t)
def sha_d(v): return hashlib.sha256(struct.pack(f"<{len(v)}d", *v)).hexdigest()

def load(rd):
    E = collections.defaultdict(dict)
    for f in sorted(glob.glob(os.path.join(rd, "results_*.jsonl"))):
        for l in open(f):
            try: r = json.loads(l)
            except Exception: continue
            E[r["engine"]][(r["prompt_id"], r["condition"])] = r
    return E

def lp_map(r):
    if r.get("logprobs") is None: return {}
    pos = r["lp_positions"] if r.get("lp_positions") is not None else range(len(r["logprobs"]))
    return dict(zip(pos, r["logprobs"]))

def compare(ref, c, dec):
    rt, ct = ref["tokens"], c["tokens"]; Lr, Lc = served(rt), served(ct)
    rs, cs = rt[:Lr], ct[:Lc]
    fd = next((i for i in range(min(Lr, Lc)) if rs[i] != cs[i]), None)
    if fd is None and Lr != Lc: fd = min(Lr, Lc)
    lim = fd if fd is not None else Lr
    rm, cm = lp_map(ref), lp_map(c)
    common = [p for p in sorted(rm) if p < lim and p in cm]
    diffs = [abs(rm[p] - cm[p]) for p in common]
    served_pos = [p for p in sorted(rm) if p < Lr]
    lp_ident = fd is None and all(p in cm and cm[p] == rm[p] for p in served_pos)
    out = {"tokens_identical": fd is None, "tokens_identical_all_256": rt == ct, "first_divergence": fd,
           "ref_served_len": Lr, "cand_served_len": Lc,
           "ref_tokens_sha256": ref["tokens_sha256"], "cand_tokens_sha256": c["tokens_sha256"],
           "ref_served_logprobs_sha256": sha_d([rm[p] for p in served_pos]),
           "cand_served_logprobs_sha256": sha_d([cm[p] for p in served_pos if p in cm]) if all(p in cm for p in served_pos) else None,
           "logprob_hash_identical": lp_ident,
           "n_logprob_positions_compared": len(common), "n_logprob_positions_differing_before_divergence": sum(1 for d in diffs if d != 0),
           "max_abs_logprob_diff_before_divergence": max(diffs) if diffs else 0.0}
    if "logits_sha256" in ref and "logits_sha256" in c:
        rp = dict(zip(ref["lp_positions"], ref["logits_sha256"])); cp = dict(zip(c["lp_positions"], c["logits_sha256"]))
        out["full_vocab_logits_identical_at_checkpoints"] = all(rp[p] == cp.get(p) for p in rp if p < Lr)
    if dec:
        rtxt, ctxt = dec(rs), dec(cs); out["text_changed"] = rtxt != ctxt; out["ref_text"] = rtxt; out["cand_text"] = ctxt
    else:
        out["text_changed"] = fd is not None
    return out

def make_decoder(tokdir):
    try:
        from tokenizers import Tokenizer
        tk = Tokenizer.from_file(os.path.join(tokdir, "tokenizer.json"))
        return lambda ids: tk.decode([i for i in ids if i not in STOP], skip_special_tokens=True)
    except Exception as e:
        print("WARN no tokenizer, text_changed falls back to token inequality:", e); return None

def pct(a, b): return f"{100.0 * a / b:.1f}%" if b else "n/a"

def main():
    rd, od = sys.argv[1], sys.argv[2]; os.makedirs(od, exist_ok=True)
    tokdir = sys.argv[sys.argv.index("--tokenizer") + 1] if "--tokenizer" in sys.argv else None
    dec = make_decoder(tokdir) if tokdir else None
    E = load(rd); allcmp = {}
    for eng, recs in E.items():
        rows = []
        for (pid, cond), r in sorted(recs.items()):
            refc = "srv_b1" if cond.startswith("srv_") else "b1"
            if cond == refc or (pid, refc) not in recs: continue
            c = compare(recs[(pid, refc)], r, dec)
            c.update(prompt_id=pid, kind=r.get("kind"), condition=cond, reference=refc, B=r.get("B"), target_row=r.get("target_row"),
                     filler_seed=r.get("filler_seed"), arrival=r.get("arrival"))
            rows.append(c)
        # cross-check: server alone vs offline alone
        for (pid, cond), r in sorted(recs.items()):
            if cond == "srv_b1" and (pid, "b1") in recs:
                c = compare(recs[(pid, "b1")], r, dec); c.update(prompt_id=pid, kind=r.get("kind"), condition="srv_b1_vs_offline_b1", reference="b1")
                rows.append(c)
        allcmp[eng] = rows
        pub = {"engine": eng, "label": LABEL.get(eng, eng), "comparisons": rows,
               "records": [{k: v for k, v in r.items()} for r in recs.values()]}
        json.dump(pub, open(os.path.join(od, f"drift_results_{eng}.json"), "w"), indent=1)
    engines = [e for e in ORDER if e in allcmp] + [e for e in allcmp if e not in ORDER]
    L = ["# Drift demo summary (Qwen2-7B-Instruct fp16, greedy, H100)", "",
         "Each row compares the target prompt's output under a load condition with the SAME engine's output for the SAME prompt alone "
         "(batch conditions vs offline B=1; server conditions vs server alone). Tokens = what a client receives (up to the first end-of-turn token).", "",
         "## Per engine (all non-reference load conditions; the repeat-alone controls are excluded here)", "",
         "| engine | prompt-conditions | identical tokens | identical logprob hash | first divergence median / max (token index) | visible text changed |",
         "|---|---|---|---|---|---|"]
    def stats(rows):
        n = len(rows); ti = sum(r["tokens_identical"] for r in rows); li = sum(r["logprob_hash_identical"] for r in rows)
        fds = [r["first_divergence"] for r in rows if r["first_divergence"] is not None]; tc = sum(r["text_changed"] for r in rows)
        md = f"{statistics.median(fds):g} / {max(fds)}" if fds else "none"
        return n, ti, li, md, tc
    summ = {}
    for e in engines:
        rows = [r for r in allcmp[e] if r["condition"] not in ("b1_rep", "srv_b1_rep", "srv_b1_vs_offline_b1")]
        n, ti, li, md, tc = stats(rows); summ[e] = {"n": n, "tokens_identical": ti, "logprob_identical": li, "text_changed": tc}
        L.append(f"| {LABEL.get(e, e)} | {n} | {ti}/{n} ({pct(ti, n)}) | {li}/{n} ({pct(li, n)}) | {md} | {tc}/{n} |")
    L += ["", "## Per engine and condition", "", "| engine | condition | n | identical tokens | identical logprob hash | first div. median / max | text changed | max abs logprob diff before divergence |", "|---|---|---|---|---|---|---|---|"]
    chart = collections.OrderedDict()
    for e in engines:
        byc = collections.defaultdict(list)
        for r in allcmp[e]: byc[r["condition"]].append(r)
        for cond in sorted(byc):
            rows = byc[cond]; n, ti, li, md, tc = stats(rows)
            mx = max(r["max_abs_logprob_diff_before_divergence"] for r in rows)
            L.append(f"| {LABEL.get(e, e)} | {cond} | {n} | {pct(ti, n)} | {pct(li, n)} | {md} | {tc} | {mx:.3g} |")
            g = CGROUP.get(cond)
            if g: chart.setdefault(g, {}).setdefault(e, []).extend(rows)
    lux = [r for r in allcmp.get("luxi_c8", [])]
    L += ["", "## Notes", "",
          "- Luxi logprobs are sampled at checkpoint positions 0,1,3,7,15,31,63,127,255 (its benchmarked batch API returns final-step logits only; "
          "each checkpoint is a separate generate call of the same batch). At those positions the full 152,064-entry logits vector is also hashed. "
          "vLLM/SGLang logprobs cover every generated position.",
          "- Luxi has no concurrent-arrival server for the benchmarked engine, so it has batch conditions only (no srv_* rows).",
          "- Luxi's batch API needs equal-length rows, so equal-length batch conditions use identical token rows for all engines; the mixed-length batch (b16_mix) is vLLM/SGLang only.",
          "- For the ~1.7k-token prompt (p20) the '64' conditions use 32 rows (Luxi's benchmarked prefill buffer holds 65,536 tokens); this applies to every engine.",
          "- Staggered-arrival timing is not reproducible run to run by design; those rows show what a server can do under live traffic, not a fixed schedule."]
    if lux and any("full_vocab_logits_identical_at_checkpoints" in r for r in lux):
        k = sum(r.get("full_vocab_logits_identical_at_checkpoints", False) for r in lux)
        L.append(f"- Luxi full-vocab logits bit-identical to its B=1 reference at every checkpoint: {k}/{len(lux)} prompt-conditions.")
    san = os.path.join(rd, "luxi_sanity.json")
    if os.path.exists(san):
        L.append(f"- Luxi sanity lines reproduce the 2026-09-26 race logits hashes bit for bit: {json.load(open(san)).get('ALL_MATCH')}.")
    open(os.path.join(od, "summary.md"), "w").write("\n".join(L) + "\n")
    json.dump({"per_engine": summ}, open(os.path.join(od, "summary.json"), "w"), indent=1)
    # examples
    X = ["# Real examples: visible answer changed under load (default engines)", ""]
    cands = [(e, r) for e in ("vllm_normal", "sglang_normal") for r in allcmp.get(e, [])
             if r["text_changed"] and r["condition"] not in ("srv_b1_vs_offline_b1",)]
    if not cands:
        X.append("No case occurred in this run in which the visible text of default vLLM or SGLang changed under load. Only logprob-level drift (if any) is reported in summary.md.")
    else:
        tot = {e: len([r for r in allcmp[e] if r["condition"] not in ("b1_rep", "srv_b1_rep", "srv_b1_vs_offline_b1")]) for e in ("vllm_normal", "sglang_normal") if e in allcmp}
        X.append("Rates (all non-reference load conditions): " + ", ".join(f"{LABEL[e]}: {sum(1 for ee, r in cands if ee == e and r['condition'] not in ('b1_rep','srv_b1_rep'))}/{n}" for e, n in tot.items()) + ". Most prompt-conditions did NOT change visibly; these are the chosen examples.")
        seen = set(); picked = []
        for e, r in sorted(cands, key=lambda x: (x[1]["first_divergence"] or 0)):
            if r["prompt_id"] in seen: continue
            seen.add(r["prompt_id"]); picked.append((e, r))
            if len(picked) == 3: break
        for e, r in picked:
            X += ["", f"## {LABEL[e]} - prompt {r['prompt_id']} ({r['kind']}), condition `{r['condition']}` vs `{r['reference']}`", "",
                  f"First differing token index: {r['first_divergence']}. Max |logprob diff| before divergence: {r['max_abs_logprob_diff_before_divergence']:.3g}.", ""]
            X += side_by_side(r.get("ref_text", ""), r.get("cand_text", ""))
            lx = next((q for q in allcmp.get("luxi_c8", []) if q["prompt_id"] == r["prompt_id"] and q["condition"] == r["condition"]), None)
            if lx: X += ["", f"Luxi, same prompt and condition: tokens identical = {lx['tokens_identical']}, logprob hash identical = {lx['logprob_hash_identical']}."]
    open(os.path.join(od, "examples.md"), "w").write("\n".join(X) + "\n")
    try: plot(chart, engines, os.path.join(od, "drift_chart.png"))
    except Exception as ex: print("chart skipped:", ex)
    print("\n".join(L[:12 + len(engines)]))

def side_by_side(a, b, ctx=40):
    aw, bw = a.split(" "), b.split(" ")
    sm = difflib.SequenceMatcher(a=aw, b=bw, autojunk=False)
    k = next((i for i, (x, y) in enumerate(zip(aw, bw)) if x != y), min(len(aw), len(bw)))
    pre = " ".join(aw[max(0, k - 25):k])
    out = ["| alone (reference) | under load |", "|---|---|",
           f"| ...{esc(pre)} **{esc(' '.join(aw[k:k + ctx]))}** | ...{esc(pre)} **{esc(' '.join(bw[k:k + ctx]))}** |", "",
           "Word diff of the full visible answers (`-` alone only, `+` under load only):", "", "```diff"]
    for op, i1, i2, j1, j2 in sm.get_opcodes():
        if op == "equal": continue
        if i2 > i1: out.append("- " + " ".join(aw[i1:i2])[:300])
        if j2 > j1: out.append("+ " + " ".join(bw[j1:j2])[:300])
    out.append("```")
    return out

def esc(s): return s.replace("|", "\\|").replace("\n", " ⏎ ")

def plot(chart, engines, path):
    import matplotlib; matplotlib.use("Agg"); import matplotlib.pyplot as plt
    groups = list(chart.keys()); fig, axes = plt.subplots(1, 2, figsize=(14, 5.2), sharey=True)
    w = 0.8 / max(1, len(engines))
    for ax, key, title in ((axes[0], "tokens_identical", "% identical output tokens vs alone"),
                           (axes[1], "logprob_hash_identical", "% identical logprob hash vs alone")):
        for i, e in enumerate(engines):
            vals = []
            for g in groups:
                rows = chart[g].get(e, []); vals.append(100.0 * sum(r[key] for r in rows) / len(rows) if rows else float("nan"))
            xs = [j + i * w for j in range(len(groups))]
            ax.bar(xs, vals, w, label=LABEL.get(e, e))
            for x_, v_ in zip(xs, vals):
                ax.text(x_, (0 if v_ != v_ else v_) + 1, "n/a" if v_ != v_ else f"{v_:.0f}", ha="center", va="bottom", fontsize=6, rotation=90)
        ax.set_xticks([j + 0.4 - w / 2 for j in range(len(groups))]); ax.set_xticklabels(groups, rotation=25, ha="right", fontsize=8)
        ax.set_title(title); ax.set_ylim(0, 115); ax.grid(axis="y", alpha=0.3)
    axes[0].set_ylabel("% of prompt-conditions"); h, l = axes[0].get_legend_handles_labels(); fig.legend(h, l, fontsize=8, loc="lower center", ncol=5)
    fig.suptitle("Qwen2-7B-Instruct fp16 greedy on H100: same prompt, different batch / arrival pattern (n/a = condition not applicable/run)", fontsize=10)
    fig.tight_layout(rect=(0, 0.07, 1, 1)); fig.savefig(path, dpi=130)

if __name__ == "__main__":
    main()
