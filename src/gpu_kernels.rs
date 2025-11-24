use anyhow::Result;
use cudarc::driver::safe::{CudaContext, CudaModule, CudaFunction, CudaSlice, LaunchConfig, CudaStream};
use std::sync::Arc;
use std::fs;

pub struct F32SincosModule {
    ctx: Arc<CudaContext>,
    stream: CudaStream,
    func: CudaFunction,
}

impl F32SincosModule {
    pub fn new() -> Result<Self>

