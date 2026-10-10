"""Compare two little-endian f64 state dumps. Six values per state."""

import hashlib
import json
import sys

import numpy as np


def load(path):
    raw = open(path, "rb").read()
    if len(raw) % 48 != 0:
        raise SystemExit(f"{path} length {len(raw)} is not a multiple of 48")
    return np.frombuffer(raw, dtype="<f8").reshape(-1, 6).copy()


def main():
    a = load(sys.argv[1])
    b = load(sys.argv[2])
    if a.shape != b.shape:
        raise SystemExit(f"shape {a.shape} vs {b.shape}")
    both = np.isfinite(a).all(axis=1) & np.isfinite(b).all(axis=1)
    pos = np.linalg.norm(a[:, :3] - b[:, :3], axis=1)
    vel = np.linalg.norm(a[:, 3:] - b[:, 3:], axis=1)
    mismatch = np.any(a.view(np.uint64) != b.view(np.uint64), axis=1)
    summary = {
        "n": int(a.shape[0]),
        "n_both_finite": int(both.sum()),
        "n_bit_mismatch": int(mismatch.sum()),
        "max_pos_km": float(pos[both].max()) if both.any() else None,
        "max_vel_km_s": float(vel[both].max()) if both.any() else None,
        "sha_a": hashlib.sha256(open(sys.argv[1], "rb").read()).hexdigest(),
        "sha_b": hashlib.sha256(open(sys.argv[2], "rb").read()).hexdigest(),
    }
    print(json.dumps(summary, indent=2))
    if summary["n_bit_mismatch"] != 0:
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
