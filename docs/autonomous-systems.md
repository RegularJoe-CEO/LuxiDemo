# LuxiEdge for Autonomous Systems

## The Problem

Your drone's navigation algorithm runs on an ARM processor at the edge. You test it on an x86 workstation in the lab. The math libraries give slightly different results. You spend weeks debugging "phantom" issues that only appear in flight.

Worse: certification requires reproducibility. DO-178C doesn't accept "floating point variance" as an excuse.

## How LuxiEdge Solves This

A receipt is the SHA-256 of one run. Re-running the same binary is stable. f32 `sin(x)` on the five gold inputs matched across macOS arm64, macOS x86_64, Linux x86_64, and Linux arm64. f64 Kepler sweeps did not all match. GPU was not measured.

## Example: Sensor Fusion

```bash
curl -X POST http://localhost:10000/evaluate -H "Content-Type: application/json" -d '{"expr":"sin(x)*cos(x)","values":[0.1,0.2,0.3,0.4,0.5],"precision":"f32"}'
```

That SHA256 hash is the receipt for the run you just made. Compare it with a later run of the same binary.

## Why This Matters for Certification

| Requirement | LuxiEdge Approach |
|-------------|-------------------|
| Reproducibility | SHA256 hash verification |
| Traceability | Log inputs + hash, replay anytime |
| Same binary | Re-running one binary is stable |
| f64 math | Uses the platform libm, so hashes can differ |

## Edge Deployment

The ARM64 binary is 1.6 MB. Runs standalone. No dependencies. No runtime. No network required after deployment.

chmod +x luxiedge-edge-arm64
./luxiedge-edge-arm64

Starts on port 10000. Your flight controller hits it locally.

## Getting Started

See the main [README](../README.md) for download and setup.

## Contact

e@ewaller.com
