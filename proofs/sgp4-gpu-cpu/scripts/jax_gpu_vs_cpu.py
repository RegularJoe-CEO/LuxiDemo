"""jaxsgp4 f64, paper commit, GPU versus CPU.

Same grid as scripts/plot_errors.py: seed 11, 100 Starlink TLEs,
29 times from 0 through 14 days at half-day steps.

Usage on a machine that already has jaxsgp4 and sgp4 installed:
  JAX_ENABLE_X64=1 JAX_PLATFORMS=cpu python jax_gpu_vs_cpu.py dump --batch per_sat --out cpu.npy
  JAX_ENABLE_X64=1 JAX_PLATFORMS=cuda python jax_gpu_vs_cpu.py dump --batch per_sat --out gpu.npy
  python jax_gpu_vs_cpu.py compare cpu.npy gpu.npy
"""

import hashlib
import json
import os
import sys

os.environ.setdefault("JAX_ENABLE_X64", "1")

import jax
import jax.numpy as jnp
import numpy as np
from importlib.resources import files
from jaxsgp4 import sgp4, tle2sat


def load_pairs(n, seed):
    data = files("jaxsgp4").joinpath("data/starlink.txt").read_text()
    lines = data.splitlines()
    tle1, tle2 = [], []
    for i in range(0, len(lines) - 1):
        if lines[i].startswith("1 ") and lines[i + 1].startswith("2 "):
            tle1.append(lines[i][:69])
            tle2.append(lines[i + 1][:69])
    np.random.seed(seed)
    idx = np.random.choice(len(tle1), size=n, replace=False)
    return [tle1[i] for i in idx], [tle2[i] for i in idx]


def as_rv(result):
    if isinstance(result, tuple):
        return result[0]
    return result


def times():
    return np.arange(0, 14.5 * 1440, 0.5 * 1440)


def propagate(tle1, tle2, tsince, batch):
    ts = jnp.asarray(tsince)
    if batch == "per_sat":
        fn = jax.jit(jax.vmap(lambda sat, t: as_rv(sgp4(sat, t)), in_axes=(None, 0)))
        rows = []
        for a, b in zip(tle1, tle2):
            rows.append(np.asarray(fn(tle2sat(a, b), ts)))
        return np.stack(rows)
    if batch == "scalar":
        fn = jax.jit(lambda sat, t: as_rv(sgp4(sat, t)))
        rows = []
        for a, b in zip(tle1, tle2):
            sat = tle2sat(a, b)
            rows.append(np.stack([np.asarray(fn(sat, t)) for t in ts]))
        return np.stack(rows)
    if batch == "all":
        sats = [tle2sat(a, b) for a, b in zip(tle1, tle2)]
        stacked = jax.tree.map(lambda *xs: jnp.stack([jnp.asarray(x) for x in xs]), *sats)
        fn = jax.jit(
            jax.vmap(
                jax.vmap(lambda sat, t: as_rv(sgp4(sat, t)), in_axes=(None, 0)),
                in_axes=(0, None),
            )
        )
        return np.asarray(fn(stacked, ts))
    raise SystemExit(f"unknown batch {batch}")


def dump_main(argv):
    batch = "per_sat"
    out = None
    n = int(os.environ.get("N_SATS", "100"))
    seed = int(os.environ.get("SEED", "11"))
    runs = 1
    i = 0
    while i < len(argv):
        if argv[i] == "--batch":
            batch = argv[i + 1]
            i += 2
        elif argv[i] == "--out":
            out = argv[i + 1]
            i += 2
        elif argv[i] == "--runs":
            runs = int(argv[i + 1])
            i += 2
        elif argv[i] == "--n":
            n = int(argv[i + 1])
            i += 2
        else:
            raise SystemExit(f"unknown arg {argv[i]}")
    if out is None:
        raise SystemExit("need --out")
    print(
        "jax", jax.__version__,
        "x64", bool(jax.config.jax_enable_x64),
        "backend", jax.default_backend(),
        "devices", jax.devices(),
        "batch", batch,
        "n", n,
        flush=True,
    )
    tle1, tle2 = load_pairs(n, seed)
    tsince = times()
    print("n_sats", len(tle1), "n_times", len(tsince), flush=True)
    for run in range(runs):
        path = out if runs == 1 else f"{out}.run{run + 1}"
        states = np.asarray(propagate(tle1, tle2, tsince, batch), dtype=np.float64)
        states = np.ascontiguousarray(states)
        raw = states.astype("<f8", copy=False).tobytes()
        sha = hashlib.sha256(raw).hexdigest()
        np.save(path, states)
        meta = {
            "path": path,
            "sha256": sha,
            "shape": list(states.shape),
            "batch": batch,
            "run": run + 1,
            "backend": jax.default_backend(),
            "device": str(jax.devices()[0]),
            "x64": bool(jax.config.jax_enable_x64),
            "jax": jax.__version__,
            "n_nonfinite": int(np.size(states) - np.isfinite(states).sum()),
        }
        print(json.dumps(meta), flush=True)
        with open(path + ".json", "w") as f:
            json.dump(meta, f, indent=2)
            f.write("\n")


def compare_main(argv):
    a = np.load(argv[0])
    b = np.load(argv[1])
    if a.shape != b.shape:
        raise SystemExit(f"shape {a.shape} vs {b.shape}")
    pos = np.linalg.norm(a[..., :3] - b[..., :3], axis=-1)
    vel = np.linalg.norm(a[..., 3:] - b[..., 3:], axis=-1)
    bit = np.any(a.view(np.uint64) != b.view(np.uint64), axis=-1)
    summary = {
        "shape": list(a.shape),
        "n_states": int(bit.size),
        "n_bit_mismatch": int(bit.sum()),
        "n_finite": int(np.isfinite(pos).sum()),
        "max_pos_km": float(np.nanmax(pos)),
        "max_vel_km_s": float(np.nanmax(vel)),
        "sha_a": hashlib.sha256(np.ascontiguousarray(a).astype("<f8").tobytes()).hexdigest(),
        "sha_b": hashlib.sha256(np.ascontiguousarray(b).astype("<f8").tobytes()).hexdigest(),
    }
    print(json.dumps(summary, indent=2))


def main():
    if len(sys.argv) < 2:
        raise SystemExit("usage: dump|compare")
    cmd = sys.argv[1]
    if cmd == "dump":
        dump_main(sys.argv[2:])
    elif cmd == "compare":
        compare_main(sys.argv[2:])
    else:
        raise SystemExit(cmd)


if __name__ == "__main__":
    main()
