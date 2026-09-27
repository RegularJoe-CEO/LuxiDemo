#!/usr/bin/env python3
"""Batch-composition drift probe for vLLM / SGLang offline engines (greedy, fp16, prefix/radix cache off).
For each fixed target prompt: target alone (B=1, reference) + repeat, target inside equal-length batches of 16 and 64
(two filler sets / two target rows; identical token rows to the Luxi arm), and a mixed-length batch of 16.
Records generated token ids and the logprob of each chosen token. Appends one JSON line per (prompt, condition).

usage: drift_offline.py ENGINE MODE OUT.jsonl
  ENGINE = vllm | sglang ; MODE = normal | det   (vllm det = VLLM_BATCH_INVARIANT=1 set by the caller; sglang det = enable_deterministic_inference)
env: DRIFT_MODEL, DRIFT_G (256), DRIFT_N_PROMPTS / DRIFT_ONLY, DRIFT_STOP_FILE, SGL_BACKEND (fa3)"""
import os, sys, json, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *

ENGINE, MODE, OUT = sys.argv[1], sys.argv[2], sys.argv[3]
MODEL = os.environ.get("DRIFT_MODEL", "/root/models/Qwen2-7B-Instruct")
G = G_DEFAULT
T, F = load_data(); streams = F["streams"]

def make_engine():
    info = {"engine": ENGINE, "mode": MODE, "dtype": "float16", "G": G}
    if ENGINE == "vllm":
        import vllm, torch
        from vllm import LLM, SamplingParams
        kw = dict(model=MODEL, dtype="float16", max_model_len=4096, enable_prefix_caching=False, gpu_memory_utilization=0.85,
                  seed=0, trust_remote_code=True)
        try: llm = LLM(generation_config="vllm", **kw)
        except TypeError: llm = LLM(**kw)
        sp = SamplingParams(temperature=0.0, max_tokens=G, ignore_eos=True, logprobs=1, repetition_penalty=1.0)
        info.update(version=vllm.__version__, torch=torch.__version__, VLLM_BATCH_INVARIANT=os.environ.get("VLLM_BATCH_INVARIANT", "0"),
                    prefix_caching=False)
        def gen(rows):
            outs = llm.generate([{"prompt_token_ids": r} for r in rows], sp, use_tqdm=False)
            res = []
            for o in outs:
                c = o.outputs[0]; toks = list(c.token_ids)
                lps = [float(d[t].logprob) for t, d in zip(toks, c.logprobs)]
                res.append((toks, lps))
            return res
        return gen, info, lambda: None
    else:
        import sglang as sgl, torch
        det = MODE == "det"
        llm = sgl.Engine(model_path=MODEL, dtype="float16", tp_size=1, context_length=4096, trust_remote_code=True,
                         mem_fraction_static=0.85, disable_radix_cache=True, attention_backend=os.environ.get("SGL_BACKEND", "fa3"),
                         enable_deterministic_inference=det, sampling_defaults="openai", log_level="warning")
        info.update(version=sgl.__version__, torch=torch.__version__, deterministic=det, radix_cache=False)
        try:
            si = llm.get_server_info()
            info["server_info"] = {k: si.get(k) for k in ("attention_backend", "sampling_backend", "chunked_prefill_size",
                                   "disable_radix_cache", "enable_deterministic_inference", "cuda_graph_max_bs") if k in si}
        except Exception as e: info["server_info"] = repr(e)
        sp = {"temperature": 0.0, "max_new_tokens": G, "ignore_eos": True, "repetition_penalty": 1.0}
        def gen(rows):
            o = llm.generate(input_ids=rows, sampling_params=sp, return_logprob=True, top_logprobs_num=0, logprob_start_len=-1)
            o = o if isinstance(o, list) else [o]; res = []
            for x in o:
                mi = x["meta_info"]; toks = [int(t[1]) for t in mi["output_token_logprobs"]]
                if "output_ids" in x and list(x["output_ids"])[-len(toks):] != toks: print("WARN output_ids != logprob ids", flush=True)
                res.append((toks, [float(t[0]) for t in mi["output_token_logprobs"]]))
            return res
        return gen, info, llm.shutdown

def main():
    gen, info, close = make_engine()
    sink = JsonlSink(OUT)
    meta_path = OUT.replace(".jsonl", "_meta.json"); info["started"] = now(); json.dump(info, open(meta_path, "w"), indent=1)
    print("ENGINE", json.dumps(info), flush=True)
    # warm-up (shapes B=1/16/64) so first-use autotuning / graph capture is not attributed to the reference call
    w = streams[0][:64]
    for B in (1, 16, 64): gen([w] * B)
    tag = f"{ENGINE}_{MODE}"
    for t in T:
        S = t["n_tokens"]
        conds = batch_conditions(S)
        order = [c for c in conds if c["cond"] != "b1_rep"] + [{"cond": "b16_mix", "B": 16, "row": 5, "seed": 777, "mixed": True}] + \
                [c for c in conds if c["cond"] == "b1_rep"]
        for c in order:
            if stop_requested(): print("STOP file seen; exiting cleanly", flush=True); close(); return
            if sink.has(t["id"], c["cond"]): continue
            rows = mixed_rows(t["ids"], c["B"], c["row"], c["seed"], streams) if c.get("mixed") else \
                   ([t["ids"]] if c["B"] == 1 else build_rows(t["ids"], c["B"], c["row"], c["seed"], streams))
            t0 = time.time(); res = gen(rows); dt = time.time() - t0
            toks, lps = res[c["row"]]
            rec = {"engine": tag, "prompt_id": t["id"], "kind": t["kind"], "condition": c["cond"], "B": c["B"], "target_row": c["row"],
                   "filler_seed": c["seed"], "mixed_lengths": bool(c.get("mixed")), "S": S, "G": G, "tokens": toks, "logprobs": lps,
                   "lp_positions": None, "tokens_sha256": sha_tokens(toks), "logprobs_sha256": sha_logprobs(lps),
                   "batch_prompt_tokens": sum(len(r) for r in rows), "wall_s": round(dt, 3), "t": now()}
            sink.write(rec)
            print(f"{tag} {t['id']} {c['cond']} B={c['B']} row={c['row']} {dt:.2f}s tok_sha={rec['tokens_sha256'][:12]}", flush=True)
    info["finished"] = now(); json.dump(info, open(meta_path, "w"), indent=1)
    close(); print("DONE", flush=True)

if __name__ == "__main__":
    main()
