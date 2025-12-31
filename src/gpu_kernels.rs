use anyhow::{Context, Result};
use cudarc::driver::{CudaDevice, LaunchAsync, LaunchConfig};
use cudarc::nvrtc::Ptx;
use half::f16;
use std::sync::Arc;

/// Default threads per block if the user does not override on the CLI.
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

        // Load the PTX module once up front so subsequent launches are cheap.
        device
            .load_ptx(
                Ptx::from_src(Self::FP16_SINCOS_PTX),
                "fp16_sincos_kernel",
                &["fp16_sincos_kernel"],
            )
            .context("Failed to load FP16 sincos PTX")?;

        Ok(Self { device })
    }

    /// Evaluate sin(x) * cos(x) using an FP16 kernel with a configurable inner loop.
    pub fn eval_sincos_fp16(
        &self,
        input: &[f32],
        iterations: u32,
        threads_per_block: u32,
    ) -> Result<Vec<f32>> {
        let n = input.len();
        let mut input_f16 = Vec::with_capacity(n);
        for &x in input {
            input_f16.push(f16::from_f32(x).to_bits());
        }

        let d_input = self
            .device
            .htod_copy(input_f16)
            .context("Failed to copy input to device")?;
        let d_output = self
            .device
            .alloc_zeros::<u16>(n)
            .context("Failed to allocate output buffer")?;

        let block_dim = threads_per_block;
        let blocks = (n as u32 + block_dim - 1) / block_dim;
        let cfg = LaunchConfig {
            grid_dim: (blocks, 1, 1),
            block_dim: (block_dim, 1, 1),
            shared_mem_bytes: 0,
        };

        let func = self
            .device
            .get_func("fp16_sincos_kernel", "fp16_sincos_kernel")
            .ok_or_else(|| anyhow::anyhow!("Kernel function not found"))?;

        unsafe {
            func.launch(cfg, (&d_input, &d_output, n as u32, iterations))
                .context("Kernel launch failed")?;
        }

        let output_f16: Vec<u16> = self
            .device
            .dtoh_sync_copy(&d_output)
            .context("Failed to copy output to host")?;
        let output: Vec<f32> = output_f16
            .iter()
            .map(|&x| f16::from_bits(x).to_f32())
            .collect();

        Ok(output)
    }
}

#[cfg(not(feature = "gpu"))]
pub struct GpuKernels;

#[cfg(not(feature = "gpu"))]
impl GpuKernels {
    pub fn new() -> Result<Self> {
        anyhow::bail!("GPU support not compiled. Build with --features gpu");
    }

    pub fn eval_sincos_fp16(
        &self,
        _input: &[f32],
        _iterations: u32,
        _threads_per_block: u32,
    ) -> Result<Vec<f32>> {
        anyhow::bail!("GPU support not compiled. Build with --features gpu");
    }
}
