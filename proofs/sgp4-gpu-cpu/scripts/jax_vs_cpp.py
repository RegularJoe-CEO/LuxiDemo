"""Stock jaxsgp4 (commit pinned by the caller) versus python-sgp4 2.25.

Replays the Zenodo plot_errors.py grid: seed 11, 100 Starlink TLEs,
29 times from 0 to 14 days at half-day steps. Runs once in JAX's default
float32 and once the caller should re-invoke with JAX_ENABLE_X64=1.
"""

import hashlib
import json
import os
import sys

import jax
import jax.numpy as jnp
import numpy as np
from importlib.resources import files
from jaxsgp4 import sgp4, tle2sat
from sgp4.api import Satrec


def load_pairs(n, seed):
    data = files("jaxsgp4").joinpath("data/starlink.txt").read_text()
    lines = data.splitlines()
    tle1, tle2 = [], []
    for i in range(0, len(lines) - 1):
        if lines[i].startswith("1 ") and lines[i + 1].startswith("2 "):
            tle1.append(lines[i][:69])
            tle2.append(lines[i + 1][:69])
    rng = np.random.default_rng(seed)
    # Match plot_errors.py, which uses np.random.seed + np.random.choice.
    np.random.seed(seed)
    idx = np.random.choice(len(tle1), size=n, replace=False)
    return [tle1[i] for i in idx], [tle2[i] for i in idx]


def as_rv(result):
    if isinstance(result, tuple):
        return result[0]
    return result


def main():
    n = int(os.environ.get("N_SATS", "100"))
    seed = int(os.environ.get("SEED", "11"))
    x64 = jax.config.jax_enable_x64
    print("jax", jax.__version__, "x64", bool(x64), "backend", jax.default_backend())
    print("devices", jax.devices())
    tle1, tle2 = load_pairs(n, seed)
    tsince = np.arange(0, 14.5 * 1440, 0.5 * 1440)
    print("n_times", len(tsince), "n_sats", len(tle1))
    fn = jax.jit(jax.vmap(lambda sat, t: as_rv(sgp4(sat, t)), in_axes=(None, 0)))

    max_pos = 0.0
    max_vel = 0.0
    n_pos_bits = 0
    n_states = 0
    n_cpp_err = 0
    n_nonfinite = 0
    pos_bytes = bytearray()
    cpp_bytes = bytearray()
    per_time_max = np.zeros(len(tsince))

    for j, (a, b) in enumerate(zip(tle1, tle2)):
        sat = tle2sat(a, b)
        cpp = Satrec.twoline2rv(a, b)
        jax_rv = np.asarray(fn(sat, jnp.asarray(tsince)))
        jd = np.arange(cpp.jdsatepoch, cpp.jdsatepoch + 14.5, 0.5)
        fr = np.full(len(tsince), cpp.jdsatepochF)
        err, r_cpp, v_cpp = cpp.sgp4_array(jd, fr)
        if jax_rv.shape != (len(tsince), 6):
            raise SystemExit(f"unexpected jax shape {jax_rv.shape}")
        r_j = jax_rv[:, :3].astype(np.float64)
        v_j = jax_rv[:, 3:].astype(np.float64)
        for k in range(len(tsince)):
            n_states += 1
            if err[k] != 0:
                n_cpp_err += 1
                continue
            if not np.all(np.isfinite(r_j[k])) or not np.all(np.isfinite(v_j[k])):
                n_nonfinite += 1
                continue
            dp = float(np.linalg.norm(r_j[k] - r_cpp[k]))
            dv = float(np.linalg.norm(v_j[k] - v_cpp[k]))
            if dp > max_pos:
                max_pos = dp
            if dv > max_vel:
                max_vel = dv
            if dp > per_time_max[k]:
                per_time_max[k] = dp
            jb = r_j[k].astype("<f8").tobytes() + v_j[k].astype("<f8").tobytes()
            cb = r_cpp[k].astype("<f8").tobytes() + v_cpp[k].astype("<f8").tobytes()
            pos_bytes += jb
            cpp_bytes += cb
            if jb != cb:
                n_pos_bits += 1
        if (j + 1) % 25 == 0:
            print(f"progress {j+1}/{len(tle1)} max_pos_km {max_pos:.6e}", flush=True)

    summary = {
        "jax": jax.__version__,
        "x64": bool(x64),
        "backend": jax.default_backend(),
        "device": str(jax.devices()[0]),
        "n_sats": len(tle1),
        "n_times": int(len(tsince)),
        "seed": seed,
        "max_pos_km": max_pos,
        "max_pos_m": max_pos * 1000.0,
        "max_vel_km_s": max_vel,
        "n_states": n_states,
        "n_bit_mismatches": n_pos_bits,
        "n_cpp_err": n_cpp_err,
        "n_nonfinite": n_nonfinite,
        "pos_km_at_day": {str(i * 0.5): float(per_time_max[i]) for i in range(len(tsince))},
        "jax_state_sha256": hashlib.sha256(pos_bytes).hexdigest(),
        "cpp_state_sha256": hashlib.sha256(cpp_bytes).hexdigest(),
    }
    out = sys.argv[1] if len(sys.argv) > 1 else "jax_vs_cpp.json"
    with open(out, "w") as f:
        json.dump(summary, f, indent=2)
        f.write("\n")
    print(json.dumps({k: summary[k] for k in summary if k != "pos_km_at_day"}, indent=2))


if __name__ == "__main__":
    main()
