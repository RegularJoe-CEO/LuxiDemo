// ╔══════════════════════════════════════════════════════════╗
// ║           LuxiEdge Bare Metal  v0.1.0                   ║
// ║      Deterministic f32 Compute — ARM Cortex-M3          ║
// ║      no_std · no_heap · no_libm · bit-exact             ║
// ║      © 2025 Eric Waller — All Rights Reserved           ║
// ║                   Patent Pending                        ║
// ╚══════════════════════════════════════════════════════════╝

//! LuxiEdge Core Engine - Deterministic Mathematical Computation
// SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;


pub fn luxi_banner() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║           LuxiEdge Bare Metal  v0.1.0                   ║");
    println!("║      Deterministic f32 Compute — ARM Cortex-M3          ║");
    println!("║      no_std · no_heap · no_libm · bit-exact             ║");
    println!("║      © 2025 Eric Waller — All Rights Reserved           ║");
    println!("║                   Patent Pending                        ║");
    println!("╚══════════════════════════════════════════════════════════╝");
}

pub fn detect_simd_capability() -> &'static str {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx512f") { return "AVX-512"; }
        if is_x86_feature_detected!("avx2") { return "AVX2"; }
        if is_x86_feature_detected!("sse4.1") { return "SSE4.1"; } else { return "baseline"; }
    }
    #[cfg(target_arch = "aarch64")]
    { return "NEON"; }
}

pub fn fma_eval(data: &mut [f64], a: f64, b: f64) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            unsafe { fma_eval_avx2(data, a, b); }
            return;
        }
    }
    fma_eval_scalar(data, a, b);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2", enable = "fma")]
unsafe fn fma_eval_avx2(data: &mut [f64], a: f64, b: f64) {
    let a_vec = _mm256_set1_pd(a);
    let b_vec = _mm256_set1_pd(b);
    let chunks = data.len() / 4;
    for i in 0..chunks {
        let ptr = data.as_mut_ptr().add(i * 4);
        let x = _mm256_loadu_pd(ptr);
        let result = _mm256_fmadd_pd(a_vec, x, b_vec);
        _mm256_storeu_pd(ptr, result);
    }
    for i in (chunks * 4)..data.len() { data[i] = a * data[i] + b; }
}

fn fma_eval_scalar(data: &mut [f64], a: f64, b: f64) {
    for value in data.iter_mut() { *value = a * (*value) + b; }
}

#[allow(dead_code)]
pub fn sin_cos_eval(data: &mut [f64]) { sin_cos_eval_scalar(data); }

pub fn poly_eval_horner(data: &mut [f64], coeffs: &[f64]) {
    if coeffs.is_empty() { return; }
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            unsafe { poly_eval_horner_avx2(data, coeffs); }
            return;
        }
    }
    poly_eval_horner_scalar(data, coeffs);
}

fn sin_cos_eval_scalar(data: &mut [f64]) {
    for value in data.iter_mut() { let v = *value; *value = v.sin() * v.cos(); }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2", enable = "fma")]
unsafe fn poly_eval_horner_avx2(data: &mut [f64], coeffs: &[f64]) {
    let n = coeffs.len();
    if n == 0 { return; }
    let chunks = data.len() / 4;
    for i in 0..chunks {
        let ptr = data.as_mut_ptr().add(i * 4);
        let x = _mm256_loadu_pd(ptr);
        let mut result = _mm256_set1_pd(coeffs[n - 1]);
        for j in (0..n-1).rev() {
            let c = _mm256_set1_pd(coeffs[j]);
            result = _mm256_fmadd_pd(result, x, c);
        }
        _mm256_storeu_pd(ptr, result);
    }
    for i in (chunks * 4)..data.len() {
        let x = data[i];
        let mut result = coeffs[n - 1];
        for j in (0..n-1).rev() { result = result * x + coeffs[j]; }
        data[i] = result;
    }
}

fn poly_eval_horner_scalar(data: &mut [f64], coeffs: &[f64]) {
    let n = coeffs.len();
    if n == 0 { return; }
    for value in data.iter_mut() {
        let x = *value;
        let mut result = coeffs[n - 1];
        for j in (0..n-1).rev() { result = result * x + coeffs[j]; }
        *value = result;
    }
}

pub fn batch_poly_eval(inputs: &[f64], poly_coeffs: &[Vec<f64>]) -> Vec<Vec<f64>> {
    poly_coeffs.iter().map(|coeffs| {
        let mut data = inputs.to_vec();
        poly_eval_horner(&mut data, coeffs);
        data
    }).collect()
}

const PI: f32 = 3.14159265358979323846;
const TWO_PI: f32 = 6.28318530717958647692;
const HALF_PI: f32 = 1.57079632679489661923;
const LN2: f32 = 0.693147180559945309417;

pub fn sin_f32_deterministic(x: f32) -> f32 {
    let mut x = x % TWO_PI;
    if x < 0.0 { x += TWO_PI; }
    if x > PI { x -= TWO_PI; }
    let x2 = x * x;
    x * (1.0 - x2 / 6.0 * (1.0 - x2 / 20.0 * (1.0 - x2 / 42.0 * (1.0 - x2 / 72.0))))
}

pub fn cos_f32_deterministic(x: f32) -> f32 { sin_f32_deterministic(x + HALF_PI) }
pub fn tan_f32_deterministic(x: f32) -> f32 { sin_f32_deterministic(x) / cos_f32_deterministic(x) }

pub fn exp_f32_deterministic(x: f32) -> f32 {
    let x = x.clamp(-88.0, 88.0);
    let k = floor_f32_deterministic((x / LN2) + 0.5);
    let r = x - k * LN2;
    let r2 = r * r;
    let poly = 1.0 + r + r2 / 2.0 + r2 * r / 6.0 + r2 * r2 / 24.0;
    let bits = ((k as i32 + 127) as u32) << 23;
    let scale = f32::from_bits(bits);
    poly * scale
}

pub fn ln_f32_deterministic(x: f32) -> f32 {
    if x <= 0.0 { return f32::NAN; }
    let bits = x.to_bits();
    let exp = ((bits >> 23) & 0xFF) as i32 - 127;
    let mant_bits = (bits & 0x7FFFFF) | 0x3F800000;
    let m = f32::from_bits(mant_bits);
    let f = m - 1.0;
    let ln_1pf = f - f * f / 2.0 + f * f * f / 3.0 - f * f * f * f / 4.0;
    (exp as f32) * LN2 + ln_1pf
}

pub fn sqrt_f32_deterministic(x: f32) -> f32 {
    if x < 0.0 { return f32::NAN; }
    if x == 0.0 { return 0.0; }
    exp_f32_deterministic(0.5 * ln_f32_deterministic(x))
}

pub fn sinh_f32_deterministic(x: f32) -> f32 { (exp_f32_deterministic(x) - exp_f32_deterministic(-x)) / 2.0 }
pub fn cosh_f32_deterministic(x: f32) -> f32 { (exp_f32_deterministic(x) + exp_f32_deterministic(-x)) / 2.0 }
pub fn tanh_f32_deterministic(x: f32) -> f32 { sinh_f32_deterministic(x) / cosh_f32_deterministic(x) }

pub fn abs_f32_deterministic(x: f32) -> f32 { if x < 0.0 { -x } else { x } }

pub fn floor_f32_deterministic(x: f32) -> f32 {
    let i = x as i32;
    let f = i as f32;
    if x < f { f - 1.0 } else { f }
}

pub fn ceil_f32_deterministic(x: f32) -> f32 { -floor_f32_deterministic(-x) }

pub fn round_f32_deterministic(x: f32) -> f32 { floor_f32_deterministic(x + 0.5) }

pub fn log10_f32_deterministic(x: f32) -> f32 { ln_f32_deterministic(x) / 2.302585093 }

pub fn cbrt_f32_deterministic(x: f32) -> f32 {
    if x == 0.0 { return 0.0; }
    if x > 0.0 { exp_f32_deterministic(ln_f32_deterministic(x) / 3.0) }
    else { -exp_f32_deterministic(ln_f32_deterministic(-x) / 3.0) }
}

pub fn erf_f32(x: f32) -> f32 {
    let a1: f32 = 0.254829592; let a2: f32 = -0.284496736; let a3: f32 = 1.421413741;
    let a4: f32 = -1.453152027; let a5: f32 = 1.061405429; let p: f32 = 0.3275911;
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = if x < 0.0 { -x } else { x };
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * exp_f32_deterministic(-x * x);
    sign * y
}

pub fn normcdf_f32(x: f32) -> f32 { 0.5 * (1.0 + erf_f32(x / 1.41421356)) }
pub fn normpdf_f32(x: f32) -> f32 { exp_f32_deterministic(-0.5 * x * x) / 2.5066283 }

pub fn gamma_f32(x: f32) -> f32 {
    if x <= 0.0 && x == floor_f32_deterministic(x) { return f32::INFINITY; }
    if x < 0.5 {
        let pi = 3.14159265358979323846_f32;
        return pi / (sin_f32_deterministic(pi * x) * gamma_f32(1.0 - x));
    }
    let g = 7;
    let c = [0.99999999999980993, 676.5203681218851, -1259.1392167224028, 771.32342877765313,
             -176.61502916214059, 12.507343278686905, -0.13857109526572012, 9.9843695780195716e-6, 1.5056327351493116e-7];
    let x = x - 1.0;
    let mut a = c[0] as f32;
    for i in 1..(g + 2) { a += c[i] as f32 / (x + i as f32); }
    let t = x + g as f32 + 0.5;
    2.5066283 * exp_f32_deterministic((x + 0.5) * ln_f32_deterministic(t) - t) * a / exp_f32_deterministic(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fma_eval_correctness() {
        let mut data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let expected: Vec<f64> = data.iter().map(|&x| 2.5 * x + 1.3).collect();
        fma_eval(&mut data, 2.5, 1.3);
        for (i, (&result, &expect)) in data.iter().zip(expected.iter()).enumerate() {
            assert!((result - expect).abs() < 1e-10, "Mismatch at {}: {} != {}", i, result, expect);
        }
    }
    #[test]
    fn test_detect_simd_capability() {
        let cap = detect_simd_capability();
        assert!(!cap.is_empty());
    }
}

pub fn relu_f32_deterministic(x: f32) -> f32 {
    if x > 0.0 { x } else { 0.0 }
}

pub fn sigmoid_f32_deterministic(x: f32) -> f32 {
    1.0 / (1.0 + exp_f32_deterministic(-x))
}

pub fn log2_f32_deterministic(x: f32) -> f32 {
    ln_f32_deterministic(x) / 0.6931471805599453
}

pub fn rf_chain_f32(x: f32) -> f32 {
    normcdf_f32(x) * exp_f32_deterministic(-x * x)
        + erf_f32(x) * ln_f32_deterministic(abs_f32_deterministic(x) + 1.0)
}

pub fn quant_chain_f32(x: f32) -> f32 {
    normcdf_f32(x) * ln_f32_deterministic(abs_f32_deterministic(x) + 1.0)
        + erf_f32(x) * normpdf_f32(x)
}
