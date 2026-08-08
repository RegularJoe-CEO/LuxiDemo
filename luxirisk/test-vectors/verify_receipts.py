#!/usr/bin/env python3
"""Recompute SHA-256 of published LuxiRisk canonical payloads.

No binary required. Exit 0 if all receipts match vectors.json.
"""
from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path


def main() -> int:
    root = Path(__file__).resolve().parent
    data = json.loads((root / "vectors.json").read_text(encoding="utf-8"))
    failed = 0
    for v in data["vectors"]:
        payload = v["canonical_payload"]
        if not payload.endswith("\n"):
            print(f"FAIL {v['id']}: payload missing trailing newline")
            failed += 1
            continue
        full = hashlib.sha256(payload.encode("utf-8")).hexdigest()
        short = full[:12]
        ok_full = full == v["receipt_full"]
        ok_short = short == v["receipt_short"]
        status = "PASS" if (ok_full and ok_short) else "FAIL"
        if status == "FAIL":
            failed += 1
        print(f"  [{status}] {v['id']}")
        print(f"         short {short}  (expect {v['receipt_short']})")
        if not ok_full:
            print(f"         full  {full}")
            print(f"         expect {v['receipt_full']}")
    print()
    if failed:
        print(f"{failed} vector(s) failed")
        return 1
    print(f"All {len(data['vectors'])} receipt vectors verified.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
