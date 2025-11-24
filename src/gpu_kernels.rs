use anyhow::Result;
use cudarc::driver::{CudaDevice, CudaSlice, CudaModule, CudaFunction, LaunchConfig};
use half::f16;
use std::sync::Arc;

#[cfg(feature = "gpu")]
pub struct Fp16SincosModule {
    device:

