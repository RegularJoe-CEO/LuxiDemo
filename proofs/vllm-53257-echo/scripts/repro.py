import concurrent.futures as cf, json, sys, time, urllib.request
from collections import Counter

URL   = "http://127.0.0.1:8000/v1/chat/completions"
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

def call(_):
    req = urllib.request.Request(URL, data=json.dumps(BODY).encode(),
                                 headers={"Content-Type": "application/json"})
    return json.loads(urllib.request.urlopen(req, timeout=600).read()
                      )["choices"][0]["message"]["content"]

n, conc = int(sys.argv[1]), int(sys.argv[2])
if conc == 1:
    outs = [call(i) for i in range(n)]
else:
    with cf.ThreadPoolExecutor(max_workers=conc) as ex:
        outs = [f.result() for f in cf.as_completed(
            [ex.submit(call, i) for i in range(n)])]

v = Counter(outs)
minority = sum(c for _, c in v.most_common()[1:])
print(f"n={len(outs)} conc={conc} DISTINCT={len(v)} "
      f"minority={minority} ({100*minority/len(outs):.2f}%)")
print("EXPECTED at temperature=0: DISTINCT=1")
