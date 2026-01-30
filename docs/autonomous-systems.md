# LuxiEdge for Autonomous Systems

## The Problem

Your drone's navigation algorithm runs on an ARM processor at the edge. You test it on an x86 workstation in the lab. The math libraries give slightly different results. You spend weeks debugging "phantom" issues that only appear in flight.

Worse: certification requires reproducibility. DO-178C doesn't accept "floating point variance" as an excuse.

## How LuxiEdge Solves This

Deterministic math. The trig functions, exponentials, and probability calculations your flight controller uses will produce identical results whether you run them on:

- ARM64 edge device (1.6 MB binary, fits on constrained hardware)
- x86 simulation workstation
- GPU cluster for batch testing
- CI/CD pipeline for regression tests

Same input, same output, same SHA256 hash. Always.

## Example: Sensor Fusion

```bash
curl -X POST http://localhost:10000/evaluate -H "Content-Type: application/json" -d '{"expr":"sin(x)*cos(x)","values":[0.1,0.2,0.3,0.4,0.5],"precision":"f32"}'
```

That SHA256 hash in the response? It's identical on your test bench and in the field.

## Why This Matters for Certification

| Requirement | LuxiEdge Approach |
|-------------|-------------------|
| Reproducibility | SHA256 hash verification |
| Traceability | Log inputs + hash, replay anytime |
| Platform independence | Same binary behavior across ARM/x86 |
| Determinism | No platform-specific math library calls |

## Edge Deployment

The ARM64 binary is 1.6 MB. Runs standalone. No dependencies. No runtime. No network required after deployment.

chmod +x luxiedge-edge-arm64
./luxiedge-edge-arm64

Starts on port 10000. Your flight controller hits it locally.

## Getting Started

See the main [README](../README.md) for download and setup.

## Contact

e@ewaller.com
