# OrbVeil catalog proof

Measured 2026-10-11. This folder holds the catalog, the recorded hashes, the speed and energy numbers, and two checkers. It does not contain propagator source.

## Workload

CelesTrak active catalog in `data/active.tle`. 15,940 objects, 3LE. File SHA-256 `ebe2271f339f4af42be8ab8b196f6c7084211122205ac7663e17e9fd1b41d0ee`. 2,677,920 bytes.

Grid: t0 2026-10-11T12:00:00Z, 60 s steps, 24 h simulated, inclusive, 1,441 epochs. 22,969,540 propagations.

Dump layout for every hash below: little-endian f64, satellite-major, then time, six fields x, y, z in km and vx, vy, vz in km/s. 1,102,537,920 bytes. The full dumps are about 1.0 GiB each and are not in this git tree.

Stock is python-sgp4 2.27 with the C++ extension (accelerated True), WGS72, opsmode `i`. That is the mode OrbVeil calls through `Satrec.twoline2rv`. Luxi rows are WGS-72, AFSPC mode.

## Hardware

Apple M1 Pro, 6 performance cores and 2 efficiency cores, 16 GB, macOS 26.6.2 arm64. This machine is not a Jetson.

Linux arm64 is a native linux/arm64 container on that same Mac. It is not qemu.

Linux x86_64 is qemu-x86_64-static 7.2.22 inside a linux/arm64 container on that same Mac, using that container's amd64 C library. Hash only. It is not a native x86_64 host. Docker `--platform linux/amd64` on this Mac reports CPU vendor VirtualApple (Rosetta). That path was not used for these hashes.

A count of fused multiply-add instructions in the disassembly was 0 on the macOS arm64 binary, the Linux arm64 binary, and the Linux x86_64 binary.

## Speed and energy

Speed is the median of 5 runs after 1 warmup. The timed window is propagation only.

Luxi, macOS arm64, 2026-10-11 13:43 to 13:44 CDT. One-minute load was 5.04 at 1 core and 7.26 at 8 cores. 8 cores means 8 workers splitting the satellite list. Wrapping checksum `56396a8fe69d1c87` at both widths.

Stock, macOS arm64, the same afternoon. 1 core is one process. 8 cores is 8 processes, one `SatrecArray` each, opsmode `i`. The 8-process dump hashed to the same digest as the 1-process dump. A host snapshot at 13:03 CDT, before these stock timings, showed load averages 5.56, 7.67, 8.76.

CPU joules are a later session, 2026-10-11 14:54 CDT, median of 3 windows after 1 warmup, on the same sped-up macOS binary. The catalog SHA-256 was unchanged. `powermetrics` samplers `cpu_power`, `gpu_power`, `ane_power`, interval 100 ms. The integral is whole-machine power over each propagation window. Apple's powermetrics manual says those subsystem power figures are estimated. This is not an energy-impact score and not a TDP estimate. Package joules were not reported.

The joule session is not the median-of-5 session. Its own median propagate times are in the second table. Records: `results/table.txt`.

| engine | cores | median s | propagations/s | CPU joules |
| --- | ---: | ---: | ---: | ---: |
| python-sgp4 2.27 | 1 | 5.332472 | 4,307,484 | 22.923342 |
| python-sgp4 2.27 | 8 | 1.034126 | 22,211,557 | 16.263594 |
| Luxi | 1 | 5.333366 | 4,306,762 | 20.677514 |
| Luxi | 8 | 1.054219 | 21,788,210 | 16.226887 |

Joule-session propagate times, median of the 3 windows that produced the joules. Combined joules add the `gpu_power` and `ane_power` samples to `cpu_power` over the same window.

| engine | cores | median s | CPU joules | combined joules |
| --- | ---: | ---: | ---: | ---: |
| python-sgp4 2.27 | 1 | 5.451755 | 22.923342 | 23.131602 |
| python-sgp4 2.27 | 8 | 0.916530 | 16.263594 | 16.267631 |
| Luxi | 1 | 5.227693 | 20.677514 | 21.634734 |
| Luxi | 8 | 0.929695 | 16.226887 | 16.297442 |

Linux arm64 and Linux x86_64 were hash runs. No speed and no joules on those two.

## Hashes

Luxi SHA-256 `a14f625475a05892277a1a198aacc79aaf18b0192c97326bbe50fcafd31f8fa3` on all three platforms. 0 errors. 22,969,540 propagations. 1,102,537,920 bytes.

| platform | threads | sha256 |
| --- | ---: | --- |
| macOS arm64 | 1 and 8 | `a14f625475a05892277a1a198aacc79aaf18b0192c97326bbe50fcafd31f8fa3` |
| Linux arm64, Docker, native | 1 | `a14f625475a05892277a1a198aacc79aaf18b0192c97326bbe50fcafd31f8fa3` |
| Linux x86_64, qemu-user | 1 | `a14f625475a05892277a1a198aacc79aaf18b0192c97326bbe50fcafd31f8fa3` |

Stock python-sgp4 2.27, opsmode `i`, 0 errors. Each platform repeated and matched itself. The three platform digests differ.

| platform | repeated runs | sha256 |
| --- | --- | --- |
| macOS arm64 | warmup + 5, all equal. 8 processes also equal | `34dd469efae3caa123526120e70c880df46afc4569e2c8204298c93952e0f2bf` |
| Linux arm64, Docker, native | warmup + 5, all equal | `b3fab0207b9a0e15d72235a07a07f6cbfd06460adcdd129c73aa3681905effb5` |
| Linux x86_64, qemu-user | warmup + 2, all equal | `77f8a382db9729a9b64024bdd949fd83d6c3f858c12cf8741f3d596b718e312e` |

| pair | states that differ | positions that differ | max km | where |
| --- | ---: | ---: | ---: | --- |
| macOS vs Linux arm64 | 6,436,873 / 22,969,540 | 5,504,153 | 7.77154707746917322e-10 | STARLINK-30145, NORAD 57496, sat 5838, time 448 |
| macOS vs qemu x86_64 | 18,668,301 / 22,969,540 | 15,975,195 | 4.22158880827356005e-08 | ZHONGXING-2D, NORAD 43920, sat 1214, time 425 |
| Linux arm64 vs qemu x86_64 | 17,249,284 / 22,969,540 | 14,128,709 | 4.22175690645485688e-08 | same ZHONGXING-2D state |

Every compared state was finite on both sides. 0 nonfinite mismatches. Max component gaps: 5.96173777012154460e-10 km (macOS vs Linux arm64) and 4.10745997214689851e-08 km (either arm64 platform vs qemu x86_64).

Luxi bytes match across the three platforms, so the cross-platform position gap for Luxi on this dump is 0.

Stock is deterministic from run to run on a platform. Stock is not deterministic across these three platforms.

## Position, Luxi versus stock on macOS

All 22,969,540 states were finite on both sides. 0 validity mismatches. Record: `results/position.txt`.

| reference | max km | where |
| --- | ---: | --- |
| python-sgp4 2.27 opsmode `i` | 6.62406383491043714e-03 | O3B FM14, NORAD 43233, sat 1051, time 0, period 287.93 min |
| opsmode `i`, near-earth only, period at or below 225 min, 15,146 objects | 7.79453790726550245e-09 | GLOBALSTAR M088, NORAD 37740, sat 384, time 426 |
| opsmode `i`, deep-space only, 794 objects | 6.62406383491043714e-03 | O3B FM14 |
| python-sgp4 opsmode `a` (AFSPC) | 1.79077728654311157e-04 | COSMOS 2510 (EKS 1), NORAD 41032, sat 780, time 1414, period 717.84 min |

6.624e-3 km is 6.624 m. It is one deep-space object. Luxi is AFSPC mode. OrbVeil calls opsmode `i`. On the 15,146 near-earth objects the gap stays under 7.8e-9 km. Against opsmode `a` on the full catalog the gap stays under 1.8e-4 km. The 6.6 m figure is that deep-space mode difference.

The stock cross-platform gap on this catalog is at most 4.22e-8 km. That is much smaller than the 6.624e-3 km mode gap.

## ISS duplicate events

OrbVeil `screen()` on this catalog, primary ISS (ZARYA) NORAD 25544, 7 days, 10 km, default 10-minute step. Measured on this Mac in 2.321841 s. Record: `results/iss.txt`.

- 6,054 events
- all 6,054 at 0.0 km
- 0 events at any other range
- six secondaries, 1,009 events each
- TCA spacing 600 s on each secondary

The six secondaries are 25575 ISS (UNITY), 26400 ISS (ZVEZDA), 26700 ISS (DESTINY), 36086 POISK, 49044 ISS (NAUKA), and 68837 PROGRESS-MS 34.

In `data/active.tle`, each of those six line pairs matches Zarya in epoch, ndot, nddot, BSTAR, inclination, right ascension, eccentricity, argument of perigee, mean anomaly, and mean motion. Catalog number, international designator, revolution count, and checksum differ. The element-set number field is 999 on all seven lines. `scripts/check_claims.py` checks those columns.

`screen()` merges a repeat only when the new TCA is within 300 s of one already stored. The coarse step is 600 s, and the measured TCA gap is 600 s, so each sample is stored. 6 times 1,009 is 6,054.

## How to verify

From this directory:

```
python3 scripts/check_sha256.py
python3 scripts/check_claims.py
```

`check_sha256.py` hashes every file named in `SHA256SUMS`. `check_claims.py` checks the catalog digest, the 15,940 count, the six ISS element columns, and that this README quotes the same digests and table numbers as `results/`.

## Limits

- This Mac is not a Jetson. Linux arm64 here is Docker on the M1 Pro.
- Linux x86_64 is qemu-user. No speed and no joules for that row.
- No joules were measured on Linux.
- CPU joules use Apple's estimated subsystem power for the whole machine. They are not a package-energy counter. The joule session and the median-of-5 session are different runs, and their propagate seconds differ.
- The median-of-5 Luxi runs were taken while the one-minute load was about 5 to 7.
- The 1,102,537,920-byte dumps are not in this repository.
- The 6.6 m gap is the deep-space AFSPC versus opsmode `i` difference on O3B FM14. It is not the near-earth gap, and it is not the stock cross-platform gap.
