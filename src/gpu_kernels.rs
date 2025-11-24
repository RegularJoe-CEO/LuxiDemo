// src/gpu_kernels.rs (entire file)
use anyhow::{anyhow, Result};
use cudarc::driver::{CudaDevice, LaunchAsync, LaunchConfig};
use cudarc::nvrtc::Ptx;
use half::f16;
use std::sync::Arc;

#[cfg(feature = "gpu")]
pub struct Fp16SincosModule {
    device: Arc<CudaDevice>,
}

#[cfg(feature = "gpu")]
impl Fp16SincosModule {
    /// Create and register the FP16 sin/cos kernel module
    pub fn new() -> Result<Self> {
        // cudarc 0.11 returns Arc<CudaDevice>, do not wrap in Arc::new
        let device: Arc<CudaDevice> = CudaDevice::new(0)?;
        // PTX path is relative to src/, keep it under src/ptx/
        let ptx = Ptx::from_src(include_str!("ptx/fp16_sincos_kernel.ptx"));
        // Load under a stable module name and register the function
        device.load_ptx(ptx, "fp16_mod", &["fp16_sincos_kernel"])?;
        Ok(Self { device })
    }

    /// Launch the FP16 sin/cos kernel on input vector (length = n)
    pub fn launch(&self, input: &[f16], n: usize) -> Result<Vec<f16>> {
        if n == 0 {
            return Ok(Vec::new());
        }

        // Use u16 bit-patterns to satisfy DeviceRepr (f16 itself doesn’t implement it)
        let input_bits: Vec<u16> = input.iter().map(|&f| f.to_bits()).collect();
        let d_input = self.device.htod_copy(input_bits)?; // NOTE: no &

        // Allocate output buffer on device (u16 for f16 bits)
        let mut d_output = self.device.alloc_zeros::<u16>(2 * n)?;

        // 1D launch sized to elements
        let cfg = LaunchConfig::for_num_elems(n as u32);

        // Resolve the kernel from the loaded module
        let func = self
            .device
            .get_func("fp16_mod", "fp16_sincos_kernel")
            .ok_or(anyhow!("Kernel not found"))?;

        // Launch
        unsafe {
            func.launch(cfg, (&d_input, &mut d_output, n as i32))?;
        }

        // Ensure device work completion
        self.device.synchronize()?;

        // Copy results back and convert to f16
        let output_bits: Vec<u16> = self.device.dtoh_sync_copy(&d_output)?;
        let output: Vec<f16> = output_bits.into_iter().map(f16::from_bits).collect();

        Ok(output)
    }
}

// compute sin(x), cos(x) using fp16 kernel with configurable inner loop
// FMA per block: 256
// PER BLOCK: 256
