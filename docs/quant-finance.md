# LuxiEdge for Quantitative Finance

## The Problem

Monte Carlo simulations run on different machines produce different results. Not by much. A few bits. But when regulators ask you to reproduce a calculation from 6 months ago, "close enough" doesn't work.

Standard math libraries use platform-specific optimizations. sin(x) on an Intel chip gives a slightly different answer than sin(x) on AMD. GPU results drift from CPU results. You end up with audit logs that can't be verified.

## How LuxiEdge Solves This

Every calculation returns a SHA256 hash. Same input, same hash, every time. Doesn't matter if you run it on:

- Linux server in your data center
- macOS laptop during development
- GPU cluster for production
- ARM edge device for low-latency execution

The hash matches. Bit-for-bit.

## Example: Option Pricing Components

curl -X POST http://localhost:10000/evaluate -H "Content-Type: application/json" -d '{"expr":"normcdf(x)","values":[-2.0,-1.0,0.0,1.0,2.0],"precision":"f32"}'

Response includes a SHA256 hash you can store with your trade records.

## Audit Trail

1. Run your pricing model
2. Store the SHA256 hash with the trade
3. Six months later, regulator asks for verification
4. Re-run the same inputs
5. Hash matches. Done.

No arguing about floating point differences. No "well, we upgraded our servers." The math is the math.

## Available Functions

All produce identical hashes across platforms:

| Function | Use Case |
|----------|----------|
| normcdf(x) | Cumulative normal distribution |
| normpdf(x) | Normal probability density |
| erf(x) | Error function |
| exp(x) | Exponential |
| ln(x) | Natural log |
| sqrt(x) | Square root |
| gamma(x) | Gamma function |

## Getting Started

See the main [README](../README.md) for download and setup.

## Contact

e@ewaller.com
