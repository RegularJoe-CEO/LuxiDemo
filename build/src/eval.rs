use sha2::{Digest, Sha256};

pub fn hash_f64_slice(values: &[f64]) -> String {
    let mut hasher = Sha256::new();
    for val in values {
        hasher.update(val.to_le_bytes());
    }
    format!("{:x}", hasher.finalize())
}

pub fn hash_f32_slice(values: &[f32]) -> String {
    let mut hasher = Sha256::new();
    for val in values {
        hasher.update(val.to_bits().to_le_bytes());
    }
    format!("{:x}", hasher.finalize())
}

pub mod lite {
    use super::hash_f64_slice;

    pub fn evaluate(expr: &str, values: &[f64], use_f64: bool) -> (Vec<f64>, String) {
        let results = if use_f64 {
            eval_f64(expr, values)
        } else {
            eval_f32(expr, values)
        };
        (results.clone(), hash_f64_slice(&results))
    }

    fn eval_f64(expr: &str, values: &[f64]) -> Vec<f64> {
        match expr {
            "sin(x)*cos(x)" => values.iter().map(|&x| x.sin() * x.cos()).collect(),
            "sin(x)" => values.iter().map(|&x| x.sin()).collect(),
            "cos(x)" => values.iter().map(|&x| x.cos()).collect(),
            "exp(x)" => values.iter().map(|&x| x.exp()).collect(),
            "ln(x)" => values.iter().map(|&x| x.ln()).collect(),
            "sqrt(x)" => values.iter().map(|&x| x.sqrt()).collect(),
            "x^2" => values.iter().map(|&x| x * x).collect(),
            "x^3" => values.iter().map(|&x| x * x * x).collect(),
            "erf(x)" => values
                .iter()
                .map(|&x| crate::engine::erf_f32(x as f32) as f64)
                .collect(),
            "normcdf(x)" => values
                .iter()
                .map(|&x| crate::engine::normcdf_f32(x as f32) as f64)
                .collect(),
            "normpdf(x)" => values
                .iter()
                .map(|&x| crate::engine::normpdf_f32(x as f32) as f64)
                .collect(),
            "gamma(x)" => values
                .iter()
                .map(|&x| crate::engine::gamma_f32(x as f32) as f64)
                .collect(),
            _ => values.to_vec(),
        }
    }

    fn eval_f32(expr: &str, values: &[f64]) -> Vec<f64> {
        match expr {
            "sin(x)*cos(x)" => values
                .iter()
                .map(|&x| (x as f32).sin() * (x as f32).cos())
                .map(|r| r as f64)
                .collect(),
            "sin(x)" => values.iter().map(|&x| (x as f32).sin() as f64).collect(),
            "cos(x)" => values.iter().map(|&x| (x as f32).cos() as f64).collect(),
            "exp(x)" => values.iter().map(|&x| (x as f32).exp() as f64).collect(),
            "ln(x)" => values.iter().map(|&x| (x as f32).ln() as f64).collect(),
            "sqrt(x)" => values.iter().map(|&x| (x as f32).sqrt() as f64).collect(),
            "x^2" => values
                .iter()
                .map(|&x| {
                    let v = x as f32;
                    (v * v) as f64
                })
                .collect(),
            "x^3" => values
                .iter()
                .map(|&x| {
                    let v = x as f32;
                    (v * v * v) as f64
                })
                .collect(),
            "erf(x)" => values
                .iter()
                .map(|&x| crate::engine::erf_f32(x as f32) as f64)
                .collect(),
            "normcdf(x)" => values
                .iter()
                .map(|&x| crate::engine::normcdf_f32(x as f32) as f64)
                .collect(),
            "normpdf(x)" => values
                .iter()
                .map(|&x| crate::engine::normpdf_f32(x as f32) as f64)
                .collect(),
            "gamma(x)" => values
                .iter()
                .map(|&x| crate::engine::gamma_f32(x as f32) as f64)
                .collect(),
            _ => values.to_vec(),
        }
    }

    pub fn supported_exprs() -> &'static [&'static str] {
        &[
            "sin(x)",
            "cos(x)",
            "sin(x)*cos(x)",
            "exp(x)",
            "ln(x)",
            "sqrt(x)",
            "x^2",
            "x^3",
            "erf(x)",
            "normcdf(x)",
            "normpdf(x)",
            "gamma(x)",
        ]
    }
}

pub mod demo {
    use super::hash_f64_slice;
    use crate::engine::*;

    pub fn evaluate(expr: &str, values: &[f64], use_f64: bool) -> (Vec<f64>, String) {
        let results = if use_f64 {
            eval_f64(expr, values)
        } else {
            eval_f32(expr, values)
        };
        (results.clone(), hash_f64_slice(&results))
    }

    fn eval_f64(expr: &str, values: &[f64]) -> Vec<f64> {
        match expr {
            "sin(x)" => values.iter().map(|&x| x.sin()).collect(),
            "cos(x)" => values.iter().map(|&x| x.cos()).collect(),
            "tan(x)" => values.iter().map(|&x| x.tan()).collect(),
            "exp(x)" => values.iter().map(|&x| x.exp()).collect(),
            "ln(x)" => values.iter().map(|&x| x.ln()).collect(),
            "log10(x)" => values.iter().map(|&x| x.log10()).collect(),
            "sqrt(x)" => values.iter().map(|&x| x.sqrt()).collect(),
            "cbrt(x)" => values.iter().map(|&x| x.cbrt()).collect(),
            "abs(x)" => values.iter().map(|&x| x.abs()).collect(),
            "floor(x)" => values.iter().map(|&x| x.floor()).collect(),
            "ceil(x)" => values.iter().map(|&x| x.ceil()).collect(),
            "round(x)" => values.iter().map(|&x| x.round()).collect(),
            "sinh(x)" => values.iter().map(|&x| x.sinh()).collect(),
            "cosh(x)" => values.iter().map(|&x| x.cosh()).collect(),
            "tanh(x)" => values.iter().map(|&x| x.tanh()).collect(),
            "sin(x)*cos(x)" => values.iter().map(|&x| x.sin() * x.cos()).collect(),
            "x^2" => values.iter().map(|&x| x * x).collect(),
            "x^3" => values.iter().map(|&x| x * x * x).collect(),
            "erf(x)" => values.iter().map(|&x| erf_f32(x as f32) as f64).collect(),
            "normcdf(x)" => values.iter().map(|&x| normcdf_f32(x as f32) as f64).collect(),
            "normpdf(x)" => values.iter().map(|&x| normpdf_f32(x as f32) as f64).collect(),
            "gamma(x)" => values.iter().map(|&x| gamma_f32(x as f32) as f64).collect(),
            "relu(x)" => values
                .iter()
                .map(|&x| relu_f32_deterministic(x as f32) as f64)
                .collect(),
            "sigmoid(x)" => values
                .iter()
                .map(|&x| sigmoid_f32_deterministic(x as f32) as f64)
                .collect(),
            "log2(x)" => values
                .iter()
                .map(|&x| log2_f32_deterministic(x as f32) as f64)
                .collect(),
            "rf_chain(x)" => values.iter().map(|&x| rf_chain_f32(x as f32) as f64).collect(),
            "quant_chain(x)" => values
                .iter()
                .map(|&x| quant_chain_f32(x as f32) as f64)
                .collect(),
            "gelu(x)" => values
                .iter()
                .map(|&x| gelu_f32(x as f32) as f64)
                .collect(),
            "silu(x)" => values
                .iter()
                .map(|&x| silu_f32(x as f32) as f64)
                .collect(),
            "rms_norm(x)" => values
                .iter()
                .map(|&x| rms_norm_scalar(x as f32) as f64)
                .collect(),
            _ => values.to_vec(),
        }
    }

    fn eval_f32(expr: &str, values: &[f64]) -> Vec<f64> {
        match expr {
            "sin(x)" => values
                .iter()
                .map(|&x| sin_f32_deterministic(x as f32) as f64)
                .collect(),
            "cos(x)" => values
                .iter()
                .map(|&x| cos_f32_deterministic(x as f32) as f64)
                .collect(),
            "tan(x)" => values
                .iter()
                .map(|&x| tan_f32_deterministic(x as f32) as f64)
                .collect(),
            "exp(x)" => values
                .iter()
                .map(|&x| exp_f32_deterministic(x as f32) as f64)
                .collect(),
            "ln(x)" => values
                .iter()
                .map(|&x| ln_f32_deterministic(x as f32) as f64)
                .collect(),
            "log10(x)" => values
                .iter()
                .map(|&x| log10_f32_deterministic(x as f32) as f64)
                .collect(),
            "sqrt(x)" => values
                .iter()
                .map(|&x| sqrt_f32_deterministic(x as f32) as f64)
                .collect(),
            "cbrt(x)" => values
                .iter()
                .map(|&x| cbrt_f32_deterministic(x as f32) as f64)
                .collect(),
            "abs(x)" => values
                .iter()
                .map(|&x| abs_f32_deterministic(x as f32) as f64)
                .collect(),
            "floor(x)" => values
                .iter()
                .map(|&x| floor_f32_deterministic(x as f32) as f64)
                .collect(),
            "ceil(x)" => values
                .iter()
                .map(|&x| ceil_f32_deterministic(x as f32) as f64)
                .collect(),
            "round(x)" => values
                .iter()
                .map(|&x| round_f32_deterministic(x as f32) as f64)
                .collect(),
            "sinh(x)" => values
                .iter()
                .map(|&x| sinh_f32_deterministic(x as f32) as f64)
                .collect(),
            "cosh(x)" => values
                .iter()
                .map(|&x| cosh_f32_deterministic(x as f32) as f64)
                .collect(),
            "tanh(x)" => values
                .iter()
                .map(|&x| tanh_f32_deterministic(x as f32) as f64)
                .collect(),
            "sin(x)*cos(x)" => values
                .iter()
                .map(|&x| {
                    sin_f32_deterministic(x as f32) as f64 * cos_f32_deterministic(x as f32) as f64
                })
                .collect(),
            "x^2" => values
                .iter()
                .map(|&x| {
                    let v = x as f32;
                    (v * v) as f64
                })
                .collect(),
            "x^3" => values
                .iter()
                .map(|&x| {
                    let v = x as f32;
                    (v * v * v) as f64
                })
                .collect(),
            "erf(x)" => values.iter().map(|&x| erf_f32(x as f32) as f64).collect(),
            "normcdf(x)" => values.iter().map(|&x| normcdf_f32(x as f32) as f64).collect(),
            "normpdf(x)" => values.iter().map(|&x| normpdf_f32(x as f32) as f64).collect(),
            "gamma(x)" => values.iter().map(|&x| gamma_f32(x as f32) as f64).collect(),
            "relu(x)" => values
                .iter()
                .map(|&x| relu_f32_deterministic(x as f32) as f64)
                .collect(),
            "sigmoid(x)" => values
                .iter()
                .map(|&x| sigmoid_f32_deterministic(x as f32) as f64)
                .collect(),
            "log2(x)" => values
                .iter()
                .map(|&x| log2_f32_deterministic(x as f32) as f64)
                .collect(),
            "rf_chain(x)" => values.iter().map(|&x| rf_chain_f32(x as f32) as f64).collect(),
            "quant_chain(x)" => values
                .iter()
                .map(|&x| quant_chain_f32(x as f32) as f64)
                .collect(),
            "gelu(x)" => values.iter().map(|&x| gelu_f32(x as f32) as f64).collect(),
            "silu(x)" => values.iter().map(|&x| silu_f32(x as f32) as f64).collect(),
            "rms_norm(x)" => values
                .iter()
                .map(|&x| rms_norm_scalar(x as f32) as f64)
                .collect(),
            _ => values.to_vec(),
        }
    }

    fn gelu_f32(x: f32) -> f32 {
        0.5 * x * (1.0 + erf_f32(x / 1.41421356))
    }

    fn silu_f32(x: f32) -> f32 {
        x * sigmoid_f32_deterministic(x)
    }

    fn rms_norm_scalar(x: f32) -> f32 {
        x / (x * x + 1e-5).sqrt()
    }

    pub fn supported_exprs() -> &'static [&'static str] {
        &[
            "sin(x)",
            "cos(x)",
            "tan(x)",
            "exp(x)",
            "ln(x)",
            "log10(x)",
            "log2(x)",
            "sqrt(x)",
            "cbrt(x)",
            "abs(x)",
            "floor(x)",
            "ceil(x)",
            "round(x)",
            "sinh(x)",
            "cosh(x)",
            "tanh(x)",
            "sin(x)*cos(x)",
            "x^2",
            "x^3",
            "erf(x)",
            "normcdf(x)",
            "normpdf(x)",
            "gamma(x)",
            "relu(x)",
            "sigmoid(x)",
            "rf_chain(x)",
            "quant_chain(x)",
            "gelu(x)",
            "silu(x)",
            "rms_norm(x)",
        ]
    }
}