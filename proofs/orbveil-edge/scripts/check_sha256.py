"""Verify SHA256SUMS beside this script's parent directory."""

import hashlib
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def main() -> int:
    text = (ROOT / "SHA256SUMS").read_text()
    failed = 0
    checked = 0
    for line in text.splitlines():
        if not line.strip():
            continue
        digest, name = line.split(None, 1)
        name = name.strip()
        if name.startswith("*"):
            name = name[1:]
        path = ROOT / name
        got = hashlib.sha256(path.read_bytes()).hexdigest()
        checked += 1
        if got != digest:
            print(f"FAIL {name}")
            print(f"  expected {digest}")
            print(f"  got      {got}")
            failed += 1
    print(f"checked {checked} failed {failed}")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
