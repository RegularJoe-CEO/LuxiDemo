<!-- SPDX-FileCopyrightText: 2025 Eric Waller -->
<!-- SPDX-License-Identifier: LicenseRef-eRock-Business-1.0 -->

# eRock Technical Overview

## Abstract

This document provides a technical description of the eRock codebase architecture, implementation details, and computational methods. The content is intended for academic study and scientific analysis.

## 1. System Architecture

### 1.1 Repository Structure

The repository contains multiple components:

- **erock_clean/**: Complete implementation of expression evaluator with lexer, parser, and interpreter
- **edge/**: HTTP API server (Axum-based) for expression evaluation and root-finding endpoints
- **src/**: Core library with runtime, compute dispatcher, and security modules (implementation details redacted)
- **benches/**: Performance benchmarking infrastructure using Criterion.rs
- **products/**: Product-specific documentation for Edge and Core variants

### 1.2 Core Components

#### Expression Evaluator (erock_clean)

The expression evaluation system consists of three stages:

1. **Lexer** (`tokens` function)
   - Tokenizes input strings into structured token stream
   - Supports: numbers (f64), variables (alphanumeric identifiers), operators (+, -, *, /, ^), parentheses, assignment (=)
   - Uses peekable character iterator for lookahead parsing
   - Handles whitespace and validates character sequences

2. **Parser** (`parse` function)
   - Implements recursive descent parser with operator precedence
   - Precedence hierarchy: exponentiation (^) > multiplication/division (*, /) > addition/subtraction (+, -)
   - Constructs Abstract Syntax Tree (AST) with node types:
     - Number literals
     - Variable references
     - Binary operations (left operand, operator, right operand)
     - Unary operations (negation)
     - Assignment statements (variable = expression)
     - Parenthesized expressions
   - Parser state: token index, token vector, bounds checking

3. **Interpreter** (`interpret` function)
   - Tree-walking interpreter evaluating AST nodes recursively
   - Variable storage: HashMap<String, Vec<f64>>
   - Vector-based variable representation enables batch evaluation (uses first element for scalar compatibility)
   - Safe division: checks divisor magnitude against epsilon threshold
   - Assignment statements insert computed values into variable map

#### SIMD Evaluation

The `simd_eval_over_x` function (referenced in edge/src/main.rs and benchmarks) performs vectorized evaluation:
- Evaluates expression across array of x values
- Returns Vec<f64> with computed y values
- Leverages SIMD instructions for parallel computation (architecture-specific: ARM NEON, x86 AVX2, RISC-V Vector Extension)

### 1.3 Root-Finding Algorithms

#### Bisection Method (`/bisect` endpoint)

Classical bisection algorithm implementation:
- Input: expression, lower bound (lo), upper bound (hi), optional variables, tolerance, max iterations
- Algorithm:
  1. Evaluate f(lo) and f(hi)
  2. Verify bracket validity: opposite signs required (f(lo) × f(hi) < 0)
  3. Iteratively narrow interval:
     - Compute midpoint: mid = (lo + hi) / 2
     - Evaluate f(mid)
     - Update bracket based on sign comparison
     - Terminate when |hi - lo| ≤ tolerance or max iterations reached
- Returns: root value, function value at root, iteration count, bracket validity flag
- Convergence: linear, guaranteed within valid bracket

#### Auto-bracketing Bisection (`/bisect_auto` endpoint)

Enhanced bisection with automatic bracket discovery:
- Input: expression, initial guess, step size, max expansions, optional variables
- Two-phase algorithm:
  1. **Bracket Search Phase** (exponential expansion):
     - Start at guess point, evaluate f(guess)
     - Test points at guess ± step in alternating pattern
     - Double step size after each iteration
     - Terminate when sign change detected or max expansions reached
  2. **Bisection Phase**: Standard bisection on discovered bracket
- Advantage: No manual bracket specification required
- Risk: May fail to find bracket if root distant from guess or step size inappropriate
- Returns: root, function value, discovered bracket bounds, iteration counts, expansion count

### 1.4 HTTP API Server (edge/)

Built on Axum web framework (v0.7) with async Tokio runtime:

**Endpoints:**
- `POST /evaluate`: Vectorized expression evaluation
- `POST /bisect`: Root finding with supplied bracket
- `POST /bisect_auto`: Root finding with automatic bracketing
- `GET /health`: Service health check and version information

**Data Flow:**
1. JSON request deserialized to Rust structs
2. Expression tokenized by lexer
3. Tokens parsed to AST
4. AST interpreted with variable context
5. Results serialized to JSON response

**Port:** 8080 (default)
**Concurrency:** Multi-threaded Tokio runtime handles concurrent requests

## 2. Runtime System (src/runtime/)

### 2.1 Hardware Probe (`HwProbe`)

Platform detection and capability discovery:
- **Fields:**
  - `arch`: CPU architecture string ("aarch64", "x86_64", "riscv64")
  - `simd`: Vector of supported SIMD instruction sets (["neon", "avx2", "vext"])
  - `cpu_count`: Number of logical processors
  - `battery_mv`: Optional battery voltage in millivolts
- **Purpose:** Enables adaptive algorithm selection based on hardware capabilities

### 2.2 Battery-Aware Throttling

Power management policy function:
- Input: HwProbe reference
- Output: CPU capacity percentage (0-100)
- Policy thresholds:
  - < 3500 mV: 60% capacity (conservative)
  - < 3700 mV: 80% capacity
  - ≥ 3700 mV: 100% capacity
- Application: Adjusts computational load based on power availability

### 2.3 Offload Queue

Asynchronous task queuing system:
- Lock-free queue implementation (details redacted)
- Async submission interface
- Integration with compute dispatcher
- Buffer size: 1024 tasks

## 3. Compute Dispatcher (src/compute/)

### 3.1 Operation Types

`ComputeOp` enum defines supported operations:
- **Gemm** (General Matrix Multiply): parameters include matrix dimensions (m, n, k) and scalar multiplier (alpha)
- **Fft** (Fast Fourier Transform): size and precision parameters

### 3.2 Precision Modes

Adaptive precision selection based on power state:
- **Fp32**: 32-bit floating point (standard)
- **Fp16**: 16-bit floating point (power-constrained)
- **Int8**: 8-bit integer (severe power constraint)

Selection logic:
- Battery < 3700 mV → Int8
- Battery < 3900 mV → Fp16
- Otherwise → Fp32

### 3.3 Dispatcher Architecture

Central routing and execution coordination:
- Inputs: HwProbe (hardware capabilities), Enclave (security context)
- Process flow:
  1. Precision selection based on battery voltage
  2. Operation integrity verification (zero-knowledge proof stub)
  3. Protected execution in security enclave
  4. Architecture-specific kernel dispatch (RISC-V VEXT, ARM NEON, x86 FMA)
  5. Post-operation integrity validation

## 4. Security System (src/security/)

### 4.1 Enclave

Trusted Execution Environment abstraction:
- Initialization: TEE/TPM session establishment and attestation
- Secure memory pool allocation (encrypted memory)
- Operation integrity verification using zero-knowledge proof techniques
- Protected execution environment for sensitive computations
- Anti-tamper checksums and rollback protection

### 4.2 Platform Attestation

Cryptographic attestation mechanism:
- Generates attestation tokens validating platform integrity
- TPM-based secure key storage (implementation details redacted)
- Periodic integrity monitoring loop

## 5. Performance Characteristics

### 5.1 Benchmark Methodology

Performance testing uses Criterion.rs framework:
- **Scalar benchmark**: Loop-based evaluation over 100,000 points
- **SIMD benchmark**: Vectorized evaluation using f64x4 SIMD lanes
- Hardware: Apple M1 Pro MacBook Pro (reference platform)

### 5.2 Reported Metrics

From BENCHMARK_DATA.md:
- Expression evaluation throughput: 193,421 operations/second
- Single operation latency: 5.17 microseconds
- Root finding latency: 89 microseconds
- Precision: 9.5e-08 tolerance

Energy efficiency measurements (from BENCHMARK_DATA.md):
- Baseline system power (idle): 783 mW
- System power under load: 596 mW
- Note: The reported under-load power being lower than idle represents the measured values from the benchmark. This may reflect measurement methodology or system-specific power management behavior.
- Energy per operation: 3.08 microjoules (baseline calculation: 55.6 μJ)

API endpoint latencies:
- `/health`: < 1 ms
- `/evaluate`: 7.04 ms
- Root finding endpoints (`/bisect`, `/bisect_auto`): ~8.93 ms

### 5.3 Architectural Optimizations

- SIMD instruction utilization: 4-wide f64 vectors (AVX2/NEON)
- Cache tiling for matrix operations
- Loop unrolling in fused kernels
- Hardware-specific code paths (x86_64, ARM64, RISC-V)
- Zero-copy data structures where possible

## 6. Data Types and Formats

### 6.1 Token Types

```rust
enum Token {
    Number(f64),           // Floating-point literal
    Variable(String),       // Identifier
    Operator(String),       // +, -, *, /, ^, (, ), =
}
```

### 6.2 AST Node Types

```rust
enum ASTNode {
    Number(f64),                                    // Literal value
    Variable(String),                                // Variable reference
    Binary(Box<ASTNode>, String, Box<ASTNode>),    // Binary operation
    Unary(String, Box<ASTNode>),                   // Unary operation
    Assignment(String, Box<ASTNode>),              // Variable assignment
    Paren(Box<ASTNode>),                           // Parenthesized expression
}
```

### 6.3 API Request/Response Schemas

**EvalReq:**
- `expr`: String (expression to evaluate)
- `x`: Vec<f64> (input values)
- `vars`: Optional<HashMap<String, Vec<f64>>> (variable bindings)

**BisectReq:**
- `expr`: String
- `lo`: f64 (lower bracket bound)
- `hi`: f64 (upper bracket bound)
- `vars`: Optional<HashMap<String, Vec<f64>>>
- `tol`: Optional<f64> (default: 1e-9)
- `max_iter`: Optional<usize> (default: 60)

**BisectAutoReq:**
- `expr`: String
- `guess`: f64 (starting point)
- `step`: Optional<f64> (initial step size, default: 1.0)
- `max_expand`: Optional<usize> (expansion limit, default: 20)
- `vars`: Optional<HashMap<String, Vec<f64>>>
- `tol`: Optional<f64> (default: 1e-9)
- `max_iter`: Optional<usize> (default: 60)

## 7. Implementation Notes

### 7.1 Error Handling

- Division by zero: Checked using epsilon threshold (f64::EPSILON)
- Bracket validation: Verifies opposite signs at bracket endpoints
- Parse errors: Returns descriptive error strings
- Undefined variables: Returns error with variable name

### 7.2 Numerical Considerations

- Floating-point arithmetic: IEEE 754 double precision
- Tolerance: Default 1e-9 for root finding
- Bracket expansion: Geometric progression (factor of 2)
- Midpoint calculation: 0.5 * (lo + hi) prevents overflow

### 7.3 Concurrency Model

- Async/await with Tokio runtime
- Multi-threaded work-stealing scheduler
- Lock-free queues for task submission
- Message passing via mpsc channels (1024 buffer capacity)

## 8. Dependencies

Core Rust crates:
- **axum** (0.7): Web framework
- **tokio** (1.40): Async runtime
- **serde/serde_json** (1.0): Serialization
- **criterion** (0.5): Benchmarking framework
- **reqwest** (0.12): HTTP client library

## 9. Build and Deployment

### 9.1 Build Configuration

- Edition: Rust 2021
- Profile: Unoptimized + debug info (dev), optimized (release)
- Binary size: 8-10 MB (release)
- Memory footprint: 8-12 MB runtime
- Startup time: 12 milliseconds

### 9.2 Target Platforms

- x86_64-unknown-linux-gnu
- aarch64-unknown-linux-gnu
- x86_64-apple-darwin
- aarch64-apple-darwin

### 9.3 Container Deployment

Dockerfile provided for containerized deployment:
- Base image: Rust official builder
- Port exposure: 8080
- Runtime: Distroless or Alpine
- Container registry: ghcr.io/regularjoe-ceo/erock

## 10. Algorithms and Computational Complexity

### 10.1 Lexer Complexity

- Time: O(n) where n = input string length
- Space: O(t) where t = number of tokens
- Single pass with lookahead

### 10.2 Parser Complexity

- Time: O(t) where t = number of tokens
- Space: O(d) where d = expression depth (recursion stack)
- Recursive descent with bounded lookahead

### 10.3 Interpreter Complexity

- Time: O(n) where n = number of AST nodes (single evaluation)
- Space: O(v) where v = number of variables
- Tree traversal: post-order for evaluation

### 10.4 SIMD Evaluation Complexity

- Time: O(n/w) where n = array length, w = SIMD width (typically 4)
- Space: O(n) for output array
- Vectorization efficiency depends on expression complexity

### 10.5 Bisection Complexity

- Time: O(log₂((b-a)/ε)) where b-a = bracket width, ε = tolerance
- Space: O(1) constant
- Convergence rate: Linear (halves interval per iteration)

## 11. Code Organization Patterns

### 11.1 Module Structure

- **Separation of concerns**: Lexer, parser, interpreter in distinct functions
- **Type safety**: Strong typing with enums for tokens and AST nodes
- **Error propagation**: Result<T, String> pattern throughout
- **Async boundaries**: Clear separation between sync computation and async I/O

### 11.2 Design Patterns

- **Visitor pattern**: AST traversal in interpreter
- **Strategy pattern**: Precision selection in dispatcher
- **Factory pattern**: Hardware probe construction
- **Builder pattern**: Complex request deserialization

## 12. Testing Infrastructure

### 12.1 Unit Tests

Located in erock_clean/src/lib.rs:
- Operator precedence validation
- Variable substitution
- Batch variable evaluation (Vec<f64> usage)
- Assignment statement execution

Test expressions:
- "2 + 3 * 4" → 14.0 (precedence)
- "x + y" with x=5.0, y=10.0 → 15.0 (variables)
- "z = 7 + 3" → 10.0, inserts z into variable map (assignment)

### 12.2 Integration Tests

API server testing via curl or HTTP client:
- Expression evaluation with various operators
- Root finding with known functions
- Error handling (invalid expressions, bad brackets)
- Concurrent request handling

## 13. Numerical Methods Background

### 13.1 Bisection Method Theory

The bisection method is a root-finding algorithm based on the Intermediate Value Theorem:
- Given continuous function f(x) where f(a) and f(b) have opposite signs
- A root exists in interval [a, b]
- Repeatedly bisect interval and select subinterval containing sign change
- Guaranteed convergence for continuous functions with valid bracket

### 13.2 Expression Evaluation Theory

The implementation follows standard compiler theory:
- **Lexical analysis**: Regular grammar recognition
- **Syntax analysis**: Context-free grammar parsing
- **Semantic analysis**: Type checking (implicit, all f64)
- **Interpretation**: Direct AST execution (vs. compilation to bytecode/native)

### 13.3 SIMD Theory

Single Instruction Multiple Data parallelism:
- Single operation applied to vector of data elements
- Hardware support: AVX2 (256-bit, 4×f64), NEON (128-bit, 2×f64), RISC-V V (scalable)
- Requires data alignment and vectorizable operations
- Speedup theoretical maximum: SIMD width (4x for AVX2)
- Actual speedup: Limited by memory bandwidth, operation type, control flow

## 14. Future Research Directions

Potential areas for scientific investigation:

1. **JIT Compilation**: Cranelift-based JIT mentioned but not implemented; could provide 10-50x speedup
2. **Automatic Differentiation**: AST structure suitable for forward/reverse mode AD
3. **Interval Arithmetic**: Enhanced bracket finding with interval analysis
4. **GPU Acceleration**: Offload to GPU for massive parallelism
5. **Symbolic Simplification**: AST optimization before interpretation
6. **Multi-precision Arithmetic**: Arbitrary precision beyond f64
7. **Quantum Algorithm Adaptation**: Expression evaluation in quantum circuits

## 15. References

This implementation draws on standard computer science techniques:
- Aho, Sethi, Ullman: "Compilers: Principles, Techniques, and Tools" (parser design)
- Cormen, et al.: "Introduction to Algorithms" (numerical methods)
- Hennessy, Patterson: "Computer Architecture" (SIMD optimization)
- IEEE 754-2008: Floating-point arithmetic standard

## Appendix A: Expression Grammar

Informal grammar specification:
```
expression := term (('+' | '-') term)*
term       := factor (('*' | '/' | '^') factor)*
factor     := NUMBER
            | VARIABLE ('=' expression)?
            | '-' factor
            | '(' expression ')'
```

## Appendix B: API Examples

### Evaluate Expression
```bash
curl -X POST http://localhost:8080/evaluate \
  -H "Content-Type: application/json" \
  -d '{
    "expr": "y = x^2 + 2*x + 1",
    "x": [0.0, 1.0, 2.0, 3.0],
    "vars": {}
  }'
```

Response:
```json
{
  "y": [1.0, 4.0, 9.0, 16.0]
}
```

### Find Root with Bisection
```bash
curl -X POST http://localhost:8080/bisect \
  -H "Content-Type: application/json" \
  -d '{
    "expr": "x^2 - 2",
    "lo": 0.0,
    "hi": 2.0,
    "tol": 1e-9
  }'
```

Response:
```json
{
  "root": 1.414213562,
  "f": 0.0,
  "iters": 33,
  "bracket_ok": true
}
```

## Appendix C: System Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                     Client Application                   │
└───────────────────────┬─────────────────────────────────┘
                        │ HTTP/JSON
                        ▼
┌─────────────────────────────────────────────────────────┐
│              Axum HTTP Server (edge/)                    │
│  ┌─────────────┬────────────────┬──────────────────┐   │
│  │ /evaluate   │ /bisect        │ /bisect_auto     │   │
│  └─────────────┴────────────────┴──────────────────┘   │
└───────────────────────┬─────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────┐
│          Expression Evaluation Pipeline                  │
│  ┌──────────┐   ┌──────────┐   ┌──────────────┐       │
│  │  Lexer   │──▶│  Parser  │──▶│ Interpreter  │       │
│  │ (tokens) │   │  (AST)   │   │ (eval/SIMD)  │       │
│  └──────────┘   └──────────┘   └──────────────┘       │
└─────────────────────────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────┐
│              Core Runtime (src/)                         │
│  ┌────────────────────────────────────────────────┐    │
│  │  HwProbe: Architecture detection & capability  │    │
│  │  OffloadQueue: Async task management          │    │
│  │  Dispatcher: Compute operation routing        │    │
│  │  Enclave: TEE/security enforcement            │    │
│  └────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────┐
│              Hardware Layer                              │
│  ┌──────────────┬──────────────┬──────────────────┐   │
│  │ x86_64/AVX2  │ ARM64/NEON   │ RISC-V/VEXT      │   │
│  │ FMA kernels  │ ASIMD ops    │ Vector extension │   │
│  └──────────────┴──────────────┴──────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

---

**Document Version:** 1.0  
**Date:** October 28, 2025  
**Purpose:** Technical reference for scientific study and academic research
