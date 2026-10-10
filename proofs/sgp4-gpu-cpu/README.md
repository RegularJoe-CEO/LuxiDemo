# SGP4 GPU versus CPU

Measured 2026-10-10 on one NVIDIA H100 80GB HBM3 (driver 580.126.09) and on two CPUs. This folder is the measurements, the inputs, and the hashes. It does not contain propagator source.

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

## jaxsgp4, same code, GPU versus CPU

Repository https://github.com/cmpriestley/jaxsgp4, commit `382a41997c2ec35869da45b950506d88a38fbbc3` (Zenodo [10.5281/zenodo.19322209](https://doi.org/10.5281/zenodo.19322209), arXiv 2603.27830). JAX 0.9.2. Their published ~1e-9 km figure is jaxsgp4 versus C++ and, per the Zenodo note, was run on CPU. This section is jaxsgp4 on the H100 versus jaxsgp4 on CPU (`JAX_PLATFORMS=cpu`), same 100 Starlink TLEs (`np.random.seed(11)`), same 29 times (0 through 14 days, half-day steps), 2900 states. Three GPU runs, one CPU run. Float32 values are stored widened to float64, which keeps every float32 value exact.

| dtype | bit mismatches | max position (km) | max velocity (km/s) | GPU runs agree |
|---|---:|---:|---:|---|
| float64 | 2701/2900 | 1.5853778190904915e-09 | 1.755852416785086e-12 | yes, 3/3 |
| float32 | 2824/2900 | 2.4645502245160555 | 0.002827067119259089 | yes, 3/3 |

Float64 median position gap 1.467e-12 km, 95th percentile 1.549e-9 km. Float32 median 0.001094 km, 95th percentile 0.839 km. Float32 max position is 0.00163 km at epoch and 1.690 km at day 14. The largest float32 GPU-versus-CPU gap on this grid is 2.465 km at day 11.5.

| dtype | sha256 CPU | sha256 GPU |
|---|---|---|
| float64 | `c87e7e5b162cc143b75e8cea0da237f27a640b79a9a4ba00fe76705c13b1778c` | `e299e63536376194c7d1dd03ef1ed51563f42fa51cb6899698e0376a13fdb094` |
| float32 | `1aa09ca150c44396bec7825d1d72d2333e58e2076d6be65fa947bf3655eeb836` | `b6403a5c3f369b27fdde436734303434589ecc053a16dbf11fc6e1e7e829d6af` |

The GPU hashes match the GPU side of the earlier GPU-versus-C++ dumps below, so those are the same GPU states. Full record: `stock/jax-gpu-vs-cpu.json`.

GPU versus the python-sgp4 2.25 C++ extension on this same grid:

| dtype | max position (km) | max velocity (km/s) | bit mismatches | C++ errors |
|---|---:|---:|---:|---:|
| float32 | 3.5437082454217794 | 0.0038825369062586833 | 2900/2900 | 0 |
| float64 | 7.661152593111065e-09 | 8.330638016698895e-12 | 2890/2900 | 0 |

Per-time tables: `stock/jax-f32.json`, `stock/jax-f64.json`.

## Citra keplemon, noted only

Their GPU-versus-CPU test at commit `14114699` reproduced a 1.649 m position gap on ISS at 24 h (45/45 states differ; max velocity 6.431 mm/s). Their test tolerance is 2 m, so the test passed. A CPU-only check of SAAL against python-sgp4 on those same elements stays under 1 mm, and is 0 at ISS t=24 h. The 1.6 m is in the CUDA kernel. Counts and the printed ISS vectors are in `stock/keplemon.json`.

## Check

`scripts/compare_bins.py` reports bit mismatches and the max position and velocity gaps between two dumps. `scripts/cpp_ref.py` rebuilds a python-sgp4 2.25 dump in this layout. `scripts/jax_vs_cpp.py` replays jaxsgp4 versus C++. `scripts/jax_gpu_vs_cpu.py` dumps jaxsgp4 on whatever device JAX selected. `SHA256SUMS` covers every file in this folder except itself.
