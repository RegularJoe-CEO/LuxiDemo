pub mod runtime;
pub mod compute;
pub mod security;

// eRock SECURE: Public surface only—redacted exports
pub use runtime::{HwProbe, OffloadQueue};
pub use compute::{Dispatcher, ComputeOp, Precision};
pub use security::Enclave;

#[cfg(feature = "gpu")]
pub mod gpu_kernels;

#[cfg(feature = "vulkan")]
pub mod vulkan_fallback;


#[allow(dead_code)]
pub fn health_fields() -> (bool, bool, &'static str) { (false, false, "jit_disabled") }
pub mod luxi_eval;
// // pub use luxi_eval::simd_eval_over_x_inplace;
