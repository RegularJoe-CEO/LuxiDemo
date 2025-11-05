#!/usr/bin/env python3
import os, json, requests, sys
BASE = sys.argv[1] if len(sys.argv) > 1 else os.environ.get("BASE", "http://127.0.0.1:8080")

def jprint(label, r):
    print(f"{label}: {r.text}")

print("health:", end=" ")
r = requests.get(f"{BASE}/health"); print(r.text)

r = requests.post(f"{BASE}/evaluate", json={
    "expr": "x*x + 2*x + 1",
    "x": [0.0, 1.0, 2.0, 3.0]
}); jprint("evaluate", r)

r = requests.post(f"{BASE}/bisect", json={
    "expr": "x*x - 2",
    "lo": 1.0, "hi": 2.0, "tol": 1e-9, "max_iter": 60
}); jprint("bisect", r)

r = requests.post(f"{BASE}/bisect_auto", json={
    "expr": "x*x - 2",
    "guess": 1.0, "step": 0.5, "max_expand": 20, "tol": 1e-9, "max_iter": 60
}); jprint("bisect_auto", r)
