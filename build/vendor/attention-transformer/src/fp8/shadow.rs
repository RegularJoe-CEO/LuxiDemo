//! FP8 shadow weights: store E4M3, decode to f32 in fixed order; receipt matches gold.

use super::e4m3::{decode_e4m3, encode_e4m3};

#[derive(Clone, Debug)]
pub struct Fp8ShadowMatrix {
    pub codes: Vec<u8>,
    pub rows: usize,
    pub cols: usize,
}

#[derive(Clone, Debug)]
pub struct Fp8ShadowTensor {
    pub matrices: Vec<Fp8ShadowMatrix>,
}

/// Build shadow; returns Err if any element does not round-trip exactly to f32 bits.
pub fn build_fp8_shadow(matrix: &[f32], rows: usize, cols: usize) -> Result<Fp8ShadowMatrix, String> {
    if matrix.len() != rows * cols {
        return Err("matrix len mismatch".into());
    }
    let mut codes = Vec::with_capacity(matrix.len());
    for (i, &v) in matrix.iter().enumerate() {
        let code = encode_e4m3(v);
        let back = decode_e4m3(code);
        if back.to_bits() != v.to_bits() {
            return Err(format!(
                "fp8 shadow not exact at {i}: in={v:?} back={back:?}"
            ));
        }
        codes.push(code);
    }
    Ok(Fp8ShadowMatrix { codes, rows, cols })
}

/// Deterministic row-major decode.
pub fn decode_shadow(m: &Fp8ShadowMatrix) -> Vec<f32> {
    m.codes.iter().map(|&b| decode_e4m3(b)).collect()
}

pub fn max_abs_diff(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).abs())
        .fold(0.0f32, f32::max)
}

/// Run f32 matmul on decoded shadow weights — receipt-identical when shadow is exact.
pub fn forward_fp8_shadow_f32(
    activations: &[f32],
    shadow: &Fp8ShadowMatrix,
    m: usize,
    k: usize,
    n: usize,
) -> Vec<f32> {
    let w = decode_shadow(shadow);
    let mut out = vec![0.0f32; m * n];
    for row in 0..m {
        for col in 0..n {
            let mut acc = 0.0f32;
            for p in 0..k {
                acc += activations[row * k + p] * w[p * n + col];
            }
            out[row * n + col] = acc;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wnsm_transformer::sha256_of_f32_slice;

    #[test]
    fn fp8_shadow_matches_f32_receipt() {
        let w: Vec<f32> = (0..64)
            .map(|i| match i % 4 {
                0 => 0.0f32,
                1 => 1.0,
                2 => 2.0,
                _ => 4.0,
            })
            .collect();
        let shadow = build_fp8_shadow(&w, 8, 8).expect("exact grid");
        let a: Vec<f32> = (0..32).map(|i| (i as f32) * 0.01).collect();
        let f32_out = forward_fp8_shadow_f32(&a, &shadow, 4, 8, 8);
        let direct: Vec<f32> = {
            let mut out = vec![0.0f32; 32];
            for row in 0..4 {
                for col in 0..8 {
                    let mut acc = 0.0f32;
                    for p in 0..8 {
                        acc += a[row * 8 + p] * w[p * 8 + col];
                    }
                    out[row * 8 + col] = acc;
                }
            }
            out
        };
        assert_eq!(max_abs_diff(&f32_out, &direct), 0.0);
        assert_eq!(
            sha256_of_f32_slice(&f32_out),
            sha256_of_f32_slice(&direct)
        );
    }
}