# LuxiEdge

World's Fastest Deterministic JSON Math Engine

Bit-exact vector math (y=f(x)) via stateless REST API with SHA-256 verified outputs.

## Getting Started

### Step 1: Download the Binary

**Linux x86_64**

    curl -LO https://github.com/RegularJoe-CEO/LuxiDemo/releases/download/v2.0.0/luxiedge-linux-x86_64.zip
    unzip luxiedge-linux-x86_64.zip
    chmod +x luxiedge-linux-x86_64

**Linux ARM64**

    curl -LO https://github.com/RegularJoe-CEO/LuxiDemo/releases/download/v2.0.0/luxiedge-linux-arm64.zip
    unzip luxiedge-linux-arm64.zip
    chmod +x luxiedge-linux-arm64

**macOS ARM64 (Apple Silicon)**

    curl -LO https://github.com/RegularJoe-CEO/LuxiDemo/releases/download/v2.0.0/luxiedge-macos-arm64.zip
    unzip luxiedge-macos-arm64.zip
    chmod +x luxiedge-macos-arm64

### Step 2: Start the Server

    ./luxiedge-linux-x86_64

You should see:

    LUXIEDGE CORE ENGINE v0.1.0
    SHA-256 Verified | Powered by Elvis
    Listening on http://0.0.0.0:9090...

### Step 3: Test the Health Endpoint

    curl http://localhost:9090/health

### Step 4: Evaluate an Expression

    curl -X POST http://localhost:9090/evaluate \
      -H "Content-Type: application/json" \
      -d '{"expr":"sin(x)*cos(x)","values":[0.5,1.0,1.57,2.0,3.14],"precision":"f64"}'

Response includes results and SHA-256 hash for verification.

## Supported Expressions

    sin(x)          Sine
    cos(x)          Cosine
    exp(x)          Exponential
    ln(x)           Natural log
    sqrt(x)         Square root
    x^2             Square
    x^3             Cube
    sin(x)*cos(x)   Combined trig
    erf(x)          Error function
    normcdf(x)      Normal CDF
    normpdf(x)      Normal PDF
    gamma(x)        Gamma function

## API Reference

**POST /evaluate**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| expr | string | Yes | Expression to evaluate |
| values | array | Yes | Input values (supports 4M+ elements) |
| precision | string | No | "f32" or "f64" (default) |

## Performance

| Platform | Throughput | Efficiency |
|----------|------------|------------|
| M1 Pro CPU | 2.3M ops/sec | 546K ops/J |
| NVIDIA L4 GPU | 72.7M ops/sec | 4.4M ops/J |
| NVIDIA H100 GPU | 286.9B ops/sec | 2.35B ops/J |

## GPU Acceleration

For GPU support, download the CUDA-enabled binary and run with the --gpu flag:

    ./luxiedge-linux-x86_64-gpu --gpu

Run the GPU benchmark:

    python3 gpu_benchmark.py --device cuda --elements 4000000

Supported GPUs: NVIDIA L4, H100, RTX 4090

## Key Features

1. Bit-exact determinism: Same input always produces same output
2. SHA-256 verification: Every response includes hash for audit trails
3. Cross-platform SIMD: AVX-512, AVX2, ARM Neon with auto-detection
4. GPU acceleration: CUDA kernels for FP16/FP32
5. Large payloads: Supports 4M+ element arrays
6. Memory safe: Built in Rust

## Use Cases

1. Quant Finance: Monte Carlo simulations, risk models, audit compliance
2. Defense and Autonomous Systems: Certification-ready, edge deployment
3. Scientific Computing: Reproducible numerical results

## License

Copyright 2026 Eric Waller. All rights reserved.
6 patent families pending.

## Contact

e@ewaller.com
