"""Check the catalog and the recorded OrbVeil claims in this folder.

The 1,102,537,920-byte state dumps are not in git. This script checks the
catalog bytes, the six ISS element columns, and that README.md quotes the
same digests and table numbers as results/.
"""

from __future__ import annotations

import hashlib
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TLE = ROOT / "data" / "active.tle"

CATALOG_SHA256 = "ebe2271f339f4af42be8ab8b196f6c7084211122205ac7663e17e9fd1b41d0ee"
N_OBJECTS = 15940

LUXI = "a14f625475a05892277a1a198aacc79aaf18b0192c97326bbe50fcafd31f8fa3"
STOCK_MAC = "34dd469efae3caa123526120e70c880df46afc4569e2c8204298c93952e0f2bf"
STOCK_ARM = "b3fab0207b9a0e15d72235a07a07f6cbfd06460adcdd129c73aa3681905effb5"
STOCK_X86 = "77f8a382db9729a9b64024bdd949fd83d6c3f858c12cf8741f3d596b718e312e"

ZARYA = 25544
DOCKED = (25575, 26400, 26700, 36086, 49044, 68837)
MEAN_KEYS = (
    "epoch",
    "ndot",
    "nddot",
    "bstar",
    "inc",
    "raan",
    "ecc",
    "argp",
    "ma",
    "n",
)


def fail(msg: str) -> None:
    print(f"FAIL {msg}")


def fields(line1: str, line2: str) -> dict[str, str]:
    return {
        "epoch": line1[18:32],
        "ndot": line1[33:43],
        "nddot": line1[44:52],
        "bstar": line1[53:61],
        "elset": line1[64:68],
        "inc": line2[8:16],
        "raan": line2[17:25],
        "ecc": line2[26:33],
        "argp": line2[34:42],
        "ma": line2[43:51],
        "n": line2[52:63],
    }


def load_tles(text: str) -> dict[int, tuple[str, str, str]]:
    lines = text.splitlines()
    found: dict[int, tuple[str, str, str]] = {}
    i = 0
    n = 0
    while i < len(lines):
        line = lines[i]
        if line.startswith("1 ") and len(line) >= 69:
            norad = int(line[2:7])
            name = lines[i - 1].strip() if i and not lines[i - 1].startswith(("1 ", "2 ")) else ""
            line2 = lines[i + 1]
            found[norad] = (name, line, line2)
            n += 1
            i += 2
        else:
            i += 1
    if n != N_OBJECTS:
        raise RuntimeError(f"object count {n} != {N_OBJECTS}")
    return found


def main() -> int:
    failed = 0
    raw = TLE.read_bytes()
    digest = hashlib.sha256(raw).hexdigest()
    if digest != CATALOG_SHA256:
        fail(f"catalog sha256 {digest}")
        failed += 1
    else:
        print(f"catalog sha256 ok bytes {len(raw)}")

    try:
        found = load_tles(raw.decode("ascii"))
    except Exception as exc:
        fail(str(exc))
        return 1

    zarya = fields(found[ZARYA][1], found[ZARYA][2])
    if zarya["elset"] != " 999":
        fail(f"zarya element set {zarya['elset']!r}")
        failed += 1
    for norad in DOCKED:
        name, line1, line2 = found[norad]
        got = fields(line1, line2)
        for key in MEAN_KEYS:
            if got[key] != zarya[key]:
                fail(f"{norad} {key} {got[key]!r} != {zarya[key]!r}")
                failed += 1
        if got["elset"] != " 999":
            fail(f"{norad} element set {got['elset']!r}")
            failed += 1
        if line1 == found[ZARYA][1] or line2 == found[ZARYA][2]:
            fail(f"{norad} full line matches Zarya")
            failed += 1
        print(f"iss columns ok {norad} {name}")

    blobs = {
        "results/hashes.txt": (ROOT / "results" / "hashes.txt").read_text(),
        "results/table.txt": (ROOT / "results" / "table.txt").read_text(),
        "results/position.txt": (ROOT / "results" / "position.txt").read_text(),
        "results/iss.txt": (ROOT / "results" / "iss.txt").read_text(),
        "README.md": (ROOT / "README.md").read_text(),
    }
    required = {
        "results/hashes.txt": [LUXI, STOCK_MAC, STOCK_ARM, STOCK_X86, "6436873", "18668301", "17249284"],
        "results/table.txt": ["5.332472", "1.034126", "5.333366", "1.054219", "22.923342", "16.263594", "20.677514", "16.226887"],
        "results/position.txt": ["6.62406383491043714e-03", "7.79453790726550245e-09", "1.79077728654311157e-04"],
        "results/iss.txt": ["6054", "300", "600"],
        "README.md": [LUXI, STOCK_MAC, STOCK_ARM, STOCK_X86, "5.332472", "1.034126", "5.333366", "1.054219", "22.923342", "16.263594", "20.677514", "16.226887", "6.62406383491043714e-03", "6,054"],
    }
    for name, needles in required.items():
        text = blobs[name]
        for needle in needles:
            if needle not in text:
                fail(f"{name} missing {needle}")
                failed += 1
        print(f"quotes ok {name}")

    if "\u2014" in blobs["README.md"] or "\u2013" in blobs["README.md"]:
        fail("README contains an em dash or an en dash")
        failed += 1

    print(f"failed {failed}")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
