"""Record the echo prompt's chat-template token ids.

The ids come from the local server's tokenize route. They are checked against
the same messages rendered by the installed tokenizer. Usage:
  tokenize_echo.py OUT.json
"""
import json, sys, urllib.request

URL_CHAT = "http://127.0.0.1:8000/v1/chat/completions"
MODEL = "Qwen/Qwen2-7B-Instruct"
IDS = [
    "c_mhw9v76obuh72x98f4pb8xr3", "o_ftcj97ua0p5heapj5fpng02e",
    "c_3exww35axs739igmh42hvph1g", "c_8oold74tm25kw4uag86kyj16z",
    "c_6adnmt0f3gs4xfl4ne2x5zkjo", "c_z72re6iq4gltfujr1zrwll11y",
    "o_9klm2xr7ta4bnvq8wsdf31hgc", "c_pw4nz8tqm6ykxj02hcv5rb7ed",
]
PROMPT = ("Echo each of these identifiers back to me, one per line, prefixed "
          "with 'ECHO: '. Do not modify any character. Do not add commentary.\n\n"
          + "\n".join(IDS))
MESSAGES = [{"role": "user", "content": PROMPT}]


def post(url, body):
    req = urllib.request.Request(url, data=json.dumps(body).encode(),
                                 headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=120) as resp:
        return json.loads(resp.read().decode())


def server_ids():
    errors = []
    for url in ("http://127.0.0.1:8000/tokenize", "http://127.0.0.1:8000/v1/tokenize"):
        body = {"model": MODEL, "messages": MESSAGES, "add_generation_prompt": True}
        try:
            data = post(url, body)
        except Exception as e:
            errors.append(f"{url}: {type(e).__name__}")
            continue
        toks = data.get("tokens") or data.get("token_ids")
        if toks and isinstance(toks[0], int):
            return toks, url, data.get("count", len(toks))
        errors.append(f"{url}: unexpected keys {sorted(data)[:12]}")
    raise SystemExit("tokenize route failed: " + "; ".join(errors))


def as_id_list(value):
    # transformers 5 returns a BatchEncoding. list() on that is the field names.
    if hasattr(value, "keys") and "input_ids" in value.keys():
        value = value["input_ids"]
    if hasattr(value, "tolist"):
        value = value.tolist()
    if value and isinstance(value[0], list):
        if len(value) != 1:
            raise SystemExit(f"expected one token sequence, got {len(value)}")
        value = value[0]
    return [int(x) for x in value]


def library_ids():
    from transformers import AutoTokenizer
    tok = AutoTokenizer.from_pretrained("/root/models/Qwen2-7B-Instruct", trust_remote_code=False)
    raw = tok.apply_chat_template(MESSAGES, add_generation_prompt=True, tokenize=True)
    return as_id_list(raw)


def main():
    out = sys.argv[1]
    http_ids, url, count = server_ids()
    lib_ids = library_ids()
    if list(http_ids) != list(lib_ids):
        raise SystemExit(f"token id mismatch server={len(http_ids)} library={len(lib_ids)}")
    json.dump({"model": MODEL, "tokenize_url": url, "count": count,
               "ids": list(http_ids), "equal_to_tokenizer_chat_template": True},
              open(out, "w"))
    print(f"TOKEN_IDS count={count} url={url} equal=1")


if __name__ == "__main__":
    main()
