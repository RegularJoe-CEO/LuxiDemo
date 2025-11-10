# ARM64 / Raspberry Pi 5 Testing Guide

This guide provides instructions for validating Luxi Edge ARM Neon optimizations and multi-revolution Lambert solving on ARM64 hardware (Raspberry Pi 5, AWS Graviton, Apple Silicon, Jetson).

## Prerequisites

**Hardware:**
- Raspberry Pi 5 (recommended for power measurements)
- AWS EC2 Graviton instance (c7g.xlarge or larger)
- Apple Silicon Mac (M1/M2/M3)
- NVIDIA Jetson (Orin, Xavier, Nano)

**Software:**
- Rust ≥ 1.75.0
- Linux kernel ≥ 5.10 (for power measurement support)

## Quick Start

```bash
# Clone repository
git clone https://github.com/RegularJoe-CEO/LuxiEdge.git
cd LuxiEdge

# Build for ARM64 (automatic Neon detection)
cargo build --release --benches

# Verify ARM64 architecture
uname -m  # Should show "aarch64"
```

## Running Benchmarks

### ARM Neon SIMD Benchmarks

```bash
# Full Neon benchmark suite
cargo bench --bench neon_benchmark

# Quick validation (tests only)
cargo bench --bench neon_benchmark -- --test

# Specific categories
cargo bench --bench neon_benchmark -- polynomial
cargo bench --bench neon_benchmark -- fma
cargo bench --bench neon_benchmark -- memory_bandwidth
```

**Expected Results (ARM64):**
```
polynomial/scalar/100000    ~65 µs
polynomial/neon/100000      ~35 µs   (1.86× speedup expected)

fma/scalar/100000           ~42 µs
fma/neon/100000             ~25 µs   (1.68× speedup expected)
```

### Multi-Revolution Lambert Benchmarks

```bash
# Full Lambert suite
cargo bench --bench lambert_benchmark

# Multi-rev specific
cargo bench --bench lambert_benchmark -- multirev

# Quick results
cargo bench --bench lambert_benchmark -- --quick multirev_batch_solver
```

**Expected Results (ARM64 with Neon):**
```
multirev_batch_solver/swarm_8rev    ~8-10 µs   (2× speedup vs x86_64)
batch_tof/neon/1000                 ~XX µs     (1.5× speedup vs scalar)
```

## Power Measurement (Raspberry Pi 5)

### Method 1: Using vcgencmd (Pi-specific)

```bash
# Monitor core voltage and temperature during benchmark
watch -n 0.1 'vcgencmd measure_volts core && vcgencmd measure_temp'

# Run benchmark in another terminal
cargo bench --bench neon_benchmark -- polynomial/neon/100000
```

### Method 2: Using INA219 Power Monitor (External Hardware)

If you have an INA219 power sensor connected via I2C:

```bash
# Install i2c-tools
sudo apt-get install i2c-tools python3-smbus

# Run power monitoring script (create this)
python3 scripts/measure_power_pi5.py &
POWER_PID=$!

# Run benchmark
cargo bench --bench neon_benchmark

# Stop power monitoring
kill $POWER_PID
```

### Method 3: System Power (USB-C Power Meter)

Use an inline USB-C power meter (e.g., AVHzY CT-3) to measure total system power:

1. Connect Pi 5 through USB-C power meter
2. Start benchmark
3. Record steady-state power draw
4. Subtract idle power to get benchmark power

**Expected Power:**
- Idle: ~3-4W
- Neon benchmark active: ~6-8W
- Net benchmark power: ~3-4W

## Results Template

### ARM Neon Performance

| Benchmark | Scalar (µs) | Neon (µs) | Speedup | Power (W) |
|-----------|-------------|-----------|---------|-----------|
| polynomial/1000 | | | | |
| polynomial/10000 | | | | |
| polynomial/100000 | | | | |
| fma/100000 | | | | |
| memory_bandwidth | | | | |

### Multi-Rev Lambert Performance

| Benchmark | Time (µs) | Throughput (solves/s) | Power (W) |
|-----------|-----------|----------------------|-----------|
| single_rev | | | |
| dual_rev | | | |
| quad_rev | | | |
| swarm_8rev | | | |

### System Information

```bash
# Capture system details
echo "=== System Information ===" > arm64_results.txt
uname -a >> arm64_results.txt
cat /proc/cpuinfo | grep -E "model name|Hardware|Revision" >> arm64_results.txt
lscpu >> arm64_results.txt
free -h >> arm64_results.txt
```

## Comparison with x86_64 Results

**x86_64 Baseline (AMD EPYC 7763):**
```
polynomial/scalar/100000    65.2 µs
polynomial/neon/100000      65.6 µs  (scalar fallback)
multirev_batch_solver/swarm_8rev    16.3 µs
```

**Expected ARM64 Improvement:**
- Polynomial: 1.5-2× faster due to Neon vectorization
- Multi-rev: 1.5-2× faster due to better memory bandwidth
- Power efficiency: 5-10× better (ops/joule)

## Troubleshooting

### Neon Intrinsics Not Available

If benchmarks show no speedup:
```bash
# Check for NEON support
cat /proc/cpuinfo | grep neon

# Verify compilation for ARM64
rustc --version --verbose | grep host
```

Should show `host: aarch64-unknown-linux-gnu`

### Build Failures

```bash
# Update Rust
rustup update stable

# Clean build
cargo clean
cargo build --release --benches
```

### Performance Lower Than Expected

Check CPU governor:
```bash
# Check current governor
cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor

# Set to performance mode
sudo cpufreq-set -g performance
```

## Reporting Results

Please report results with:

1. **Hardware specs** (CPU model, RAM, OS version)
2. **Benchmark outputs** (full Criterion results)
3. **Power measurements** (method used, idle vs active)
4. **System information** (from commands above)

**Submit to:** GitHub Issues with `[ARM64 Results]` tag

## xAI / SpaceX Validation

For xAI/SpaceX deployment validation:

1. **Power efficiency target:** Document ops/joule for comparison with GPU path
2. **Latency validation:** Confirm sub-ms performance for swarm trajectory solving
3. **Thermal characteristics:** Monitor sustained performance under load
4. **Edge deployment:** Test on battery power with power budget constraints

**Contact:** See repository for xAI integration details.
