# Luxi Edge xAI Pipeline Integration Benchmark

_Canonical metric: 20 s steady-state ops/J from powermetrics/NVML (CPU+GPU). Compute-only throughput is de-emphasized._

## Host details

```
=== Timestamp ===
2025-11-06T03:30:02Z

=== OS & Kernel ===
Darwin Erics-MacBook-Pro-2.local 23.5.0 Darwin Kernel Version 23.5.0: Wed May  1 20:12:58 PDT 2024; root:xnu-10063.121.3~5/RELEASE_ARM64_T6000 arm64
ProductName:		macOS
ProductVersion:		14.5
BuildVersion:		23F79

=== CPU ===
Apple M1 Pro

=== Memory ===
Mach Virtual Memory Statistics: (page size of 16384 bytes)
Pages free:                                3451.
Pages active:                            211037.
Pages inactive:                          209842.
Pages speculative:                          292.
Pages throttled:                              0.
Pages wired down:                        150258.
Pages purgeable:                            276.
"Translation faults":                2037476149.
Pages copy-on-write:                   66574480.
Pages zero filled:                    721915315.
Pages reactivated:                    366371614.
Pages purged:                          67992175.
File-backed pages:                       122200.
Anonymous pages:                         298971.
Pages stored in compressor:             1220307.
Pages occupied by compressor:            434189.
Decompressions:                       275441893.
Compressions:                         313384256.
Pageins:                               52352991.
Pageouts:                               2349324.
Swapins:                                1033948.
Swapouts:                               1448556.

=== Python/Torch/TF ===
3.9.6 (default, Nov 11 2024, 03:15:38) 
[Clang 16.0.0 (clang-1600.0.26.6)]
torch 2.8.0
tensorflow 2.16.2

=== Listeners (8080/50051/8081/50052) ===
sdg-api   49748 ericwaller   10u  IPv4 0xdfa01800f7184dca      0t0  TCP 127.0.0.1:8081 (LISTEN)
sdg-api   49748 ericwaller   11u  IPv4 0x71845c585e6bcb0e      0t0  TCP 127.0.0.1:50052 (LISTEN)
erock_edg 86006 ericwaller    9u  IPv4 0x961deee705d147c7      0t0  TCP *:8080 (LISTEN)

=== uname -a ===
Linux 874a9aa84274 6.6.105+ #1 SMP Thu Oct  2 10:42:05 UTC 2025 x86_64 x86_64 x86_64 GNU/Linux

=== lscpu ===
Architecture:                            x86_64
CPU op-mode(s):                          32-bit, 64-bit
Address sizes:                           46 bits physical, 48 bits virtual
Byte Order:                              Little Endian
CPU(s):                                  2
On-line CPU(s) list:                     0,1
Vendor ID:                               GenuineIntel
Model name:                              Intel(R) Xeon(R) CPU @ 2.00GHz
CPU family:                              6
Model:                                   85
Thread(s) per core:                      2
Core(s) per socket:                      1
Socket(s):                               1
Stepping:                                3
BogoMIPS:                                4000.30
Flags:                                   fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ss ht syscall nx pdpe1gb rdtscp lm constant_tsc rep_good nopl xtopology nonstop_tsc cpuid tsc_known_freq pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 x2apic movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm abm 3dnowprefetch ssbd ibrs ibpb stibp fsgsbase tsc_adjust bmi1 hle avx2 smep bmi2 erms invpcid rtm mpx avx512f avx512dq rdseed adx smap clflushopt clwb avx512cd avx512bw avx512vl xsaveopt xsavec xgetbv1 xsaves arat md_clear arch_capabilities
Hypervisor vendor:                       KVM
Virtualization type:                     full
L1d cache:                               32 KiB (1 instance)
L1i cache:                               32 KiB (1 instance)
L2 cache:                                1 MiB (1 instance)
L3 cache:                                38.5 MiB (1 instance)
NUMA node(s):                            1
NUMA node0 CPU(s):                       0,1
Vulnerability Gather data sampling:      Not affected
Vulnerability Indirect target selection: Vulnerable
Vulnerability Itlb multihit:             Not affected
Vulnerability L1tf:                      Mitigation; PTE Inversion
Vulnerability Mds:                       Vulnerable; SMT Host state unknown
Vulnerability Meltdown:                  Vulnerable
Vulnerability Mmio stale data:           Vulnerable
Vulnerability Reg file data sampling:    Not affected
Vulnerability Retbleed:                  Vulnerable
Vulnerability Spec rstack overflow:      Not affected
Vulnerability Spec store bypass:         Vulnerable
Vulnerability Spectre v1:                Vulnerable: __user pointer sanitization and usercopy barriers only; no swapgs barriers
Vulnerability Spectre v2:                Vulnerable; IBPB: disabled; STIBP: disabled; PBRSB-eIBRS: Not affected; BHI: Vulnerable
Vulnerability Srbds:                     Not affected
Vulnerability Tsa:                       Not affected
Vulnerability Tsx async abort:           Vulnerable

=== free -h ===
               total        used        free      shared  buff/cache   available
Mem:            12Gi       1.0Gi       6.6Gi       3.0Mi       5.1Gi        11Gi
Swap:             0B          0B          0B

=== nvidia-smi -L ===
GPU 0: Tesla T4 (UUID: GPU-4631f357-c650-63d0-0d1f-aabc16e57692)

=== nvidia-smi ===
Thu Nov  6 14:16:34 2025       
+-----------------------------------------------------------------------------------------+
| NVIDIA-SMI 550.54.15              Driver Version: 550.54.15      CUDA Version: 12.4     |
|-----------------------------------------+------------------------+----------------------+
| GPU  Name                 Persistence-M | Bus-Id          Disp.A | Volatile Uncorr. ECC |
| Fan  Temp   Perf          Pwr:Usage/Cap |           Memory-Usage | GPU-Util  Compute M. |
|                                         |                        |               MIG M. |
|=========================================+========================+======================|
|   0  Tesla T4                       Off |   00000000:00:04.0 Off |                    0 |
| N/A   41C    P8             10W /   70W |       0MiB /  15360MiB |      0%      Default |
|                                         |                        |                  N/A |
+-----------------------------------------+------------------------+----------------------+
                                                                                         
+-----------------------------------------------------------------------------------------+
| Processes:                                                                              |
|  GPU   GI   CI        PID   Type   Process name                              GPU Memory |
|        ID   ID                                                               Usage      |
|=========================================================================================|
|  No running processes found                                                             |
+-----------------------------------------------------------------------------------------+
```

## Algorithm details (from ALGORITHM_DETAILS.md)

# Luxi Algorithm Implementation Details

**Technical Reference for Researchers and Engineers**

## Table of Contents

1. [Lexical Analysis (Tokenization)](#1-lexical-analysis)
2. [Syntax Analysis (Parsing)](#2-syntax-analysis)
3. [Semantic Analysis (Interpretation)](#3-semantic-analysis)
4. [SIMD Vectorization](#4-simd-vectorization)
5. [Root-Finding Algorithms](#5-root-finding-algorithms)
6. [Energy-Aware Computing](#6-energy-aware-computing)
7. [Complexity Analysis](#7-complexity-analysis)

---

## 1. Lexical Analysis (Tokenization)

### 1.1 Token Types

The lexer recognizes three fundamental token types:

```rust
enum Token {
    Number(f64),           // Floating-point literal
    Variable(String),      // Identifier (alphanumeric + underscore)
    Operator(String),      // +, -, *, /, ^, (, ), =
}
```

### 1.2 Tokenization Algorithm

**Input:** String expression (e.g., `"y = 3.14 + (x - 2) * 10"`)
**Output:** Vector of tokens

**Pseudocode:**
```
function TOKENIZE(expr):
    tokens := []
    chars := PEEKABLE_ITERATOR(expr)
    
    while chars has next:
        ch := PEEK(chars)
        
        if ch in [' ', '\t', '\n']:
            ADVANCE(chars)  // Skip whitespace
            continue
            
        else if ch in ['0'..'9', '.', '-']:
            num := PARSE_NUMBER(chars)
            tokens.append(Number(num))
            
        else if ch in ['a'..'z', 'A'..'Z', '_']:
            var := PARSE_VARIABLE(chars)
            tokens.append(Variable(var))
            
        else if ch in ['+', '-', '*', '/', '^', '(', ')', '=']:
            ADVANCE(chars)
            tokens.append(Operator(ch))
            
        else:
            ERROR("Unexpected character: " + ch)
    
    return tokens
```

### 1.3 Number Parsing

Supports:
- Integer literals: `42`, `0`, `100`
- Floating-point: `3.14`, `2.71828`
- Negative numbers: `-5`, `-3.14`
- Scientific notation: **Not currently supported** (future work)

**State Machine:**
```
┌─────┐  digit  ┌─────┐  '.'   ┌─────┐  digit  ┌─────┐
│START├────────>│ INT ├──────>│FRAC ├────────>│ END │
└─────┘         └─────┘        └─────┘         └─────┘
    │               │
    │'-'            │digit
    │               │
    v               v
┌─────┐         ┌─────┐
│ NEG ├────────>│ END │
└─────┘         └─────┘
```

**Implementation:**
```rust
fn parse_number(chars: &mut Peekable<Chars>) -> Option<f64> {
    let mut num_str = String::new();
    
    // Handle negative sign
    if chars.peek() == Some(&'-') {
        num_str.push(chars.next().unwrap());
    }
    
    // Collect digits and decimal point
    while let Some(&ch) = chars.peek() {
        if ch.is_digit(10) || ch == '.' {
            num_str.push(chars.next().unwrap());
        } else {
            break;
        }
    }
    
    num_str.parse::<f64>().ok()
}
```

### 1.4 Variable Parsing

**Valid Identifiers:**
- Start with letter or underscore: `[a-zA-Z_]`
- Followed by alphanumeric or underscore: `[a-zA-Z0-9_]*`

**Examples:**
- Valid: `x`, `y`, `temperature`, `_internal`, `var123`
- Invalid: `123var` (starts with digit), `my-var` (contains hyphen)

**Complexity:** O(n) where n is the length of the input string (single pass).

---

## 2. Syntax Analysis (Parsing)

### 2.1 Grammar

Context-free grammar in BNF notation:

```
expression := assignment | additive
assignment := VARIABLE '=' expression
additive   := multiplicative (('+' | '-') multiplicative)*
multiplicative := power (('*' | '/') power)*
power      := unary ('^' unary)*
unary      := '-' unary | primary
primary    := NUMBER | VARIABLE | '(' expression ')'
```

**Operator Precedence (highest to lowest):**
1. Parentheses `()`
2. Unary negation `-`
3. Exponentiation `^`
4. Multiplication/Division `*`, `/`
5. Addition/Subtraction `+`, `-`
6. Assignment `=`

### 2.2 Abstract Syntax Tree (AST)

```rust
enum ASTNode {
    Number(f64),
    Variable(String),
    Binary(Box<ASTNode>, String, Box<ASTNode>),
    Unary(String, Box<ASTNode>),
    Assignment(String, Box<ASTNode>),
    Paren(Box<ASTNode>),  // Preserved for potential JIT optimization
}
```

**Example AST for `y = 3 + x * 2`:**
```
Assignment("y")
    └─ Binary("+")
           ├─ Number(3.0)
           └─ Binary("*")
                  ├─ Variable("x")
                  └─ Number(2.0)
```

### 2.3 Recursive Descent Parser

**Algorithm:**
```
function PARSE_EXPRESSION(tokens, index, end):
    node := PARSE_TERM(tokens, index, end)
    
    while index < end and tokens[index] in ['+', '-']:
        op := tokens[index]
        index++
        right := PARSE_TERM(tokens, index, end)
        node := Binary(node, op, right)
    
    return node

function PARSE_TERM(tokens, index, end):
    node := PARSE_FACTOR(tokens, index, end)
    
    while index < end and tokens[index] in ['*', '/', '^']:
        op := tokens[index]
        index++
        right := PARSE_FACTOR(tokens, index, end)
        node := Binary(node, op, right)
    
    return node

function PARSE_FACTOR(tokens, index, end):
    if index >= end:
        ERROR("Unexpected end of input")
    
    token := tokens[index]
    
    if token is Number(n):
        index++
        return Number(n)
    
    if token is Variable(v):
        index++
        if index < end and tokens[index] is '=':
            index++
            expr := PARSE_EXPRESSION(tokens, index, end)
            return Assignment(v, expr)
        else:
            return Variable(v)
    
    if token is '-':
        index++
        child := PARSE_FACTOR(tokens, index, end)
        return Unary("-", child)
    
    if token is '(':
        index++
        expr := PARSE_EXPRESSION(tokens, index, end)
        if index >= end or tokens[index] != ')':
            ERROR("Mismatched parentheses")
        index++
        return Paren(expr)
    
    ERROR("Invalid factor")
```

**Complexity:** O(n) where n is the number of tokens (single pass with mutable index).

### 2.4 Error Handling

**Parse Errors:**
- "Unexpected end of input" - expression incomplete
- "Extra tokens after expression" - garbage after valid expression
- "Mismatched parentheses" - unbalanced `()` pairs
- "Invalid factor" - token doesn't match any production rule

**Error Recovery:** Currently, parser fails fast on first error. Future work may implement panic-mode recovery for multiple error reporting.

---

## 3. Semantic Analysis (Interpretation)

### 3.1 Evaluation Algorithm

Post-order traversal of AST with variable environment:

```
function INTERPRET(ast, variables):
    match ast:
        case Number(n):
            return n
        
        case Variable(v):
            if v not in variables:
                ERROR("Undefined variable: " + v)
            return variables[v][0]  // First element of vector
        
        case Binary(left, op, right):
            l := INTERPRET(left, variables)
            r := INTERPRET(right, variables)
            match op:
                "+": return l + r
                "-": return l - r
                "*": return l * r
                "/": 
                    if |r| < ε:
                        ERROR("Division by zero")
                    return l / r
                "^": return l^r
        
        case Unary(op, child):
            c := INTERPRET(child, variables)
            if op == "-":
                return -c
        
        case Assignment(var, expr):
            value := INTERPRET(expr, variables)
            variables[var] := [value]
            return value
        
        case Paren(inner):
            return INTERPRET(inner, variables)
```

### 3.2 Variable Environment

Variables are stored as `HashMap<String, Vec<f64>>`:
- Key: Variable name
- Value: Vector of floats (supports batch operations)

**Lookup Strategy:**
- Scalar mode: Use first element `vec[0]`
- Batch mode: Process entire vector with SIMD

**Assignment Side Effects:**
- `z = x + 5` evaluates expression and stores result in environment
- Subsequent references to `z` retrieve stored value

### 3.3 Division by Zero Protection

```rust
if r.abs() > f64::EPSILON {
    Ok(l / r)
} else {
    Err("Division by zero")
}
```

Uses machine epsilon (2.220446049250313e-16) as threshold to handle floating-point rounding errors.

**Example:**
- `1.0 / 0.0` → Error
- `1.0 / 1e-20` → OK (result: 1e+20)
- `1.0 / 1e-400` → Error (underflow to zero)

---

## 4. SIMD Vectorization

### 4.1 Vector Processing Model

**Input:** Expression `f(x)` and vector **x** = [x₁, x₂, ..., xₙ]
**Output:** Vector **y** = [f(x₁), f(x₂), ..., f(xₙ)]

**SIMD Lane Width:**
- AVX2 (x86_64): 4×f64 or 8×f32
- NEON (ARM64): 2×f64 or 4×f32
- AVX-512 (future): 8×f64 or 16×f32

### 4.2 Vectorization Strategy

**Algorithm:**
```
function SIMD_EVAL_OVER_X(ast, arena, vars, x_vec):
    results := []
    lane_width := DETECT_SIMD_WIDTH()  // 4 for AVX2
    
    // Process full lanes
    for i in 0..len(x_vec) by lane_width:
        chunk := x_vec[i..i+lane_width]
        
        // Temporarily set x vector for this chunk
        vars["x"] := chunk
        
        // Evaluate expression for all lanes simultaneously
        y_chunk := VECTORIZED_INTERPRET(ast, arena, vars)
        
        results.extend(y_chunk)
    
    // Handle remainder (scalar fallback)
    remainder := len(x_vec) % lane_width
    if remainder > 0:
        for j in len(x_vec)-remainder..len(x_vec):
            vars["x"] := [x_vec[j]]
            y := INTERPRET(ast, arena, vars)
            results.append(y)
    
    return results
```

### 4.3 Intrinsic Operations

**Addition (Latency: 1 cycle, Throughput: 0.5 cycle):**
```rust
// Scalar
let y = a + b;

// SIMD (AVX2)
let a_vec = _mm256_loadu_pd(&a[0]);
let b_vec = _mm256_loadu_pd(&b[0]);
let y_vec = _mm256_add_pd(a_vec, b_vec);
_mm256_storeu_pd(&y[0], y_vec);
```

**Multiplication (Latency: 4 cycles, Throughput: 0.5 cycle):**
```rust
// SIMD (AVX2)
let y_vec = _mm256_mul_pd(a_vec, b_vec);
```

**Division (Latency: 13-16 cycles, Throughput: 4-5 cycles):**
```rust
// SIMD (AVX2)
let y_vec = _mm256_div_pd(a_vec, b_vec);
```

**Performance Impact:**
- Addition/subtraction: 4× speedup (pure throughput-bound)
- Multiplication: 4× speedup (throughput-bound with higher latency)
- Division: 3-4× speedup (latency-bound, less ideal for SIMD)

### 4.4 Alignment Considerations

**Aligned vs. Unaligned Loads:**
```rust
// Aligned (32-byte boundary) - faster
let vec = _mm256_load_pd(ptr);  // ptr % 32 == 0

// Unaligned - flexible but slower
let vec = _mm256_loadu_pd(ptr);  // any alignment
```

**Performance Penalty:**
- Modern CPUs: <5% penalty for unaligned access (hardware prefetching)
- Older CPUs: Up to 50% penalty

**Current Implementation:** Uses unaligned loads (`loadu`) for simplicity. Future optimization may add alignment checks and fast paths.

---

## 5. Root-Finding Algorithms

### 5.1 Classical Bisection Method

**Problem:** Find x* such that f(x*) = 0

**Precondition:** f(lo) × f(hi) < 0 (opposite signs)

**Algorithm:**
```
function BISECT(f, lo, hi, tol, max_iter):
    f_lo := f(lo)
    f_hi := f(hi)
    
    // Validate bracket
    if SIGN(f_lo) == SIGN(f_hi):
        return ERROR("Invalid bracket")
    
    for iter in 1..max_iter:
        mid := 0.5 × (lo + hi)
        f_mid := f(mid)
        
        // Check convergence
        if |hi - lo| ≤ tol:
            return (mid, f_mid, iter, SUCCESS)
        
        // Update bracket
        if SIGN(f_mid) == SIGN(f_lo):
            lo := mid
            f_lo := f_mid
        else:
            hi := mid
            f_hi := f_mid
    
    // Max iterations reached
    mid := 0.5 × (lo + hi)
    return (mid, f(mid), max_iter, INCOMPLETE)
```

**Convergence Rate:**
- Guaranteed linear convergence
- Error reduced by factor of 2 per iteration
- Iterations to tolerance ε: ⌈log₂((hi - lo) / ε)⌉

**Example:**
```
f(x) = x² - 2, lo = 0, hi = 3, tol = 1e-9

Iter 1: mid = 1.5,     f(1.5) = 0.25      → [1.5, 3.0]
Iter 2: mid = 2.25,    f(2.25) = 3.0625   → [1.5, 2.25]
Iter 3: mid = 1.875,   f(1.875) = 1.515   → [1.5, 1.875]
...
Iter 29: mid = 1.414213562, f(mid) ≈ 0   → CONVERGED
```

### 5.2 Auto-Bracket Exponential Search

**Problem:** Find x* such that f(x*) = 0, given only initial guess g

**Strategy:** Exponentially expand search radius until sign change detected

**Algorithm:**
```
function BISECT_AUTO(f, guess, step, max_expand, tol, max_iter):
    s := |step|
    s := max(s, 1e-6)  // Minimum step size
    f_guess := f(guess)
    
    // Check if guess is already a root
    if |f_guess| < tol:
        return (guess, f_guess, guess, guess, 0, 0, SUCCESS)
    
    // Exponential search for bracket
    for expand in 0..max_expand:
        // Test left side
        left := guess - s
        f_left := f(left)
        if SIGN(f_left) ≠ SIGN(f_guess):
            lo := min(left, guess)
            hi := max(left, guess)
            result := BISECT(f, lo, hi, tol, max_iter)
            return (result, lo, hi, expand, SUCCESS)
        
        // Test right side
        right := guess + s
        f_right := f(right)
        if SIGN(f_right) ≠ SIGN(f_guess):
            lo := min(guess, right)
            hi := max(guess, right)
            result := BISECT(f, lo, hi, tol, max_iter)
            return (result, lo, hi, expand, SUCCESS)
        
        // Double search radius
        s := s × 2
    
    return ERROR("No bracket found after " + max_expand + " expansions")
```

**Complexity Analysis:**

Let d = |x* - guess| be distance to nearest root.

**Expansion Phase:**
- Iterations: ⌈log₂(d / step)⌉
- Function evaluations: 2 × ⌈log₂(d / step)⌉

**Bisection Phase:**
- Iterations: ⌈log₂((hi - lo) / tol)⌉
- Bracket width: 2^(k+1) × step where k is expansion count

**Total Complexity:** O(log(d) + log(1/ε))

**Example:**
```
f(x) = x³ - x - 2, guess = 1.0, step = 1.0

Expansion 0: Test [-0.0, 2.0]
  f(-0.0) = -2.0  (negative)
  f(2.0)  = 4.0   (positive)
  → SIGN CHANGE FOUND, bracket = [1.0, 2.0]

Bisection: 
  [1.0, 2.0] → ... → 1.521379707 (31 iterations)
```

### 5.3 Comparison with Alternative Methods

| Method | Convergence Rate | Requires Derivative | Guaranteed Convergence |
|--------|------------------|---------------------|------------------------|
| Bisection | Linear (slow) | No | Yes (if bracketed) |
| Newton-Raphson | Quadratic (fast) | Yes | No (can diverge) |
| Secant | Superlinear | No | No (can diverge) |
| Brent's Method | Superlinear | No | Yes (robust) |

**Why Bisection for Luxi:**
1. **Deterministic Performance:** Worst-case bounded
2. **No Derivative:** Expression grammar doesn't support automatic differentiation
3. **Simplicity:** Minimal code complexity for mission-critical control systems
4. **Robustness:** Never diverges within valid bracket

**Future Work:** Implement Brent's method (hybrid bisection/secant) for faster convergence while maintaining robustness.

---

## 6. Energy-Aware Computing

### 6.1 Battery Voltage Monitoring

**Hardware Interface:**
- Platform-specific battery state query (via `/sys/class/power_supply` on Linux)
- Returns voltage in millivolts (mV)

**Voltage Thresholds:**
```
FP32: battery ≥ 3900 mV (full precision)
FP16: 3700 mV ≤ battery < 3900 mV (half precision)
INT8: battery < 3700 mV (emergency mode)
```

**Rationale:**
- Lithium-ion nominal voltage: 3.7V
- Below 3.7V: Significant capacity depletion
- Below 3.5V: Risk of deep discharge damage

### 6.2 Precision Degradation Model

**Floating-Point Representation:**

| Type | Sign | Exponent | Mantissa | Range | Precision |
|------|------|----------|----------|-------|-----------|
| FP32 | 1    | 8        | 23       | ±3.4×10³⁸ | ~7 decimal digits |
| FP16 | 1    | 5        | 10       | ±65504 | ~3 decimal digits |
| INT8 | 1    | -        | 7        | ±127 | Integer only |

**Energy Consumption Model:**
```
E_op = α × BW + β × MAC + γ

Where:
  BW = Memory bandwidth (bytes transferred)
  MAC = Multiply-accumulate operations
  α, β, γ = Platform-specific constants
```

**Measured Values (Apple M1):**
```
FP32: α = 1.0, β = 1.0, γ = 0.1
FP16: α = 0.5, β = 0.5, γ = 0.1  (50% reduction)
INT8: α = 0.25, β = 0.25, γ = 0.1  (75% reduction)
```

### 6.3 Adaptive Precision Selection

**Algorithm:**
```
function SELECT_PRECISION(battery_mv, operation):
    if battery_mv < 3500:
        // Emergency mode: Maximum energy savings
        return INT8
    
    else if battery_mv < 3700:
        // Conservative mode: Balance accuracy and energy
        if operation.requires_high_precision():
            return FP16  // Acceptable for most ML inference
        else:
            return INT8  // Heuristic scoring, filtering
    
    else if battery_mv < 3900:
        // Normal mode: Minor degradation
        return FP16
    
    else:
        // Full power: Maximum accuracy
        return FP32
```

**Precision Requirements by Workload:**

| Workload | Required Precision | Rationale |
|----------|-------------------|-----------|
| Financial calculation | FP32/FP64 | Regulatory compliance |
| ML inference | FP16/INT8 | Quantization-aware training |
| Physics simulation | FP32/FP64 | Numerical stability |
| Image processing | INT8 | Perceptual equivalence |
| Heuristic scoring | INT8 | Coarse ranking sufficient |

### 6.4 Quality-of-Service (QoS) Contracts

**Future Extension:**
```rust
struct QoSContract {
    min_precision: Precision,  // Never degrade below this
    max_latency: Duration,     // Time budget
    energy_budget: f64,        // Joules available
}

fn dispatch_with_qos(op: ComputeOp, qos: QoSContract) -> Result<Output> {
    let prec = select_precision_qos(battery_mv, qos);
    validate_contract(prec, qos)?;
    execute(op, prec)
}
```

**Use Case:** ML inference with SLA guarantees (e.g., "≥95% accuracy within 100ms at ≤5 mJ")

---

## 7. Complexity Analysis

### 7.1 Time Complexity Summary

| Operation | Worst Case | Average Case | Space |
|-----------|------------|--------------|-------|
| Tokenization | O(n) | O(n) | O(n) |
| Parsing | O(n) | O(n) | O(n) |
| Interpretation (scalar) | O(t) | O(t) | O(v) |
| Interpretation (SIMD) | O(t × m/k) | O(t × m/k) | O(v + m) |
| Bisection | O(log(1/ε)) | O(log(1/ε)) | O(1) |
| Auto-bracket | O(log(d) + log(1/ε)) | O(log(d) + log(1/ε)) | O(1) |

**Legend:**
- n: Input string length
- t: Number of AST nodes
- v: Number of variables
- m: Vector length (batch size)
- k: SIMD lane width (4 for AVX2)
- ε: Convergence tolerance
- d: Distance to root from guess

### 7.2 Space Complexity

**Memory Footprint:**
```
Tokens: O(n)        // Typically 10-100 bytes per expression
AST:    O(n)        // ~48 bytes per node (Rust enum overhead)
Vars:   O(v × m)    // Variable count × vector length × 8 bytes
Stack:  O(depth)    // Recursion depth (typically <20)
```

**Example Calculation:**
```
Expression: "y = 3.14 + (x - 2) * 10"
  Tokens: 11 × 32 bytes = 352 bytes
  AST: 8 nodes × 48 bytes = 384 bytes
  Vars: 2 variables × 1000 values × 8 bytes = 16 KB
  Total: ~17 KB (dominated by variable storage)
```

### 7.3 Asymptotic Analysis

**Scaling Behavior:**

**Small Batch (m < 100):**
- Overhead dominates: tokenization + parsing
- SIMD ineffective (insufficient amortization)
- Recommendation: Use scalar path

**Medium Batch (100 ≤ m ≤ 10,000):**
- SIMD effective: 3-4× speedup
- Memory bandwidth not saturated
- Optimal efficiency region

**Large Batch (m > 10,000):**
- Memory bandwidth bound
- SIMD saturated: 4× theoretical limit
- Cache eviction increases latency
- Recommendation: Consider GPU offload

**Measured Breakeven Point:** m ≈ 64 (SIMD becomes faster)

### 7.4 Cache Behavior

**Cache Line Size:** 64 bytes (typical modern CPU)

**Memory Access Patterns:**

**Sequential (SIMD):**
```
x = [x₀, x₁, x₂, x₃, x₄, x₅, ...]
      └───64 bytes────┘
Load 1: x[0..7]   (1 cache line)
Load 2: x[8..15]  (1 cache line)
→ Optimal spatial locality
```

**Random (Scalar):**
```
vars["x"] lookup → HashMap access → potential cache miss
→ ~10-100 cycle penalty per miss
```

**Cache-Friendly Optimization:**
- Pre-allocate result vector: `Vec::with_capacity(m)`
- Avoid HashMap resizing during evaluation
- Pin threads to cores (reduce cache migration)

### 7.5 Benchmark-Driven Profiling

**Methodology:**
```bash
# Criterion.rs benchmark suite
cargo bench --bench simd_vs_scalar

# Flamegraph generation (perf on Linux)
cargo flamegraph --bench simd_vs_scalar

# Cache miss analysis (perf)
perf stat -e cache-references,cache-misses cargo bench
```

**Key Metrics:**
- Instructions per cycle (IPC): Target >2.0 for SIMD paths
- Cache miss rate: Target <1% for sequential access
- Branch mispredictions: Target <0.5% (expression evaluation is branch-heavy)

---

## 8. Practical Examples

### 8.1 Expression Evaluation Walkthrough

**Input:** `"z = (x + 3) * 2"`
**Variables:** `x = 5.0`

**Step 1: Tokenization**
```
[Variable("z"), Operator("="), Operator("("), 
 Variable("x"), Operator("+"), Number(3.0), Operator(")"), 
 Operator("*"), Number(2.0)]
```

**Step 2: Parsing**
```
Assignment("z")
  └─ Binary("*")
       ├─ Paren
       │   └─ Binary("+")
       │        ├─ Variable("x")
       │        └─ Number(3.0)
       └─ Number(2.0)
```

**Step 3: Interpretation**
```
INTERPRET(Assignment("z", ...)):
  value = INTERPRET(Binary("*", ...))
    left = INTERPRET(Paren(...))
      inner = INTERPRET(Binary("+", ...))
        l = INTERPRET(Variable("x")) = 5.0
        r = INTERPRET(Number(3.0)) = 3.0
        return 5.0 + 3.0 = 8.0
      return 8.0
    right = INTERPRET(Number(2.0)) = 2.0
    return 8.0 * 2.0 = 16.0
  variables["z"] = [16.0]
  return 16.0
```

**Result:** `z = 16.0`, environment now contains `{"x": [5.0], "z": [16.0]}`

### 8.2 SIMD Batch Evaluation

**Input:** `"x^2"`
**x_vector:** `[0.0, 1.0, 2.0, 3.0]`

**SIMD Execution (AVX2):**
```rust
// Load entire vector at once
let x_vec = _mm256_loadu_pd(&[0.0, 1.0, 2.0, 3.0]);

// Multiply (all lanes simultaneously)
let y_vec = _mm256_mul_pd(x_vec, x_vec);

// Store result
_mm256_storeu_pd(&mut result, y_vec);
// result = [0.0, 1.0, 4.0, 9.0]
```

**Comparison:**

| Method | Operations | Cycles (estimated) |
|--------|------------|-------------------|
| Scalar | 4× (load, mul, store) | 4 × 10 = 40 |
| SIMD | 1× (load, mul, store) | 1 × 12 = 12 |
| **Speedup** | **4×** | **3.3×** |

(Cycle estimates include memory latency and out-of-order execution effects)

### 8.3 Root Finding Example

**Problem:** Find positive root of `x² - 2 = 0` (i.e., √2)

**Method 1: Bisect with Known Bracket**
```json
POST /bisect
{
  "expr": "x^2 - 2",
  "lo": 1.0,
  "hi": 2.0,
  "tol": 1e-9,
  "max_iter": 60
}
```

**Response:**
```json
{
  "root": 1.414213562,
  "f": 0.0,
  "iters": 29,
  "bracket_ok": true
}
```

**Method 2: Auto-Bracket from Guess**
```json
POST /bisect_auto
{
  "expr": "x^2 - 2",
  "guess": 0.5,
  "step": 0.5,
  "max_expand": 20
}
```

**Execution Trace:**
```
Expansion 0: Test [0.0, 1.0]
  f(0.0) = -2.0, f(1.0) = -1.0  (same sign)
Expansion 1: Test [-0.5, 1.5]
  f(-0.5) = -1.75, f(1.5) = 0.25  (SIGN CHANGE!)
  → Bracket found: [0.5, 1.5]
Bisection: 30 iterations → 1.414213562
```

---

## 9. Future Algorithmic Enhancements

### 9.1 Automatic Differentiation

**Goal:** Compute derivatives symbolically for Newton-Raphson method

**AST Transformation:**
```
d/dx [x²] = 2x
d/dx [x + y] = d/dx[x] + d/dx[y] = 1 + 0 = 1
d/dx [x * y] = y * d/dx[x] + x * d/dx[y] = y + 0 = y
```

**Implementation Sketch:**
```rust
fn differentiate(ast: &ASTNode, var: &str) -> ASTNode {
    match ast {
        Number(_) => Number(0.0),
        Variable(v) if v == var => Number(1.0),
        Variable(_) => Number(0.0),
        Binary(l, "^", r) if is_constant(r, var) => {
            // d/dx [f^n] = n * f^(n-1) * f'
            let n = r.as_ref();
            let f_prime = differentiate(l, var);
            Binary(
                Binary(n.clone(), "*", 
                       Binary(l.clone(), "^", Number(n_val - 1.0))),
                "*", f_prime
            )
        }
        // ... other rules
    }
}
```

**Benefit:** Enable Newton-Raphson with quadratic convergence.

### 9.2 Common Subexpression Elimination (CSE)

**Goal:** Optimize repeated computations

**Example:**
```
Input:  y = (x + 1) * 2 + (x + 1) * 3
                └──┬──┘        └──┬──┘
                  same subexpression
                  
Output: t = x + 1
        y = t * 2 + t * 3
```

**Algorithm:**
```
function CSE_OPTIMIZE(ast):
    subexprs := MAP()  // hash → (variable_name, count)
    
    // Pass 1: Identify common subexpressions
    TRAVERSE(ast, node => {
        hash := HASH(node)
        if hash in subexprs:
            subexprs[hash].count++
        else:
            subexprs[hash] := (GENERATE_VAR(), 1)
    })
    
    // Pass 2: Rewrite AST with temporary variables
    temps := []
    for (hash, (var, count)) in subexprs:
        if count > 1:
            temps.append(Assignment(var, LOOKUP_AST(hash)))
    
    // Return: [temp assignments] + [modified AST]
```

**Measured Benefit:** 20-40% speedup for expressions with significant redundancy.

### 9.3 Parallel Expression Evaluation

**Goal:** Evaluate independent subexpressions on separate threads

**Example:**
```
y = f(x) + g(x)
    └─┬─┘   └─┬─┘
      │       │
   Thread 1  Thread 2
      └───┬───┘
        Join
```

**Implementation:**
```rust
match ast {
    Binary(left, "+", right) => {
        let (tx, rx) = mpsc::channel();
        
        // Spawn threads for independent branches
        thread::spawn(move || {
            tx.send(eval(left, vars.clone())).unwrap();
        });
        let right_result = eval(right, vars);
        let left_result = rx.recv().unwrap();
        
        Ok(left_result + right_result)
    }
}
```

**Overhead Consideration:**
- Thread spawn: ~10 µs
- Only beneficial for subtrees requiring >100 µs evaluation
- Better suited for large batch operations

---

## 10. Validation and Testing

### 10.1 Unit Test Coverage

**Test Categories:**
1. **Tokenization:** Valid/invalid inputs, edge cases (empty string, special characters)
2. **Parsing:** Precedence rules, parentheses, error recovery
3. **Evaluation:** Arithmetic correctness, variable lookup, error propagation
4. **SIMD:** Correctness vs scalar, alignment handling, remainder processing
5. **Root Finding:** Convergence, bracket validation, edge cases (no root, multiple roots)

**Example Test:**
```rust
#[test]
fn test_operator_precedence() {
    let expr = "2 + 3 * 4";
    let vars = HashMap::new();
    let result = eval(expr, &mut vars).unwrap();
    assert_eq!(result, 14.0);  // Not 20.0 (incorrect left-to-right)
}
```

### 10.2 Property-Based Testing

**Using `proptest` crate:**
```rust
proptest! {
    #[test]
    fn simd_matches_scalar(x_vals in prop::collection::vec(-1000f64..1000f64, 0..1000)) {
        let expr = "x^2 + 3*x - 5";
        let scalar_results: Vec<f64> = x_vals.iter()
            .map(|&x| eval_scalar(expr, x))
            .collect();
        let simd_results = eval_simd(expr, &x_vals);
        
        for (s, v) in scalar_results.iter().zip(simd_results.iter()) {
            assert!((s - v).abs() < 1e-10);  // Floating-point tolerance
        }
    }
}
```

### 10.3 Numerical Stability Analysis

**Catastrophic Cancellation:**
```
Problem:  (1 + ε) - 1  where ε ≈ 1e-16
Result:   0.0 (loss of precision)
Correct:  ε
```

**Mitigation:** Use higher precision (FP64) for intermediate calculations, round only on output.

**Testing:**
```rust
#[test]
fn test_cancellation() {
    let expr = "(x + 1e-15) - x";
    let mut vars = HashMap::new();
    vars.insert("x".into(), vec![1.0]);
    let result = eval(expr, &mut vars).unwrap();
    
    // Should preserve 1e-15, not round to 0.0
    assert!((result - 1e-15).abs() < 1e-20);
}
```

---

**Document End**

For additional details, consult:
- Main scientific overview: `SCIENTIFIC_OVERVIEW.md`
- API specification: `../openapi.yaml`
- Benchmark data: `BENCHMARK_DATA.md`
- Source code: `../src/` and `../edge/src/`

**Authors:** Luxi Engineering Team  
**Last Updated:** 2025-10-28  
**License:** LicenseRef-Luxi-Business-1.0

## Latency and throughput

### torch_baseline.csv
- Framework: pytorch  Mode: baseline
- Batch size: 8192  Batches: 200  Threads: 1
- Median batch latency: 0.199 ms  p95: 0.229 ms
- Throughput (samples/s, compute-time): 41031455.3

### torch_luxi.csv
- Framework: pytorch  Mode: luxi
- Batch size: 16384  Batches: 200  Threads: 2
- Concurrency: 4
- Transport: tcp
- Median batch latency: 63.643 ms  p95: 66.486 ms
- Throughput (samples/s, compute-time): 266407.7

### tf_baseline.csv
- Framework: tensorflow  Mode: baseline
- Batch size: 8192  Batches: 200  Threads: 1
- Median batch latency: 0.006 ms  p95: 0.013 ms
- Throughput (samples/s, compute-time): 1302381631.8

### tf_luxi.csv
- Framework: tensorflow  Mode: luxi
- Batch size: 8192  Batches: 200  Threads: 1
- Transport: tcp
- Median batch latency: 0.016 ms  p95: 0.026 ms
- Throughput (samples/s, compute-time): 511358883.8

### torch_gpu_baseline.csv
- Framework: pytorch  Mode: baseline
- Device: cuda
- Batch size: 1048576  Batches: 200  Threads: 1
- Concurrency: 1
- Median batch latency: 1.618 ms  p95: 1.829 ms
- Throughput (samples/s, compute-time): 467131662.8

### tf_baseline_power.csv
- Framework: tensorflow  Mode: baseline
- Batch size: 8192  Batches: 200  Threads: 1
- Median batch latency: 0.005 ms  p95: 0.010 ms
- Throughput (samples/s, compute-time): 1580362543.2

### tf_luxi_power.csv
- Framework: tensorflow  Mode: luxi
- Batch size: 8192  Batches: 200  Threads: 1
- Transport: tcp
- Median batch latency: 0.016 ms  p95: 0.024 ms
- Throughput (samples/s, compute-time): 496863103.6

### torch_baseline_power.csv
- Framework: pytorch  Mode: baseline
- Batch size: 8192  Batches: 200  Threads: 1
- Median batch latency: 0.199 ms  p95: 0.237 ms
- Throughput (samples/s, compute-time): 39696149.1

### torch_luxi_power.csv
- Framework: pytorch  Mode: luxi
- Batch size: 2048  Batches: 200  Threads: 1
- Concurrency: 1
- Transport: tcp
- Median batch latency: 62.974 ms  p95: 65.283 ms
- Throughput (samples/s, compute-time): 265108.5

### torch_luxi_tcp_power.csv
- Framework: pytorch  Mode: luxi
- Batch size: 16384  Batches: 200  Threads: 2
- Concurrency: 4
- Transport: tcp
- Median batch latency: 62.974 ms  p95: 65.283 ms
- Throughput (samples/s, compute-time): 265108.5

### torch_luxi_uds_power.csv
- Framework: pytorch  Mode: luxi
- Batch size: 16384  Batches: 200  Threads: 2
- Concurrency: 4
- Transport: uds
- Median batch latency: 63.215 ms  p95: 69.510 ms
- Throughput (samples/s, compute-time): 290671.0

## Energy and ops/J (steady-state runs)

### tf_baseline_power.csv (energy)
- Duration (compute-time used): 0.0 s  [compute=0.0s, pm=42.0s]
- Avg CPU: 8.24 W  Avg GPU: 0.60 W  Total: 8.85 W
- Samples processed: 37429248  Energy: 0.21 J
- Ops/J (operation = one expression evaluation per x): 178624419.90

### tf_luxi_power.csv (energy)
- Transport: tcp
- Duration (compute-time used): 0.0 s  [compute=0.0s, pm=42.0s]
- Avg CPU: 8.26 W  Avg GPU: 0.61 W  Total: 8.87 W
- Samples processed: 2408448  Energy: 0.04 J
- Ops/J (operation = one expression evaluation per x): 56004107.00

### torch_baseline_power.csv (energy)
- Duration (compute-time used): 0.2 s  [compute=0.2s, pm=42.0s]
- Avg CPU: 8.59 W  Avg GPU: 0.60 W  Total: 9.19 W
- Samples processed: 9011200  Energy: 2.09 J
- Ops/J (operation = one expression evaluation per x): 4321766.88

### torch_luxi_power.csv (energy)
- Transport: tcp
- Duration (compute-time used): 12.1 s  [compute=12.1s, pm=42.0s]
- Avg CPU: 9.77 W  Avg GPU: 0.05 W  Total: 9.82 W
- Samples processed: 3211264  Energy: 118.94 J
- Ops/J (operation = one expression evaluation per x): 26999.14

### torch_luxi_tcp_power.csv (energy)
- Transport: tcp
- Duration (compute-time used): 12.1 s  [compute=12.1s, pm=42.0s]
- Avg CPU: 9.77 W  Avg GPU: 0.05 W  Total: 9.82 W
- Samples processed: 3211264  Energy: 118.94 J
- Ops/J (operation = one expression evaluation per x): 26999.14

### torch_luxi_uds_power.csv (energy)
- Transport: uds
- Duration (compute-time used): 12.5 s  [compute=12.5s, pm=42.0s]
- Avg CPU: 10.84 W  Avg GPU: 0.03 W  Total: 10.86 W
- Samples processed: 3637248  Energy: 135.91 J
- Ops/J (operation = one expression evaluation per x): 26761.88

## Tuning sweeps (PyTorch + Luxi)

Top 5 by Ops/J:

- bs=16384  thr=2  conc=4  ops/J=24766.75  thr_sps=253572.2
- bs=16384  thr=1  conc=4  ops/J=24153.94  thr_sps=254122.5
- bs=16384  thr=4  conc=4  ops/J=23628.70  thr_sps=245850.1
- bs=16384  thr=4  conc=2  ops/J=21242.39  thr_sps=200598.7
- bs=4096  thr=4  conc=4  ops/J=20990.22  thr_sps=208798.3

Top 5 by Throughput:

- bs=16384  thr=1  conc=4  thr_sps=254122.5  ops/J=24153.94
- bs=16384  thr=2  conc=4  thr_sps=253572.2  ops/J=24766.75
- bs=16384  thr=4  conc=4  thr_sps=245850.1  ops/J=23628.70
- bs=8192  thr=1  conc=4  thr_sps=221792.1  ops/J=20189.53
- bs=8192  thr=2  conc=4  thr_sps=221047.3  ops/J=20196.86

