# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0

# Luxi Edge Architecture Guide for Researchers

**A Comprehensive Technical Reference**

## Executive Summary

This document provides a complete architectural overview of Luxi Edge, designed for researchers, engineers, and academics seeking to understand the system's design principles, component interactions, and scientific foundations. Luxi Edge represents a high-performance computational microservice combining SIMD acceleration, real-time optimization, and stateless API design for edge and data center deployments.

---

## 1. System Overview

### 1.1 Mission Statement

Provide ultra-fast numeric computation at the edge through:
- **SIMD-accelerated evaluation** for vectorized operations
- **Deterministic root-finding** with auto-bracketing algorithms
- **Edge-optimized design** with minimal resource footprint
- **Stateless API** enabling horizontal scaling

### 1.2 Two-Tier Architecture

```
┌───────────────────────────────────────────────────────────────┐
│                     Luxi Core™                                │
│           Portfolio Optimization & Market Integration          │
│              (Optional multi-site orchestration)               │
│                                                                 │
│  • API Adapters for external systems                           │
│  • Fleet-wide optimization with risk controls                  │
│  • Real-time telemetry aggregation                            │
│  • Revenue: Enterprise SaaS + analytics services               │
└───────────────┬────────────────────────────┬──────────────────┘
                │                            │
    ┌───────────▼──────────┐     ┌──────────▼───────────┐
    │   Luxi Edge™        │     │   Luxi Edge™        │
    │  (Compute Layer)     │ ... │  (Compute Layer)     │
    │                      │     │                      │
    │ • Expression eval    │     │ • Expression eval    │
    │ • Root-finding       │     │ • Root-finding       │
    │ • SIMD acceleration  │     │ • SIMD acceleration  │
    │ • HTTP API           │     │ • HTTP API           │
    │                      │     │                      │
    │ Hardware: ARM64/x86  │     │ Hardware: ARM64/x86  │
    └──────────────────────┘     └──────────────────────┘
```

> **Note:** Luxi SDG™ is a separate product maintained in a private repository and not covered here.

### 1.3 Design Principles

**Stateless API:**
- No server-side state between requests
- Enables horizontal scaling and load balancing
- Simplified deployment and maintenance

**Determinism:**
- No non-deterministic algorithms in critical paths
- Reproducible behavior for audit and compliance
- Consistent results across platforms

**Performance:**
- SIMD vectorization for batch operations
- Zero-copy parsing where possible
- Memory-efficient evaluation without garbage collection

---

## 2. Component Deep Dive

### 2.1 Luxi Edge™: Computational Microservice

#### 2.1.1 Purpose
Provides high-performance numeric expression evaluation and root-finding with SIMD acceleration.

#### 2.1.2 Hardware Requirements

**Processor:**
- Architecture: ARM64 (recommended) or x86_64
- SIMD: NEON (ARM) or AVX2 (x86) for vectorized computation
- Cores: 2+ (1 for I/O, 1+ for computation)
- Clock: 1.0 GHz minimum

**Memory:**
- RAM: 512 MB minimum, 1 GB recommended
- Storage: 100 MB for OS, 10 MB for binary, 100 MB for logs

**I/O Interfaces:**
- Modbus RTU/TCP for industrial equipment (HVAC, chillers)
- GPIO for relay control (older equipment)
- Ethernet for network connectivity
- Optional: RS-485, CAN bus, BACnet/IP

**Power:**
- Input: 5V DC (USB-C) or 12V DC (industrial)
- Consumption: 2-5W typical, 8W peak
- Battery backup: Optional (UPS integration)

#### 2.1.3 Software Stack

```
┌─────────────────────────────────────────┐
│         Application Layer               │
│  (Luxi Edge Binary - Rust)             │
│  • HTTP API Server (Axum)               │
│  • Expression Evaluator                 │
│  • I/O State Machine                    │
└────────────┬────────────────────────────┘
             │
┌────────────▼────────────────────────────┐
│         Runtime Layer                   │
│  • Tokio Async Executor                 │
│  • Work-stealing Scheduler              │
│  • Lock-free Queues                     │
└────────────┬────────────────────────────┘
             │
┌────────────▼────────────────────────────┐
│      Operating System                   │
│  • Linux (Debian/Alpine)                │
│  • Real-time kernel patches (optional)  │
└────────────┬────────────────────────────┘
             │
┌────────────▼────────────────────────────┐
│         Hardware                        │
│  • ARM Cortex-A53/A72 or Intel Atom    │
│  • NEON / AVX2 SIMD units               │
└─────────────────────────────────────────┘
```

#### 2.1.4 API Endpoints

**Health Check:**
```
GET /health
Response: {"service": "luxi_edge", "version": "0.1.0", "status": "ok"}
```

**Expression Evaluation:**
```
POST /evaluate
Request: {"expr": "y = 3.14 + x * 2", "x": [0,1,2], "vars": {"pi": [3.14]}}
Response: {"y": [3.14, 5.14, 7.14]}
```

**Root Finding (Bracketed):**
```
POST /bisect
Request: {"expr": "x^2 - 2", "lo": 0.0, "hi": 3.0, "tol": 1e-9}
Response: {"root": 1.414213562, "f": 0.0, "iters": 29, "bracket_ok": true}
```

**Root Finding (Auto-Bracket):**
```
POST /bisect_auto
Request: {"expr": "x^3 - x - 2", "guess": 1.0, "step": 1.0}
Response: {"root": 1.521, "f": 0.0, "lo": 1.0, "hi": 2.0, "iters": 31, "expansions": 1}
```

#### 2.1.5 Performance Characteristics

| Metric | Value |
|--------|-------|
| Startup Time | 12 ms |
| Binary Size | 8-10 MB |
| Memory Footprint | 8-12 MB (resident) |
| API Latency (p50) | 7 ms |
| API Latency (p99) | 15 ms |
| Max Throughput | 193k ops/sec (evaluation) |
| Energy Efficiency | 3.08 µJ/op (SIMD) vs 55.6 µJ/op (scalar) |

### 2.3 Luxi Core™: Portfolio Orchestration

#### 2.3.1 Architecture

**Multi-Tenancy:**
```
Core Instance
  ├─ Tenant A (Organization 1)
  │    ├─ Site A1
  │    ├─ Site A2
  │    └─ Site A3
  ├─ Tenant B (Organization 2)
  │    └─ Site B1
  └─ Tenant C (Organization 3)
       ├─ Site C1
       └─ Site C2
```

**Data Isolation:**
- Separate PostgreSQL schemas per tenant
- API key scoping (tenant_id in JWT claims)
- Row-level security (RLS) policies

#### 2.3.2 Fleet Optimization

**Aggregation Model:**
```
Total_capacity = Σ[sites] flexible_load[site]

Constraint:
  Must maintain diversity (no single point of failure)
  No single site contributes >20% of portfolio
```

**Risk Controls:**
- Maximum dispatch depth per site: 80% of flexible load
- Reserve margin: 10% held back for uncertainty
- Geographic diversification: Sites span multiple weather zones

#### 2.3.3 Market Integration

**ISO/RTO API Adapters:**

| Market | API Type | Settlement Interval |
|--------|----------|---------------------|
| CAISO | REST + WebSocket | 5-minute |
| PJM | SOAP (legacy) | Hourly |
| ERCOT | REST | 15-minute |
| NYISO | FTP + REST | 5-minute |

**Adapter Pattern:**
```rust
trait MarketAdapter {
    fn fetch_prices(&self) -> Result<Vec<Price>>;
    fn submit_bid(&self, capacity_mw: f64) -> Result<BidId>;
    fn query_settlement(&self, date: Date) -> Result<Settlement>;
}

struct CAISOAdapter { /* ... */ }
impl MarketAdapter for CAISOAdapter { /* ... */ }

struct PJMAdapter { /* ... */ }
impl MarketAdapter for PJMAdapter { /* ... */ }
```

**Market-Agnostic Core:**
```rust
fn optimize(prices: Vec<Price>, sites: Vec<Site>) -> DispatchPlan {
    // Independent of market specifics
    // Only requires: time series of prices + site capabilities
}
```

---

## 3. Computational Core: Expression Engine

### 3.1 Design Rationale

**Why Custom Evaluator?**
1. **Determinism:** Third-party libraries (Python/NumPy) have non-deterministic thread scheduling
2. **Performance:** SIMD-optimized path achieves 13.7× speedup vs generic implementations
3. **Security:** Memory-safe Rust eliminates buffer overflows in parsing/evaluation
4. **Auditability:** Small codebase (~1500 LOC) enables formal verification

### 3.2 Execution Pipeline

```
Input String
    │
    ▼
┌──────────────┐
│ Tokenization │  O(n)   Single-pass lexical analysis
└──────┬───────┘
       │
       ▼
┌──────────────┐
│   Parsing    │  O(n)   Recursive descent with precedence
└──────┬───────┘
       │
       ▼
┌──────────────┐
│     AST      │         Abstract syntax tree representation
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Optimization │  O(n)   (Future: CSE, constant folding)
└──────┬───────┘
       │
       ├─────────────┬──────────────┐
       ▼             ▼              ▼
┌─────────────┐  ┌─────────────┐  ┌─────────────┐
│   JIT       │  │   SIMD      │  │   Scalar    │
│ (x86_64)    │  │  (AVX2)     │  │  (Fallback) │
└─────────────┘  └─────────────┘  └─────────────┘
       │             │              │
       └─────────────┴──────────────┘
                     │
                     ▼
                  Result
```

### 3.3 SIMD Implementation Details

**Lane Utilization:**

```
Input: x = [x₀, x₁, x₂, x₃, x₄, x₅, x₆, x₇, x₈, x₉]
AVX2 (4 lanes):
  Iteration 0: [x₀, x₁, x₂, x₃] → [y₀, y₁, y₂, y₃]
  Iteration 1: [x₄, x₅, x₆, x₇] → [y₄, y₅, y₆, y₇]
  Scalar:      [x₈, x₉]         → [y₈, y₉]

Efficiency: 8/10 = 80% vectorized (2 remainder elements)
```

**Instruction Throughput (Intel Skylake):**

| Instruction | Latency | Throughput | Ports |
|-------------|---------|------------|-------|
| VADDPD | 4 cycles | 0.5 cycles | p0,p1 |
| VMULPD | 4 cycles | 0.5 cycles | p0,p1 |
| VDIVPD | 13-16 cycles | 4-5 cycles | p0 |
| VFMADD231PD | 4 cycles | 0.5 cycles | p0,p1 |

**Optimization:** Use FMA (fused multiply-add) when available: `a + b * c` → `VFMADD(a, b, c)`

### 3.4 Numerical Stability

**Floating-Point Concerns:**

**Example: Catastrophic Cancellation**
```
f(x) = √(x + 1) - √x  (naive)

For x = 1e16:
  √(x + 1) ≈ 1e8  (rounded)
  √x       ≈ 1e8  (rounded)
  Result   = 0    (WRONG! Should be ~5e-9)
```

**Mitigation: Rationalization**
```
f(x) = (√(x + 1) - √x) × (√(x + 1) + √x) / (√(x + 1) + √x)
     = [(x + 1) - x] / (√(x + 1) + √x)
     = 1 / (√(x + 1) + √x)  (numerically stable)
```

**Testing:**
```rust
#[test]
fn test_numerical_stability() {
    // Kahan summation for large vectors
    let xs: Vec<f64> = (0..1_000_000).map(|i| (i as f64) * 1e-10).collect();
    let sum_naive: f64 = xs.iter().sum();
    let sum_kahan = kahan_sum(&xs);
    
    assert!((sum_naive - sum_kahan).abs() < 1e-6);
}
```

---

## 4. Security Architecture

### 4.1 Threat Model

**Adversarial Capabilities:**
- Control over OS/hypervisor (root/admin access)
- Network eavesdropping (passive monitoring)
- Code injection attempts (exploit vulnerabilities)

**Adversarial Goals:**
- Inflate telemetry data to increase financial payouts
- Disrupt control operations (denial of service)
- Exfiltrate sensitive data (customer usage patterns)

**Assumptions (Trusted):**
- TEE/TPM hardware (Intel SGX, ARM TrustZone)
- Cryptographic primitives (AES, RSA, ECC)
- Boot firmware (UEFI Secure Boot)

### 4.2 Defense Mechanisms

#### 4.2.1 Trusted Execution Environment (TEE)

**Intel SGX Integration:**
```rust
#[enclave_trusted]
fn compute_settlement(usage_data: &[f64]) -> Settlement {
    // Runs in encrypted memory (EPC)
    // OS cannot read/modify execution
    let total_kwh: f64 = usage_data.iter().sum();
    let cost = total_kwh * PRICE_PER_KWH;
    
    Settlement {
        total_kwh,
        cost,
        signature: sign_with_enclave_key(total_kwh, cost),
    }
}
```

**Attestation Flow:**
```
1. Enclave boots, measures code hash (PCR0)
2. Requests attestation quote from Intel
3. Intel signs: "Enclave with hash H is running on genuine SGX"
4. Send quote to verifier (Luxi Core)
5. Core checks: hash matches expected binary + Intel signature valid
6. If OK, provision secrets to enclave
```

#### 4.2.2 Zero-Knowledge Proofs (Conceptual)

**Goal:** Prove "I computed result R correctly" without revealing inputs/code

**zk-SNARK Example:**
```
Public Input:  output_hash = SHA256(R)
Private Input: R, computation_trace
Proof:         π

Verifier checks: VERIFY(π, output_hash) = TRUE
                 ⇒ R was computed correctly, but R remains secret
```

**Application:** Settlement verification without exposing customer load profiles.

**Status:** Research integration (not production-ready).

#### 4.2.3 Anti-Tamper Monitoring

**Code Integrity:**
```rust
const EXPECTED_HASH: [u8; 32] = include_bytes!("binary_hash.bin");

fn verify_self_integrity() -> Result<()> {
    let self_path = std::env::current_exe()?;
    let self_bytes = std::fs::read(self_path)?;
    let actual_hash = sha256(&self_bytes);
    
    if actual_hash != EXPECTED_HASH {
        // Alert + shutdown
        eprintln!("SECURITY: Binary tampering detected!");
        std::process::exit(1);
    }
    Ok(())
}
```

**Periodic Checks:**
```rust
tokio::spawn(async {
    let mut interval = tokio::time::interval(Duration::from_secs(60));
    loop {
        interval.tick().await;
        if let Err(e) = verify_self_integrity() {
            alert_security_team(e);
        }
    }
});
```

### 4.3 Communication Security

**TLS 1.3 Configuration:**
```rust
use rustls::{ServerConfig, NoClientAuth};

let mut config = ServerConfig::new(NoClientAuth::new());
config.set_single_cert(cert_chain, private_key)?;
config.versions = vec![ProtocolVersion::TLSv1_3];
config.ciphersuites = vec![
    CipherSuite::TLS13_AES_256_GCM_SHA384,
    CipherSuite::TLS13_CHACHA20_POLY1305_SHA256,
];
```

**Certificate Pinning:**
```rust
fn validate_peer_cert(cert: &Certificate) -> Result<()> {
    let expected_pubkey_hash = load_pinned_hash();
    let actual_pubkey_hash = sha256(cert.public_key());
    
    if actual_pubkey_hash != expected_pubkey_hash {
        return Err("Certificate pin mismatch".into());
    }
    Ok(())
}
```

---

## 5. Deployment Architecture

### 5.1 Containerization

**Note:** Container images are illustrative examples only. No public container registry is available at this time.

**Multi-Stage Dockerfile:**
```dockerfile
# Stage 1: Build
FROM rust:1.75-slim as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY edge ./edge
RUN cargo build --release --bin luxi_edge

# Stage 2: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/luxi_edge /usr/local/bin/
EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=3s CMD curl -f http://localhost:8080/health || exit 1
USER nobody
CMD ["luxi_edge"]
```

**Image Size Optimization:**
- Multi-stage build: 50 MB (vs 2 GB with full Rust toolchain)
- Static linking: No dynamic library dependencies
- Strip symbols: `cargo build --release` with `strip = true`

### 5.2 Kubernetes Deployment

**Example Deployment Manifest (illustrative):**
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: luxi-edge
spec:
  replicas: 3
  selector:
    matchLabels:
      app: luxi-edge
  template:
    metadata:
      labels:
        app: luxi-edge
    spec:
      containers:
      - name: luxi-edge
        image: ghcr.io/regularjoe-ceo/luxi-edge:latest  # Illustrative only - not published
        ports:
        - containerPort: 8080
        resources:
          requests:
            memory: "32Mi"
            cpu: "100m"
          limits:
            memory: "128Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 3
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 1
          periodSeconds: 5
```

**Horizontal Pod Autoscaler:**
```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: luxi-edge-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: luxi-edge
  minReplicas: 3
  maxReplicas: 100
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Pods
    pods:
      metric:
        name: http_requests_per_second
      target:
        type: AverageValue
        averageValue: "1000"
```

### 5.3 Observability

**Metrics (Prometheus):**
```rust
use prometheus::{Histogram, Counter, register_histogram, register_counter};

lazy_static! {
    static ref EVAL_DURATION: Histogram = register_histogram!(
        "luxi_eval_duration_seconds",
        "Expression evaluation latency"
    ).unwrap();
    
    static ref API_REQUESTS: Counter = register_counter!(
        "luxi_api_requests_total",
        "Total API requests"
    ).unwrap();
}

async fn evaluate_handler(req: Json<EvalReq>) -> Json<EvalResp> {
    let _timer = EVAL_DURATION.start_timer();
    API_REQUESTS.inc();
    
    // ... evaluation logic ...
}
```

**Grafana Dashboard Queries:**
```promql
# P99 latency
histogram_quantile(0.99, rate(luxi_eval_duration_seconds_bucket[5m]))

# Requests per second
rate(luxi_api_requests_total[1m])

# Error rate
rate(luxi_api_errors_total[5m]) / rate(luxi_api_requests_total[5m])
```

**Distributed Tracing (Jaeger):**
```rust
use opentelemetry::trace::Tracer;

async fn evaluate(req: EvalReq) -> Result<EvalResp> {
    let tracer = global::tracer("luxi");
    let mut span = tracer.start("evaluate");
    
    span.set_attribute("expr_length", req.expr.len() as i64);
    span.set_attribute("x_count", req.x.len() as i64);
    
    let tokens = tokenize(&req.expr)?;
    span.add_event("tokenization_complete", vec![]);
    
    let ast = parse(tokens)?;
    span.add_event("parsing_complete", vec![]);
    
    let result = interpret(ast, &req.vars)?;
    span.end();
    
    Ok(result)
}
```

---

## 6. Performance Engineering

### 6.1 Profiling Methodology

**CPU Profiling (perf):**
```bash
# Record profile
perf record -g -F 999 ./target/release/luxi_edge

# Generate flamegraph
perf script | stackcollapse-perf.pl | flamegraph.pl > flame.svg

# Annotate source
perf annotate --stdio
```

**Memory Profiling (valgrind):**
```bash
valgrind --tool=massif --massif-out-file=massif.out ./target/release/luxi_edge
ms_print massif.out
```

**Benchmark Suite:**
```bash
# Scalar vs SIMD sweep (long running)
cargo bench --bench simd_vs_scalar -- --sample-size 10

# Calculus workloads (derivative / gradient / Newton)
cargo bench --bench my_benchmark -- --sample-size 20

# Capture baselines for regression tracking
cargo bench --bench simd_vs_scalar -- --save-baseline before --sample-size 10
# ... apply optimization ...
cargo bench --bench simd_vs_scalar -- --baseline before --sample-size 10
```

### 6.2 Optimization Techniques

**Hot Path Inlining:**
```rust
#[inline(always)]
fn fast_pow(base: f64, exp: i32) -> f64 {
    match exp {
        0 => 1.0,
        1 => base,
        2 => base * base,
        3 => base * base * base,
        _ => base.powi(exp),  // Fallback
    }
}
```

**Loop Unrolling:**
```rust
// Before
for i in 0..len {
    result[i] = compute(data[i]);
}

// After (manual unrolling, factor 4)
let chunks = len / 4;
for i in 0..chunks {
    result[i*4]   = compute(data[i*4]);
    result[i*4+1] = compute(data[i*4+1]);
    result[i*4+2] = compute(data[i*4+2]);
    result[i*4+3] = compute(data[i*4+3]);
}
// Handle remainder...
```

**Branch Prediction Hints:**
```rust
#[cold]
#[inline(never)]
fn handle_error(e: Error) {
    eprintln!("Error: {:?}", e);
}

fn process(x: f64) -> Result<f64> {
    if likely(x > 0.0) {  // Compiler hint: branch likely taken
        Ok(x.sqrt())
    } else {
        handle_error(Error::Negative)
    }
}
```

### 6.3 Cache Optimization

**Data Structure Alignment:**
```rust
#[repr(align(64))]  // Cache line alignment
struct CacheOptimized {
    hot_data: [f64; 8],    // Frequently accessed
    _padding: [u8; 0],
    cold_data: Vec<f64>,   // Infrequent access
}
```

**False Sharing Avoidance:**
```rust
// Bad: Different threads modify adjacent fields
struct Shared {
    counter_a: AtomicU64,  // Thread A
    counter_b: AtomicU64,  // Thread B (same cache line!)
}

// Good: Pad to separate cache lines
#[repr(align(64))]
struct PaddedCounter(AtomicU64);

struct Shared {
    counter_a: PaddedCounter,
    counter_b: PaddedCounter,
}
```

---

## 7. Testing Strategy

### 7.1 Unit Tests

**Coverage Target:** >90% line coverage

```bash
cargo tarpaulin --out Html --output-dir coverage
```

**Example:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_division_by_zero() {
        let expr = "1 / 0";
        let result = eval(expr, &mut HashMap::new());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Division by zero");
    }
    
    #[test]
    fn test_undefined_variable() {
        let expr = "x + y";
        let mut vars = HashMap::new();
        vars.insert("x".into(), vec![1.0]);
        let result = eval(expr, &mut vars);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable: y"));
    }
}
```

### 7.2 Integration Tests

**HTTP API Testing:**
```rust
#[tokio::test]
async fn test_evaluate_endpoint() {
    let app = create_app();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/evaluate")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"expr": "x + 1", "x": [1.0, 2.0]}"#))
                .unwrap()
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let result: EvalResp = serde_json::from_slice(&body).unwrap();
    assert_eq!(result.y, vec![2.0, 3.0]);
}
```

### 7.3 Property-Based Testing

**Using `proptest`:**
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn simd_equals_scalar(
        x_vals in prop::collection::vec(-1000f64..1000f64, 1..1000)
    ) {
        let expr = "x^2 + 3*x - 5";
        
        let scalar: Vec<f64> = x_vals.iter()
            .map(|&x| eval_scalar(expr, x))
            .collect();
        
        let simd = eval_simd(expr, &x_vals);
        
        for (s, v) in scalar.iter().zip(simd.iter()) {
            prop_assert!((s - v).abs() < 1e-10);
        }
    }
}
```

### 7.4 Load Testing

**Using `wrk` (HTTP benchmarking):**
```bash
wrk -t4 -c100 -d30s --latency \
    -s evaluate.lua \
    http://localhost:8080/evaluate
```

**Lua Script (`evaluate.lua`):**
```lua
request = function()
    body = '{"expr": "x^2 + 3*x - 5", "x": [1,2,3,4,5]}'
    return wrk.format("POST", "/evaluate", 
        {["Content-Type"] = "application/json"}, 
        body)
end
```

---

## 8. Research Extensions

### 8.1 Quantum-Resistant Cryptography

**Post-Quantum Signature Schemes:**

| Algorithm | Key Size | Signature Size | Security Level |
|-----------|----------|----------------|----------------|
| SPHINCS+ | 64 bytes | 17 KB | NIST Level 5 |
| Dilithium | 2.5 KB | 3.3 KB | NIST Level 3 |
| Falcon | 1.8 KB | 1.3 KB | NIST Level 5 |

**Migration Path:**
```rust
enum SignatureScheme {
    ECDSA(EcdsaKey),     // Current (vulnerable to quantum)
    Dilithium(PQKey),    // Post-quantum (NIST finalist)
}

fn sign(data: &[u8], scheme: SignatureScheme) -> Vec<u8> {
    match scheme {
        SignatureScheme::ECDSA(key) => ecdsa_sign(data, key),
        SignatureScheme::Dilithium(key) => dilithium_sign(data, key),
    }
}
```

### 8.2 Federated Learning

**Use Case:** Learn global optimization policy without sharing customer data

**Algorithm (FedAvg):**
```
1. Central server initializes model θ₀
2. For each round t:
   a. Server sends θₜ to K clients
   b. Each client trains on local data: θₜ₊₁⁽ᵏ⁾ = θₜ - η∇L(θₜ, Dₖ)
   c. Clients send Δθ⁽ᵏ⁾ = θₜ₊₁⁽ᵏ⁾ - θₜ to server
   d. Server averages: θₜ₊₁ = θₜ + (1/K)Σ Δθ⁽ᵏ⁾
3. Repeat until convergence
```

**Privacy Guarantee:** Individual data never leaves client (only gradients shared).

### 8.3 Formal Verification

**Goal:** Prove correctness of critical algorithms (parsing, evaluation)

**Tool:** Coq proof assistant

**Example Theorem:**
```coq
Theorem parse_deterministic :
  forall (tokens : list Token) (ast1 ast2 : ASTNode),
    parse tokens = Some ast1 ->
    parse tokens = Some ast2 ->
    ast1 = ast2.
Proof.
  intros. rewrite H in H0. inversion H0. reflexivity.
Qed.
```

**Verified Properties:**
- Parsing is deterministic (same input → same AST)
- Evaluation is pure (no side effects except assignment)
- Root-finding converges for well-bracketed inputs

---

## 9. Conclusion

The Luxi Suite represents a holistic approach to software-defined energy management, combining:
- **High-performance computing** (SIMD, async I/O, memory safety)
- **Rigorous algorithms** (proven convergence, deterministic behavior)
- **Security** (TEE, TLS, integrity monitoring)
- **Scalability** (stateless design, horizontal scaling, multi-tenancy)

This architecture enables deployment across diverse scales (SMB to data center) and regulatory environments (global markets) while maintaining scientific rigor suitable for academic scrutiny and financial settlement.

---

**Document Metadata:**
- SPDX-FileCopyrightText: 2025 Eric Waller
- SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0
- Classification: Public Technical Documentation
- Intended Audience: Researchers, Engineers, Academic Reviewers
- Contact: e@ewaller.com

**Related Documents:**
- `SCIENTIFIC_OVERVIEW.md` - High-level scientific description
- `ALGORITHM_DETAILS.md` - Deep algorithmic analysis
- `../openapi.yaml` - API specification
- `../benchmarks/BENCHMARK_DATA.md` - Performance measurements
- `benchmarks/README.md` - Benchmark navigation hub and raw exports

**Revision History:**
- v1.0 (2025-10-28): Initial publication
