# LuxiEdge for Robotics & FSD

## The Problem

Your FSD algorithm works perfectly in simulation. Deploy to the vehicle and sensor fusion drifts. Inverse kinematics gives slightly different joint angles. You spend weeks debugging floating point differences instead of improving the product.

## How LuxiEdge Solves This

Development laptop, HIL simulator, and production ECU all produce identical results. Debug once.

## Motion & Control Expressions

| Function | Use Case |
|----------|----------|
| cos((x^2 - 1) / 2) | Inverse Kinematics (2-Link Arm) |
| exp(-x^2 / (2 * sigma^2)) | Kalman Gain Weighting |
| atan(x / (1 + x^2)) | Tire Slip Angle |
| sin(x)^2 * (1 - exp(-x/tau)) | Motor Torque Curve |
| sin(x^2) / x | Clothoid Curvature (Smooth Steering) |
| sqrt(x^2 + y^2 + z^2) | Lidar Point Distance |

## Example

```bash
curl -X POST http://localhost:10000/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"exp(-x^2 / 2)","values":[0.1,0.5,1.0,2.0],"precision":"f32"}'
```

## Dev-to-Deployment Workflow

1. Develop in Python/MATLAB on workstation
2. Validate with LuxiEdge, store hashes
3. Deploy same expressions to edge (ARM64)
4. Hashes match = no floating point surprises

## Contact

e@ewaller.com
