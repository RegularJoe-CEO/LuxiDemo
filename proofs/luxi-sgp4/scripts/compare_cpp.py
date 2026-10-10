"""Euclidean gap between a stored dump and python-sgp4 2.25.

Layout matches the dumps in this folder: little-endian f64, satellite-major
then time, six fields. Blank lines, '#' lines, and lines that do not start
with '1' or '2' are skipped. python error 6 still carries a position; other
python errors must line up with a NaN state in the dump.
"""

import json
import sys
from pathlib import Path

import numpy as np
from sgp4.api import Satrec


def tle_lines(path):
    rows = []
    for line in Path(path).read_text().splitlines():
        line = line.rstrip("\r").rstrip()
        if not line or line.startswith("#"):
            continue
        if line[0] not in ("1", "2"):
            continue
        rows.append(line)
    return rows


def minute_lines(path):
    rows = []
    for line in Path(path).read_text().splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        rows.append(line)
    return rows


def main():
    tle_path, minutes_path, dump_path = sys.argv[1:4]
    rows = tle_lines(tle_path)
    if len(rows) % 2 != 0:
        raise SystemExit(f"{tle_path} has an odd number of TLE lines")
    times = [float(x) for x in minute_lines(minutes_path)]
    dump = np.fromfile(dump_path, dtype="<f8").reshape(-1, 6)
    expect = (len(rows) // 2) * len(times)
    if dump.shape[0] != expect:
        raise SystemExit(f"{dump_path} has {dump.shape[0]} states, expected {expect}")
    max_pos = 0.0
    max_vel = 0.0
    worst = ""
    n_finite = 0
    n_both_nan = 0
    item = 0
    opmode = None
    for i in range(0, len(rows), 2):
        sat = Satrec.twoline2rv(rows[i][:69], rows[i + 1][:69])
        opmode = sat.operationmode
        for t in times:
            err, r, v = sat.sgp4_tsince(float(t))
            pred = dump[item]
            item += 1
            if err not in (0, 6):
                if np.isfinite(pred).any():
                    raise SystemExit(f"python error {err} at t={t} but the dump is finite")
                n_both_nan += 1
                continue
            if not np.isfinite(pred).all():
                raise SystemExit(f"python returned a state at t={t} (err={err}) but the dump is NaN")
            dp = float(np.linalg.norm(pred[:3] - np.array(r, dtype=np.float64)))
            dv = float(np.linalg.norm(pred[3:] - np.array(v, dtype=np.float64)))
            n_finite += 1
            if dp > max_pos:
                max_pos = dp
                worst = f"pair {i // 2} t={t}"
            if dv > max_vel:
                max_vel = dv
    summary = {
        "n": expect,
        "n_finite": n_finite,
        "n_both_rejected": n_both_nan,
        "operationmode": opmode,
        "max_pos_km": max_pos,
        "max_vel_km_s": max_vel,
        "worst_pos": worst,
        "pos_tol_1e-6_km": "PASS" if max_pos < 1e-6 else "FAIL",
        "vel_tol_1e-9_km_s": "PASS" if max_vel < 1e-9 else "FAIL",
    }
    print(json.dumps(summary, indent=2))
    if summary["pos_tol_1e-6_km"] != "PASS" or summary["vel_tol_1e-9_km_s"] != "PASS":
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
