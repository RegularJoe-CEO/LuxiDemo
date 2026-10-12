"""python-sgp4 2.25 (C++ extension) reference dump, same layout as luxi-sgp4 grid."""

import sys
from pathlib import Path

import numpy as np
from sgp4.api import Satrec


def kept(path):
    rows = []
    for line in Path(path).read_text().splitlines():
        line = line.rstrip("\r")
        if not line or line.startswith("#"):
            continue
        rows.append(line)
    return rows


def main():
    tle_path, minutes_path, out_path = sys.argv[1:]
    rows = kept(tle_path)
    times = [float(x) for x in kept(minutes_path)]
    out = np.empty((len(rows) // 2 * len(times), 6), dtype="<f8")
    n_err = 0
    item = 0
    for i in range(0, len(rows), 2):
        sat = Satrec.twoline2rv(rows[i][:69], rows[i + 1][:69])
        for t in times:
            err, r, v = sat.sgp4_tsince(t)
            if err != 0:
                n_err += 1
                out[item] = np.nan
            else:
                out[item, :3] = r
                out[item, 3:] = v
            item += 1
    out.tofile(out_path)
    print(f"n {item} n_errors {n_err} bytes {out.nbytes}")


if __name__ == "__main__":
    main()
