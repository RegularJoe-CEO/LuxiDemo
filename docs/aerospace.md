# LuxiEdge for Aerospace & Orbital Mechanics

## The Problem

Trajectory calculations run on ground simulators, then again on flight computers. Different hardware, different floating point results. When your satellite misses its injection burn by 0.001%, that is kilometers of error.

## How LuxiEdge Solves This

Same math, same SHA256 hash, whether running on a workstation in mission control or an ARM processor in orbit.

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

## Ground-to-Flight Verification

1. Run trajectory sim on ground (Linux x86_64)
2. Store SHA256 hash
3. Load same calculation on flight computer (ARM64)
4. Verify hash matches before commit

No bit drift. No "close enough."

## Contact

e@ewaller.com
