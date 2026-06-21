//! Deterministic LayerNorm using Welford online statistics (single-pass, stable).

use crate::welford::WelfordState;

/// Single-pass LayerNorm.
pub fn layernorm(input: &[f32], gamma: &[f32], beta: &[f32], eps: f32) -> Vec<f32> {
    let mut state = WelfordState::new();
    for &x in input {
        state.update(x);
    }
    let mean = state.mean;
    let std = state.std(eps);

    input
        .iter()
        .zip(gamma.iter())
        .zip(beta.iter())
        .map(|((&x, &g), &b)| (x - mean) / std * g + b)
        .collect()
}

/// Batched LayerNorm.
pub fn layernorm_batched(
    input: &[f32],
    gamma: &[f32],
    beta: &[f32],
    batch_size: usize,
    hidden_dim: usize,
    eps: f32,
) -> Vec<f32> {
    let mut output = vec![0.0; input.len()];

    for b in 0..batch_size {
        let start = b * hidden_dim;
        let end = start + hidden_dim;
        let row = &input[start..end];

        let mut state = WelfordState::new();
        for &x in row {
            state.update(x);
        }
        let mean = state.mean;
        let std = state.std(eps);

        for i in 0..hidden_dim {
            output[start + i] = (row[i] - mean) / std * gamma[i] + beta[i];
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::layernorm;

    // Naive reference LayerNorm (mean/var over the vector, then scale+shift).
    fn naive_layernorm(input: &[f32], gamma: &[f32], beta: &[f32], eps: f32) -> Vec<f32> {
        let n = input.len() as f32;
        let mean = input.iter().sum::<f32>() / n;
        let var = input.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / n;
        let std = (var + eps).sqrt();
        input
            .iter()
            .zip(gamma.iter())
            .zip(beta.iter())
            .map(|((&x, &g), &b)| (x - mean) / std * g + b)
            .collect()
    }

    #[test]
    fn layernorm_matches_naive_small_vector() {
        let x = [1.0f32, 2.0, 3.0, 4.0];
        let g = [1.0f32, 1.0, 1.0, 1.0];
        let b = [0.0f32, 0.0, 0.0, 0.0];
        let eps = 1e-5;

        let ours = layernorm(&x, &g, &b, eps);
        let naive = naive_layernorm(&x, &g, &b, eps);

        for (o, n) in ours.iter().zip(naive.iter()) {
            assert!((o - n).abs() < 1e-5, "layernorm mismatch {} vs {}", o, n);
        }
    }
}
