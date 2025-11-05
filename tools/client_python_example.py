#!/usr/bin/env python3
import argparse, json, sys, urllib.request, urllib.error

def post_json(url: str, payload: dict):
    data = json.dumps(payload).encode("utf-8")
    req = urllib.request.Request(url, data=data, headers={"Content-Type": "application/json"})
    try:
        with urllib.request.urlopen(req, timeout=20) as resp:
            body = resp.read().decode("utf-8")
            print(body)
            return resp.status, body
    except urllib.error.HTTPError as e:
        sys.stderr.write(f"HTTP {e.code} for {url}\n")
        sys.stderr.write(e.read().decode("utf-8", errors="ignore") + "\n")
        sys.exit(1)

def add_precision(url: str, precision: str|None):
    if not precision:
        return url
    sep = "&" if "?" in url else "?"
    return f"{url}{sep}precision={precision}"

def main():
    ap = argparse.ArgumentParser(description="Luxi Edge client example (PR-05 precision ready)")
    ap.add_argument("--base", default="http://localhost:8080", help="Base URL, default http://localhost:8080")
    ap.add_argument("--precision", choices=["f64","f32","auto"], help="Optional precision hint (query parameter). Older servers ignore it.")
    sub = ap.add_subparsers(dest="cmd", required=True)

    pe = sub.add_parser("evaluate")
    pe.add_argument("--expr", required=True)
    pe.add_argument("--x", type=float, required=True)

    pb = sub.add_parser("bisect")
    pb.add_argument("--expr", required=True)
    pb.add_argument("--a", type=float, required=True)
    pb.add_argument("--b", type=float, required=True)
    pb.add_argument("--tol", type=float, default=1e-9)

    pa = sub.add_parser("bisect_auto")
    pa.add_argument("--expr", required=True)
    pa.add_argument("--tol", type=float, default=1e-9)

    args = ap.parse_args()
    if args.cmd == "evaluate":
        url = add_precision(f"{args.base}/evaluate", args.precision)
        post_json(url, {"expression": args.expr, "x": args.x})
    elif args.cmd == "bisect":
        url = add_precision(f"{args.base}/bisect", args.precision)
        post_json(url, {"expression": args.expr, "a": args.a, "b": args.b, "tolerance": args.tol})
    elif args.cmd == "bisect_auto":
        url = add_precision(f"{args.base}/bisect_auto", args.precision)
        post_json(url, {"expression": args.expr, "tolerance": args.tol})

if __name__ == "__main__":
    main()
