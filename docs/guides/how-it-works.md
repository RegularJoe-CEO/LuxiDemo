<!-- SPDX-FileCopyrightText: 2025 Eric Waller -->
<!-- SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0 -->

# How the Luxi Edge Math Service Works

**A Simple Guide for Everyone**

Luxi Edge is a math service that runs on a server and answers math questions over HTTP. Think of it like a super-fast calculator that can handle complex mathematical expressions, find where functions cross zero, and calculate slopes and gradients.

---

## What Can It Do?

Luxi Edge provides six main mathematical capabilities through simple HTTP endpoints:

### 1. **Evaluate Expressions** (`POST /evaluate`)

Calculate the value of a mathematical expression for many inputs at once.

**What it does:** Takes an expression like `x*x + 2*x + 1` and a list of x values, then computes the result for each x using SIMD (fast parallel computation).

**Example:**
```bash
curl -X POST http://localhost:8080/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"x*x + 2*x + 1", "x":[0, 1, 2, 3]}'

# Response: {"y":[1.0, 4.0, 9.0, 16.0]}
```

**Why it's useful:** Process hundreds or thousands of data points extremely fast for real-time analytics, simulations, or data pipelines.

---

### 2. **Calculate Derivatives** (`POST /evaluate_derivative`) 🆕

Find both the value and the slope (rate of change) of a function at given points.

**What it does:** For each x value, computes both f(x) and f'(x) (the derivative). Uses numerical differentiation with a small step size.

**Example:**
```bash
curl -X POST http://localhost:8080/evaluate_derivative \
  -H "Content-Type: application/json" \
  -d '{"expr":"x*x", "x":[1, 2, 3]}'

# Response: {"y":[1.0, 4.0, 9.0], "dy_dx":[2.0, 4.0, 6.0]}
```

**Why it's useful:** Understand how quickly things are changing—like velocity from position, or profit margins from sales volume.

---

### 3. **Compute Gradients** (`POST /gradient`) 🆕

Calculate the gradient (all partial derivatives) of a multi-variable function.

**What it does:** Takes a function with multiple variables (like `x*x + y*y + z`) and computes how the function changes with respect to each variable at a specific point.

**Example:**
```bash
curl -X POST http://localhost:8080/gradient \
  -H "Content-Type: application/json" \
  -d '{"expr":"x*x + y*y", "vars":{"x":3, "y":4}}'

# Response: {"value":25.0, "gradient":{"x":6.0, "y":8.0}}
```

**Why it's useful:** Optimize multi-variable systems, understand sensitivities, or train machine learning models.

---

### 4. **Find Roots with Newton's Method** (`POST /newton`) 🆕

Find where a function equals zero using Newton-Raphson method, with automatic fallback to bisection if Newton fails.

**What it does:** Starts from initial guesses and iteratively refines them to find exact roots. If Newton's method struggles (bad derivative, divergence), automatically switches to the more reliable bisection method.

**Example:**
```bash
curl -X POST http://localhost:8080/newton \
  -H "Content-Type: application/json" \
  -d '{"expr":"x*x - 4", "guesses":[1.0, -1.0]}'

# Response: {"results":[
#   {"guess":1.0, "root":2.0, "f":0.0, "newton_iters":4, "converged":true, "used_fallback":false},
#   {"guess":-1.0, "root":-2.0, "f":0.0, "newton_iters":4, "converged":true, "used_fallback":false}
# ]}
```

**Why it's useful:** Solve equations quickly with the speed of Newton's method but the reliability of bisection. Great for calibration, parameter fitting, and solving equilibrium conditions.

---

### 5. **Find Roots with Bisection** (`POST /bisect`)

Find where a function equals zero when you know the answer is between two points.

**What it does:** Given a bracket `[lo, hi]` where the function has opposite signs at the endpoints, repeatedly splits the interval in half until the root is found with high precision.

**Example:**
```bash
curl -X POST http://localhost:8080/bisect \
  -H "Content-Type: application/json" \
  -d '{"expr":"x*x - 4", "lo":0, "hi":3}'

# Response: {"root":2.0, "f":0.0, "iters":33, "bracket_ok":true}
```

**Why it's useful:** When you know roughly where a root is, this method is rock-solid and always converges.

---

### 6. **Find Roots with Auto-Bracketing** (`POST /bisect_auto`)

Find where a function equals zero starting from just a guess—the service finds the bracket for you.

**What it does:** Starts from your guess and exponentially expands outward until it finds a bracket where the function changes sign, then runs bisection.

**Example:**
```bash
curl -X POST http://localhost:8080/bisect_auto \
  -H "Content-Type: application/json" \
  -d '{"expr":"x*x - 4", "guess":1.0}'

# Response: {"root":2.0, "f":0.0, "lo":1.0, "hi":2.0, "iters":0, "bracket_ok":true, "expansions":1}
```

**Why it's useful:** Most convenient when you have a rough idea where the answer is but don't want to manually find a bracket.

---

## New Math Features 🆕

Recently added capabilities:

1. **`/evaluate_derivative`** - Compute function values and their derivatives simultaneously
2. **`/gradient`** - Calculate gradients for multi-variable optimization
3. **`/newton`** - Advanced root-finding that tries Newton-Raphson first, falls back to bisection automatically

These additions make Luxi Edge suitable for:
- **Machine learning**: gradient-based optimization
- **Scientific computing**: sensitivity analysis, parameter estimation
- **Engineering**: fast equation solving with guaranteed convergence

---

## How It Works Under the Hood

### Expression Parsing
When you send an expression like `x*x + 2*x + 1`, Luxi Edge:
1. **Tokenizes** it (breaks it into pieces: `x`, `*`, `x`, `+`, `2`, `*`, `x`, `+`, `1`)
2. **Parses** it into an Abstract Syntax Tree (AST)
3. **Evaluates** the AST for your input values

### SIMD Acceleration
For `/evaluate`, the service processes multiple x values in parallel using SIMD (Single Instruction, Multiple Data) instructions. On modern CPUs, this means computing 2-8 results simultaneously, making it much faster than computing one at a time.

### Numerical Derivatives
For `/evaluate_derivative` and `/gradient`, derivatives are computed numerically using finite differences:
```
f'(x) ≈ (f(x + h) - f(x - h)) / (2h)
```
where `h` is a small step (default: 1e-6).

### Newton-Raphson with Fallback
The `/newton` endpoint is smart:
1. **Try Newton's method first**: Fast convergence (quadratic) when the function is well-behaved
   - Formula: `x_next = x - f(x)/f'(x)`
2. **Detect failure**: If derivative is too small, steps diverge, or non-finite values appear
3. **Fall back to bisection**: Reliable, guaranteed convergence for continuous functions
4. **Auto-bracket**: Even finds the bracket automatically if needed

---

## When to Use Each Endpoint

| **Use Case** | **Endpoint** |
|-------------|-------------|
| Evaluate many points quickly | `/evaluate` |
| Need slopes/rates of change | `/evaluate_derivative` |
| Optimize multi-variable functions | `/gradient` |
| Find roots with best performance | `/newton` |
| Find roots with known bracket | `/bisect` |
| Find roots from rough guess | `/bisect_auto` |

---

## Performance Characteristics

Performance metrics based on benchmark suite results:

- **Speed**: 87× faster than Python/NumPy, 5.5× faster than C++
- **Memory**: 25× less than Python, same as optimized C++
- **Energy**: 50% less power than Python, 33% less than C++
- **Precision**: Default float64 (f64), configurable
- **Concurrency**: Stateless design allows horizontal scaling

See [BENCHMARK_DATA.md](../benchmarks/BENCHMARK_DATA.md) for complete methodology, test conditions, and detailed performance metrics.

---

## Example Workflow: Calibrate a Model

Let's say you have a model `revenue = price * demand(price)` where `demand(price) = 1000 - 50*price`, and you want to find the price that gives a specific target revenue.

```bash
# Step 1: Define the equation (target revenue - actual revenue = 0)
# If target is $12,000: 12000 - price*(1000 - 50*price) = 0
# Simplifies to: 12000 - 1000*price + 50*price*price = 0

# Step 2: Find the price(s) using Newton's method
curl -X POST http://localhost:8080/newton \
  -H "Content-Type: application/json" \
  -d '{"expr":"12000 - 1000*x + 50*x*x", "guesses":[10.0, 15.0]}'

# Result: Two prices achieve $12,000 revenue (quadratic has two roots)
```

---

## Additional Resources

- **API Reference**: See [openapi.yaml](../openapi.yaml) for full API specification
- **Architecture**: [docs/ARCHITECTURE.md](ARCHITECTURE.md) for system design
- **Algorithms**: [docs/ALGORITHM_DETAILS.md](ALGORITHM_DETAILS.md) for implementation details
- **Scientific Background**: [docs/SCIENTIFIC_OVERVIEW.md](SCIENTIFIC_OVERVIEW.md) for academic reference

---

© 2025 Eric Waller. All rights reserved.
