# Citra and Luxi on one H100

Measured 2026-10-11. This folder is the recorded numbers and a checker. It has no propagator source, no kernel, and no build command.

## keplemon parity

keplemon `67d64970a9061caa3b337ad3c1a20262b0a5a25b`. The repo test `test_gpu_cpu_parity_all_regimes` passed on an NVIDIA H100 80GB HBM3, driver 580.126.09. Nine satellites, 45 states, 0 failures. The test printed max position error 1.649 m on ISS (ZARYA) at t=4, which is 24 h, and max velocity error 6.431 mm/s. Its own timers printed CPU 0.04 ms and GPU 0.06 ms. There is no state-dump hash.

Printed ISS position at that time, km:

- CPU: (1679.170301, -4777.212550, -4542.414712)
- GPU: (1679.171092, -4777.213697, -4542.415593)
- diff: (0.000791, -0.001148, -0.000881)

TDRS 3 is not in this test. The run did not split the 1.649 m into separate terms. Joules of the propagate window were not measured.

## Luxi relaunch

The same near-earth image recorded on 2026-10-10 was launched again. Times below are the wall time of the whole process after one warmup. They are not a kernel-only propagate window. Both hashes match the published CPU payloads in `proofs/luxi-sgp4`.

| grid | states | errors | median s | sha256 |
| --- | ---: | ---: | ---: | --- |
| jax100, 100 x 29 | 2900 | 0 | 0.451879 | `26eafb2b244fc1d4e24a7f561378774cf934f3976364fd7cc402853975d7175e` |
| starlink-382a419, 9341 x 29 | 270889 | 79 | 0.505075 | `42d0ca2ca598dae1553ecfae12ca8204b3dae5ceb67e9afa76e36dcb40a75c09` |

jax100 runs, seconds: 0.45936, 0.451879, 0.440776, 0.444795, 0.454266.

starlink runs, seconds: 0.506278, 0.517403, 0.504902, 0.505075, 0.495941.

The 79 starlink errors are rejected states on the published payload, on every platform that ran that file.

## Not run

The OrbVeil catalog (15,940 x 1,441) was not propagated on this GPU. The catalog hash `a14f625475a05892277a1a198aacc79aaf18b0192c97326bbe50fcafd31f8fa3` was not produced here.

Samples with 0% GPU utilization in the board-power log average 72.47 W (3,264 samples, mean 72.47390931372547 W). That is idle board power. It is not a propagate-window energy.

## Check

```
python3 scripts/check_claims.py
```
