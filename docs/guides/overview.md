<!-- SPDX-FileCopyrightText: 2025 Eric Waller -->
<!-- SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0 -->

# Luxi Edge Overview

## Product

### **Luxi Edge**
High-performance microservice for ultra-fast numeric calculations. **Dual-platform deployment:** optimized for edge devices (CPU SIMD) and data centers (GPU acceleration).

---

## Deployment Options

### **Edge/IoT Deployment (CPU SIMD)**
Optimized for resource-constrained devices and real-time applications.

**Hardware:**
- ARM64/x86_64 processors with NEON/AVX2 vector extensions
- 512 MB RAM minimum
- 596mW power consumption under load

**Performance:**
- **193k ops/sec** throughput
- **<1ms latency** for small batches
- **13.7× faster** than scalar baseline
- **18× better** energy efficiency

**Ideal for:**
- IoT sensors and edge devices
- Battery-powered systems
- Real-time control loops
- Cost-sensitive deployments
- Latency-critical applications

---

### **Data Center Deployment (GPU Acceleration)**
Validated on NVIDIA L4 GPU (November 8, 2025) for high-throughput workloads.

**Hardware:**
- NVIDIA GPUs with CUDA compute capability 8.9+ (L4, A100, H100)
- PCIe 3.0/4.0 x16 interface
- 4 GB GPU memory minimum (24 GB recommended)

**Performance:**
- **72.7M ops/sec** (72,727,273) — **377× faster than CPU SIMD**
- **55ms latency** for 4M element batch
- **16.4W power** measured via NVML
- **4.44M ops/sec/W** energy efficiency

**Ideal for:**
- Large-scale batch analytics
- High-throughput data pipelines
- Scientific simulations (>100k elements)
- Cloud-native applications (AWS, GCP, RunPod)
- Maximum performance requirements

---

## When to Use GPU vs CPU

| Factor | Use CPU SIMD | Use GPU Acceleration |
|--------|--------------|----------------------|
| **Batch Size** | <10k elements | >10k elements |
| **Latency** | <10ms required | 50ms+ acceptable |
| **Power Budget** | <1W available | 10-50W available |
| **Deployment** | Edge, IoT, embedded | Data center, cloud |
| **Cost** | Minimal infrastructure | GPU instance costs |
| **Throughput** | <200k ops/sec | >10M ops/sec target |

---

## Technical Summary

All performance metrics reflect measured results from production deployments:
- **CPU SIMD:** Validated on ARM64/x86_64 with NEON/AVX2 (October 2025)
- **GPU:** Validated on NVIDIA L4 via RunPod (November 8, 2025)

See [../benchmarks/BENCHMARK_DATA.md](../benchmarks/BENCHMARK_DATA.md) for methodology and [../benchmarks/GPU_L4_RESULTS.md](../benchmarks/GPU_L4_RESULTS.md) for comprehensive GPU analysis.

