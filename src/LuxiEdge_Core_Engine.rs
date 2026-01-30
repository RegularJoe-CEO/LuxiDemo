use std::f32::consts::PI;

pub fn sin_f32(x: f32) -> f32 {
    let x = x % (2.0 * PI);
    let x2 = x * x;
    let x3 = x2 * x;
    let x5 = x3 * x2;
    let x7 = x5 * x2;
    let x9 = x7 * x2;
    let x11 = x9 * x2;
    x - x3 / 6.0 + x5 / 120.0 - x7 / 5040.0 + x9 / 362880.0 - x11 / 39916800.0
}

pub fn cos_f32(x: f32) -> f32 {
    let x = x % (2.0 * PI);
    let x2 = x * x;
    let x4 = x2 * x2;
    let x6 = x4 * x2;
    let x8 = x6 * x2;
    let x10 = x8 * x2;
    1.0 - x2 / 2.0 + x4 / 24.0 - x6 / 720.0 + x8 / 40320.0 - x10 / 3628800.0
}

pub fn exp_f32(x: f32) -> f32 {
    if x > 88.0 { return f32::MAX; }
    if x < -88.0 { return 0.0; }
    let mut sum = 1.0_f32;
    let mut term = 1.0_f32;
    for i in 1..12 {
        term = term * x / i as f32;
        sum += term;
    }
    sum
}

pub fn ln_f32(x: f32) -> f32 {
    if x <= 0.0 { return f32::NAN; }
    if x == 1.0 { return 0.0; }
    let mut result = 0.0_f32;
    let mut y = x;
    while y >= 1.5 {
        y /= 2.0;
        result += 0.6931471805599453_f32;
    }
    while y < 0.5 {
        y *= 2.0;
        result -= 0.6931471805599453_f32;
    }
    let z = (y - 1.0) / (y + 1.0);
    let z2 = z * z;
    let z3 = z2 * z;
    let z5 = z3 * z2;
    let z7 = z5 * z2;
    result + 2.0 * (z + z3 / 3.0 + z5 / 5.0 + z7 / 7.0)
}

pub fn sqrt_f32(x: f32) -> f32 {
    if x < 0.0 { return f32::NAN; }
    if x == 0.0 { return 0.0; }
    let mut y = x;
    let mut prev = 0.0_f32;
    for _ in 0..10 {
        if (y - prev).abs() < 1e-6 { break; }
        prev = y;
        y = 0.5 * (y + x / y);
    }
    y
}

pub fn erf_f32(x: f32) -> f32 {
    let a1 =  0.254829592_f32;
    let a2 = -0.284496736_f32;
    let a3 =  1.421413741_f32;
    let a4 = -1.453152027_f32;
    let a5 =  1.061405429_f32;
    let p  =  0.3275911_f32;
    let sign = if x < 0.0 { -1.0_f32 } else { 1.0_f32 };
    let x = x.abs();
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * exp_f32(-x * x);
    sign * y
}

pub fn normcdf_f32(x: f32) -> f32 {
    0.5 * (1.0 + erf_f32(x / sqrt_f32(2.0)))
}

pub fn normpdf_f32(x: f32) -> f32 {
    exp_f32(-0.5 * x * x) / sqrt_f32(2.0 * PI)
}

// Fixed gamma function - Lanczos approximation
pub fn gamma_f32(x: f32) -> f32 {
    if x <= 0.0 { return f32::NAN; }
    
    // Use recursion for small values: Gamma(x) = Gamma(x+1) / x
    let mut x = x;
    let mut result = 1.0_f32;
    while x < 1.0 {
        result /= x;
        x += 1.0;
    }
    
    // Lanczos coefficients
    let p: [f64; 6] = [
        1.000000000190015,
        76.18009172947146,
        -86.50532032941677,
        24.01409824083091,
        -1.231739572450155,
        0.1208650973866179,
    ];
    
    let x64 = x as f64;
    let mut ser = p[0];
    for (i, &coef) in p.iter().enumerate().skip(1) {
        ser += coef / (x64 + i as f64);
    }
    
    let tmp = x64 + 5.5;
    let sqrt_2pi = (2.0 * std::f64::consts::PI).sqrt();
    let gamma_val = sqrt_2pi * ser * (tmp.powf(x64 + 0.5) * (-tmp).exp()) / x64;
    
    (gamma_val * result as f64) as f32
}
