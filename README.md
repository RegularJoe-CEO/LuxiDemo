# eRock – Deterministic, Energy‑Efficient Math Microservice

`eRock` is a small Rust microservice that provides two numeric operations over a simple HTTP/JSON API:

- **Expression evaluation** – compute `y = f(x)` for numeric arrays using compiled code and hardware SIMD.
- **Root finding** – robust bisection (with optional auto‑bracketing) to solve `f(t)=0` within a tolerance.

Built for edge and server workloads, `eRock` uses minimal CPU cycles so your devices run longer or handle more load. It isn’t a stream processor or an AI engine – it does fast numeric math and does it extremely efficiently.

## Why `eRock`?

- **Energy efficient** – Because `eRock` is compiled and uses SIMD, it finishes calculations in microseconds, letting CPUs return to idle and saving battery or power draw. In drones and IoT nodes, that means longer missions or smaller batteries. In data centers, it means lower racks and cooling costs.
- **Fast** – Expression evaluation runs on arrays with near‑linear scaling, and root finding converges within defined iteration caps. The service adds minimal overhead beyond network latency.
- **Deterministic** – You define the formula and tolerance; `eRock` always returns the same result for the same inputs. This makes it suitable for safety‑critical uses.
- **Portable** – Runs on x86‑64 and ARM64, and can be compiled statically. Deploy it on a companion computer, industrial PC, or server.

## Usage

See `openapi.yaml` for endpoint definitions. Typical calls look like:

```bash
curl -X POST http://localhost:8000/evaluate -H 'Content-Type: application/json' \
  -d '{"expr": "x^2 - 4", "x": [3.0, 4.0]}'
# -> returns [5.0, 12.0]

curl -X POST http://localhost:8000/bisect_auto -H 'Content-Type: application/json' \
  -d '{"expr": "x^2 - 4", "guess": 1.0, "tol": 1e-6}'
# -> returns ~2.0

