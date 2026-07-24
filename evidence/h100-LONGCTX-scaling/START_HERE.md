# Long-context / scaling pack (H100 NVL host, 2026-07-11)

**Public:** https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/h100-LONGCTX-scaling

This pack measures **O(N) versus O(N²) memory behavior** at multi-k and 32k
context lengths.

## Headline

### Memory scaling (core claim)

| Model | Log-log slope | Interpretation |
|-------|--------------:|----------------|
| **Waller streaming state** | **~1.000** (analytical) / **~1.003** (measured 256-8192) | **O(N)** |
| **Dense N×N score matrix** | **~2.000** / **~2.002** | **O(N²)** |

| seq | Waller state MB | Dense scores MB | Reduction |
|----:|----------------:|----------------:|----------:|
| 1 024 | 0.52 | 8.4 | **16×** |
| 4 096 | 2.1 | 134 | **64×** |
| 8 192 | 4.2 | 537 | **128×** |
| 32 768 | 16.8 | 4 295 | **256.0×** |
| 131 072 | 67.1 | 68 719 | **1024.0×** (analytical; 131k *timing* skipped) |

NPOW witness (fast gate): mem slopes **1.000 / 2.000**, reduction **42.7×** @ 8192 anchor.  
WNSM bus recovery of NPOW: max_diff **4.77e-07** on this host. This result is
not bit-zero.

### Measured CPU long_context_bench (head_dim=64)

| seq | waller_ms | std_ms | waller_mem_MB | std_mem_MB | mem ratio |
|----:|----------:|-------:|--------------:|-----------:|----------:|
| 256 | 3.5 | 3.0 | 0.13 | 0.52 | **4.0×** |
| 1024 | 54.4 | 48.3 | 0.52 | 8.39 | **16.0×** |
| 4096 | 867.9 | 769.5 | 2.1 | 134.22 | **64.0×** |
| 8192 | 3463.2 | 3085.9 | 4.19 | 536.87 | **128.0×** |

**Note:** At short seq, waller *time* can lose to dense (O(N²) FLOPs still apply); the **memory** ratio is the scaling story.

### CUDA long-ctx @ 32k (H100, GPU under load)

| Metric | Value |
|--------|------:|
| Shape | seq=**32768**, h=4096, heads=32 |
| Path | waller (clustered fixture) |
| Median attn ms | **7097.7** |
| Edge deletion | **0.999711** |
| Contact bytes avoided (mesh probe) | **34,348,204,032** (~32 GiB class) |
| Board power median | **228.1 W** (not idle) |
| Power samples | 1816 |

## Exact commands

```bash
# Memory slopes + NPOW (fast; no 131k timing)
LUXI_NPOW_FAST=1 ./target/release/examples/npow_scaling_proof
./target/release/examples/long_context_bench

# CUDA long-ctx 32k under load
./target/release/examples/cuda_longctx_attn_bench
```

The full NPOW 131k wall-clock timing was not run. The analytical ladder
includes the 131k memory estimate.

## Files

- `LONGCTX_SCALING_SUMMARY.json`
- `mem_ladder.csv`  -  512 → 131072 memory table
- `long_context_bench.log`  -  measured 256-8192
- `npow_fast.log`  -  O(N)/O(N²) witness
- `cuda_longctx_32k.json` + `.log`  -  32k GPU run + pynvml

## Honest bounds

- Memory O(N) vs O(N²) is the primary long-ctx claim; **time** is not claimed O(N) for full-seq recompute.
- 131k **memory** reduction is analytical/extrapolated from the same formulas validated at 256-8192.
- CUDA 32k run is clustered long-ctx path with mesh contact-byte accounting  -  not a full HF 7B model.

Contact: Eric Waller · e@ewaller.com
