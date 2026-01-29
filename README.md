# LuxiEdge

**World's Fastest Deterministic JSON Math Engine**

---

### The Problem
Floating-point math shouldn't be a gamble.

If you run a complex vector operation on an Intel CPU, an ARM chip, and an Nvidia H100, you will often get three slightly different results due to IEEE 754 variances. In high-stakes fields like Quant Finance or Autonomous Systems, "close enough" isn't good enough.

### The Solution
LuxiEdge forces **bit-exact determinism** across all platforms. It runs as a stateless, single-binary REST API. You send it a math expression and an array; it returns the result and a SHA-256 hash. 

If that hash changes, your hardware is lying to you.

It is also fast. On an H100 with our CUDA kernels, we are hitting **286 Billion ops/sec** with 0% error.

---

## Quickstart

No dependencies. No Docker. No complex install. Just a binary.

### 1. Grab the executable
Download the release for your hardware from the [Releases Page](https://github.com/RegularJoe-CEO/LuxiDemo/releases).

*   **Mac (Apple Silicon):** `luxiedge-macos-arm64`
*   **Linux (GPU/H100/L4):** `luxiedge-linux-x86_64`
*   **Linux (ARM/Edge):** `luxiedge-linux-arm64`

### 2. Run it
```bash
chmod +x luxiedge-macos-arm64  # Or your specific binary
./luxiedge-macos-arm64
# Listening on http://0.0.0.0:9090...
```

### 3. Test it (The Proof)
Send it some math. We support arrays of up to 4 million elements per request.
```bash
curl -X POST http://localhost:9090/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"sin(x)*cos(x)", "values":[0.5, 1.0, 1.57], "precision":"f64"}'
```
You will get back a JSON response with your results and a **SHA-256 hash**. That hash is your audit trail.

---

## Why "Quant" Functions?
Most math libraries satisfy themselves with basic trig. We specifically implemented widely-used quantitative finance functions because they are the workhorses of risk modeling.

*   `erf(x)` - Error Function
*   `normcdf(x)` - Normal CDF
*   `normpdf(x)` - Normal PDF
*   `gamma(x)` - Gamma Function

We optimized these to run at the same speed as `sin(x)`—even on the GPU.

## Performance
We map power usage to performance because efficiency matters at the edge.

| Platform | Throughput | Efficiency |
|----------|------------|------------|
| **NVIDIA H100** | 286.9B ops/sec | 2.35B ops/J |
| **NVIDIA L4** | 72.7M ops/sec | 4.4M ops/J |
| **M1 Pro CPU** | 2.3M ops/sec | 546K ops/J |

*Note: To unlock H100/L4 speeds, you must use the Linux-x86_64 binary on a machine with CUDA drivers.*

---

## License & Contact
Copyright 2026 Eric Waller. All rights reserved.  
6 patent families pending.

**Contact:** [e@ewaller.com](mailto:e@ewaller.com)
