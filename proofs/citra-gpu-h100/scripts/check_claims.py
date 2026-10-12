"""Check the H100 record against this folder and the results log."""

import json
import math
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REPO = ROOT.parents[1]


def median(values):
    ordered = sorted(values)
    return ordered[len(ordered) // 2]


def main() -> int:
    data = json.loads((ROOT / "results" / "h100.json").read_text())
    readme = (ROOT / "README.md").read_text()
    results = (REPO / "RESULTS.md").read_text()
    failed = 0

    def need(text, label):
        nonlocal failed
        if text not in readme or text not in results:
            print(f"FAIL missing {label}: {text}")
            failed += 1

    luxi = data["luxi_relaunch"]
    for name in ("jax100", "starlink_382a419"):
        row = luxi[name]
        if median(row["runs_s"]) != row["median_s"]:
            print(f"FAIL median {name}")
            failed += 1
        need(row["sha256"], name)
        need(str(row["median_s"]), name)

    prior = json.loads((REPO / "proofs" / "luxi-sgp4" / "results" / "h100.json").read_text())
    prior_hash = {item["grid"]: item["sha256"] for item in prior["launches"]}
    if prior_hash["jax100"] != luxi["jax100"]["sha256"]:
        print("FAIL jax100 hash does not match proofs/luxi-sgp4")
        failed += 1
    if prior_hash["starlink-382a419"] != luxi["starlink_382a419"]["sha256"]:
        print("FAIL starlink hash does not match proofs/luxi-sgp4")
        failed += 1

    kep = data["keplemon"]
    diff = kep["iss_24h_diff_km_printed"]
    meters = math.sqrt(sum(x * x for x in diff)) * 1000.0
    if round(meters, 3) != kep["max_position_m_printed"]:
        print(f"FAIL printed diff is {meters} m, not {kep['max_position_m_printed']}")
        failed += 1
    need("1.649", "keplemon max")
    if data["orbveil_catalog"]["ran"] or data["orbveil_catalog"]["produced_on_this_gpu"]:
        print("FAIL catalog was not run")
        failed += 1
    if luxi["joules"] is not None or data["board_power"]["propagate_window_joules"] is not None:
        print("FAIL joules were not measured")
        failed += 1
    need("72.47", "idle watts")
    if "a14f625475a05892277a1a198aacc79aaf18b0192c97326bbe50fcafd31f8fa3" not in readme:
        print("FAIL catalog hash missing from README")
        failed += 1

    print(f"citra check failed {failed}")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
