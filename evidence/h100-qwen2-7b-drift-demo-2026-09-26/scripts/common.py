"""Shared helpers for the drift demo drivers (vLLM / SGLang / analysis). No engine imports here."""
import json, os, hashlib, struct, time, random

KIT = os.path.dirname(os.path.abspath(__file__))
STOP_TOKENS = (151645, 151643)          # <|im_end|>, <|endoftext|>
G_DEFAULT = int(os.environ.get("DRIFT_G", "256"))
MAX_ROWS = 65536                         # Luxi benchmarked prefill buffer (B*S); applied to every engine for comparability

def load_data():
    P = json.load(open(os.path.join(KIT, "data", "prompts.json"), encoding="utf-8"))
    F = json.load(open(os.path.join(KIT, "data", "filler_pool.json"), encoding="utf-8"))
    n = int(os.environ.get("DRIFT_N_PROMPTS", "0") or 0)
    only = [x for x in os.environ.get("DRIFT_ONLY", "").split(",") if x]
    T = P["targets"]
    if only: T = [t for t in T if t["id"] in only]
    elif n: T = T[:n]
    return T, F

def big_b(S, want=64):
    b = want
    while b > 1 and b * S > MAX_ROWS: b //= 2
    return b

def batch_conditions(S):
    """Equal-length batch conditions shared by Luxi, vLLM and SGLang (identical token rows)."""
    B64 = big_b(S, 64)
    return [
        {"cond": "b1", "B": 1, "row": 0, "seed": None},
        {"cond": "b1_rep", "B": 1, "row": 0, "seed": None},
        {"cond": "b16_f1", "B": 16, "row": 0, "seed": 1000},
        {"cond": "b16_f2", "B": 16, "row": 8, "seed": 5000},
        {"cond": "b64_f1", "B": B64, "row": 0, "seed": 1000},
        {"cond": "b64_f2", "B": B64, "row": B64 // 2, "seed": 5000},
    ]

def build_rows(target_ids, B, row, seed, streams):
    """Row `row` = target; other rows k=1..B-1 (in order, skipping the target) = streams[(seed+k) % n][:S].
    Mirrors the Luxi driver rule exactly (cycling only if a stream were shorter than S)."""
    S = len(target_ids); rows = []; n = len(streams)
    for r in range(B):
        if r == row: rows.append(list(target_ids)); continue
        k = r + 1 if r < row else r
        src = streams[(seed + k) % n]
        rows.append([src[j % len(src)] for j in range(S)])
    return rows

def mixed_rows(target_ids, B, row, seed, streams, lo=16, hi=1200):
    """Mixed-length batch (vLLM/SGLang only; Luxi's batch API needs equal-length rows)."""
    rng = random.Random(seed); rows = []; n = len(streams)
    for r in range(B):
        if r == row: rows.append(list(target_ids)); continue
        L = rng.randrange(lo, hi); src = streams[rng.randrange(n)]; off = rng.randrange(0, max(1, len(src) - L))
        rows.append(src[off:off + L])
    return rows

def sha_tokens(t): return hashlib.sha256(struct.pack(f"<{len(t)}i", *t)).hexdigest()
def sha_logprobs(v): return hashlib.sha256(struct.pack(f"<{len(v)}d", *[float(x) for x in v])).hexdigest()

def served_len(tokens):
    """Tokens a client would receive: up to and including the first stop token."""
    for i, t in enumerate(tokens):
        if t in STOP_TOKENS: return i + 1
    return len(tokens)

class JsonlSink:
    """Append-only, flushed per record -> partial results survive crashes / STOP. Supports resume."""
    def __init__(self, path):
        self.path = path; self.done = set()
        if os.path.exists(path):
            for l in open(path):
                try: r = json.loads(l); self.done.add((r["prompt_id"], r["condition"]))
                except Exception: pass
        self.f = open(path, "a")
    def has(self, pid, cond): return (pid, cond) in self.done
    def write(self, rec):
        self.f.write(json.dumps(rec) + "\n"); self.f.flush(); os.fsync(self.f.fileno()); self.done.add((rec["prompt_id"], rec["condition"]))

def stop_requested():
    p = os.environ.get("DRIFT_STOP_FILE", "/workspace/drift/STOP")
    return os.path.exists(p)

def now(): return time.strftime("%Y-%m-%dT%H:%M:%S%z")
