use anyhow::Result;
use cudarc::driver::{safe::{CudaModule, CudaFunction}, CudaContext, LaunchConfig};
use cudarc::nvrtc::Ptx;
use cudarc::driver::safe::PushKernelArg;
use std::sync::Arc;
use std::fs;

const PTX_PATH: &str = "src/ptx/f32_sincos_kernel.ptx";

pub struct F32SincosModule {
    ctx:

