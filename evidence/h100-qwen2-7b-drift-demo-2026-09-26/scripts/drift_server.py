#!/usr/bin/env python3
"""Arrival-pattern drift probe through the OpenAI-compatible server (vLLM `vllm serve` / SGLang `sglang.launch_server`).
Per target prompt: srv_b1 (target alone = server reference), srv_stagA / srv_stagB (target sent while seeded background
requests of mixed lengths arrive with exponential inter-arrival gaps and are in flight), srv_b1_rep (alone again).
Greedy (temperature 0), repetition_penalty 1.0, ignore_eos, fixed max_tokens; token ids + chosen-token logprobs returned by the server.

usage: drift_server.py ENGINE MODE OUT.jsonl   (starts and stops the server itself)
env: DRIFT_MODEL, DRIFT_G, DRIFT_PORT (18000), DRIFT_N_PROMPTS / DRIFT_ONLY, DRIFT_STOP_FILE, SGL_BACKEND, VLLM_BATCH_INVARIANT"""
import os, sys, json, time, random, threading, subprocess, signal, urllib.request
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *

ENGINE, MODE, OUT = sys.argv[1], sys.argv[2], sys.argv[3]
MODEL = os.environ.get("DRIFT_MODEL", "/root/models/Qwen2-7B-Instruct")
PORT = int(os.environ.get("DRIFT_PORT", "18000")); URL = f"http://127.0.0.1:{PORT}"
NAME = "qwen2-7b-instruct"; G = G_DEFAULT
T, F = load_data(); streams = F["streams"]
STAG = {"srv_stagA": {"n_fill": 24, "gap_mean_s": 0.08, "t_target_s": 0.6, "seed": 100},
        "srv_stagB": {"n_fill": 56, "gap_mean_s": 0.04, "t_target_s": 1.5, "seed": 900}}

def server_cmd():
    if ENGINE == "vllm":
        return ["/root/vllm129/bin/vllm", "serve", MODEL, "--dtype", "float16", "--max-model-len", "4096", "--no-enable-prefix-caching",
                "--gpu-memory-utilization", "0.85", "--host", "127.0.0.1", "--port", str(PORT), "--served-model-name", NAME,
                "--generation-config", "vllm", "--seed", "0"]
    c = ["/root/sgl/bin/python", "-m", "sglang.launch_server", "--model-path", MODEL, "--dtype", "float16", "--context-length", "4096",
         "--disable-radix-cache", "--attention-backend", os.environ.get("SGL_BACKEND", "fa3"), "--mem-fraction-static", "0.85",
         "--host", "127.0.0.1", "--port", str(PORT), "--served-model-name", NAME, "--sampling-defaults", "openai", "--log-level", "warning"]
    if MODE == "det": c.append("--enable-deterministic-inference")
    return c

def post(payload, timeout=600):
    req = urllib.request.Request(URL + "/v1/completions", data=json.dumps(payload).encode(), headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=timeout) as r: return json.loads(r.read())

def payload(ids, max_tokens, want_lp):
    p = {"model": NAME, "prompt": ids, "max_tokens": max_tokens, "temperature": 0.0, "top_p": 1.0, "repetition_penalty": 1.0,
         "ignore_eos": True, "stream": False}
    if ENGINE == "vllm":
        if want_lp: p.update(logprobs=1, return_token_ids=True, return_tokens_as_token_ids=True)
    else:
        p["top_k"] = -1
        if want_lp: p.update(logprobs=0, return_token_ids=True)
    return p

def parse(resp):
    c = resp["choices"][0]; lp = c.get("logprobs") or {}
    toks = c.get("token_ids")
    if toks is None and lp.get("tokens"): toks = [int(s.split("token_id:")[1]) for s in lp["tokens"]]
    return [int(x) for x in toks], [float(x) for x in lp.get("token_logprobs") or []]

def target(ids):
    t0 = time.time(); toks, lps = parse(post(payload(ids, G, True))); return toks, lps, time.time() - t0

def staggered(t, spec):
    rng = random.Random(spec["seed"] * 1000 + int(t["id"][1:]))
    plan = []; tt = 0.0
    for _ in range(spec["n_fill"]):
        tt += rng.expovariate(1.0 / spec["gap_mean_s"]); L = rng.randrange(16, 1500); src = streams[rng.randrange(len(streams))]
        off = rng.randrange(0, len(src) - L); plan.append((tt, src[off:off + L], rng.randrange(32, 320)))
    state = {"inflight": 0, "errors": 0}; lock = threading.Lock(); out = {}
    def filler(at, ids, mt, t0):
        time.sleep(max(0.0, t0 + at - time.time()))
        with lock: state["inflight"] += 1
        try: post(payload(ids, mt, False))
        except Exception: state["errors"] += 1
        finally:
            with lock: state["inflight"] -= 1
    def tgt(t0):
        time.sleep(max(0.0, t0 + spec["t_target_s"] - time.time()))
        with lock: out["inflight_at_send"] = state["inflight"]
        out["res"] = target(t["ids"])
    t0 = time.time() + 0.05
    th = [threading.Thread(target=filler, args=(a, i, m, t0)) for a, i, m in plan] + [threading.Thread(target=tgt, args=(t0,))]
    for x in th: x.start()
    for x in th: x.join()
    return out["res"], {"inflight_at_send": out.get("inflight_at_send"), "filler_errors": state["errors"], "n_fill": spec["n_fill"],
                        "gap_mean_s": spec["gap_mean_s"], "t_target_s": spec["t_target_s"], "fill_prompt_tokens": sum(len(p[1]) for p in plan)}

def wait_ready(proc, limit=1200):
    t0 = time.time()
    while time.time() - t0 < limit:
        if proc.poll() is not None: raise RuntimeError(f"server exited rc={proc.returncode}")
        try:
            urllib.request.urlopen(URL + "/v1/models", timeout=5).read(); return time.time() - t0
        except Exception: time.sleep(3)
    raise RuntimeError("server not ready")

def main():
    log = open(OUT.replace(".jsonl", "_server.log"), "a")
    cmd = server_cmd(); print("SERVER", " ".join(cmd), flush=True)
    proc = subprocess.Popen(cmd, stdout=log, stderr=subprocess.STDOUT, start_new_session=True, env=os.environ.copy())
    tag = f"{ENGINE}_{MODE}"
    try:
        ready = wait_ready(proc); print(f"ready after {ready:.0f}s", flush=True)
        meta = {"engine": ENGINE, "mode": MODE, "dtype": "float16", "G": G, "server_cmd": [c if c != MODEL else "<model>" for c in cmd],
                "VLLM_BATCH_INVARIANT": os.environ.get("VLLM_BATCH_INVARIANT", "0"), "staggered_specs": STAG, "ready_s": round(ready, 1), "started": now()}
        json.dump(meta, open(OUT.replace(".jsonl", "_meta.json"), "w"), indent=1)
        # warm-up
        post(payload(streams[0][:64], 16, True)); staggered_warm = STAG["srv_stagA"]
        sink = JsonlSink(OUT)
        for t in T:
            for cond in ("srv_b1", "srv_stagA", "srv_stagB", "srv_b1_rep"):
                if stop_requested(): print("STOP file seen", flush=True); return
                if sink.has(t["id"], cond): continue
                extra = {}
                if cond in STAG: (toks, lps, dt), extra = staggered(t, STAG[cond])
                else: toks, lps, dt = target(t["ids"])
                rec = {"engine": tag, "prompt_id": t["id"], "kind": t["kind"], "condition": cond, "B": None, "target_row": None,
                       "filler_seed": STAG.get(cond, {}).get("seed"), "S": t["n_tokens"], "G": G, "tokens": toks, "logprobs": lps,
                       "lp_positions": None, "tokens_sha256": sha_tokens(toks), "logprobs_sha256": sha_logprobs(lps), "wall_s": round(dt, 3),
                       "arrival": extra, "t": now()}
                sink.write(rec)
                print(f"{tag} {t['id']} {cond} {dt:.2f}s inflight={extra.get('inflight_at_send')} tok_sha={rec['tokens_sha256'][:12]}", flush=True)
        print("DONE", flush=True)
    finally:
        try: os.killpg(proc.pid, signal.SIGTERM); proc.wait(60)
        except Exception:
            try: os.killpg(proc.pid, signal.SIGKILL)
            except Exception: pass

if __name__ == "__main__":
    main()
