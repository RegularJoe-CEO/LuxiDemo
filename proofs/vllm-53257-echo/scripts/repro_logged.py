"""Same client as repro.py, plus one JSONL record per request.

A non-200 response or a transport failure makes the process exit 3 so the
caller can discard the run and repeat it. Records are written only after
every request in the run has succeeded.
"""
import concurrent.futures as cf, json, sys, time, urllib.error, urllib.request
from collections import Counter

URL = "http://127.0.0.1:8000/v1/chat/completions"
MODEL = "Qwen/Qwen2-7B-Instruct"

IDS = [
    "c_mhw9v76obuh72x98f4pb8xr3",  "o_ftcj97ua0p5heapj5fpng02e",
    "c_3exww35axs739igmh42hvph1g", "c_8oold74tm25kw4uag86kyj16z",
    "c_6adnmt0f3gs4xfl4ne2x5zkjo", "c_z72re6iq4gltfujr1zrwll11y",
    "o_9klm2xr7ta4bnvq8wsdf31hgc", "c_pw4nz8tqm6ykxj02hcv5rb7ed",
]
PROMPT = ("Echo each of these identifiers back to me, one per line, prefixed "
          "with 'ECHO: '. Do not modify any character. Do not add commentary.\n\n"
          + "\n".join(IDS))
BODY = {"model": MODEL, "messages": [{"role": "user", "content": PROMPT}],
        "temperature": 0.0, "max_tokens": 2000}


def call(i):
    t0 = time.time()
    req = urllib.request.Request(URL, data=json.dumps(BODY).encode(),
                                 headers={"Content-Type": "application/json"})
    try:
        resp = urllib.request.urlopen(req, timeout=600)
        raw = resp.read()
        status = getattr(resp, "status", 200)
        t1 = time.time()
        content = json.loads(raw)["choices"][0]["message"]["content"]
        return {"i": i, "t_start": t0, "t_end": t1, "http_status": status,
                "completion": content, "error": None}
    except urllib.error.HTTPError as e:
        return {"i": i, "t_start": t0, "t_end": time.time(), "http_status": e.code,
                "completion": None, "error": "http"}
    except Exception as e:
        return {"i": i, "t_start": t0, "t_end": time.time(), "http_status": 0,
                "completion": None, "error": type(e).__name__}


def main():
    n, conc = int(sys.argv[1]), int(sys.argv[2])
    out_path = sys.argv[3]
    meta = json.loads(sys.argv[4]) if len(sys.argv) > 4 else {}
    if conc == 1:
        recs = [call(i) for i in range(n)]
    else:
        with cf.ThreadPoolExecutor(max_workers=conc) as ex:
            recs = [f.result() for f in cf.as_completed(ex.submit(call, i) for i in range(n))]
    bad = [r for r in recs if r["error"] or r["http_status"] != 200 or r["completion"] is None]
    if bad:
        print(f"TRANSPORT n_bad={len(bad)} example_status={bad[0]['http_status']} "
              f"example_error={bad[0]['error']}", file=sys.stderr)
        sys.exit(3)
    outs = [r["completion"] for r in recs]
    v = Counter(outs)
    minority = sum(c for _, c in v.most_common()[1:])
    print(f"n={len(outs)} conc={conc} DISTINCT={len(v)} "
          f"minority={minority} ({100 * minority / len(outs):.2f}%)")
    print("EXPECTED at temperature=0: DISTINCT=1")
    with open(out_path, "w") as f:
        for r in sorted(recs, key=lambda x: x["i"]):
            r.update(meta)
            f.write(json.dumps(r, ensure_ascii=False) + "\n")


if __name__ == "__main__":
    main()
