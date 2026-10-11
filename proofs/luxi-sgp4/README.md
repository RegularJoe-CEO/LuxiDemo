# Luxi SGP4 evidence

Measured 2026-10-10. This folder holds inputs, state dumps, hashes, and checkers. It does not contain propagator source, PTX, or kernel code.

WGS-72, AFSPC mode. Little-endian f64. Satellite-major, then time. Six fields: x, y, z in km, then vx, vy, vz in km/s. A rejected state is six NaNs. Lines that are blank, start with `#`, or do not start with `1` or `2` are skipped, so a 3LE name line is not a TLE.

## What was run

The CPU path covers near-earth and deep-space (Brouwer period above 225 minutes). The GPU build is near-earth only. Deep-space is not in that GPU build.

jax100 and the full starlink file were run on four platforms. The bytes matched.

| platform | how it was run |
| --- | --- |
| macOS arm64 | rustc 1.92.0 (ded5c06cf 2025-12-08), release |
| Linux arm64 | local container |
| Linux x86_64 | cross-compiled ELF, Debian qemu-user-static 7.2 inside a local arm64 `rust:1-bookworm` container. Not a native x86_64 machine. |
| NVIDIA H100 80GB HBM3 | driver 580.126.09. Near-earth GPU build. jax100 launched twice. starlink-382a419 launched once. |

The keplemon set and the Vallado SGP4-VER check were run on macOS arm64 only.

## Inputs

| file | what it is |
| --- | --- |
| `grids/jax100.tle` | 100 Starlink TLEs, `numpy` seed 11, from jaxsgp4 commit `382a41997c2ec35869da45b950506d88a38fbbc3` |
| `grids/jax100.minutes` | 0 through 14 days, half-day steps, 29 times |
| `grids/starlink-382a419.txt` | full starlink catalog from that same commit |
| `grids/keplemon.tle` | 9 satellites from the Citra parity set. The checksum digit was recomputed. Element columns were not changed. |
| `grids/keplemon.minutes` | 0, 10, 60, 360, 1440 minutes |

Deep-space objects in the keplemon file: LES-5, GPS BIIR-2, NAVSTAR 62, GLONASS-M 736, LAGEOS 1.

## Hashes

| grid | satellites | times | errors | bytes | sha256 |
| --- | ---: | ---: | ---: | ---: | --- |
| `states/jax100.bin` | 100 | 29 | 0 | 139200 | `26eafb2b244fc1d4e24a7f561378774cf934f3976364fd7cc402853975d7175e` |
| `states/starlink-382a419.bin` | 9341 | 29 | 79 | 13002672 | `42d0ca2ca598dae1553ecfae12ca8204b3dae5ceb67e9afa76e36dcb40a75c09` |
| `states/keplemon.bin` | 9 | 5 | 0 | 2160 | `87b46f78c620a1672e8b93d65bcca713b0386be60ea4df6edde25c4745be48a2` |

H100, same bytes as macOS arm64, Linux arm64, and Linux x86_64:

| launch | errors | bytes | sha256 | match |
| --- | ---: | ---: | --- | --- |
| jax100 run 1 | 0 | 139200 | `26eafb2b244fc1d4e24a7f561378774cf934f3976364fd7cc402853975d7175e` | yes |
| jax100 run 2 | 0 | 139200 | `26eafb2b244fc1d4e24a7f561378774cf934f3976364fd7cc402853975d7175e` | yes |
| starlink-382a419 | 79 | 13002672 | `42d0ca2ca598dae1553ecfae12ca8204b3dae5ceb67e9afa76e36dcb40a75c09` | yes |

The 79 starlink errors are rejected states, stored as NaN, on every platform that ran that file. Record: `results/h100.json`.

`states/keplemon.bin` is byte-identical to `proofs/sgp4-gpu-cpu/states/keplemon.bin` on the earlier evidence branch (`0b8dfc44d24936d74b7ba8dec4dfab82cd37e120`). That older file came from a different propagator and was measured on macOS, Linux x86_64, and an H100. This folder's keplemon dump was produced on macOS arm64 only. This engine was not run on Linux or on a GPU for that grid.

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

keplemon worst position is NAVSTAR 62 at 1440 min. Worst velocity is CALSPHERE 1 at 1440 min. The NAVSTAR gap is the maximum over all nine satellites, so it is the deep-space maximum as well. Per-state keplemon values: `results/keplemon-vs-python.json`.

The jax100 and full-file rows were measured on macOS against the same python-sgp4 build. Both large-grid hashes above were reproduced on macOS after the deep-space path existed, and neither grid reported a deep-space rejection.

## Stock jaxsgp4, same code, GPU versus CPU

This section is stock jaxsgp4, not the Luxi dumps above. Repository https://github.com/cmpriestley/jaxsgp4, commit `382a41997c2ec35869da45b950506d88a38fbbc3`, JAX 0.9.2, on the same H100 versus `JAX_PLATFORMS=cpu`. Their published figure near 1e-9 km is jaxsgp4 versus C++, and the Zenodo note says that comparison ran on CPU. The numbers here are GPU versus CPU of the same JAX code.

Float64, 100 satellites, 29 times, 2900 states, per-satellite batch:

- 2701/2900 states differ
- max position `1.5853778190904915e-09` km
- max velocity `1.755852416785086e-12` km/s
- three GPU runs, identical
- CPU sha256 `c87e7e5b162cc143b75e8cea0da237f27a640b79a9a4ba00fe76705c13b1778c`
- GPU sha256 `e299e63536376194c7d1dd03ef1ed51563f42fa51cb6899698e0376a13fdb094`

Float64, full catalog, 9341 satellites, 29 times, 270889 states, one GPU run:

- 253026/270889 states differ
- max position `7.76981796099376e-05` km (7.77 cm), NORAD 57440, day 14
- max velocity `2.2696791545603468e-11` km/s
- CPU sha256 `0ccd0ef2dcc6844b75fdf2b7dff63a03432a3d17e1a88b202d163b881a92287d`
- GPU sha256 `e8e03f8597e207ae18c5020cc8aa870ce9ebaa91c83c3737744d6324364db366`

Full record: `results/jaxsgp4-gpu-vs-cpu-f64.json`. The 100-satellite float32 row from the same session is on branch `proofs/sgp4-gpu-cpu` at `0b8dfc44d24936d74b7ba8dec4dfab82cd37e120`.

## How to verify

From this directory:

```
python3 scripts/check_sha256.py
python3 scripts/compare_cpp.py grids/keplemon.tle grids/keplemon.minutes states/keplemon.bin
python3 scripts/compare_cpp.py grids/jax100.tle grids/jax100.minutes states/jax100.bin
python3 scripts/compare_bins.py states/keplemon.bin ../sgp4-gpu-cpu/states/keplemon.bin
shasum -a 256 states/jax100.bin states/starlink-382a419.bin states/keplemon.bin
```

`compare_cpp.py` needs python-sgp4 2.25 and exits non-zero if a finite pair exceeds 1e-6 km or 1e-9 km/s. The full starlink file is large. The same script accepts it. `compare_bins.py` exits non-zero on any bit mismatch. `SHA256SUMS` covers every file here except itself.

## Limits

- The GPU build is near-earth only. The H100 hashes cover jax100 and starlink-382a419. They do not cover deep-space.
- Deep-space was measured on macOS arm64 only. That is the keplemon set and the Vallado cases with period above 225 minutes. Linux did not run Vallado. Linux did not run keplemon.
- Linux x86_64 was qemu-user. It was not a native x86_64 host, and an amd64 container image was not used.
- The keplemon bytes match an older evidence file. That match does not mean this engine was run on a GPU for those nine satellites.
