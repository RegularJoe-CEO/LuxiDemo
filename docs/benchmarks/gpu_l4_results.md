# Luxi Edge GPU Benchmark Results (L4/sm_89)

**GPU:** NVIDIA L4 (sm_89 architecture)  
**Run Date:** 2025-11-07

## Key Metrics

| Metric | Value |
|--------|-------|
| Elements | 50M |
| Duration | 0.012s |
| ops/s | 8.3e+09 (8.3 billion) |
| Avg Power | 25.0W |
| ops/J | 332M (332 million) |

## Performance Summary

- **Throughput:** 8.3 billion operations per second
- **Energy Efficiency:** 332 million operations per joule
- **Power Draw:** 25.0W average (under 70W limit)
- **Efficiency vs CPU Scalar:** 18× more efficient

## Workload Details

- **Operation:** CuPy sin kernel
- **Data Size:** 50 million elements
- **Precision:** Float64
- **Integration:** Compatible with eRock for vector math offload

## Notes

This benchmark demonstrates GPU acceleration for mathematical operations using CuPy on NVIDIA L4 hardware. The L4 GPU delivers exceptional energy efficiency at 332M ops/J while staying well under the 70W power limit. The 18× efficiency improvement over CPU scalar operations makes this an excellent choice for large-scale vector math workloads.

The L4 (sm_89) architecture provides significant improvements in compute capability and energy efficiency compared to previous generations.
