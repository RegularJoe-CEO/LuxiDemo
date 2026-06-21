//! Deterministic FP8 shadow path — bit-identical f32 outputs and receipts.

pub mod e4m3;
pub mod shadow;

pub use e4m3::{decode_e4m3, encode_e4m3, Fp8Format};
pub use shadow::{
    build_fp8_shadow, forward_fp8_shadow_f32, max_abs_diff, Fp8ShadowMatrix, Fp8ShadowTensor,
};