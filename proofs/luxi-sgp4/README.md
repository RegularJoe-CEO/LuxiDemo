# SGP4 and SDP4 CPU evidence

Measured 2026-10-10 on macOS arm64, rustc 1.92.0 (ded5c06cf 2025-12-08). This folder is inputs, state dumps, hashes, and checkers. It does not contain propagator source.

WGS-72, AFSPC mode. Little-endian f64. Satellite-major, then time. Six fields: x, y, z in km, then vx, vy, vz in km/s. A rejected state is six NaNs. Lines that are blank, start with `#`, or do not start with `1` or `2` are skipped, so a 3LE name line is not a TLE.

## State dumps

| grid | satellites | times | errors | bytes | sha256 |
| --- | ---: | ---: | ---: | ---: | --- |
| `states/jax100.bin` | 100 | 29 | 0 | 139200 | `26eafb2b244fc1d4e24a7f561378774cf934f3976364fd7cc402853975d7175e` |
| `states/starlink-382a419.bin` | 9341 | 29 | 79 | 13002672 | `42d0ca2ca598dae1553ecfae12ca8204b3dae5ceb67e9afa76e36dcb40a75c09` |
| `states/keplemon.bin` | 9 | 5 | 0 | 2160 | `87b46f78c620a1672e8b93d65bcca713b0386be60ea4df6edde25c4745be48a2` |

jax100 is 100 Starlink TLEs, `numpy` seed 11, from jaxsgp4 commit `382a41997c2ec35869da45b950506d88a38fbbc3`. Times are 0 through 14 days at half-day steps (`grids/jax100.minutes`). The full-file dump uses the same times and `grids/starlink-382a419.txt` from that commit. The 79 errors are rejected states, stored as NaN.

`states/keplemon.bin` is byte-identical to `proofs/sgp4-gpu-cpu/states/keplemon.bin` on the earlier evidence branch. That older file was produced by a different propagator and was measured on more than one machine. This folder's copy was produced on macOS arm64 only. No GPU hash was measured for these dumps.

## Vallado

SGP4-VER vectors shipped with the public `sgp4` crate 2.4.0. macOS arm64, absolute component error:

- 33 cases, 666 states compared, 2 expected errors, 0 left unpropagated
- max position `2.01884176931343973e-7` km
- max velocity `5.21046317203399667e-10` km/s
- both under 1e-6 km and 1e-9 km/s
- worst position: t = 1844335 min, component 1, line starting `1 20413U 83020D   05`

Recorded in `results/vallado.json`. Linux did not run this check.

## python-sgp4 2.25

C++ extension, `Satrec.twoline2rv` then `sgp4_tsince`. Euclidean distance. The recorded `operationmode` on these objects is `i`.

| grid | finite states | max position (km) | max velocity (km/s) |
| --- | ---: | ---: | ---: |
| keplemon | 45 | `1.28293770415279431e-10` | `7.82885949990241418e-14` |
| jax100 | 2900 | `3.93281479172719306e-09` | `4.35656849785738474e-12` |
| starlink-382a419 | 270810 | `2.15928358126833828e-08` | `1.41577741935982986e-10` |

keplemon worst position is NAVSTAR 62 at 1440 min. Worst velocity is CALSPHERE 1 at 1440 min. Times are 0, 10, 60, 360, and 1440 minutes. LES-5, GPS BIIR-2, NAVSTAR 62, GLONASS-M 736, and LAGEOS 1 are the deep-space objects in that file. The NAVSTAR gap is the maximum over all nine satellites. Per-state values: `results/keplemon-vs-python.json`.

The jax100 and full-file rows were measured on macOS against the same python-sgp4 build. Both payload hashes above were reproduced after the deep-space path existed, and neither grid reported a deep-space rejection. `scripts/compare_cpp.py` recomputes the keplemon and jax100 gaps from the dumps in this folder. The full starlink file is large; the same script accepts it.

## Check

From this directory:

```
python3 scripts/check_sha256.py
python3 scripts/compare_cpp.py grids/keplemon.tle grids/keplemon.minutes states/keplemon.bin
python3 scripts/compare_bins.py states/keplemon.bin ../sgp4-gpu-cpu/states/keplemon.bin
```

`compare_cpp.py` exits non-zero if a finite pair exceeds 1e-6 km or 1e-9 km/s. `SHA256SUMS` covers every file here except itself.
