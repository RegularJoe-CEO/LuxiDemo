// SPDX-FileCopyrightText: 2025 Eric Waller
// SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0

//! SIMD-optimized operations for Luxi Edge
//!
//! Provides vectorized implementations of common mathematical operations
//! with automatic fallback to scalar code when SIMD is unavailable.
//!
//! Supported SIMD instruction sets:
//! - AVX-512 (x86_64 with avx512f): 8x f64 lanes
//! - AVX2 (x86_64 with avx2): 4x f64 lanes  
//! - ARM Neon (aarch64): 2x f64 lanes
//! - Scalar (fallback)

use std::arch::is_x86_feature_detected;

/// Polynomial evaluation: 2x³ - 3x² + 5x - 1
/// Used for benchmarking vectorized arithmetic operations
pub fn polynomial_eval(data: &mut [f64]) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx512f") {
            unsafe { polynomial_eval_avx512(data); }
            return;
        }
        if is_x86_feature_detected!("avx2") {
            unsafe { polynomial_eval_avx2(data); }
            return;
        }
    }
    
    #[cfg(target_arch = "aarch64")]
    {
        polynomial_eval_neon(data);
        return;
    }
    
    polynomial_eval_scalar(data);
}

/// FMA operations: (x * 2.5 + 1.3) * x + 0.7
/// Tests fused multiply-add performance
pub fn fma_eval(data: &mut [f64]) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx512f") {
            unsafe { fma_eval_avx512(data); }
            return;
        }
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            unsafe { fma_eval_avx2(data); }
            return;
        }
    }
    
    #[cfg(target_arch = "aarch64")]
    {
        fma_eval_neon(data);
        return;
    }
    
    fma_eval_scalar(data);
}

/// Sin*Cos evaluation: sin(x) * cos(x)
/// Common trigonometric operation in physics simulations
pub fn sin_cos_eval(data: &mut [f64]) {
    // Note: No SIMD sin/cos in standard intrinsics
    // Would require approximations like Sleef library
    // For now, use scalar libm which is already optimized
    sin_cos_eval_scalar(data);
}

// ============================================================================
// Scalar Implementations (Baseline)
// ============================================================================

fn polynomial_eval_scalar(data: &mut [f64]) {
    for value in data.iter_mut() {
        let x = *value;
        *value = 2.0 * x * x * x - 3.0 * x * x + 5.0 * x - 1.0;
    }
}

fn fma_eval_scalar(data: &mut [f64]) {
    for value in data.iter_mut() {
        let x = *value;
        *value = x.mul_add(2.5, 1.3).mul_add(x, 0.7);
    }
}

fn sin_cos_eval_scalar(data: &mut [f64]) {
    for value in data.iter_mut() {
        let v = *value;
        *value = v.sin() * v.cos();
    }
}

// ============================================================================
// AVX-512 Implementations (8x f64 lanes)
// ============================================================================

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
unsafe fn polynomial_eval_avx512(data: &mut [f64]) {
    use std::arch::x86_64::*;
    
    let len = data.len();
    let lanes = 8; // AVX-512 processes 8 f64s at once
    let full_vecs = len / lanes;
    
    let two = _mm512_set1_pd(2.0);
    let three = _mm512_set1_pd(3.0);
    let five = _mm512_set1_pd(5.0);
    let one = _mm512_set1_pd(1.0);
    
    for i in 0..full_vecs {
        let idx = i * lanes;
        let ptr = data.as_ptr().add(idx);
        
        // Load 8 f64 values
        let x = _mm512_loadu_pd(ptr);
        
        // x²
        let x2 = _mm512_mul_pd(x, x);
        // x³
        let x3 = _mm512_mul_pd(x2, x);
        
        // 2x³
        let term1 = _mm512_mul_pd(two, x3);
        // -3x²
        let term2 = _mm512_mul_pd(three, x2);
        // 5x
        let term3 = _mm512_mul_pd(five, x);
        
        // 2x³ - 3x²
        let result = _mm512_sub_pd(term1, term2);
        // + 5x
        let result = _mm512_add_pd(result, term3);
        // - 1
        let result = _mm512_sub_pd(result, one);
        
        let out_ptr = data.as_mut_ptr().add(idx);
        _mm512_storeu_pd(out_ptr, result);
    }
    
    // Handle remainder with scalar code
    for i in (full_vecs * lanes)..len {
        let x = data[i];
        data[i] = 2.0 * x * x * x - 3.0 * x * x + 5.0 * x - 1.0;
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
unsafe fn fma_eval_avx512(data: &mut [f64]) {
    use std::arch::x86_64::*;
    
    let len = data.len();
    let lanes = 8;
    let full_vecs = len / lanes;
    
    let c1 = _mm512_set1_pd(2.5);
    let c2 = _mm512_set1_pd(1.3);
    let c3 = _mm512_set1_pd(0.7);
    
    for i in 0..full_vecs {
        let idx = i * lanes;
        let ptr = data.as_ptr().add(idx);
        let x = _mm512_loadu_pd(ptr);
        
        // x * 2.5 + 1.3 using FMA
        let temp = _mm512_fmadd_pd(x, c1, c2);
        // temp * x + 0.7 using FMA
        let result = _mm512_fmadd_pd(temp, x, c3);
        
        let out_ptr = data.as_mut_ptr().add(idx);
        _mm512_storeu_pd(out_ptr, result);
    }
    
    // Handle remainder
    for i in (full_vecs * lanes)..len {
        let x = data[i];
        data[i] = x.mul_add(2.5, 1.3).mul_add(x, 0.7);
    }
}

// ============================================================================
// AVX2 Implementations (4x f64 lanes)
// ============================================================================

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn polynomial_eval_avx2(data: &mut [f64]) {
    use std::arch::x86_64::*;
    
    let len = data.len();
    let lanes = 4; // AVX2 processes 4 f64s at once
    let full_vecs = len / lanes;
    
    let two = _mm256_set1_pd(2.0);
    let three = _mm256_set1_pd(3.0);
    let five = _mm256_set1_pd(5.0);
    let one = _mm256_set1_pd(1.0);
    
    for i in 0..full_vecs {
        let idx = i * lanes;
        let ptr = data.as_ptr().add(idx);
        let x = _mm256_loadu_pd(ptr);
        
        let x2 = _mm256_mul_pd(x, x);
        let x3 = _mm256_mul_pd(x2, x);
        
        let term1 = _mm256_mul_pd(two, x3);
        let term2 = _mm256_mul_pd(three, x2);
        let term3 = _mm256_mul_pd(five, x);
        
        let result = _mm256_sub_pd(term1, term2);
        let result = _mm256_add_pd(result, term3);
        let result = _mm256_sub_pd(result, one);
        
        let out_ptr = data.as_mut_ptr().add(idx);
        _mm256_storeu_pd(out_ptr, result);
    }
    
    // Handle remainder
    for i in (full_vecs * lanes)..len {
        let x = data[i];
        data[i] = 2.0 * x * x * x - 3.0 * x * x + 5.0 * x - 1.0;
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2", enable = "fma")]
unsafe fn fma_eval_avx2(data: &mut [f64]) {
    use std::arch::x86_64::*;
    
    let len = data.len();
    let lanes = 4;
    let full_vecs = len / lanes;
    
    let c1 = _mm256_set1_pd(2.5);
    let c2 = _mm256_set1_pd(1.3);
    let c3 = _mm256_set1_pd(0.7);
    
    for i in 0..full_vecs {
        let idx = i * lanes;
        let ptr = data.as_ptr().add(idx);
        let x = _mm256_loadu_pd(ptr);
        
        let temp = _mm256_fmadd_pd(x, c1, c2);
        let result = _mm256_fmadd_pd(temp, x, c3);
        
        let out_ptr = data.as_mut_ptr().add(idx);
        _mm256_storeu_pd(out_ptr, result);
    }
    
    // Handle remainder
    for i in (full_vecs * lanes)..len {
        let x = data[i];
        data[i] = x.mul_add(2.5, 1.3).mul_add(x, 0.7);
    }
}

// ============================================================================
// ARM Neon Implementations (2x f64 lanes)
// ============================================================================

#[cfg(target_arch = "aarch64")]
fn polynomial_eval_neon(data: &mut [f64]) {
    use std::arch::aarch64::*;
    
    let len = data.len();
    let lanes = 2; // Neon processes 2 f64s at once
    let full_vecs = len / lanes;
    
    unsafe {
        let two = vdupq_n_f64(2.0);
        let three = vdupq_n_f64(3.0);
        let five = vdupq_n_f64(5.0);
        let one = vdupq_n_f64(1.0);
        
        for i in 0..full_vecs {
            let idx = i * lanes;
            let ptr = data.as_ptr().add(idx);
            let x = vld1q_f64(ptr);
            
            let x2 = vmulq_f64(x, x);
            let x3 = vmulq_f64(x2, x);
            
            let term1 = vmulq_f64(two, x3);
            let term2 = vmulq_f64(three, x2);
            let term3 = vmulq_f64(five, x);
            
            let result = vsubq_f64(term1, term2);
            let result = vaddq_f64(result, term3);
            let result = vsubq_f64(result, one);
            
            let out_ptr = data.as_mut_ptr().add(idx);
            vst1q_f64(out_ptr, result);
        }
    }
    
    // Handle remainder
    for i in (full_vecs * lanes)..len {
        let x = data[i];
        data[i] = 2.0 * x * x * x - 3.0 * x * x + 5.0 * x - 1.0;
    }
}

#[cfg(target_arch = "aarch64")]
fn fma_eval_neon(data: &mut [f64]) {
    use std::arch::aarch64::*;
    
    let len = data.len();
    let lanes = 2;
    let full_vecs = len / lanes;
    
    unsafe {
        let c1 = vdupq_n_f64(2.5);
        let c2 = vdupq_n_f64(1.3);
        let c3 = vdupq_n_f64(0.7);
        
        for i in 0..full_vecs {
            let idx = i * lanes;
            let ptr = data.as_ptr().add(idx);
            let x = vld1q_f64(ptr);
            
            // x * 2.5 + 1.3
            let temp = vfmaq_f64(c2, x, c1);
            // temp * x + 0.7
            let result = vfmaq_f64(c3, temp, x);
            
            let out_ptr = data.as_mut_ptr().add(idx);
            vst1q_f64(out_ptr, result);
        }
    }
    
    // Handle remainder
    for i in (full_vecs * lanes)..len {
        let x = data[i];
        data[i] = x.mul_add(2.5, 1.3).mul_add(x, 0.7);
    }
}

// ============================================================================
// CPU Feature Detection
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimdCapability {
    Avx512,
    Avx2,
    Neon,
    Scalar,
}

/// Detect available SIMD capabilities at runtime
pub fn detect_simd_capability() -> SimdCapability {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx512f") {
            return SimdCapability::Avx512;
        }
        if is_x86_feature_detected!("avx2") {
            return SimdCapability::Avx2;
        }
    }
    
    #[cfg(target_arch = "aarch64")]
    {
        // Neon is always available on aarch64
        return SimdCapability::Neon;
    }
    
    SimdCapability::Scalar
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_eval_correctness() {
        let mut data: Vec<f64> = vec![1.0, 2.0, 3.0, -1.0, 0.5];
        let expected: Vec<f64> = data.iter()
            .map(|&x| 2.0 * x * x * x - 3.0 * x * x + 5.0 * x - 1.0)
            .collect();
        
        polynomial_eval(&mut data);
        
        for (i, (&result, &expect)) in data.iter().zip(expected.iter()).enumerate() {
            assert!((result - expect).abs() < 1e-10, 
                "Mismatch at index {}: {} != {}", i, result, expect);
        }
    }

    #[test]
    fn test_fma_eval_correctness() {
        let mut data: Vec<f64> = vec![1.0, 2.0, 3.0, -1.0, 0.5];
        let expected: Vec<f64> = data.iter()
            .map(|&x| x.mul_add(2.5, 1.3).mul_add(x, 0.7))
            .collect();
        
        fma_eval(&mut data);
        
        for (i, (&result, &expect)) in data.iter().zip(expected.iter()).enumerate() {
            assert!((result - expect).abs() < 1e-10,
                "Mismatch at index {}: {} != {}", i, result, expect);
        }
    }

    #[test]
    fn test_detect_simd_capability() {
        let cap = detect_simd_capability();
        println!("Detected SIMD capability: {:?}", cap);
        // Should detect at least Scalar
        assert!(cap == SimdCapability::Scalar 
                || cap == SimdCapability::Avx2 
                || cap == SimdCapability::Avx512
                || cap == SimdCapability::Neon);
    }
}
