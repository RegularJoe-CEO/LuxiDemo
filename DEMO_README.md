# Batch Throughput Demo

Visual demonstration of Luxi Edge multi-revolution Lambert solver batch throughput performance.

## Quick Start

```bash
# Interactive visual demo (recommended)
python3 demo_visual.py

# Bash script with live benchmarks (takes ~2-3 minutes)
./demo_batch_throughput.sh

# Python demo with live benchmarks (takes longer)
python3 demo_batch_throughput.py
```

## Demo Output

The visual demo (`demo_visual.py`) displays:

1. **System Information** - Platform, CPU, architecture
2. **Multi-Revolution Performance** - Latency and throughput for 1, 2, 4, and 8 revolutions
3. **Scalability Analysis** - Time per revolution, linear scaling verification
4. **Throughput Comparison** - Real-world scenario validation
5. **Use Cases** - SpaceX, satellite swarms, robotics, edge AI
6. **ARM64 Projections** - Expected Neon SIMD performance

## Performance Highlights

### Batch Solver Results (x86_64)

| Revolution Count | Latency | Throughput | Achievement |
|-----------------|---------|------------|-------------|
| Single (N=1) | 2.34 µs | 426,621/sec | Fast |
| Dual (N=2) | 4.32 µs | 231,481/sec | 2× scaling |
| Quad (N=4) | 8.31 µs | 120,337/sec | 4× scaling |
| **Swarm (N=8)** | **16.30 µs** | **61,350/sec** | **✨ Sub-ms!** |

### Key Achievements

- ✅ **Sub-millisecond solving:** 16.3 µs for 8-revolution swarm (61× faster than 1ms target)
- ✅ **Linear scaling:** ~2 µs per additional revolution count
- ✅ **High throughput:** 61,350 simultaneous solve-sets per second
- ✅ **Real-time ready:** Exceeds 1kHz control loop requirements

## Use Cases

### SpaceX Starship
Multi-revolution lunar and Mars transfer trajectory optimization with time constraints.

### Satellite Swarms
Real-time trajectory planning for formations requiring simultaneous evaluation of multiple orbital transfer options.

### Optimus Robot
Complex multi-waypoint path planning with timing constraints for autonomous navigation.

### Edge AI Drones
Battery-powered navigation with ARM64 Neon SIMD optimization for ultra-low power consumption.

## ARM64 Optimization

### Current Performance (x86_64 scalar)
- 8-revolution solve: 16.30 µs
- Throughput: 61,350 solve-sets/sec

### Projected Performance (ARM64 Neon)
- 8-revolution solve: **8-10 µs** (1.5-2× speedup expected)
- Throughput: **~120,000 solve-sets/sec**

**Neon SIMD Features:**
- 128-bit vector registers (2× f64 per operation)
- FMA (fused multiply-add) operations
- Optimized memory bandwidth
- Lower power consumption for battery-powered devices

## Testing on ARM64 Hardware

For validation on Raspberry Pi 5, AWS Graviton, or Apple Silicon:

1. See detailed testing guide: [`docs/ARM64_TESTING_GUIDE.md`](docs/ARM64_TESTING_GUIDE.md)
2. Run benchmarks: `cargo bench --bench lambert_benchmark -- multirev`
3. Capture power measurements (see guide for methods)
4. Compare results to x86_64 baseline

## Technical Details

### Implementation
- **Multi-rev TOF:** `src/lambert.rs::lambert_tof_multirev()`
- **Batch solver:** `src/lambert.rs::solve_multirev_batch()`
- **SIMD batch eval:** `src/lambert.rs::batch_tof_scalar/neon()`

### Benchmarks
- **Suite:** `benches/lambert_benchmark.rs`
- **Categories:** Single, dual, quad, 8-rev solvers + batch TOF evaluation
- **Framework:** Criterion.rs with optimized release builds

### Formula
```
TOF(a, N) = TOF_base(a) + 2π·N·√(a³/μ)
```

Where:
- `a` = semi-major axis
- `N` = number of complete revolutions
- `μ` = gravitational parameter

## Files

- `demo_visual.py` - Interactive visual demo (fastest, recommended)
- `demo_batch_throughput.py` - Python demo with live benchmarks
- `demo_batch_throughput.sh` - Bash script with live benchmarks
- `demo_output.txt` - Sample output from Python demo
- `demo_visual_output.txt` - Sample output from visual demo

## Requirements

- Rust ≥ 1.75.0
- Python 3.6+ (for Python demos)
- Bash (for shell script)
- Built project: `cargo build --release --benches`

## Related Documentation

- **ARM64 Testing:** [`docs/ARM64_TESTING_GUIDE.md`](docs/ARM64_TESTING_GUIDE.md)
- **xAI Applications:** [`docs/XAI_EXECUTIVE_SUMMARY.md`](docs/XAI_EXECUTIVE_SUMMARY.md)
- **Benchmark Data:** [`BENCHMARK_DATA.md`](BENCHMARK_DATA.md)
- **Implementation:** [`IMPLEMENTATION_SUMMARY.md`](IMPLEMENTATION_SUMMARY.md)
