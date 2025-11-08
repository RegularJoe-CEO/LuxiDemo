# Luxi Edge GPU Benchmark Results (L4/sm_89)

**GPU:** NVIDIA L4 (sm_89 architecture)  
**Historical Run:** 2025-11-07 (CuPy sin kernel baseline)  
**Production Validation:** 2025-11-08 (Luxi Edge HTTP API on RunPod)

## Latest Production Results (November 8, 2025)

**Deployment:** RunPod NVIDIA L4, Luxi Edge HTTP server on port 3000

| Metric | Value |
|--------|-------|
| **Throughput** | **72,727,273 ops/sec** (72.7M) |
| **Latency** | **55ms** for 4,000,000 elements |
| **Power** | **16.4W** (measured via NVML) |
| **Efficiency** | **4.44M ops/sec/W** |
| **Speedup vs CPU SIMD** | **377× faster** |

**Test Payload:** 4,000,000 f32 elements, expression: `sin(x)*cos(x)`, seed=42

See [GPU_L4_RESULTS.md](GPU_L4_RESULTS.md) for comprehensive production benchmark analysis, optimization roadmap, and path to 600M ops/J efficiency target.

---

## Historical Baseline (November 7, 2025)

**CuPy sin kernel benchmark (50M elements):**

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
