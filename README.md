# LuxiEdge

Deterministic JSON math engine. Bit-exact vector computation (y=f(x)) via stateless REST API.

Same input, same output. Every time. Every platform. CPU or GPU. ARM or x86. Windows, Linux, Mac. The hash matches. No floating point drift. No platform variance. No surprises during audit.

## Benchmarks

TestFort validated, December 2025.

| Platform | Throughput | Efficiency | Error Rate |
|----------|------------|------------|------------|
| H100 | 286.94B ops/sec | 2.35B ops/joule | 0% |
| L4 | 30.7B ops/sec | 426M ops/joule | 0% |

p95 latency: 1.47ms

Sustained: 444.4T ops/hr at 200 concurrent users

## Download

Linux x86_64:

    curl -LO https://github.com/RegularJoe-CEO/LuxiDemo/releases/download/v2.0.0/luxiedge-linux-x86_64
    chmod +x luxiedge-linux-x86_64

Linux ARM64:

    curl -LO https://github.com/RegularJoe-CEO/LuxiDemo/releases/download/v2.0.0/luxiedge-linux-arm64
    chmod +x luxiedge-linux-arm64

## Usage

Start the server:

    ./luxiedge-linux-x86_64

Evaluate an expression:

    curl -X POST http://localhost:9090/evaluate \
      -H "Content-Type: application/json" \
      -d '{"expr":"sin(x)*cos(x)","values":[0.5,1.0,1.57,2.0,3.14],"precision":"f32"}'

## Functions

sin, cos, tan, exp, log, sqrt, abs, floor, ceil, round, pow, min, max, sinh, cosh

Binary operators: +, -, *, /, ^, %

## Validation

Independent verification available:

- TestFort QA Lab report
- PFLB load testing results
- OpenBenchmarking.org submission

## Contact

30-day evaluation available.

eric@luxiedge.com

luxiedge.com

(c) 2026 Eric Waller. 6 patent families pending.
