//! MLP block implementations, including the full-expansion variant required for
//! true mathematical WNSM (Waller Null-Space Multiplexing).

use crate::activations::gelu;
use crate::welford::WelfordState;

/// Basic MLP: Linear -> GELU -> Linear.
pub fn mlp_block(
    input: &[f32],
    w1: &[f32],
    b1: &[f32],
    w2: &[f32],
    b2: &[f32],
    hidden_dim: usize,
    mlp_dim: usize,
) -> Vec<f32> {
    let mut intermediate = vec![0.0; mlp_dim];
    for i in 0..mlp_dim {
        let mut sum = b1[i];
        for j in 0..hidden_dim {
            sum += input[j] * w1[j * mlp_dim + i];
        }
        intermediate[i] = gelu(sum);
    }

    let mut output = vec![0.0; hidden_dim];
    for i in 0..hidden_dim {
        let mut sum = b2[i];
        for j in 0..mlp_dim {
            sum += intermediate[j] * w2[j * hidden_dim + i];
        }
        output[i] = sum;
    }
    output
}

/// MLP that also returns the full post-GELU expanded activation.
/// This is the key enabler for real WNSM — the null space lives in these dimensions.
pub fn mlp_block_with_full_expansion(
    input: &[f32],
    w1: &[f32],
    b1: &[f32],
    w2: &[f32],
    b2: &[f32],
    hidden_dim: usize,
    mlp_dim: usize,
) -> (Vec<f32>, Vec<f32>) {
    let mut intermediate = vec![0.0; mlp_dim];
    for i in 0..mlp_dim {
        let mut sum = b1[i];
        for j in 0..hidden_dim {
            sum += input[j] * w1[j * mlp_dim + i];
        }
        intermediate[i] = gelu(sum);
    }

    let mut output = vec![0.0; hidden_dim];
    for i in 0..hidden_dim {
        let mut sum = b2[i];
        for j in 0..mlp_dim {
            sum += intermediate[j] * w2[j * hidden_dim + i];
        }
        output[i] = sum;
    }

    (output, intermediate)
}

/// Fused MLP + LayerNorm + Residual using Welford (matches historical behavior).
pub fn fused_mlp_layernorm(
    input: &[f32],
    residual: &[f32],
    w1: &[f32],
    b1: &[f32],
    w2: &[f32],
    b2: &[f32],
    gamma: &[f32],
    beta: &[f32],
    hidden_dim: usize,
    mlp_dim: usize,
    eps: f32,
) -> Vec<f32> {
    let mlp_out = mlp_block(input, w1, b1, w2, b2, hidden_dim, mlp_dim);

    let mut combined: Vec<f32> = mlp_out
        .iter()
        .zip(residual.iter())
        .map(|(&m, &r)| m + r)
        .collect();

    let mut state = WelfordState::new();
    for &x in &combined {
        state.update(x);
    }
    let mean = state.mean;
    let std = state.std(eps);

    for i in 0..hidden_dim {
        combined[i] = (combined[i] - mean) / std * gamma[i] + beta[i];
    }
    combined
}

#[cfg(test)]
mod tests {
    use super::{mlp_block, mlp_block_with_full_expansion};
    use crate::activations::gelu;

    // Naive reference MLP for verification.
    fn naive_mlp(
        input: &[f32],
        w1: &[f32],
        b1: &[f32],
        w2: &[f32],
        b2: &[f32],
        hidden: usize,
        mlp: usize,
    ) -> Vec<f32> {
        let mut inter = vec![0.0f32; mlp];
        for i in 0..mlp {
            let mut s = b1[i];
            for j in 0..hidden {
                s += input[j] * w1[j * mlp + i];
            }
            inter[i] = gelu(s);
        }
        let mut out = vec![0.0f32; hidden];
        for i in 0..hidden {
            let mut s = b2[i];
            for j in 0..mlp {
                s += inter[j] * w2[j * hidden + i];
            }
            out[i] = s;
        }
        out
    }

    #[test]
    fn mlp_block_matches_naive_small() {
        let h = 4usize;
        let m = 8usize;
        let inp = vec![0.1f32, 0.2, 0.3, 0.4];
        // Simple weights (not random to keep deterministic)
        let w1 = (0..h * m).map(|i| (i as f32) * 0.01).collect::<Vec<_>>();
        let b1 = vec![0.0f32; m];
        let w2 = (0..m * h).map(|i| (i as f32) * 0.02).collect::<Vec<_>>();
        let b2 = vec![0.0f32; h];

        let ours = mlp_block(&inp, &w1, &b1, &w2, &b2, h, m);
        let naive = naive_mlp(&inp, &w1, &b1, &w2, &b2, h, m);

        for (o, n) in ours.iter().zip(naive.iter()) {
            assert!((o - n).abs() < 1e-4, "mlp mismatch {} vs {}", o, n);
        }
    }

    #[test]
    fn mlp_with_expansion_returns_correct_shapes() {
        let h = 3usize;
        let m = 5usize;
        let inp = vec![0.5f32; h];
        let w1 = vec![0.1f32; h * m];
        let b1 = vec![0.0; m];
        let w2 = vec![0.2f32; m * h];
        let b2 = vec![0.0; h];

        let (out, inter) = mlp_block_with_full_expansion(&inp, &w1, &b1, &w2, &b2, h, m);
        assert_eq!(out.len(), h);
        assert_eq!(inter.len(), m);
    }
}
