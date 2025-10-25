pub mod runtime;
pub mod compute;
pub mod security;

// eRock SECURE: Public surface only—redacted exports
pub use runtime::{HwProbe, OffloadQueue};
pub use compute::{Dispatcher, ComputeOp, Precision};
pub use security::Enclave;
