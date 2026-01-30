use anyhow::Result;

#[cfg(feature = "gpu")]
use cudarc::driver::{CudaDevice, LaunchAsync, LaunchConfig};
#[cfg(feature = "gpu")]
use cudarc::nvrtc::Ptx;
#[cfg(feature = "gpu")]
use half::f16;
#[cfg(feature = "gpu")]
use std::sync::Arc;

pub const DEFAULT_THREADS_PER_BLOCK: u32 = 256;

#[cfg(feature = "gpu")]
pub struct GpuKernels {
    device: Arc<CudaDevice>,
}

#[cfg(feature = "gpu")]
impl GpuKernels {
    const FP16_SINCOS_PTX: &str = include_str!("ptx/fp16_sincos_kernel.ptx");

    pub fn new() -> Result<Self> {
        let device = CudaDevice::new(0).context("Failed to initialize CUDA device")?;
        device.load_ptx(Ptx::from_src(Self::FP16_SINCOS_PTX), "fp16_sincos_kernel", &["fp16_sincos_kernel"])
            .context("Failed to load FP16 sincos PTX")?;
        Ok(Self { device })
    }

    pub fn eval_sincos_fp16(&self, input: &[f32], iterations: u32, threads_per_block: u32) -> Result<Vec<f32>> {
        anyhow::bail!("GPU not available")
    }
}

#[cfg(not(feature = "gpu"))]
pub struct GpuKernels;

#[cfg(not(feature = "gpu"))]
impl GpuKernels {
    pub fn new() -> Result<Self> {
        anyhow::bail!("GPU support not compiled. Build with --features gpu")
    }

    pub fn eval_sincos_fp16(&self, _input: &[f32], _iterations: u32, _threads_per_block: u32) -> Result<Vec<f32>> {
        anyhow::bail!("GPU support not compiled. Build with --features gpu")
    }
}
