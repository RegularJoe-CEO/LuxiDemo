"""Check the powermetrics medians against the log, the README, and RESULTS.md."""

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REPO = ROOT.parents[1]

WINDOWS = (
    "sdn_luxi_1",
    "sdn_luxi_8",
    "sdn_stock_1",
    "sdn_stock_8",
    "kep6_luxi_1",
    "kep6_stock_1",
    "pc_luxi_1",
    "pc_luxi_8",
    "pc_stock_1",
)


def median(values):
    ordered = sorted(values)
    return ordered[len(ordered) // 2]


def main() -> int:
    rows = {}
    runs = {name: [] for name in WINDOWS}
    for line in (ROOT / "results" / "energy-job3.txt").read_text().splitlines():
        parts = line.split()
        if len(parts) < 3 or parts[0] not in WINDOWS:
            continue
        if parts[1] == "median_cpu_j":
            rows.setdefault(parts[0], {})["cpu"] = parts[2]
        elif parts[1] == "median_combined_j":
            rows.setdefault(parts[0], {})["combined"] = parts[2]
        elif parts[1] == "run" and "cpu_j" in parts:
            runs[parts[0]].append(float(parts[parts.index("cpu_j") + 1]))

    readme = (ROOT / "README.md").read_text()
    results = (REPO / "RESULTS.md").read_text()
    failed = 0
    for name in WINDOWS:
        got = rows.get(name, {})
        if "cpu" not in got or "combined" not in got:
            print(f"FAIL missing median {name}")
            failed += 1
            continue
        if f"{median(runs[name]):.6f}" != got["cpu"]:
            print(f"FAIL recomputed median {name}")
            failed += 1
        if got["cpu"] not in readme or got["cpu"] not in results:
            print(f"FAIL cpu joules not quoted {name} {got['cpu']}")
            failed += 1
        if got["combined"] not in readme:
            print(f"FAIL combined joules not in README {name}")
            failed += 1
    if "Apple" not in readme or "estimated" not in readme:
        print("FAIL estimate label missing")
        failed += 1
    print(f"energy check failed {failed}")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
