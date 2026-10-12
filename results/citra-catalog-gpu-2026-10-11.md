# OrbVeil catalog on one H100 — GPU determinism, speed, energy (2026-10-11)

**Hardware:** NVIDIA H100 80GB HBM3, driver 580.126.09, CUDA 12.4, Ubuntu 22.04.5.
**Workload:** CelesTrak active catalog, 15,940 objects × 1,441 epochs = 22,969,540 propagations (1,102,537,920-byte output).

## Determinism
| engine | GPU vs own CPU | errors | runs |
|---|---|---|---|
| Luxi SGP4 (f64) | **identical** — SHA-256 `a14f625475a05892277a1a198aacc79aaf18b0192c97326bbe50fcafd31f8fa3` | 0 | 6 timed launches, all same hash; 5-run set `runs_match yes` |
| keplemon `67d6497` | **differs** — 32-object slice: 46,112 / 46,112 states differ, max 2.042 m, max 5.655e-6 km/s | 315 failed states per full-catalog pass | GPU self-consistent across 5 runs |

keplemon's own parity test (9 sats × 5 times) passes its 2 m tolerance with a max of 1.649 m (ISS, 24 h).

Gates (Luxi GPU vs published jaxsgp4 CPU hashes): jax100 `26eafb2b…d7175e` (0 errors) and starlink-382a419 `42d0ca2c…a75c09` (79 errors, identical flags) — both match.

PTX SHA-256: `8327c37ada8dab174d75d9e810be18fba8d83a2324a060cb82c46f201005770c`.

## Speed
| engine | clock | mean | rate |
|---|---|---|---|
| Luxi | device event, launch→sync | 74.696 ms (75.1521, 74.6383, 74.1159, 74.5979, 75.1564, 74.5132) | 3.075×10^8 prop/s |
| keplemon | host window incl. copies + host scatter | 1,509.635 ms | ≈1.522×10^7 states/s |

Different clocks; not a like-for-like ratio.

## Energy (nvidia-smi board power, ~100 ms sampling, 4,160 samples)
- keplemon, six catalog windows: **1,096.857 J** total (186.606 / 182.735 / 178.386 / 184.960 / 182.826 / 181.343 J), 119–122 W.
- Luxi: **no joule total** — ~75 ms kernels are shorter than the sampling interval; 4 of 6 windows had no sample. (Two windows held one sample each: 9.115 J, 9.313 J — single readings, not a total.)
- Idle ≈72.8 W; file peak 126.08 W (between kernels).
