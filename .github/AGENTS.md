# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0

# Agent Integration Guide — Luxi Edge

This document provides comprehensive instructions for AI agents, LLMs, and automated tools to effectively interface with the Luxi Edge repository.

> **🌐 Web Version**: For a web-friendly overview of this guide, visit [https://e-rock-compute-eric589.replit.app/agents.html](https://e-rock-compute-eric589.replit.app/agents.html)

## Important Legal Notice

**COPYRIGHT AND OWNERSHIP**: All contributions, improvements, modifications, and derivative works created by AI agents, LLMs, or any automated assistance tools when working with this repository are the exclusive property of Eric Waller. By interfacing with this repository, you acknowledge that:

1. Any code, documentation, or other materials you generate or contribute become the property of Eric Waller
2. You assign all rights, title, and interest in your work to Eric Waller
3. This applies to all work products, whether committed to the repository or not
4. This ownership transfer is automatic and immediate upon creation

## Repository Overview

**Product**: Luxi Edge — High-performance numeric computation microservice  
**Internal Crate Name**: `erock` (for build compatibility)  
**Public Brand**: Luxi Edge  
**Technology Stack**: Rust, Axum, Tokio, SIMD acceleration  
**API Type**: Deterministic, stateless JSON REST API  
**Default Port**: 8080

## Quick Start for AI Agents

**Note:** The internal crate/module is named "erock" for historical reasons. Public branding is "Luxi Edge."

### One-Command Bootstrap

**For AI agents, run this first:**

```bash
bash scripts/agent_bootstrap.sh
```

This single command will:
1. ✅ Check prerequisites (cargo, curl)
2. ✅ Build Luxi Edge in release mode
3. ✅ Launch server on port 8080
4. ✅ Run smoke tests to verify all endpoints
5. ✅ Report PASS/FAIL for each test

**Required Tools:**
- `cargo` (Rust toolchain) - Install from https://rustup.rs/
- `curl` - For HTTP requests
- `python3` - For JSON parsing in tests

### Expected API Responses

Base URL: http://localhost:8080

**GET /ping**
```
pong
```

**GET /health**
```json
{
  "service": "erock_edge",
  "status": "ok",
  "version": "0.1.0",
  "jit_available": false,
  "jit_enabled": false,
  "jit_reason": "jit_disabled"
}
```

**POST /evaluate**
```bash
curl -X POST http://localhost:8080/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"2*x+sin(x)", "x":[3.14]}'
```
Response:
```json
{"y":[6.281592652916487]}
```

**POST /bisect**
```bash
curl -X POST http://localhost:8080/bisect \
  -H "Content-Type: application/json" \
  -d '{"expr":"x*x - 4", "lo":0, "hi":3}'
```
Response:
```json
{
  "root": 1.9999999998835847,
  "f": -4.656612873077393e-10,
  "iters": 33,
  "bracket_ok": true
}
```

**POST /bisect_auto**
```bash
curl -X POST http://localhost:8080/bisect_auto \
  -H "Content-Type: application/json" \
  -d '{"expr":"x*x - 4", "guess":2.0}'
```
Response:
```json
{
  "root": 2.0,
  "f": 0.0,
  "lo": 2.0,
  "hi": 2.0,
  "iters": 0,
  "bracket_ok": true,
  "expansions": 0
}
```

## OpenAI-style function tools
```json
[
  {
    "type": "function",
    "function": {
      "name": "luxiedge_evaluate",
      "description": "Vectorized evaluation y=f(x) over an array (SIMD lanes).",
      "parameters": {
        "type": "object",
        "required": ["expr", "x"],
        "properties": {
          "expr": { "type": "string", "description": "Assignment 'y = ...' optional." },
          "x": { "type": "array", "items": { "type": "number" } },
          "vars": { "type": "object", "additionalProperties": { "type": "number" } }
        }
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "luxiedge_bisect",
      "description": "Root finding with a supplied bracket [lo, hi].",
      "parameters": {
        "type": "object",
        "required": ["expr", "lo", "hi"],
        "properties": {
          "expr": { "type": "string" },
          "lo": { "type": "number" },
          "hi": { "type": "number" },
          "vars": { "type": "object", "additionalProperties": { "type": "number" } },
          "tol": { "type": "number", "default": 1e-9 },
          "max_iter": { "type": "integer", "default": 60 }
        }
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "luxiedge_bisect_auto",
      "description": "Auto‑bracket around a guess using exponential expansion, then bisection.",
      "parameters": {
        "type": "object",
        "required": ["expr", "guess"],
        "properties": {
          "expr": { "type": "string" },
          "guess": { "type": "number" },
          "step": { "type": "number", "default": 1.0 },
          "max_expand": { "type": "integer", "default": 20 },
          "vars": { "type": "object", "additionalProperties": { "type": "number" } },
          "tol": { "type": "number", "default": 1e-9 },
          "max_iter": { "type": "integer", "default": 60 }
        }
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "luxiedge_health",
      "description": "Health probe and version.",
      "parameters": { "type": "object", "properties": {} }
    }
  }
]
```

## Repository Structure

```
LuxiEdge/
├── Cargo.toml           # Root workspace manifest (crate name: "erock")
├── LICENSE              # Luxi Business License v1.0
├── README.md            # Public-facing documentation
├── BENCHMARK_DATA.md    # Performance metrics and methodology
├── docs/
│   └── benchmarks/      # Centralized benchmark docs and raw exports
├── TECHNICAL_OVERVIEW.md # Detailed technical reference
├── .github/
│   └── AGENTS.md        # This file - AI agent integration guide
│
├── edge/                # HTTP server implementation
│   ├── Cargo.toml       # Package: erock_edge
│   └── src/main.rs      # Axum server, routes: /evaluate, /bisect, /bisect_auto, /health
│
├── src/                 # Core library
│   └── lib.rs           # Expression evaluation, SIMD, root-finding
│
├── docs/                # Comprehensive documentation
│   ├── README.md        # Documentation index
│   ├── ARCHITECTURE.md  # System design
│   ├── SCIENTIFIC_OVERVIEW.md # Academic reference
│   └── ALGORITHM_DETAILS.md   # Implementation details
│
├── benches/             # Performance benchmarks
│   └── edge_suite.rs    # Criterion benchmarks
│
├── marketing-site/      # Static HTML pages
└── tools/               # Development utilities
```

## Building and Running

### Prerequisites
- Rust 1.75+ (`rustup install stable`)
- Cargo (included with Rust)

### Build Commands
```bash
# Check compilation (fast, no optimizations)
cargo check

# Build debug version
cargo build

# Build optimized release version
cargo build --release

# Build the edge server specifically
cd edge && cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Running the Server
```bash
# Development mode (from repository root)
cargo run --bin erock_edge

# Or from edge directory
cd edge && cargo run

# Production mode (optimized)
./target/release/erock_edge

# Server starts on http://localhost:8080
```

### Testing the API
```bash
# Health check
curl http://localhost:8080/health

# Evaluate expression
curl -X POST http://localhost:8080/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"x^2 + 3*x - 5", "x":[0,1,2,3,4]}'

# Root finding with bracket
curl -X POST http://localhost:8080/bisect \
  -H "Content-Type: application/json" \
  -d '{"expr":"x^2 - 2", "lo":0, "hi":3}'

# Root finding with auto-bracket
curl -X POST http://localhost:8080/bisect_auto \
  -H "Content-Type: application/json" \
  -d '{"expr":"x^2 - 2", "guess":1}'
```

## Code Modification Guidelines for AI Agents

### Understanding the Naming Convention
- **Public Brand**: Always use "Luxi Edge" in user-facing documentation, comments, error messages
- **Internal Crate Names**: `erock`, `erock_edge` in Cargo.toml and code (DO NOT change these)
- This dual naming exists for historical/build compatibility reasons

### Key Files to Modify When...

**Adding new API endpoints:**
- `edge/src/main.rs` - Add route handlers
- `openapi.yaml` - Update API specification
- `docs/ARCHITECTURE.md` - Document the new endpoint

**Improving performance:**
- `src/lib.rs` - Core evaluation logic
- `benches/edge_suite.rs` - Add benchmarks
- `BENCHMARK_DATA.md` - Document results

**Updating documentation:**
- `README.md` - High-level overview
- `docs/README.md` - Documentation index
- `docs/SCIENTIFIC_OVERVIEW.md` - Technical details
- `docs/ARCHITECTURE.md` - System design

**Changing dependencies:**
- `Cargo.toml` - Root dependencies
- `edge/Cargo.toml` - Server dependencies
- Run `cargo update` after changes

### Code Style Requirements
1. **SPDX Headers**: All new files must include:
   ```rust
   // SPDX-FileCopyrightText: 2025 Eric Waller
   // SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0
   ```

2. **Error Handling**: Use Result<T, E> pattern, avoid panics in production code

3. **Documentation**: Add doc comments for public APIs:
   ```rust
   /// Evaluates a numeric expression over a vector of inputs.
   ///
   /// # Arguments
   /// * `expr` - Mathematical expression as a string
   /// * `x` - Vector of input values
   ///
   /// # Returns
   /// Vector of computed results
   pub fn evaluate(expr: &str, x: &[f64]) -> Result<Vec<f64>, Error>
   ```

4. **Testing**: Add unit tests for new functionality:
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_evaluate() {
           // Test implementation
       }
   }
   ```

### Common Tasks for AI Agents

#### Task: Add a New Math Function
1. Open `src/lib.rs`
2. Locate the expression parser/evaluator
3. Add the function to the supported operations
4. Add tests in the same file
5. Update `docs/SCIENTIFIC_OVERVIEW.md` to document the new function
6. Run `cargo test` to verify

#### Task: Optimize Performance
1. Identify bottleneck using `cargo bench`
2. Modify implementation in `src/lib.rs`
3. Re-run benchmarks to measure improvement
4. Update `BENCHMARK_DATA.md` with new metrics
5. Document changes in commit message

#### Task: Fix a Bug
1. Reproduce the bug with a test case
2. Identify root cause (use `cargo clippy` for hints)
3. Implement fix
4. Verify with tests: `cargo test`
5. Ensure no regressions: `cargo bench`

#### Task: Add Documentation
1. Identify which document needs updating (see Repository Structure)
2. Maintain consistent voice: technical, precise, academic
3. Include code examples where applicable
4. Link to related sections using relative paths
5. Verify markdown renders correctly

## API Reference (OpenAI-style function tools)

```json
[
  {
    "type": "function",
    "function": {
      "name": "luxiedge_evaluate",
      "description": "Vectorized evaluation y=f(x) over an array (SIMD lanes).",
      "parameters": {
        "type": "object",
        "required": ["expr", "x"],
        "properties": {
          "expr": { "type": "string", "description": "Assignment 'y = ...' optional." },
          "x": { "type": "array", "items": { "type": "number" } },
          "vars": { "type": "object", "additionalProperties": { "type": "number" } }
        }
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "luxiedge_bisect",
      "description": "Root finding with a supplied bracket [lo, hi].",
      "parameters": {
        "type": "object",
        "required": ["expr", "lo", "hi"],
        "properties": {
          "expr": { "type": "string" },
          "lo": { "type": "number" },
          "hi": { "type": "number" },
          "vars": { "type": "object", "additionalProperties": { "type": "number" } },
          "tol": { "type": "number", "default": 1e-9 },
          "max_iter": { "type": "integer", "default": 60 }
        }
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "luxiedge_bisect_auto",
      "description": "Auto‑bracket around a guess using exponential expansion, then bisection.",
      "parameters": {
        "type": "object",
        "required": ["expr", "guess"],
        "properties": {
          "expr": { "type": "string" },
          "guess": { "type": "number" },
          "step": { "type": "number", "default": 1.0 },
          "max_expand": { "type": "integer", "default": 20 },
          "vars": { "type": "object", "additionalProperties": { "type": "number" } },
          "tol": { "type": "number", "default": 1e-9 },
          "max_iter": { "type": "integer", "default": 60 }
        }
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "luxiedge_health",
      "description": "Health probe and version.",
      "parameters": { "type": "object", "properties": {} }
    }
  }
]
```

## Debugging and Troubleshooting

### Common Issues

**Issue**: `cargo build` fails with dependency errors
- **Solution**: Run `cargo update` then `cargo build` again

**Issue**: Server won't start on port 8080
- **Solution**: Port may be in use. Kill existing process: `lsof -ti:8080 | xargs kill -9`

**Issue**: Tests fail after code changes
- **Solution**: Run `cargo test -- --nocapture` to see detailed output

**Issue**: Performance degradation after changes
- **Solution**: Run `cargo bench` to identify regression. Compare with baseline in BENCHMARK_DATA.md

### Useful Commands
```bash
# Format code to Rust standards
cargo fmt

# Lint code for potential issues
cargo clippy

# Check for outdated dependencies
cargo outdated

# Generate documentation
cargo doc --open

# Clean build artifacts
cargo clean
```

## Performance Considerations

When modifying code, maintain these performance characteristics:
- **Latency**: <1ms for health endpoint, 7-9ms for compute operations
- **Throughput**: ~193k operations/second
- **Power**: ~596mW under load
- **Memory**: <12MB resident

Run benchmarks before and after changes:
```bash
cargo bench --bench edge_suite
```

## Security Guidelines

1. **Never** commit secrets, API keys, or credentials
2. **Always** validate user input before processing
3. **Use** the type system to prevent invalid states
4. **Sanitize** expressions before evaluation to prevent injection
5. **Review** any external dependencies for vulnerabilities

## Getting Help

- **Documentation**: Start with `docs/README.md`
- **API Specification**: See `openapi.yaml`
- **Code Examples**: Check `benches/` and test modules
- **Architecture**: Read `docs/ARCHITECTURE.md`
- **Performance**: Review `BENCHMARK_DATA.md`

## Web Resources

For a web-friendly overview and additional integration resources, visit:

**🌐 AI Agent Integration Hub**: [https://e-rock-compute-eric589.replit.app/agents.html](https://e-rock-compute-eric589.replit.app/agents.html)

Additional web resources:
- [Luxi Edge Website](https://e-rock-compute-eric589.replit.app/)
- [Performance Metrics](https://e-rock-compute-eric589.replit.app/data.html)
- [Edge Computing Overview](https://e-rock-compute-eric589.replit.app/edge.html)

## Licensing Reminder

All work products created while interfacing with this repository are subject to the copyright ownership terms stated at the beginning of this document. By proceeding, you accept these terms.

---

**Last Updated**: 2025-11-05  
**Document Version**: 1.2  
**Maintained By**: Eric Waller
