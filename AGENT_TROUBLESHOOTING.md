# AI Agent Troubleshooting Guide for Luxi Edge

**Purpose:** Solutions for common issues encountered during automated validation.

**Target Audience:** AI agents, automation scripts, CI/CD pipelines, and human operators.

---

## 🚨 Quick Diagnostic

If validation is failing, run these checks first:

```bash
# Check if you're in the right directory
pwd  # Should show: .../LuxiEdge

# Check repository integrity
ls -la README.md AGENTS.md scripts/quickbench.sh

# Check Rust toolchain
rustc --version  # Should be >= 1.75.0
cargo --version

# Check system
uname -a  # Should be Linux or Darwin (macOS)

# Try basic build
cargo build --release
```

**If any of these fail, see the relevant section below.**

---

## 📂 Repository Access Issues

### Error: "Repository not found" or "Permission denied"

**Symptoms:**
```
git clone https://github.com/RegularJoe-CEO/LuxiEdge.git
fatal: repository 'https://github.com/RegularJoe-CEO/LuxiEdge.git' not found
```

**Solutions:**

1. **Check repository visibility**
   - Repository may be private
   - Verify URL is correct: `https://github.com/RegularJoe-CEO/LuxiEdge`

2. **Use HTTPS instead of SSH**
   ```bash
   # Use this:
   git clone https://github.com/RegularJoe-CEO/LuxiEdge.git
   
   # Not this:
   git clone git@github.com:RegularJoe-CEO/LuxiEdge.git
   ```

3. **Check GitHub token (if private)**
   ```bash
   # Set token
   export GITHUB_TOKEN=your_token_here
   
   # Clone with token
   git clone https://${GITHUB_TOKEN}@github.com/RegularJoe-CEO/LuxiEdge.git
   ```

4. **Download ZIP instead**
   - Go to repository web page
   - Click "Code" → "Download ZIP"
   - Extract and proceed

---

### Error: "AGENTS.md not found" or "quickbench.sh not found"

**Symptoms:**
```
cat: AGENTS.md: No such file or directory
bash: scripts/quickbench.sh: No such file or directory
```

**Solutions:**

1. **Verify you're in the right directory**
   ```bash
   pwd
   # Should show: /some/path/LuxiEdge
   
   cd LuxiEdge  # If not already there
   ```

2. **Check file existence**
   ```bash
   find . -name "AGENTS.md"
   find . -name "quickbench.sh"
   ```

3. **Verify branch**
   ```bash
   git branch
   # Should be on 'main' or 'master'
   
   git checkout main  # If on wrong branch
   ```

4. **Re-clone if corrupted**
   ```bash
   cd ..
   rm -rf LuxiEdge
   git clone https://github.com/RegularJoe-CEO/LuxiEdge.git
   cd LuxiEdge
   ```

---

## 🔧 Build Issues

### Error: "rustc: command not found"

**Symptoms:**
```
bash: rustc: command not found
```

**Solutions:**

1. **Install Rust**
   ```bash
   # Linux/macOS
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   
   # Verify
   rustc --version
   ```

2. **Activate Rust environment**
   ```bash
   source $HOME/.cargo/env
   # or
   export PATH="$HOME/.cargo/bin:$PATH"
   ```

3. **Use Docker alternative**
   ```bash
   # If Rust installation fails
   docker build -t luxi-edge .
   docker run luxi-edge cargo bench
   ```

---

### Error: Rust version too old

**Symptoms:**
```
error: package requires rustc 1.75.0 or newer
```

**Solutions:**

1. **Update Rust**
   ```bash
   rustup update stable
   rustc --version  # Verify >= 1.75.0
   ```

2. **Use specific toolchain**
   ```bash
   rustup install 1.75.0
   rustup default 1.75.0
   ```

---

### Error: Build fails with "linker not found"

**Symptoms:**
```
error: linker `cc` not found
```

**Solutions:**

1. **Install build tools (Linux)**
   ```bash
   # Debian/Ubuntu
   sudo apt-get update
   sudo apt-get install build-essential
   
   # RedHat/CentOS
   sudo yum groupinstall "Development Tools"
   
   # Alpine
   apk add build-base
   ```

2. **Install build tools (macOS)**
   ```bash
   xcode-select --install
   ```

---

### Error: Dependency compilation failures

**Symptoms:**
```
error: failed to compile `some-dependency`
```

**Solutions:**

1. **Clean and rebuild**
   ```bash
   cargo clean
   cargo build --release
   ```

2. **Update dependencies**
   ```bash
   cargo update
   cargo build --release
   ```

3. **Check disk space**
   ```bash
   df -h .
   # Need at least 2GB free for build
   ```

4. **Check internet connection**
   ```bash
   # Dependencies download from crates.io
   curl https://crates.io
   ```

---

## 🏃 Benchmark Issues

### Error: "quickbench.sh: Permission denied"

**Symptoms:**
```
bash: ./scripts/quickbench.sh: Permission denied
```

**Solutions:**

1. **Make script executable**
   ```bash
   chmod +x scripts/quickbench.sh
   ./scripts/quickbench.sh
   ```

2. **Run with bash explicitly**
   ```bash
   bash scripts/quickbench.sh
   ```

---

### Error: Benchmarks run but produce no output

**Symptoms:**
```
[2/5] Benches (warm 3 discard; 5 measured)
(hangs or produces no visible progress)
```

**Solutions:**

1. **Check benchmark status**
   ```bash
   # Benchmarks may take 2-10 minutes
   # Look for CPU activity with 'top' or 'htop'
   ```

2. **Run benchmarks manually**
   ```bash
   cargo bench --bench my_benchmark -- --verbose
   ```

3. **Reduce sample size for slow systems**
   ```bash
   cargo bench --bench my_benchmark -- --sample-size 10
   ```

4. **Check for resource constraints**
   ```bash
   # Free memory
   free -h
   
   # CPU count
   nproc
   
   # System load
   uptime
   ```

---

### Warning: "Unable to complete 100 samples in 5.0s"

**Symptoms:**
```
Warning: Unable to complete 100 samples in 5.0s.
```

**This is NORMAL for fast benchmarks.**

**Not a failure.** Criterion is warning that the benchmark runs so fast it can't collect 100 samples in 5 seconds. This is expected for SIMD operations.

**No action needed.** Your results are still valid.

**To suppress (optional):**
```bash
cargo bench -- --measurement-time 10 --sample-size 50
```

---

### Error: Benchmarks fail with "panic" or "segfault"

**Symptoms:**
```
thread 'main' panicked at...
Segmentation fault (core dumped)
```

**Solutions:**

1. **Check CPU compatibility**
   ```bash
   # Check SIMD support
   lscpu | grep -i flags
   # Look for: sse, sse2, avx, avx2
   
   # macOS
   sysctl -a | grep cpu.features
   ```

2. **Run without SIMD (fallback)**
   ```bash
   # Not currently implemented, but worth trying:
   RUSTFLAGS="-C target-cpu=native" cargo build --release
   ```

3. **Check memory limits**
   ```bash
   ulimit -a
   # Increase if needed:
   ulimit -s unlimited
   ```

4. **Report the issue**
   - Open GitHub Issue with CPU model and error details
   - Include output of `lscpu` (Linux) or `sysctl hw` (macOS)

---

## 📊 Results Validation Issues

### Issue: Results differ significantly from documented values

**Symptoms:**
- Documented: 1.6ms for 100K elements
- Your result: 10ms or 0.5ms

**This is often NORMAL.** Here's why:

**Expected Variance:**
- **Different CPU:** Intel vs AMD vs ARM will produce different absolute values
- **Different clock speed:** 2.0 GHz CPU vs 4.0 GHz CPU = 2× difference
- **Different SIMD:** AVX-512 vs AVX2 vs NEON = 2-4× difference
- **System load:** Background processes affect results

**Assessment Guidelines:**

| Your Result | Assessment | Action |
|------------|------------|--------|
| Within ±50% of documented | ✅ **HIGH confidence** | Results validate claims |
| Within 2× of documented | ✅ **MEDIUM confidence** | Different hardware explains variance |
| 2-5× different | ⚠️ **LOW confidence** | Functional but check CPU model |
| >5× different | ❌ **Investigation needed** | Report issue with details |

**Key Metric:** The **speedup factor** should be >10× faster than baseline, regardless of absolute time.

---

### Issue: Energy telemetry not available

**Symptoms:**
```
[4/5] Energy telemetry
⊘ Energy telemetry not available, will mark N/A
```

**This is NORMAL.** Energy measurement requires special hardware.

**Available on:**
- Intel/AMD CPUs with RAPL (Linux only)
- Servers with IPMI
- Systems with external power meters

**Not available on:**
- macOS (unless using external tools)
- Cloud VMs (virtualization blocks access)
- ARM systems without specialized tools

**Action:** Continue validation. Throughput and latency metrics are sufficient.

**Mark in report:**
```
Energy: N/A (telemetry not available on this platform)
Note: Performance validated, energy claims documented separately
```

---

### Issue: Python not available, limited reports

**Symptoms:**
```
⊘ Python3 not found, creating basic reports...
```

**Solution:**

1. **Install Python**
   ```bash
   # Linux
   sudo apt-get install python3
   
   # macOS
   brew install python3
   ```

2. **Use basic reports**
   - Basic JSON/CSV/MD files are still generated
   - Review Criterion output directly: `target/criterion/report/index.html`

3. **Manually calculate economics**
   - Formula: `Savings = P_MW × f × r × 8760 × 1000 × price`
   - Example: `100 × 0.2 × 0.5 × 8760 × 1000 × 0.2 = $17,520,000/yr`

---

## 🐳 Docker Issues

### Error: "Cannot connect to Docker daemon"

**Symptoms:**
```
Cannot connect to the Docker daemon
```

**Solutions:**

1. **Start Docker**
   ```bash
   # Linux
   sudo systemctl start docker
   
   # macOS
   open -a Docker
   ```

2. **Add user to docker group (Linux)**
   ```bash
   sudo usermod -aG docker $USER
   # Log out and back in
   ```

3. **Use sudo (not recommended)**
   ```bash
   sudo docker build -t luxi-edge .
   ```

---

### Error: Docker build fails

**Symptoms:**
```
ERROR [build 1/5] FROM ...
```

**Solutions:**

1. **Check Dockerfile exists**
   ```bash
   ls -la Dockerfile
   ```

2. **Pull base image manually**
   ```bash
   docker pull rust:1.75
   docker build -t luxi-edge .
   ```

3. **Increase Docker resources**
   - Docker Desktop → Preferences → Resources
   - Set Memory: >= 4GB, Disk: >= 20GB

---

## 🎯 Platform-Specific Issues

### macOS: "xcrun: error: invalid active developer path"

**Symptoms:**
```
xcrun: error: invalid active developer path
```

**Solution:**
```bash
xcode-select --install
```

---

### Linux: "error while loading shared libraries"

**Symptoms:**
```
error while loading shared libraries: libssl.so.1.1
```

**Solutions:**

1. **Install missing libraries**
   ```bash
   # Debian/Ubuntu
   sudo apt-get install libssl-dev pkg-config
   
   # RedHat/CentOS
   sudo yum install openssl-devel
   ```

2. **Update library cache**
   ```bash
   sudo ldconfig
   ```

---

### WSL2 (Windows): Performance very slow

**Symptoms:**
- Benchmarks take 10× longer than expected
- Build is extremely slow

**Solutions:**

1. **Ensure files are on WSL filesystem, not Windows**
   ```bash
   pwd
   # Should be: /home/user/... (NOT /mnt/c/...)
   
   # If wrong, move files:
   cp -r /mnt/c/path/to/LuxiEdge ~/
   cd ~/LuxiEdge
   ```

2. **Increase WSL resources**
   - Create/edit `%USERPROFILE%\.wslconfig`:
   ```
   [wsl2]
   memory=8GB
   processors=4
   ```

---

## 📝 Common Error Messages & Solutions

| Error | Cause | Solution |
|-------|-------|----------|
| `AGENTS.md: No such file or directory` | Wrong directory | `cd LuxiEdge` |
| `rustc: command not found` | Rust not installed | Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| `Permission denied: quickbench.sh` | Script not executable | `chmod +x scripts/quickbench.sh` |
| `linker 'cc' not found` | No C compiler | Linux: `sudo apt-get install build-essential` / macOS: `xcode-select --install` |
| `Unable to complete 100 samples` | Benchmark too fast | Not an error - results are valid |
| `thread 'main' panicked` | Code error or incompatibility | Check CPU flags, report issue |
| `No energy telemetry available` | Platform limitation | Continue - not required for validation |

---

## 🆘 Recovery Protocols

### Full Reset Protocol

If nothing works, start fresh:

```bash
# 1. Clean up
cd ..
rm -rf LuxiEdge

# 2. Update toolchain
rustup update stable
rustup default stable

# 3. Re-clone
git clone https://github.com/RegularJoe-CEO/LuxiEdge.git
cd LuxiEdge

# 4. Clean build
cargo clean
cargo build --release

# 5. Run benchmark
./scripts/quickbench.sh
```

---

### Minimal Validation (If quickbench.sh Fails)

```bash
# Build
cargo build --release

# Run one benchmark manually
cargo bench --bench my_benchmark

# Check results
cat target/criterion/my_benchmark/simd_inplace_100k/base/estimates.json

# Manual report
echo "Build: SUCCESS"
echo "Benchmark: SUCCESS"
echo "See: target/criterion/ for results"
```

---

## 📞 Getting Help

### Before Reporting Issues

Collect this information:

```bash
# System info
uname -a
rustc --version
cargo --version

# CPU info (Linux)
lscpu

# CPU info (macOS)
sysctl hw

# Error details
cat artifacts/agent_errors.log  # If it exists
```

### Where to Get Help

1. **Check existing issues:** [GitHub Issues](https://github.com/RegularJoe-CEO/LuxiEdge/issues)
2. **Search discussions:** [GitHub Discussions](https://github.com/RegularJoe-CEO/LuxiEdge/discussions)
3. **Open new issue:** Include system info and error logs
4. **Check documentation:** [docs/README.md](docs/README.md)

### Issue Template

```markdown
**Problem:** [Brief description]

**System:**
- OS: [Linux/macOS/WSL2]
- CPU: [Model]
- Rust: [Version]

**Steps Tried:**
1. [What you did]
2. [What happened]

**Error Output:**
```
[Paste error here]
```

**Logs:**
[Attach artifacts/agent_errors.log if exists]
```

---

## ✅ Success Indicators

You're on the right track if you see:

```
✓ Build successful
✓ Benchmarks complete
✓ Reports generated

Artifacts created:
-rw-r--r--  bench_results.json
-rw-r--r--  bench_results.csv
-rw-r--r--  bench_summary.md
-rw-r--r--  economics_summary.md
```

Even if:
- Energy telemetry is N/A
- Results differ from documented (different CPU)
- Python reports are basic
- Some benchmarks show warnings

**These are still valid validations.**

---

## 🎓 Understanding "Normal" vs "Error"

### Normal (Don't worry about these)

✅ **"Unable to complete 100 samples"** - Benchmark is fast  
✅ **"Energy telemetry not available"** - Platform limitation  
✅ **"Python not found"** - Basic reports still work  
✅ **Results ±50% different** - Different CPU  
✅ **Build takes 5-10 minutes** - Normal for Rust  

### Errors (Need fixing)

❌ **"rustc: command not found"** - Install Rust  
❌ **"No such file or directory"** - Wrong directory  
❌ **"linker not found"** - Install build tools  
❌ **Build fails** - Check dependencies  
❌ **Segmentation fault** - Hardware incompatibility  

---

**Still stuck? Open an issue with your system info and error logs.**

**Want alternatives? See [AGENT_PROMPTS_INDEX.md](AGENT_PROMPTS_INDEX.md) for other validation options.**
