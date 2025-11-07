<!-- SPDX-FileCopyrightText: 2025 Eric Waller -->
<!-- SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0 -->

# Luxi Edge

Luxi Edge is a production-ready microservice that delivers ultra-fast numeric computation—expression evaluation, batched Newton–Raphson with automatic bisection fallback, and gradient analysis…

## Quick Start

**For AI agents and automated tools**, bootstrap with:
```bash
bash scripts/agent_bootstrap.sh
```

**For humans**, see Docker instructions below or [build from source](#building-from-source).

> **🤖 AI Agents**: See [`docs/agents.md`](docs/agents.md) for comprehensive integration instructions, API documentation, and copyright terms.

> **🧠 Humans**: Use this README as the entry point for understanding Luxi Edge and getting started.

## Why Luxi Edge

- **Deterministic performance** – SIMD-optimized expression evaluation keeps latency predictable for mission-critical workloads.
- **Advanced calculus tooling** – Built-in derivative, gradient, and root-finding services with Newton/bisection orchestration.
- **Deploy anywhere** – Runs efficiently on edge hardware but scales out to data center footprints with the same binary.
- **Operational clarity** – OpenAPI spec, Docker images, and benchmark suites let you validate and monitor performance quickly.

## Key Results

Last updated: 2025-01-18

### CPU/SIMD Performance
- SIMD runtime speedup: **13.7× faster** (0.52 ms vs 7.10 ms for 100k ops)
- Throughput: **193k ops/s** vs 14k ops/s (**13.7× higher**)
- Energy per operation: **3.08 µJ** vs 55.6 µJ (**18× lower**)
- Load power draw: **596 mW** vs 783 mW (**24% drop**)

### GPU Acceleration (NVIDIA L4)
- Throughput: **8.3B ops/s** (50M elements in 0.012s)
- Energy efficiency: **332M ops/J** (18× better than CPU scalar)
- Power draw: **25.0W** average (under 70W limit)

For detailed metrics, see [`docs/benchmarks/BENCHMARK_DATA.md`](docs/benchmarks/BENCHMARK_DATA.md) and [`docs/benchmarks/gpu_l4_results.md`](docs/benchmarks/gpu_l4_results.md).

## API Overview

Core endpoints documented in [`openapi.yaml`](openapi.yaml):

**Health Check:**
```bash
curl http://localhost:8080/health
curl http://localhost:8080/ping  # Returns "pong"
```

**Expression Evaluation:**
```bash
curl -X POST http://localhost:8080/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"2*x+sin(x)", "x":[3.14]}'
# Returns: {"y":[6.281592652916487]}
```

**Root Finding (with bracket):**
```bash
curl -X POST http://localhost:8080/bisect \
  -H "Content-Type: application/json" \
  -d '{"expr":"x*x - 4", "lo":0, "hi":3}'
```

**Root Finding (auto-bracket):**
```bash
curl -X POST http://localhost:8080/bisect_auto \
  -H "Content-Type: application/json" \
  -d '{"expr":"x*x - 4", "guess":2.0}'
```

See [`openapi.yaml`](openapi.yaml) for complete API specification and additional endpoints.

## Docker Setup

**Using Docker (Easiest):**
```bash
# Pull and run the latest image
docker pull ghcr.io/regularjoe-ceo/luxi-edge:latest
docker run -d -p 8080:8080 --name luxi-edge ghcr.io/regularjoe-ceo/luxi-edge:latest

# Test the API
curl http://localhost:8080/health
```

**Using Docker Compose (Recommended):**
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

## Building from Source

```bash
# Requires Rust 1.75+
cargo build --release
cd edge && cargo run --release
```

For detailed build instructions, see [`docs/agents.md`](docs/agents.md).

## Agents

AI agents and automated tools should use:
- **Integration Guide**: [`docs/agents.md`](docs/agents.md) - Complete agent documentation
- **Bootstrap Script**: `bash scripts/agent_bootstrap.sh` - One-command setup and testing
- **Smoke Tests**: `bash scripts/agent_smoke.sh` - Verify all endpoints

## For Humans

If you are a human user (not an automated agent), start with these resources:

- Quick overview and getting started: [README.md](README.md)
- Docker setup (recommended): [Docker Setup](#docker-setup)
- Build from source: [Building from Source](#building-from-source)
- API overview and example calls: [API Overview](#api-overview) and [openapi.yaml](openapi.yaml)
- How it works (conceptual guide): [docs/guides/how-it-works.md](docs/guides/how-it-works.md)
- Benchmarks and performance data: [docs/benchmarks/BENCHMARK_DATA.md](docs/benchmarks/BENCHMARK_DATA.md)

## Benchmarks

Performance benchmarks are centralized under [`docs/benchmarks/`](docs/benchmarks/):
- **[`docs/benchmarks/BENCHMARK_DATA.md`](docs/benchmarks/BENCHMARK_DATA.md)** - Latest performance metrics
- **[`docs/benchmarks/COMPARATIVE_ANALYSIS.md`](docs/benchmarks/COMPARATIVE_ANALYSIS.md)** - Cross-tool comparisons
- **[`benches/`](benches/)** - Criterion benchmark source code

Run benchmarks:
```bash
cargo bench --bench edge_suite      # SIMD runtime benchmarks
cargo bench --bench my_benchmark    # Fallback calculus benchmarks
```

## Repository Layout

```
LuxiEdge/
├── edge/                    # HTTP server implementation
├── src/                     # Core library (expression evaluation, SIMD, root-finding)
├── docs/                    # Documentation
│   ├── agents.md           # AI agent integration guide (canonical)
│   ├── guides/             # User-focused guides
│   ├── technical/          # Technical deep-dives
│   ├── benchmarks/         # Benchmark results and analysis
│   └── legal/              # Legal and policy documents
├── benches/                 # Criterion benchmark harnesses
├── scripts/                 # Automation scripts (bootstrap, smoke tests, etc.)
├── tools/                   # Development utilities
├── holding/                 # Archived legacy/backup files
├── openapi.yaml            # API specification
└── README.md               # This file
```

## Documentation

For detailed technical documentation, see the `docs/` directory:
- **`docs/guides/how-it-works.md`** ⭐ **Start here!** - Simple explanation of the math service and new features
- **`docs/agents.md`** - AI agent integration guide (canonical location)
- `docs/technical/scientific-overview.md` - Technical reference and benchmarks
- `docs/technical/architecture.md` - System architecture
- `docs/technical/algorithms.md` - Algorithm details
- `docs/benchmarks/` - Centralized benchmark navigation and raw data exports
- `docs/legal/` - Contributing, security, and legal information

See [`docs/README.md`](docs/README.md) for the complete documentation index.

## Security & Contributing

- **Security**: See [`docs/legal/security.md`](docs/legal/security.md)
- **Contributing**: See [`docs/legal/contributing.md`](docs/legal/contributing.md)
- **Code of Conduct**: Professional conduct is expected in all interactions

## About the "erock" Internal Name

This codebase uses the internal module and crate name "erock" for historical reasons and build compatibility. The public product name is **Luxi Edge**. Think of "erock" as the engineering/build identifier.

## Precision Parameter

Optional query parameter `precision=f64|f32|auto` on `/evaluate`, `/bisect`, `/bisect_auto`.
- Current server builds compute in f64; older servers may ignore this parameter.
- Python example client:
```bash
./tools/client_python_example.py --base http://localhost:8080 --precision f32 evaluate --expr "x*x+2*x+1" --x 3
```

---

© 2025 Eric Waller. All rights reserved.
## Benchmarks

### Loaded SIMD Win (64k f64, Nov 6 2025)
- **ops/J**: 399,029 (SIMD active; 16x scalar baseline of 24k).
- **Latency**: Mean 1.28s/req (oha: 100 req, 5 concurrency; p95 1.30s; total 25.56s).
- **Throughput**: 2,503k ops/s (64M ops total).
- **Power**: 6.28W avg (M1 Pro; efficient SIMD).
- **Repro**: `sin(x)*cos(x)` on uniform(-10,10) batch (seed=42); powermetrics + oha.
