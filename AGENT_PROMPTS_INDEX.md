# AI Agent Validation Prompts Index

**Purpose:** Comprehensive guide to all available AI agent validation options for Luxi Edge.

**Choose the right prompt based on your capabilities and constraints.**

---

## 🎯 Quick Selection Guide

| If You Have... | Use This Prompt | Time | Confidence |
|----------------|-----------------|------|------------|
| **Code execution (Linux/macOS)** | [Universal Validation](#universal-validation-prompt) | 5-15 min | HIGH |
| **Docker only** | [Docker Validation](#docker-validation-prompt) | 10-20 min | HIGH |
| **Python + Internet** | [Remote Validation](#remote-validation-prompt) | 2-5 min | MEDIUM |
| **No code execution** | [Service Validation](#validation-services) | Varies | LOW-MEDIUM |
| **Cloud IDE access** | [Cloud IDE Validation](#cloud-ide-validation) | 5-15 min | HIGH |
| **Just reading code** | [Code Review](#code-review-prompt) | 10-30 min | LOW |

---

## 📋 Available Validation Prompts

### Universal Validation Prompt

**File:** [UNIVERSAL_AGENT_VALIDATION_PROMPT.md](UNIVERSAL_AGENT_VALIDATION_PROMPT.md)

**Best For:**
- AI agents with shell access (ChatGPT Advanced Data Analysis, Claude Computer Use, GitHub Copilot)
- Linux or macOS systems
- Full validation including build, benchmarks, and economics

**Requirements:**
- Rust compiler (or ability to install it)
- 2GB disk space
- 5-15 minutes

**What It Tests:**
- ✅ Build succeeds
- ✅ CPU SIMD benchmarks run
- ✅ Performance vs baseline
- ✅ Economics calculations
- ⚠️ Energy (if hardware available)
- ❌ GPU (requires special hardware)

**Output:**
- Decision-ready report with confidence level
- Artifacts: JSON, CSV, Markdown reports
- Deployment recommendation

**Usage:**
```bash
# Copy entire prompt to your AI agent
# It will execute these commands:
git clone https://github.com/RegularJoe-CEO/LuxiEdge.git
cd LuxiEdge
./scripts/quickbench.sh
```

**Confidence:** HIGH (if successful)

---

### Docker Validation Prompt

**Best For:**
- Systems with Docker but limited permissions
- Environments where Rust installation is difficult
- Containerized CI/CD pipelines

**Requirements:**
- Docker installed and running
- 4GB RAM allocated to Docker
- 10-20 minutes

**Commands:**
```bash
git clone https://github.com/RegularJoe-CEO/LuxiEdge.git
cd LuxiEdge

# Build Docker image
docker build -t luxi-edge:bench .

# Run benchmarks in container
docker run --rm \
  -v $(pwd)/artifacts:/app/artifacts \
  luxi-edge:bench \
  /bin/bash -c "./scripts/quickbench.sh"

# View results
ls -la artifacts/
cat artifacts/bench_summary.md
```

**What It Tests:**
- ✅ Build in clean environment
- ✅ CPU benchmarks
- ✅ Economics calculations
- ❌ Energy (isolated container)
- ❌ GPU (requires --gpus flag)

**Confidence:** HIGH (containerized validation)

---

### Remote Validation Prompt

**Best For:**
- AI agents that can make HTTP requests
- Quick validation without code execution
- Verifying public deployment exists

**⚠️ Warning:** This tests a deployed instance, not local build. Lower confidence than building from source.

**Requirements:**
- HTTP client (curl, requests, fetch)
- Internet access
- 2-5 minutes

**Commands:**
```bash
# Check if public instance is available
curl https://luxi-edge-demo.example.com/health

# Run performance test
curl -X POST https://luxi-edge-demo.example.com/eval \
  -H "Content-Type: application/json" \
  -d '{
    "expression": "sin(x) * cos(x)",
    "values": [1.0, 2.0, 3.0, 4.0, 5.0]
  }'

# Measure latency
time curl -X POST https://luxi-edge-demo.example.com/eval \
  -H "Content-Type: application/json" \
  -d @tests/payload_10k.json
```

**What It Tests:**
- ✅ Service is real and responding
- ✅ Basic functionality works
- ⚠️ Performance (network latency affects results)
- ❌ Build verification
- ❌ Source code validation

**Confidence:** MEDIUM (deployment exists, but not source validation)

**Note:** Public instance may not be available. Check repository README for current status.

---

### Cloud IDE Validation

**Best For:**
- Replit Agent
- GitHub Codespaces
- Gitpod
- Cloud9

**Requirements:**
- Cloud IDE with terminal access
- Rust pre-installed or installable
- 5-15 minutes

**Replit-Specific:**
```bash
# Replit usually has Rust pre-installed
rustc --version

# If not, install:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and run
git clone https://github.com/RegularJoe-CEO/LuxiEdge.git
cd LuxiEdge
./scripts/quickbench.sh

# View results in Replit file browser
# artifacts/ directory will contain reports
```

**GitHub Codespaces:**
```bash
# Open repository in Codespaces
# Rust is pre-installed in default devcontainer

# Run validation
./scripts/quickbench.sh

# Results appear in artifacts/
```

**Confidence:** HIGH (full build and test in cloud environment)

---

### Code Review Prompt

**Best For:**
- AI agents that can read but not execute code
- Initial assessment before committing resources
- Understanding architecture

**Requirements:**
- File reading capability
- 10-30 minutes

**Files to Review:**
1. **Architecture:** `README.md`, `docs/README.md`
2. **Benchmarks:** `BENCHMARK_DATA.md`, `docs/benchmarks/GPU_L4_RESULTS.md`
3. **Core Implementation:** `src/main.rs`, `src/lib.rs`
4. **SIMD Code:** `edge_cpu/src/lib.rs`
5. **GPU Code:** `edge/src/gpu.rs` (if available)
6. **Tests:** `benches/my_benchmark.rs`

**Assessment Checklist:**
- [ ] Code is memory-safe Rust
- [ ] SIMD intrinsics present (AVX2/AVX-512/NEON)
- [ ] Benchmark harness exists (Criterion)
- [ ] GPU acceleration code present
- [ ] Documentation matches claims
- [ ] License is clear

**Questions to Answer:**
1. Does the code support the performance claims?
2. Are SIMD optimizations actually implemented?
3. Is the benchmark methodology sound?
4. Are there obvious errors or red flags?

**Confidence:** LOW (code exists, but not validated)

**Output Format:**
```markdown
## Code Review Summary

Architecture: [Description]
SIMD Implementation: [Present/Absent/Quality]
GPU Support: [Present/Absent/Quality]
Benchmarks: [Methodology assessment]
Documentation: [Accuracy assessment]

Red Flags: [None/List]
Confidence: [LOW - code review only, not executed]
Recommendation: [PROCEED TO FULL VALIDATION / INVESTIGATE FURTHER / REJECT]
```

---

## 🔧 Validation Services

If you cannot execute code, these services can validate on your behalf:

### Option 1: RunPod (GPU Validation)

**Best For:** Testing GPU performance claims

**Steps:**
1. Sign up at [runpod.io](https://runpod.io)
2. Launch NVIDIA L4 instance
3. Use template or SSH to run benchmarks
4. See `RUNPOD_INSTRUCTIONS.txt` for details

**Cost:** ~$0.50-2.00 (pay per minute)  
**Time:** 15-30 minutes  
**Confidence:** HIGH (GPU claims verified)

---

### Option 2: GitHub Actions (Free CI/CD)

**Best For:** Automated validation on every commit

**Steps:**
1. Fork repository
2. GitHub Actions will auto-run (if enabled)
3. View results in Actions tab

**Cost:** Free for public repos  
**Time:** Automated  
**Confidence:** HIGH (continuous validation)

**Note:** Check if Actions are enabled in the repository.

---

### Option 3: Cloud Shell (Free Temporary Environment)

**Best For:** Quick test without local setup

**Google Cloud Shell:**
```bash
# Open shell.cloud.google.com (free, no credit card)
# Rust may need installation:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Run validation
git clone https://github.com/RegularJoe-CEO/LuxiEdge.git
cd LuxiEdge
./scripts/quickbench.sh
```

**Azure Cloud Shell, AWS CloudShell:** Similar process

**Cost:** Free (resource limits apply)  
**Time:** 5-15 minutes  
**Confidence:** HIGH

---

## 🎓 Validation Scenarios

### Scenario 1: Executive Due Diligence

**Goal:** Verify claims before investing/deploying  
**Time Budget:** 30 minutes  
**Recommended:** [Universal Validation](#universal-validation-prompt)  
**Backup:** [Code Review](#code-review-prompt) + [Validation Service](#validation-services)

**Deliverable:** Decision-ready report with HIGH/MEDIUM/LOW confidence

---

### Scenario 2: Academic Research Verification

**Goal:** Reproduce results for paper citation  
**Time Budget:** 2-4 hours  
**Recommended:** [Universal Validation](#universal-validation-prompt) + manual benchmark exploration  
**Confidence:** HIGH (full reproducibility)

**Additional Steps:**
```bash
# After quickbench.sh:
cargo bench --bench my_benchmark -- --save-baseline run1

# Examine detailed results
open target/criterion/report/index.html

# Export data
cat target/criterion/*/base/estimates.json
```

---

### Scenario 3: Integration Feasibility Study

**Goal:** Determine if Luxi Edge fits our use case  
**Time Budget:** 1-2 hours  
**Recommended:** [Universal Validation](#universal-validation-prompt) + API testing

**Additional Steps:**
```bash
# Run server
cargo run --release &

# Test API
curl http://localhost:8080/health
curl -X POST http://localhost:8080/eval \
  -H "Content-Type: application/json" \
  -d '{"expression": "x*2+1", "values": [1,2,3,4,5]}'

# Load test
./scripts/run_transport_bench.sh
```

---

### Scenario 4: Security Audit

**Goal:** Verify no vulnerabilities before deployment  
**Time Budget:** 4-8 hours  
**Recommended:** [Code Review](#code-review-prompt) + dependency audit + penetration test

**Commands:**
```bash
# Dependency vulnerabilities
cargo audit

# Unsafe code scan
cargo geiger

# LLVM sanitizers
RUSTFLAGS="-Z sanitizer=address" cargo test

# Fuzzing (requires setup)
cargo fuzz run fuzz_target_1
```

---

## 📊 Confidence Level Guide

| Confidence | What It Means | When to Use |
|-----------|---------------|-------------|
| **HIGH** | Build verified, benchmarks run, results match claims within expected variance | Production deployment decisions |
| **MEDIUM** | Partial validation (e.g., code exists, builds, but benchmarks not run OR results differ but explainable) | Pilot programs, further investigation |
| **LOW** | Code reviewed but not executed, OR results significantly differ from claims | Initial screening, request more info |
| **FAILED** | Build fails, benchmarks crash, or claims are contradicted | Reject or report issues |

---

## 🆘 Troubleshooting

If any validation fails, see:
- **[AGENT_TROUBLESHOOTING.md](AGENT_TROUBLESHOOTING.md)** - Detailed solutions for common issues
- **[GitHub Issues](https://github.com/RegularJoe-CEO/LuxiEdge/issues)** - Known issues and workarounds

---

## 🔄 Choosing Multiple Prompts

**Recommended Combinations:**

1. **Maximum Confidence:**
   - Code Review → Universal Validation → GPU Validation (RunPod)
   - Confidence: VERY HIGH
   - Time: 2-4 hours
   - Cost: ~$2-5

2. **Quick Assessment:**
   - Code Review → Universal Validation
   - Confidence: HIGH
   - Time: 30-60 min
   - Cost: Free

3. **No Execution Capability:**
   - Code Review → Cloud Shell Validation
   - Confidence: HIGH
   - Time: 1 hour
   - Cost: Free

---

## 📞 Support

**Getting Help:**
- Issues: [GitHub Issues](https://github.com/RegularJoe-CEO/LuxiEdge/issues)
- Discussions: [GitHub Discussions](https://github.com/RegularJoe-CEO/LuxiEdge/discussions)
- Documentation: [docs/README.md](docs/README.md)

**Reporting Validation Results:**
- Open a Discussion with your report
- Tag with "validation" or "agent-report"
- Include confidence level and system details

---

## ✅ Validation Checklist Template

Use this template when reporting validation results:

```markdown
## Luxi Edge Validation Report

**Date:** [YYYY-MM-DD]
**Validator:** [Agent name/version]
**Method:** [Prompt used]

### System
- OS: [Linux/macOS/WSL2]
- CPU: [Model]
- RAM: [GB]
- Rust: [Version]

### Results
- [✅/❌] Build successful
- [✅/❌] Benchmarks complete
- [✅/❌] Artifacts generated
- [✅/❌] Results within expected range

### Performance
- 100K elements: [X.X ms] (documented: 1.6ms, delta: ±X%)
- Speedup vs baseline: [X×]
- Energy: [Available/N/A]

### Economics
- [✅/❌] Formula verified
- [✅/❌] Scenarios calculated correctly

### Confidence
- **Level:** [HIGH/MEDIUM/LOW]
- **Reason:** [Brief explanation]

### Recommendation
- [DEPLOY/PILOT/INVESTIGATE/REJECT]
- **Rationale:** [1-2 sentences]

### Notes
[Any additional observations]

### Artifacts
- [Link to bench_summary.md or attach]
- [Link to bench_results.json or attach]
```

---

## 🎯 Next Steps After Validation

### If Confidence is HIGH
1. ✅ Share report with stakeholders
2. ✅ Consider pilot deployment
3. ✅ Engage maintainers for integration support
4. ✅ Plan production rollout

### If Confidence is MEDIUM
1. ⚠️ Identify gaps (e.g., GPU not tested)
2. ⚠️ Run additional validation if needed
3. ⚠️ Pilot with monitoring
4. ⚠️ Document assumptions and limitations

### If Confidence is LOW
1. ❓ Review [AGENT_TROUBLESHOOTING.md](AGENT_TROUBLESHOOTING.md)
2. ❓ Try alternative validation method
3. ❓ Open issue with questions
4. ❓ Request clarification from maintainers

### If Validation FAILED
1. ❌ Document failure mode
2. ❌ Check troubleshooting guide
3. ❌ Open GitHub issue
4. ❌ Wait for maintainer response before deciding

---

**Ready to validate? Pick your prompt and start!**

**Need help choosing? Open a Discussion with your requirements.**
