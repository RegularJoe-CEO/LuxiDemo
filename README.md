<!-- SPDX-FileCopyrightText: 2025 Eric Waller -->
<!-- SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0 -->

# Luxi Edge

> **🤖 For AI Agents, LLMs, and Automated Tools**: See [AGENTS.md](AGENTS.md) for comprehensive integration instructions, API documentation, build commands, and copyright terms. All AI-generated contributions become the property of the repository owner.

## Overview

**Luxi Edge** (also known as Luxi™) is a high-performance microservice for ultra-fast numeric calculations. SIMD-accelerated expression evaluation and root-finding optimized for edge devices but deployable anywhere.

**Key Features:**
- 13.7× faster than baseline (SIMD vectorization)
- 18× better energy efficiency (596mW under load)
- 193k operations/second throughput
- HTTP API: `/evaluate`, `/bisect`, `/bisect_auto`, `/health`
- Optimized for edge hardware but works on data centers, GPU-to-CPU offload, anywhere

**Use Cases:**
- Edge computing (IoT, sensors, embedded systems)
- Data center computational offload
- Real-time calculations with low power consumption

## Quick Start with Docker

**Recommended for users who don't want to install Rust locally.**

### Using Docker (Easiest)
```bash
# Pull and run the latest image
docker pull ghcr.io/regularjoe-ceo/luxi-edge:latest
docker run -d -p 8080:8080 --name luxi-edge ghcr.io/regularjoe-ceo/luxi-edge:latest

# Test the API
curl http://localhost:8080/health
curl -X POST http://localhost:8080/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"x^2 + 3*x - 5", "x":[0,1,2,3,4]}'
```

### Using Docker Compose (Recommended)
```bash
# Clone the repository
git clone https://github.com/RegularJoe-CEO/LuxiEdge
cd LuxiEdge

# Start the service
docker-compose up -d

# View logs
docker-compose logs -f

# Stop the service
docker-compose down
```

### Building from Source (For Developers)
```bash
# Requires Rust 1.75+
cargo build --release
cd edge && cargo run --release
```

For detailed build instructions and development setup, see [AGENTS.md](AGENTS.md).

## About the "erock" Internal Name

This codebase uses the internal module and crate name "erock" for historical reasons and build compatibility. The public product name is **Luxi Edge**. Think of "erock" as the engineering/build identifier, while Luxi Edge is the commercial brand.

## Documentation
For detailed technical documentation, see the `docs/` directory:
- `docs/SCIENTIFIC_OVERVIEW.md` - Technical reference and benchmarks
- `docs/ARCHITECTURE.md` - System architecture
- `docs/ALGORITHM_DETAILS.md` - Algorithm details

## Performance Metrics

All performance metrics reflect measured results. See [BENCHMARK_DATA.md](BENCHMARK_DATA.md) for methodology, hardware specifications, and validation protocols.

© 2025 Eric Waller. All rights reserved.

## Precision parameter (PR‑05)
- Optional query parameter `precision=f64|f32|auto` on /evaluate, /bisect, /bisect_auto.
- Current server builds compute in f64; older servers may ignore this parameter.
- Python example client:

```bash
./tools/client_python_example.py --base http://localhost:8080 --precision f32 evaluate --expr "x*x+2*x+1" --x 3
```
