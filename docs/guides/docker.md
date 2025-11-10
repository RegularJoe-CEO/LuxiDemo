<!-- SPDX-FileCopyrightText: 2025 Eric Waller -->
<!-- SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0 -->

# Luxi Edge — Docker Quick Start

## Pull and Run

**CPU-Optimized (Default):**
```bash
# Pull the latest image
docker pull ghcr.io/regularjoe-ceo/luxi-edge:latest

# Run the container
docker run -d -p 8080:8080 --name luxi-edge ghcr.io/regularjoe-ceo/luxi-edge:latest

# Or run with automatic restart
docker run -d -p 8080:8080 --name luxi-edge --restart unless-stopped \
  ghcr.io/regularjoe-ceo/luxi-edge:latest
```

**GPU-Accelerated (NVIDIA L4 Validated - November 8, 2025):**
```bash
# Run with GPU support (requires NVIDIA Container Toolkit)
docker run -d -p 3000:3000 --gpus all --name luxi-edge-gpu \
  ghcr.io/regularjoe-ceo/luxi-edge:gpu-latest

# Verified on: RunPod NVIDIA L4, AWS/GCP GPU instances
# Performance: 72.7M ops/sec @ 16.4W power
# Compatibility: CUDA 11.x/12.x, sm_89+ (L4, H100, H200)
# See docs/benchmarks/GPU_L4_RESULTS.md for details
```

**ARM64 Edge Deployment (Validated - November 10, 2025):**
```bash
# Run on ARM64 platforms (Apple Silicon, AWS Graviton, Jetson, Raspberry Pi 5)
docker run -d -p 8080:8080 --name luxi-edge-arm \
  ghcr.io/regularjoe-ceo/luxi-edge:latest-arm64

# Verified on: Apple M1/M2, AWS Graviton3, Jetson Orin Nano, Raspberry Pi 5
# Performance: 1.2-2.7B ops/sec (NEON SIMD)
# Energy: 400M-2.67B ops/J (ultra-efficient for battery/edge)
# See docs/ARM64_TESTING_GUIDE.md for platform-specific optimization
```

## Verify It's Running

```bash
# Health check
curl -s http://127.0.0.1:8080/health | python3 -m json.tool

# Evaluate an expression
curl -s http://127.0.0.1:8080/evaluate \
  -H 'Content-Type: application/json' \
  -d '{"expr":"x*x + 2*x + 1","x":[0,1,2,3]}' | python3 -m json.tool
```

## Using Docker Compose

Create `docker-compose.yml`:
```yaml
version: '3.8'
services:
  luxi-edge:
    image: ghcr.io/regularjoe-ceo/luxi-edge:latest
    ports:
      - "8080:8080"
    restart: unless-stopped
```

Then run:
```bash
docker-compose up -d
```

## Container Details

- **Image**: `ghcr.io/regularjoe-ceo/luxi-edge:latest`
- **Port**: 8080 (CPU), 3000 (GPU)
- **Precision**: default f64 (double precision), optional f32 for GPU
- **Size**: ~50MB CPU image, ~200MB GPU image (includes CUDA runtime)
- **Platforms**: linux/amd64 (x86_64), linux/arm64 (ARM64/Neon)
- **SIMD Support**: Auto-detects AVX-512, AVX2 (x86_64) or NEON (ARM64)

## Environment Variables

- **LUXI_PORT**: Server port (default: 8080 for CPU, 3000 for GPU)
- **LUXI_LOG_LEVEL**: Log verbosity (default: info, options: debug, info, warn, error)
- **CUDARC_CUDA_VERSION**: CUDA version for GPU build (e.g., 12010 for CUDA 12.1)
- **CUDA_VISIBLE_DEVICES**: Limit GPU visibility (e.g., "0" for first GPU only)
- **LUXI_BATTERY_MONITOR**: Battery monitoring path for ARM edge devices (e.g., /sys/class/power_supply/battery)

## Troubleshooting

**Port already in use:**
```bash
# Use a different port
docker run -p 8081:8080 ghcr.io/regularjoe-ceo/luxi-edge:latest
```

**View logs:**
```bash
docker logs luxi-edge
docker logs -f luxi-edge  # Follow logs
```

**Stop and remove:**
```bash
docker stop luxi-edge
docker rm luxi-edge
```

## Building Locally

If you want to build your own image:
```bash
git clone https://github.com/RegularJoe-CEO/LuxiEdge
cd LuxiEdge

# Build CPU image (default)
docker build -t luxi-edge:local .
docker run -p 8080:8080 luxi-edge:local

# Build GPU image (requires NVIDIA base image)
docker build -f Dockerfile.gpu -t luxi-edge:local-gpu .
docker run --gpus all -p 3000:3000 luxi-edge:local-gpu

# Build for ARM64 (cross-compilation)
docker buildx build --platform linux/arm64 -t luxi-edge:local-arm64 .
```

## Advanced: RunPod Cloud GPU Deployment

Deploy to RunPod for instant GPU access:
```bash
# See RUNPOD_INSTRUCTIONS.txt for complete guide
# Quick start:
runpod deploy \
  --gpu-type "NVIDIA L4" \
  --image ghcr.io/regularjoe-ceo/luxi-edge:gpu-latest \
  --port 3000

# Benchmark on deployed instance
python3 gpu_bench.py --host <runpod-instance-ip> --port 3000
```

See [docs/benchmarks/GPU_L4_RESULTS.md](../benchmarks/GPU_L4_RESULTS.md) for RunPod benchmark methodology.

## Platform-Specific Guides

- **ARM64 Deployment**: [docs/ARM64_TESTING_GUIDE.md](../ARM64_TESTING_GUIDE.md)
- **GPU Optimization**: [docs/benchmarks/GPU_L4_RESULTS.md](../benchmarks/GPU_L4_RESULTS.md)
- **Energy Efficiency**: [docs/NEON_ENERGY_PROBABILISTIC_TOF_QUICKSTART.md](../NEON_ENERGY_PROBABILISTIC_TOF_QUICKSTART.md)
- **Space Applications**: [docs/RAD_HARD_SPACE_APPLICATIONS.md](../RAD_HARD_SPACE_APPLICATIONS.md)
