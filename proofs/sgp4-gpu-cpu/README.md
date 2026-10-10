# SGP4 GPU versus CPU

Measured 2026-10-10 on one NVIDIA H100 80GB HBM3 (driver 580.126.09) and on two CPUs. This folder is the measurements, the inputs, and the hashes. It does not contain propagator source.

## Stock tools

### Citra keplemon

- Repository: https://github.com/citra-space/keplemon
- Commit: `14114699ef60b8c169c55d1b9d75645c8bb37f57` (2026-03-20, "added ephemeris tle fit")
- Test: `tests/test_gpu_cpu_parity.rs`, function `test_gpu_cpu_parity_all_regimes`
- Nine satellites: ISS (ZARYA), STARLINK-1007, NOAA 18, CALSPHERE 1, LES-5, GPS BIIR-2 (PRN 13), NAVSTAR 62, GLONASS-M 736, LAGEOS 1
- Times from epoch: 0, 10 min, 1 h, 6 h, 24 h (45 states)
- Max position difference: 1.649 m, ISS (ZARYA) at 24 h
- Max velocity difference: 6.431 mm/s
- Distinct bits: 45/45 position, 45/45 velocity, 45/45 states nonzero
- Failures: 0. Their tolerance is 2 m and 15 mm/s. The test passed.
- Timing on this run: CPU 0.15 ms, GPU 0.05 ms
- ISS at 24 h, as printed by that test:
  - CPU: (1679.170301, -4777.212550, -4542.414712) km
  - GPU: (1679.171092, -4777.213697, -4542.415593) km
- The same file's `test_dual_mode_equivalence_leo_geo` also passed. That test compares two host APIs. The 1.649 m figure is from `test_gpu_cpu_parity_all_regimes`.

### Cambridge jaxsgp4

- Repository: https://github.com/cmpriestley/jaxsgp4
- Commit: `382a41997c2ec35869da45b950506d88a38fbbc3` (2026-03-24, "bump version"), the commit pinned by Zenodo [10.5281/zenodo.19322209](https://doi.org/10.5281/zenodo.19322209) for arXiv 2603.27830. Later GitHub HEAD is a different tree.
- JAX 0.9.2 on CUDA, python-sgp4 2.25, numpy 2.0.2
- Grid from their plot script: `np.random.seed(11)`, 100 Starlink TLEs from package `data/starlink.txt` truncated to 69 columns, `tsince = arange(0, 14.5*1440, 0.5*1440)` (29 times, 0 through 14 days). 2900 states. The C++ side uses the same half-day steps from each satellite epoch.
- GPU versus the C++ extension, this machine:

| dtype | max position (km) | max velocity (km/s) | bit mismatches | C++ errors |
|---|---:|---:|---:|---:|
| float32 | 3.5437082454217794 | 0.0038825369062586833 | 2900/2900 | 0 |
| float64 | 7.661152593111065e-09 | 8.330638016698895e-12 | 2890/2900 | 0 |

Float32 max position by half-day grows from 0.0090 km at epoch to 2.965 km at day 14. The largest float32 gap in the grid is 3.544 km at day 13.5. Full tables are in `stock/jax-f32.json` and `stock/jax-f64.json`.

## Luxi f64 states

WGS-72, AFSPC mode. Little-endian f64. Satellite-major, then time. Six fields: x, y, z in km, then vx, vy, vz in km/s.

One dump was produced on each of:

- macOS arm64, rustc 1.92.0 (ded5c06cf 2025-12-08)
- Linux x86_64, rustc 1.99.0 (b940084d7 2026-09-28)
- the H100, PTX targeting sm_70 from rustc 1.101.0-nightly (32dba69d6 2026-10-09), launched through the CUDA driver API

The three dumps for a grid are the same bytes.

| grid | states | errors | sha256 | cross-device bit mismatches |
|---|---:|---:|---|---:|
| `grids/jax100` | 2900 | 0 | `aaabbfe5a7d4ed5c759006f6f280bc9dcce2d313d981331ef9eeef083ace90e4` | 0 |
| `grids/keplemon` | 45 | 0 | `87b46f78c620a1672e8b93d65bcca713b0386be60ea4df6edde25c4745be48a2` | 0 |

`states/jax100.bin` and `states/keplemon.bin` are those dumps.

Vallado vectors shipped with the public `sgp4` crate 2.4.0: 33 cases, 666 successful states, 2 expected error states. Max absolute position error 2.01884176931343973e-7 km. Max absolute velocity error 5.21046317203399667e-10 km/s. Both sit under that crate's tolerances (1e-6 km and 1e-9 km/s). macOS arm64 and Linux x86_64 printed the same digits, including the worst case (`t=1844335`, component 1, line starting `1 20413U 83020D   05`).

Against python-sgp4 2.25 on the same grids:

| grid | max position (km) | max velocity (km/s) | states with any bit difference |
|---|---:|---:|---:|
| jax100 | 3.9351208275573275e-09 | 4.313937740979949e-12 | 2876/2900 |
| keplemon | 1.1877875453002714e-10 | 7.828859499902414e-14 | 43/45 |

The C++ jax100 dump sha256 is `4439b27bc286969b724b21f1f7fd063beb88b12ac3f0d59a3af9c14b7eb2e852`, the same file hash as the C++ side of the stock jaxsgp4 run. Zero non-finite states on either Luxi grid.

The keplemon element columns are the ones in Citra's test. Several of those published lines have a last digit that is not the standard TLE checksum. `grids/keplemon.tle` recomputes only that digit. Citra's test binary uses the original strings; their parser accepts them. On the corrected lines, the Luxi ISS position at 24 h is (1679.17030067, -4777.21254970, -4542.41471214) km, which prints as their CPU vector at the 0.001 m they showed.

## Check

`scripts/compare_bins.py` reports bit mismatches and the max position and velocity gaps between two dumps. `scripts/cpp_ref.py` rebuilds a python-sgp4 2.25 dump in this layout. `scripts/jax_vs_cpp.py` replays the stock jaxsgp4 grid. `SHA256SUMS` covers every file in this folder except itself.
