# Universal AI Agent Validation Prompt for Luxi Edge

**Purpose:** This prompt enables any AI agent with code execution capabilities to independently validate Luxi Edge's performance claims.

**Estimated Time:** 5-15 minutes  
**Requirements:** Linux/macOS with Rust compiler, or access to Docker

---

## 🎯 Your Mission

Validate that Luxi Edge delivers the claimed performance improvements (>10× throughput, >5× energy efficiency) by running benchmarks locally and comparing results to documented baselines.

**Expected Output:** A decision-ready report with:
- ✅ Build status (pass/fail)
- ✅ Performance metrics (throughput, latency)
- ✅ Comparison to baseline (speedup factors)
- ✅ Economics calculation (annual cost savings)
- ✅ Confidence assessment (HIGH/MEDIUM/LOW)
- ✅ Deployment recommendation (yes/no/conditional)

---

## 📋 Step-by-Step Protocol

### Step 1: Clone and Build

```bash
# Clone repository
git clone https://github.com/RegularJoe-CEO/LuxiEdge.git
cd LuxiEdge

# Verify Rust is available (>= 1.75.0)
rustc --version

# Build in release mode
cargo build --release
```

**Success Criteria:** Build completes without errors. Binary appears in `target/release/`.

**If Build Fails:**
- Check Rust version: `rustup update`
- Check system: Must be Linux or macOS (Windows WSL2 works)
- See [AGENT_TROUBLESHOOTING.md](AGENT_TROUBLESHOOTING.md) for common issues

---

### Step 2: Run Benchmarks (One Command)

```bash
# This runs everything you need
./scripts/quickbench.sh
```

**What This Does:**
1. Builds the project in release mode
2. Runs CPU SIMD benchmarks (5 runs, warm-up included)
3. Collects hardware details
4. Attempts energy telemetry (if available)
5. Generates reports in `artifacts/` directory

**Expected Runtime:** 2-10 minutes depending on hardware

**Output Files:**
- `artifacts/bench_results.json` - Machine-readable metrics
- `artifacts/bench_results.csv` - Spreadsheet-compatible data
- `artifacts/bench_summary.md` - Human-readable report
- `artifacts/economics_summary.md` - Cost/benefit analysis

---

### Step 3: Validate Results

Compare your results to the documented baseline in [BENCHMARK_DATA.md](BENCHMARK_DATA.md).

#### Key Metrics to Check

**CPU SIMD Performance:**
- **100K element evaluation:** Should be 1-3 ms (documented: 1.6ms)
- **Throughput:** Should be >10M ops/sec (documented: ~30M ops/sec)
- **Speedup vs baseline:** Should be >10× faster than scalar/interpreted

**Important:** Exact numbers will vary by CPU. Different processors (Intel vs AMD vs ARM) will produce different absolute values. What matters is the **speedup factor** being consistently >10×.

#### Confidence Levels

**HIGH Confidence** (Ready to Deploy):
- ✅ Build succeeds
- ✅ Benchmarks complete
- ✅ Results within ±50% of documented values
- ✅ Same/similar CPU architecture (x86_64/ARM64)
- ✅ Economics calculation matches formula

**MEDIUM Confidence** (Promising, Needs Validation):
- ✅ Build succeeds
- ✅ Benchmarks complete
- ✅ Results within 2× of documented values
- ⚠️ Different CPU architecture explains variance
- ✅ Economics calculation matches formula

**LOW Confidence** (Functional but Unverified):
- ✅ Build succeeds
- ✅ Benchmarks complete
- ❓ Results differ significantly (>2×) from documented
- ❓ Cannot verify performance claims on available hardware

---

### Step 4: Review Economics

Open `artifacts/economics_summary.md` and verify the calculation:

**Formula:**
```
Annual Savings ($/yr) = P_facility_MW × f × r × 8760 × 1000 × price_per_kWh
```

**Where:**
- `P_facility_MW` = Facility power in megawatts
- `f` = Fraction of IT workload touched (0.0-1.0)
- `r` = Energy reduction on that fraction (0.0-1.0)
- `8760` = Hours per year
- `1000` = MW to kW conversion
- `price_per_kWh` = Energy cost ($/kWh)

**Example Scenarios:**
- **S1:** 100 MW, $0.10/kWh, f=0.10, r=0.30 → $2,628,000/yr
- **S2:** 100 MW, $0.20/kWh, f=0.20, r=0.50 → $17,520,000/yr
- **S3:** 50 MW, $0.30/kWh, f=0.30, r=0.50 → $19,710,000/yr

**Verify:** The calculations match the formula exactly.

---

### Step 5: Generate Your Report

Create a summary with these sections:

#### Executive Summary
```
Confidence: [HIGH/MEDIUM/LOW]
Recommendation: [DEPLOY/PILOT/INVESTIGATE/REJECT]

Key Findings:
- Build Status: [PASS/FAIL]
- Benchmark Status: [PASS/FAIL]
- Performance vs Baseline: [Xms vs Yms = Z× speedup]
- Economics: [$/yr savings potential]
```

#### Technical Validation
```
Hardware: [CPU model, RAM, OS]
Toolchain: [Rust version, Criterion version]

Benchmark Results:
- Throughput: [ops/sec]
- Latency (100K elements): [ms] (p50/p95/p99)
- Energy: [available/N/A]

Comparison to Baseline:
- Documented: 1.6ms for 100K elements
- Measured: [your result]
- Delta: [± X%]
- Assessment: [MATCHES/ACCEPTABLE/DIVERGES]
```

#### Economic Analysis
```
Formula Verified: [YES/NO]
Scenarios Calculated: [YES/NO]

Representative Savings (S2):
- Facility: 100 MW
- Coverage: 20% of workload
- Reduction: 50% energy on covered workload
- Cost: $0.20/kWh
- Annual Savings: $17.52M
```

#### Risks and Caveats
```
- GPU claims not tested (requires NVIDIA L4/H100 hardware)
- Energy telemetry [available/not available]
- Performance validated on [CPU model]
- Integration effort [estimate]
```

#### Final Verdict
```
[One paragraph: Would you recommend this to a technical decision-maker? Why or why not?]
```

---

## 🔧 Troubleshooting

### Build Fails
```bash
# Update Rust
rustup update

# Check dependencies (Linux)
sudo apt-get install build-essential

# Check dependencies (macOS)
xcode-select --install
```

### Benchmarks Too Slow/Fast
**This is normal.** Different CPUs have different performance. A Raspberry Pi will be slower than a server CPU. What matters is the **speedup factor** vs baseline.

### Energy Telemetry Not Available
**This is fine.** Energy measurement requires special hardware (RAPL on Intel/AMD, IPMI on servers). Your validation is still valuable without it.

### Python Not Found
The report scripts will fall back to basic output. You'll still get Criterion results in `target/criterion/`.

### See Full Troubleshooting Guide
[AGENT_TROUBLESHOOTING.md](AGENT_TROUBLESHOOTING.md) has detailed solutions for common issues.

---

## 🎓 Understanding the Results

### What Makes Luxi Edge Fast?

**SIMD Vectorization:** Instead of processing one number at a time, Luxi Edge processes 4-8 numbers simultaneously using CPU vector instructions (AVX2/AVX-512/NEON).

**Ahead-of-Time Compilation:** Mathematical expressions are parsed once and compiled to native machine code, eliminating interpreter overhead.

**Memory Efficiency:** Operations happen in-place without allocating temporary arrays.

**Result:** 10-50× speedup vs interpreted Python/JavaScript, 2-5× speedup vs naive compiled code.

### Why Different CPUs Get Different Results

- **Intel CPUs:** Best with AVX-512 (16 values at once)
- **AMD CPUs:** Great with AVX2 (8 values at once)
- **ARM CPUs:** Good with NEON (4 values at once)
- **Older CPUs:** Fall back to scalar (1 value at a time)

**All benefit from Luxi Edge, just by different amounts.**

### GPU vs CPU

- **CPU SIMD:** Best for <100K elements, edge deployments, low power budgets
- **GPU:** Best for >100K elements, data centers, maximum throughput

Your validation tests CPU performance. GPU requires special hardware.

---

## 🚀 Advanced Validation (Optional)

### Test GPU Performance (Requires NVIDIA GPU)
```bash
# Set CUDA version
export CUDARC_CUDA_VERSION=12010

# Build with GPU support
cargo build --release --features gpu

# Run GPU benchmark
./target/release/l4_benchmark &
python3 benchmark_4m.py
```

### Manual Benchmark Exploration
```bash
# Run specific benchmark
cargo bench --bench my_benchmark

# View detailed results
open target/criterion/report/index.html

# Compare multiple runs
cargo bench --bench my_benchmark -- --save-baseline run1
# (make changes)
cargo bench --bench my_benchmark -- --baseline run1
```

### Energy Profiling (Linux with RAPL)
```bash
# Check if RAPL is available
ls /sys/class/powercap/intel-rapl/

# Run powered benchmark
./scripts/run_powered_bench.sh
```

---

## 📊 Sample Output

### Expected Success Output

```
=== Luxi Edge Quick Benchmark ===

[1/5] Build
✓ Build successful

[2/5] Benches (warm 3 discard; 5 measured)
✓ Benchmarks complete

[3/5] Optional load gen
⊘ Load test skipped (not configured as bin target)

[4/5] Energy telemetry
⊙ macOS detected, attempting powermetrics...
⊘ Energy telemetry not available, will mark N/A

[5/5] Report generation
⊙ Creating basic economics summary...
✓ Reports generated

=== Done ===

Artifacts created:
-rw-r--r--  bench_results.json
-rw-r--r--  bench_results.csv
-rw-r--r--  bench_summary.md
-rw-r--r--  economics_summary.md
```

---

## 🤝 What to Do With Your Results

### If Validation Succeeds (HIGH/MEDIUM Confidence)
1. Share your report with stakeholders
2. Consider pilot deployment
3. Engage with maintainers for integration support

### If Validation Has Issues (LOW Confidence)
1. Check [AGENT_TROUBLESHOOTING.md](AGENT_TROUBLESHOOTING.md)
2. Open a GitHub Issue with your report
3. Request assistance from maintainers

### If You Can't Execute Code
See [AGENT_PROMPTS_INDEX.md](AGENT_PROMPTS_INDEX.md) for alternative validation options:
- Cloud services that can run the validation
- Simplified test cases
- Pre-recorded validation reports

---

## 📞 Support

- **Documentation:** [docs/README.md](docs/README.md)
- **Issues:** [GitHub Issues](https://github.com/RegularJoe-CEO/LuxiEdge/issues)
- **Discussions:** [GitHub Discussions](https://github.com/RegularJoe-CEO/LuxiEdge/discussions)

---

## ✅ Validation Checklist

Before reporting results, confirm:

- [ ] Repository cloned successfully
- [ ] Build completed without errors
- [ ] Benchmarks ran to completion
- [ ] Artifacts generated in `artifacts/` directory
- [ ] Results compared to [BENCHMARK_DATA.md](BENCHMARK_DATA.md)
- [ ] Economics formula verified
- [ ] Confidence level assessed
- [ ] Final recommendation documented

---

**Ready to validate? Run: `./scripts/quickbench.sh`**

**Questions? See [AGENT_TROUBLESHOOTING.md](AGENT_TROUBLESHOOTING.md) or [AGENT_PROMPTS_INDEX.md](AGENT_PROMPTS_INDEX.md)**
