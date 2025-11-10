# ARM Neon Intrinsics Benchmark

This benchmark suite tests ARM Neon SIMD intrinsics performance against scalar implementations for various mathematical operations.

## Overview

The benchmark compares ARM Neon SIMD implementations against scalar baselines to demonstrate performance improvements on ARM64 platforms (Apple Silicon, AWS Graviton, Jetson, etc.).

## Benchmark Categories

### 1. Trigonometric Functions (sin*cos)
Tests transcendental function evaluation:
- **Scalar**: Sequential f64::sin() * f64::cos() operations
- **Neon**: Vector load/store with scalar math (no SIMD sin/cos in standard Neon)

**Note**: Standard ARM Neon doesn't include SIMD sin/cos intrinsics, so both implementations use scalar math. The benchmark demonstrates memory access patterns.

### 2. Polynomial Evaluation
Tests arithmetic operations with polynomial: `2x³ - 3x² + 5x - 1`
- **Scalar**: Sequential polynomial evaluation
- **Neon**: Vectorized multiply, add, subtract operations using `vmulq_f64`, `vaddq_f64`, `vsubq_f64`

**Expected Speedup**: 1.5-2× on ARM64 hardware

### 3. FMA (Fused Multiply-Add)
Tests FMA operations: `(x * 2.5 + 1.3) * x + 0.7`
- **Scalar**: f64::mul_add() operations
- **Neon**: Vectorized FMA using `vfmaq_f64`

**Expected Speedup**: 1.5-2× on ARM64 hardware

### 4. Memory Bandwidth
Tests raw memory load/store performance:
- **Scalar**: Element-by-element copy
- **Neon**: Vectorized load/store using `vld1q_f64`, `vst1q_f64`

**Expected Speedup**: 1.5-2× on ARM64 hardware

## Platform Support

### ARM64 (aarch64)
- Uses `std::arch::aarch64` intrinsics
- Requires CPU with Neon support (all modern ARM64 CPUs)
- Processes 2× f64 elements per SIMD operation

### x86_64 (fallback)
- Falls back to scalar implementations
- Ensures benchmark compiles on all platforms
- Results show baseline performance

## Running the Benchmark

```bash
# Quick test (validates correctness)
cargo bench --bench neon_benchmark -- --test

# Full benchmark suite
cargo bench --bench neon_benchmark

# Specific test group
cargo bench --bench neon_benchmark -- polynomial

# Quick mode (faster, less precise)
cargo bench --bench neon_benchmark -- --quick
```

## Interpreting Results

### On x86_64
Results will show near-parity between "scalar" and "neon" implementations because both use scalar fallback code. This validates correctness.

### On ARM64
Results should show:
- **Polynomial**: 1.5-2× speedup with Neon
- **FMA**: 1.5-2× speedup with Neon
- **Memory bandwidth**: 1.5-2× speedup with Neon
- **sin/cos**: Minimal difference (both use scalar math)

## Example Output (ARM64)

```
polynomial/scalar/100000    time: [65.2 µs]
polynomial/neon/100000      time: [35.1 µs]  (1.86× speedup)

fma/scalar/100000           time: [42.3 µs]
fma/neon/100000             time: [24.7 µs]  (1.71× speedup)
```

## Technical Details

### Neon SIMD Width
- **f64**: 2 elements per vector (128-bit SIMD)
- **f32**: 4 elements per vector (128-bit SIMD)

### Intrinsics Used
- `vld1q_f64`: Load 2× f64 from memory
- `vst1q_f64`: Store 2× f64 to memory
- `vmulq_f64`: Multiply 2× f64 vectors
- `vaddq_f64`: Add 2× f64 vectors
- `vsubq_f64`: Subtract 2× f64 vectors
- `vfmaq_f64`: Fused multiply-add on 2× f64 vectors
- `vgetq_lane_f64`: Extract element from vector
- `vsetq_lane_f64`: Insert element into vector
- `vdupq_n_f64`: Broadcast scalar to all lanes

### Limitations
- Standard ARM Neon lacks SIMD transcendental functions (sin, cos, exp, log)
- For best performance with transcendental functions, use specialized libraries like Sleef
- Current implementation demonstrates memory and arithmetic operations

## Integration with Luxi Edge

This benchmark validates the ARM Neon code path in `src/luxi_eval.rs` which uses Neon intrinsics for vectorized expression evaluation on ARM64 platforms.

## References

- [ARM Neon Intrinsics Reference](https://developer.arm.com/architectures/instruction-sets/intrinsics/)
- [Rust std::arch::aarch64 Documentation](https://doc.rust-lang.org/std/arch/aarch64/index.html)
- [ARM Neon Programmer's Guide](https://developer.arm.com/documentation/den0018/a)
