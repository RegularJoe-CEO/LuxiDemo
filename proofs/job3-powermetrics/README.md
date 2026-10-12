# Job 3 CPU joules, Apple powermetrics estimates

Recorded 2026-10-11 on an Apple M1 Pro. The sampler is `powermetrics` with `cpu_power`, `gpu_power`, and `ane_power` at 100 ms. The integral is whole-machine power over each marked window. Apple's powermetrics manual says these subsystem power figures are estimated. This is not an energy-impact score and not a TDP estimate. Package joules were not reported (`package_j nan`).

The median below is `median_cpu_j` from `results/energy-job3.txt`. Combined joules add `gpu_power` and `ane_power` to `cpu_power` over the same window. This session is not the median-of-5 speed session. Its own propagate seconds are in that file. No output hash was taken for these windows.

| window | what ran | median CPU joules | median combined joules | complete runs |
| --- | --- | ---: | ---: | ---: |
| sdn_luxi_1 | first 2,000 catalog objects, 24 h grid, 1 thread | 2.961125 | 3.016960 | 3 |
| sdn_luxi_8 | same proxy, 8 threads | 2.351371 | 2.361786 | 3 |
| sdn_stock_1 | python-sgp4 on that proxy, 1 worker | 2.947740 | 3.005551 | 3 |
| sdn_stock_8 | python-sgp4 on that proxy, 8 workers | 2.282849 | 2.294403 | 3 |
| kep6_luxi_1 | six keplemon TLEs, 24 h grid, 1 thread | 0.004226 | 0.004600 | 3 |
| kep6_stock_1 | python-sgp4 on those six TLEs, 1 worker | 0.011669 | 0.011878 | 3 |
| pc_luxi_1 | 53 CARA CDMs, mode 64, 2,000 repeats, 1 thread | 1.095122 | 1.114808 | 5 |
| pc_luxi_8 | same Pc repeats, 8 threads | 0.836139 | 0.840565 | 5 |
| pc_stock_1 | stock Python Pc, 1 core, same repeats | 11.120009 | 11.286445 | 5 |

There is no 8-worker stock Pc row and no 8-worker stock row for the six TLEs in this file. Linux was not measured.

## Check

```
python3 scripts/check_claims.py
```
