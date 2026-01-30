# LuxiEdge for Energy Efficiency

## The Numbers

| Hardware | Throughput | Power Draw | Efficiency |
|----------|------------|------------|------------|
| NVIDIA L4 | 30.7B ops/sec | 72W | 426M ops/joule |
| NVIDIA H100 | 286.94B ops/sec | 117.2W | 2.45B ops/joule |

Validated by TestFort. 0% error rate. 1.47ms p95 latency.

## The Problem

You are paying for compute you do not need. General-purpose math libraries waste cycles on edge cases your workload never hits. GPU utilization sits at 60% while your power bill climbs.

## How LuxiEdge Solves This

Purpose-built for dense vector math. No bloat. No unnecessary precision. Just the operations you need at the throughput you need.

The H100 benchmark: 444.4 trillion operations in one hour under 200 concurrent users. That is not a typo.

## Battery-Constrained Compute

For edge devices and mobile deployments, efficiency is not optional. The ARM64 binary delivers deterministic math at a fraction of the power draw of general-purpose solutions.

| Metric | LuxiEdge ARM64 |
|--------|----------------|
| Binary size | 1.6 MB |
| Cold start | < 100ms |
| Memory footprint | < 50 MB |
| Dependencies | None |

No runtime. No JIT warmup. No garbage collection pauses.

## Example: Batch Processing at Scale

curl -X POST http://localhost:10000/evaluate -H "Content-Type: application/json" -d '{"expr":"exp(x)*sin(x)","values":[0.001,0.002,0.003],"precision":"f32"}'

Pass millions of values. Get results back with a SHA256 hash for verification.

## Tracking Power

On GPU deployments, monitor power draw during batch operations:

nvidia-smi -q -d POWER

Compare ops/joule against your current solution. The difference funds itself.

## Getting Started

See the main [README](../README.md) for download and setup.

## Contact

e@ewaller.com
