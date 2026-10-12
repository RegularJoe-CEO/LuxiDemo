# SGP4 determinism across chips: SAAL, python-sgp4, Luxi

**Question:** given identical inputs, does each engine return bit-identical output on different CPUs?

## Setup
- Catalog: 15,940-object public GP set, sha256 `ebe2271f339f4af42be8ab8b196f6c7084211122205ac7663e17e9fd1b41d0ee` (the file is not re-hosted)
- Grid: t0 2026-10-11T12:00Z, 24 h, 60 s steps → 22,969,540 states per engine per platform
- Output: TEME x y z vx vy vz, little-endian float64, compared bit-for-bit (SHA-256) and by position difference
- Engines:
  - SAAL v9.6 Sgp4Prop as distributed via the open-source `saal` crate 1.3.3, black-box API calls
  - python-sgp4 2.27 SatrecArray, WGS72, opsmode 'i'
  - Luxi f64 SGP4, a closed binary in AFSPC/WGS72 mode
- Single thread. Every engine was repeatable run-to-run on the same platform.

## 1. Cross-chip identity
| Engine | Platforms compared | Bit-identical states | Max position gap |
|---|---|---|---|
| Luxi | Linux x86_64, Linux arm64, macOS arm64, NVIDIA H100 | **100%** (one hash, `a14f6254…f8fa3`) | 0 |
| SAAL (ds50 UTC input) | Linux x86_64 vs Linux arm64 | 28.15% | 7.06 µm |
| SAAL (ds50 UTC input) | macOS M1 build native vs Intel build under Rosetta 2 | 9.21% | 7.06 µm |
| SAAL (minutes-since-epoch input) | macOS native vs Rosetta 2 | 9.21% | 7.47 µm |
| python-sgp4 | Linux x86_64 vs Linux arm64 | 24.9% | 42 µm |

None of the four macOS SAAL hashes matches any Linux SAAL hash.

## 2. Accuracy vs SAAL
| Pair | Time mode | Max | p99 | Median |
|---|---|---|---|---|
| Luxi vs SAAL | minutes-since-epoch (like-for-like) | 0.48 mm | 1.3 µm | 0.08 µm |
| Luxi vs SAAL | ds50 UTC | 2.86 mm | 2.15 mm | 0.67 mm |
| SAAL ds50 vs SAAL mse (same binary) | — | 2.9 mm | — | 0.67 mm |
| python-sgp4 vs SAAL | minutes-since-epoch | 6.62 m (2 objects, opsmode 'i' vs AFSPC) | 1.2 µm | — |

## 3. Speed (single thread, median full pass of 22.97M states)
| Engine | Linux x86_64 | macOS arm64 |
|---|---|---|
| Luxi | 4.53 s (5.07 M/s) | 5.19 s (4.42 M/s) |
| SAAL ds50 UTC | 6.34 s (3.62 M/s) | 7.29 s (3.15 M/s) |
| SAAL minutes-since-epoch | 10.07 s (2.28 M/s) | 11.69 s (1.96 M/s) |
| SAAL under Rosetta 2 (ds50 / mse) | — | 18.11 s / 36.10 s |
| python-sgp4 | 8.98 s (2.56 M/s) | not run |

## 4. Energy (macOS arm64, package CPU via powermetrics at 500 ms; per single pass ≈ avg W × median s)
| Engine | Avg CPU W | ≈ J per pass |
|---|---|---|
| Luxi | 3.65 | ≈19 |
| SAAL ds50 UTC, native | 3.94 | ≈29 |
| SAAL mse, native | 4.28 | ≈50 |
| SAAL ds50 UTC, Rosetta 2 | 3.69 | ≈67 |
| SAAL mse, Rosetta 2 | 4.61 | ≈166 |

## Caveats
- The SAAL cross-chip gaps are micrometres. The finding is non-reproducibility, not inaccuracy.
- SAAL's arm64 and x86 runs are separately compiled binaries. On Linux their build dates also differ (Feb 10 vs Jun 11 2025); on macOS both builds are dated Jun 11 2025.
- Linux arm64 runs were under qemu emulation. Correctness holds, but those runs carry no speed results.
- Rosetta 2 is emulation, so its speed and energy reflect Rosetta, not SAAL.
- The timing harnesses differ: Luxi used its own timing mode, while the SAAL and python-sgp4 runs used a loop that writes to a buffer. Treat ratios as indicative. Rosetta runs are warmup + 1 run.
- Energy is whole-package CPU with no idle subtraction, and was measured on macOS only (no RAPL on the Linux VM).
- python-sgp4 was not run on macOS (version mismatch) and was run in opsmode 'i', which explains its two metre-level outliers.
- The Mac-vs-Linux SAAL comparison is by hash only.
- No affiliation with or endorsement by any government body is implied. SAAL is used as-is as a reference implementation.
