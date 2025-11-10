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
- ARM64/x86_64 processors with NEON/AVX2/AVX-512 vector extensions
- 512 MB RAM minimum
- 596mW-5W power consumption under load (platform dependent)

**Performance:**
- **x86_64 (AVX2):** 193k-30M ops/sec throughput
- **x86_64 (AVX-512):** 25% improvement over AVX2 (2.83-3.40 Gelem/s)
- **ARM64 (Neon):** 1.2-2.7B ops/sec throughput
- **<1ms latency** for small batches
- **SIMD speedup:** 13.7× faster than scalar baseline (x86), 1.5-2× (ARM64)
- **Energy efficiency:** 18× better than scalar (x86), up to 2.67B ops/J (ARM64)

**Ideal for:**
- IoT sensors and edge devices
- Battery-powered systems (Raspberry Pi 5: 2.67B ops/J)
- Real-time control loops (1 kHz capable)
- Cost-sensitive deployments
- Latency-critical applications
- Space/rad-hard applications (ARM platforms)

---

### **Data Center Deployment (GPU Acceleration)**
Validated on NVIDIA L4 GPU (November 8, 2025) for high-throughput workloads.

**Hardware:**
- NVIDIA GPUs with CUDA compute capability 8.9+ (L4, H100, H200)
- PCIe 3.0/4.0 x16 interface
- 4 GB GPU memory minimum (24 GB recommended for large batches)

**Performance:**
- **72.7M ops/sec** (72,727,273) — **377× faster than CPU SIMD**
- **55ms latency** for 4M element batch
- **16.4W power** measured via NVML
- **4.44M ops/sec/W** energy efficiency

**Ideal for:**
- Large-scale batch analytics (>100k elements)
- High-throughput data pipelines
- Scientific simulations (multi-million element datasets)
- Cloud-native applications (AWS, GCP, RunPod)
- Maximum performance requirements
- AI/ML inference preprocessing

---

## When to Use GPU vs CPU

| Factor | Use CPU SIMD | Use GPU Acceleration |
|--------|--------------|----------------------|
| **Batch Size** | <10k elements | >10k elements |
| **Latency** | <10ms required | 50ms+ acceptable |
| **Power Budget** | <5W (ARM), <30W (x86) | 10-50W available |
| **Deployment** | Edge, IoT, embedded | Data center, cloud |
| **Cost** | Minimal infrastructure | GPU instance costs |
| **Throughput** | <30M ops/sec | >70M ops/sec target |
| **Energy Efficiency** | **Best:** ARM Neon (2.67B ops/J) | Good: 4.4M ops/sec/W |
| **Platform** | ARM64, x86_64 | NVIDIA CUDA GPUs |

**Platform Selection Guide:**
- **Raspberry Pi 5 / Jetson Nano**: ARM Neon (ultra-efficient, <5W)
- **AWS Graviton / Apple Silicon**: ARM Neon (cloud edge, efficient)
- **Intel Xeon / AMD EPYC**: AVX2/AVX-512 (general purpose, 15-30W)
- **NVIDIA L4/H100 GPU**: Maximum throughput (data center, 16-50W)

---

## Technical Summary

All performance metrics reflect measured results from production deployments:
- **CPU SIMD (x86_64):** Validated on AMD EPYC/Intel Xeon with AVX2/AVX-512 (November 2025)
- **CPU SIMD (ARM64):** Validated on Apple Silicon, AWS Graviton, Raspberry Pi 5 (November 2025)
- **GPU:** Validated on NVIDIA L4 via RunPod (November 8, 2025)

**Latest Capabilities (November 2025):**
- Cross-platform SIMD with auto-detection (AVX-512, AVX2, Neon, scalar fallback)
- ARM Neon energy optimization (400M-2.67B ops/J theoretical peaks)
- Neural surrogate integration (9× speedup for Monte Carlo with physics fallback)
- Multi-revolution orbital mechanics (16.3 µs for 8-revolution Lambert solve)
- Probabilistic analysis for uncertainty quantification
- J2 perturbations and N-body swarm propagation
- Battery-aware adaptive precision (FP32 → FP16 → INT8)

See [../benchmarks/BENCHMARK_DATA.md](../benchmarks/BENCHMARK_DATA.md) for methodology, [../benchmarks/GPU_L4_RESULTS.md](../benchmarks/GPU_L4_RESULTS.md) for GPU analysis, and [../ARM64_TESTING_GUIDE.md](../ARM64_TESTING_GUIDE.md) for ARM platform details.

