<!-- SPDX-FileCopyrightText: 2025 Eric Waller -->
<!-- SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0 -->

# Luxi Edge Overview

## Product

### **Luxi Edge**
High-performance microservice for ultra-fast numeric calculations. One codebase optimized for edge but deployable anywhere.

**Key metrics (CPU SIMD):**
- 13.7× faster than baseline (SIMD vectorization)
- 18× better energy efficiency
- 193k ops/sec throughput
- 596mW under load (optimized for edge)

**Key metrics (GPU - NVIDIA L4, validated Nov 8, 2025):**
- **72.7M ops/sec** (72,727,273) — 377× faster than CPU SIMD
- **55ms latency** for 4M element evaluation
- **16.4W power** measured via NVML
- **4.44M ops/sec/W** energy efficiency

**Use cases:**
- Edge computing (IoT, sensors, embedded) — CPU optimized
- Data center computational offload — GPU accelerated
- High-throughput analytics — GPU accelerated
- Any platform needing rapid calculations

All performance metrics reflect measured results. See [../benchmarks/BENCHMARK_DATA.md](../benchmarks/BENCHMARK_DATA.md) for methodology and [../benchmarks/GPU_L4_RESULTS.md](../benchmarks/GPU_L4_RESULTS.md) for GPU analysis.

