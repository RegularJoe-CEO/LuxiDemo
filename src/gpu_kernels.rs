use anyhow::Result;
use cudarc::driver::safe::{CudaDevice, CudaSlice, CudaModule, CudaFunction, LaunchConfig};
use cudarc::nvrtc::Ptx;
use std::fs;

pub struct F32SincosModule {
    device: CudaDevice,
    func: Cuda

