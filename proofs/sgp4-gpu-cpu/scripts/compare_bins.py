"""Compare two little-endian f64 state dumps. Six values per state."""

import hashlib
import json
import sys

import numpy as np


def load(path):
    raw = open(path, "rb").read()
    if len(raw) % (6 * 8) != 0:
        raise SystemExit(f"{path} length {len(raw)} is not a multiple of 48")
    return np.frombuffer(raw, dtype="<f8").reshape(-1, 6).copy()


def main():
    a = load(sys.argv[1])
    b = load(sys.argv[2])
    if a.shape != b.shape:
        raise SystemExit(f"shape {a.shape} vs {b.shape}")
    pos = np.linalg.norm(a[:, :3] - b[:, :3], axis=1)
    vel = np.linalg.norm(a[:, 3:] - b[:, 3:], axis=1)
    finite = np.isfinite(pos) & np.isfinite(vel)
    bit_mismatch = np.any(a.view(np.uint64) != b.view(np.uint64), axis=1)
    summary = {
        "n": int(a.shape[0]),
        "n_finite": int(finite.sum()),
        "n_bit_mismatch": int(bit_mismatch.sum()),
        "max_pos_km": float(np.nanmax(pos)) if finite.any() else None,
        "max_pos_m": float(np.nanmax(pos) * 1000.0) if finite.any() else None,
        "max_vel_km_s": float(np.nanmax(vel)) if finite.any() else None,
        "sha_a": hashlib.sha256(open(sys.argv[1], "rb").read()).hexdigest(),
        "sha_b": hashlib.sha256(open(sys.argv[2], "rb").read()).hexdigest(),
    }
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
