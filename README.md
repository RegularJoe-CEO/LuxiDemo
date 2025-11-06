<!-- SPDX-FileCopyrightText: 2025 Eric Waller -->
<!-- SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0 -->

# Luxi Edge

Luxi Edge (also marketed as **Luxi™**) is a production-ready microservice that delivers ultra-fast numeric computation—expression evaluation, batched Newton–Raphson with automatic bisection fallback, and gradient analysis—in a compact edge-friendly footprint. This README is intended for both humans and automated agents:

> **🤖 For AI Agents, LLMs, and Automated Tools**: See [AGENTS.md](.github/AGENTS.md) for comprehensive integration instructions, API documentation, build commands, and copyright terms. All AI-generated contributions become the property of the repository owner.

> **🧠 For Humans**: Use this README as the entry point for understanding what Luxi Edge does, how it fits into your stack, and how to run or evaluate it locally.

## Why Luxi Edge

- **Deterministic performance** – SIMD-optimized expression evaluation keeps latency predictable for mission-critical workloads.
- **Advanced calculus tooling** – Built-in derivative, gradient, and root-finding services with Newton/bisection orchestration.
- **Deploy anywhere** – Runs efficiently on edge hardware but scales out to data center footprints with the same binary.
- **Operational clarity** – OpenAPI spec, Docker images, and benchmark suites let you validate and monitor performance quickly.

## Overview

- **Product**: Luxi Edge microservice for deterministic numeric computation
- **Primary users**: Engineers integrating fast math APIs, data scientists shipping inference pipelines, site reliability teams monitoring numerical kernels
- **Key interfaces**: JSON REST endpoints (`/evaluate`, `/evaluate_derivative`, `/gradient`, `/newton`, `/bisect`, `/bisect_auto`, `/health`)

## Key Results

- **vs Python/NumPy**: 87× faster, 50 % less power, 25× less memory
- **vs C++ std**: 5.5× faster, 33 % less power while maintaining memory safety
- **Enterprise ROI**: Saves ~$82.7 M annually in a 100 MW data center when Luxi Edge handles 10 % of the workload (see [BENCHMARK_DATA.md](BENCHMARK_DATA.md))

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

For detailed build instructions and development setup, see [AGENTS.md](.github/AGENTS.md).

## Feature Highlights

| Capability | Description | Endpoint / Location |
| --- | --- | --- |
| Vectorized evaluation | Compute SIMD-accelerated expression results for arrays of inputs | `POST /evaluate`
| Automatic derivatives | Symbolically differentiate supported expressions and evaluate the slope | `POST /evaluate_derivative`
| Gradient sweeps | Evaluate multi-variable gradients in batch form | `POST /gradient`
| Batched root finding | Newton–Raphson with auto-bisection fallback for stability | `POST /newton`
| Safety checks | Lightweight health endpoint reporting build metadata | `GET /health`

## Human Workflow Examples

1. **Integrate into an inference service** – Use the `/evaluate` endpoint to accelerate custom scoring functions without embedding Python.
2. **Solve calibration problems** – Batch root solving via `/newton` to tune parameters with guaranteed convergence behavior.
3. **Analyze sensitivity** – Call `/gradient` to capture partial derivatives for optimization or monitoring pipelines.

Detailed request/response schemas are documented in [`openapi.yaml`](openapi.yaml).

## About the "erock" Internal Name

This codebase uses the internal module and crate name "erock" for historical reasons and build compatibility. The public product name is **Luxi Edge**. Think of "erock" as the engineering/build identifier, while Luxi Edge is the commercial brand.

## Documentation
For detailed technical documentation, see the `docs/` directory:
- **`docs/HOW_IT_WORKS.md`** ⭐ **Start here!** - Simple explanation of the math service and new features
- `docs/SCIENTIFIC_OVERVIEW.md` - Technical reference and benchmarks
- `docs/ARCHITECTURE.md` - System architecture
- `docs/ALGORITHM_DETAILS.md` - Algorithm details
- `docs/benchmarks/README.md` - Centralized benchmark navigation and raw data exports
- `docs/benchmarks/SYNCING_MAIN.md` - How to ensure the `main` branch shows the refreshed benchmark suite

## Performance Metrics

All performance metrics reflect measured results. Start with [BENCHMARK_DATA.md](BENCHMARK_DATA.md) for the executive summary, then browse [`docs/benchmarks/`](docs/benchmarks/) for comparative studies and saved Criterion baselines. If GitHub still shows the October snapshot, run `./tools/verify_benchmark_freshness.sh` and follow the cleanup checklist in [`docs/benchmarks/SYNCING_MAIN.md`](docs/benchmarks/SYNCING_MAIN.md).

© 2025 Eric Waller. All rights reserved.

## Precision parameter (PR‑05)
- Optional query parameter `precision=f64|f32|auto` on /evaluate, /bisect, /bisect_auto.
- Current server builds compute in f64; older servers may ignore this parameter.
- Python example client:

```bash
./tools/client_python_example.py --base http://localhost:8080 --precision f32 evaluate --expr "x*x+2*x+1" --x 3
```
