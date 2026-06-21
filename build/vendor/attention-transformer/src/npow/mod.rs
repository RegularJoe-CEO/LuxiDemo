//! NPOW — N-power scaling witness carried in the WNSM null-space payload bus.
//!
//! Encodes measured O(N) Waller vs O(N²) standard memory scaling for receipt-backed proof.

pub mod scaling;
pub mod witness;

pub use scaling::{
    fit_log_log_slope, memory_bytes_standard, memory_bytes_waller, run_scaling_samples,
    run_scaling_samples_full, ScalingSample,
};
pub use witness::{
    decode_npow_payload, encode_npow_payload, install_identity_null_basis, NpowPayload,
    NPOW_MAGIC, NPOW_PAYLOAD_DIM,
};