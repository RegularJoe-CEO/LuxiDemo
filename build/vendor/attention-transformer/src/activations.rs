//! Activation functions used by the attention transformer.
//! All are deterministic and match the reference implementations from the research lineage.

/// GELU activation (exact, deterministic).
#[inline]
pub fn gelu(x: f32) -> f32 {
    0.5 * x * (1.0 + ((2.0_f32 / std::f32::consts::PI).sqrt() * (x + 0.044715 * x.powi(3))).tanh())
}

/// SiLU / Swish activation (deterministic).
#[inline]
pub fn silu(x: f32) -> f32 {
    x / (1.0 + (-x).exp())
}

/// ReLU activation (deterministic).
#[inline]
pub fn relu(x: f32) -> f32 {
    x.max(0.0)
}

/// Apply GELU in-place (for fused paths).
pub fn gelu_inplace(data: &mut [f32]) {
    for x in data.iter_mut() {
        *x = gelu(*x);
    }
}
