# LuxiEdge for Aerospace & Orbital Mechanics

## The Problem

Trajectory calculations run on ground simulators, then again on flight computers. Different hardware, different floating point results. When your satellite misses its injection burn by 0.001%, that is kilometers of error.

## How LuxiEdge Solves This

The output hash is the SHA-256 of the little-endian f64 bytes of the values that were computed. Re-running the same binary is stable. A matching hash was not measured for every machine, and a mismatch on a long f64 sweep is not by itself a trajectory error.

## Orbital Mechanics Expressions

| Function | Use Case |
|----------|----------|
| x - e*sin(x) | Kepler Equation (Eccentric Anomaly) |
| sqrt((2/x) - (1/a)) | Vis-Viva (Orbital Velocity) |
| sqrt(x) * (sqrt(2/(1+x)) - 1) | Hohmann Transfer Delta-V |
| exp(-x/H) | Atmospheric Drag Decay |
| (3*cos(x)^2 - 1) / 2 | Gravitational Harmonics (J2) |
| (1 - cos(sqrt(x))) / x | Lambert Problem (Time of Flight) |

## Example

```bash
curl -X POST http://localhost:10000/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"x - 0.5*sin(x)","values":[0.1,0.5,1.0,2.0],"precision":"f64"}'
```

## What was measured

f64 Kepler solve for REBOUND PR #849, `e=0.1`, `M=0.991`. Residual was 0 on every machine below. SHA-256 `f9d23a13612bb6f140acdcab79463594a46a9c9c02c17f9fe77f79a944991758` on macOS arm64, macOS x86_64, Linux x86_64, and Linux arm64. The same hash came from a CPython `math.sin` reference on the Mac.

The REBOUND #934 log sweep (`e=0.1`, 100 mean anomalies from `1e-16` to `1`) kept a max residual of `5.551e-17` everywhere. The sweep hash matched on macOS x86_64, Linux x86_64, and Linux arm64 (`f7e97ade3e320ba58cd827f3bb2f6db98a2d04e532810dddd78ba0a2a668efb5`) and differed on macOS arm64 (`aee00f24c9919b3217caf6a0f957e257590b807ca7b5f1171743fb16feba5b6e`).

ANISE #340 (`e=0.6`, 720 points, max residual `4.441e-16`) did not share one hash: Linux x86_64 and Linux arm64 agreed (`edfcf59dd0b238bab7cabf397ed3da9948734edf06a79620ffcef69f8bee7a7f`); macOS arm64 was `03f91b713c6c0da3a67305a17768a31fea0403c8ae8d50a0ab1449add16f4049`; macOS x86_64 was `24cae59fe541abcc873a512e3a6e7bfbede018e2572f0498f6ded3ea1b9b8b49`.

GPU was not measured. This branch is not a release. pykep #198 is an STM tolerance report, not a Lambert input triple, so no pykep hash is published here.

## Contact

e@ewaller.com
