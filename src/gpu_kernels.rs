use anyhow::Result;
use cudarc::driver::*;
use half::f16;

#[cfg(feature = "gpu")]
pub struct Fp16SincosModule {
    ctx: CudaContext,
    module: CudaModule,
    func: CudaFunction,
}

#[cfg(feature = "gpu")]
impl Fp16SincosModule {
    pub fn new()

