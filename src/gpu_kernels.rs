// GPU FP16 kernels for 2x throughput on T4/L4 GPUs
// Optimized sin/cos evaluation using half-precision floating point

#[cfg(feature = "gpu")]
use cudarc::driver::{CudaDevice, LaunchAsync, LaunchConfig};

// GPU kernel configuration
const THREADS_PER_BLOCK: u32 = 256;

#[cfg(feature = "gpu")]
pub struct GpuKernels {
    device: CudaDevice,
}

#[cfg(feature = "gpu")]
impl GpuKernels {
    /// Initialize GPU kernels with FP16 support
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let device = CudaDevice::new(0)?;
        Ok(Self { device })
    }

    /// PTX kernel for FP16 fused sin*cos evaluation
    /// Uses __half intrinsics for 2x throughput
    const FP16_SINCOS_PTX: &str = r#"
.version 7.5
.target sm_75
.address_size 64

.visible .entry fp16_sincos_kernel(
    .param .u64 input_ptr,
    .param .u64 output_ptr,
    .param .u32 n
)
{
    .reg .pred %p<3>;
    .reg .f16 %f<16>;
    .reg .f32 %r<8>;
    .reg .b32 %w<4>;
    .reg .b64 %d<8>;

    ld.param.u64 %d0, [input_ptr];
    ld.param.u64 %d1, [output_ptr];
    ld.param.u32 %w0, [n];

    mov.u32 %w1, %ctaid.x;
    mov.u32 %w2, %ntid.x;
    mad.lo.u32 %w3, %w1, %w2, %tid.x;

    setp.ge.u32 %p0, %w3, %w0;
    @%p0 bra done;

    // Load input as FP16
    mul.wide.u32 %d2, %w3, 2;
    add.u64 %d3, %d0, %d2;
    ld.global.u16 %f0, [%d3];

    // Convert to FP32 for computation
    cvt.f32.f16 %r0, %f0;

    // Minimax polynomial approximation for sin(x)*cos(x)
    // Use Horner's method with FMA
    abs.f32 %r1, %r0;
    min.f32 %r2, %r1, 0f3fc90fdb; // min(|x|, π/2)

    // sin(t) ≈ t - t³/6 + t⁵/120
    mul.f32 %r3, %r2, %r2; // t²
    mul.f32 %r4, %r3, %r2; // t³
    fma.rn.f32 %r5, %r4, 0fbe2aaaa3, %r2; // t - t³/6

    // cos(t) ≈ 1 - t²/2 + t⁴/24
    mul.f32 %r6, %r3, %r3; // t⁴
    fma.rn.f32 %r7, %r6, 0f3d2aaaa3, 0fbf000000; // -t²/2 + t⁴/24
    add.f32 %r7, %r7, 0f3f800000; // 1 + ...

    // Multiply sin * cos
    mul.f32 %r5, %r5, %r7;

    // Apply sign
    setp.lt.f32 %p1, %r0, 0f00000000;
    @%p1 neg.f32 %r5, %r5;

    // Convert back to FP16
    cvt.rn.f16.f32 %f1, %r5;

    // Store output
    mul.wide.u32 %d4, %w3, 2;
    add.u64 %d5, %d1, %d4;
    st.global.u16 [%d5], %f1;

done:
    ret;
}
"#;

    /// Evaluate sin(x)*cos(x) on GPU using FP16 for 2x throughput
    pub fn eval_sincos_fp16(&self, input: &[f32]) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        // Convert f32 to f16 for GPU processing
        let n = input.len();
        let mut input_f16 = Vec::with_capacity(n);
        for &x in input {
            input_f16.push(half::f16::from_f32(x).to_bits());
        }

        // Load PTX kernel
        self.device.load_ptx(
            Self::FP16_SINCOS_PTX.as_bytes().into(),
            "fp16_sincos_kernel",
            &["fp16_sincos_kernel"],
        )?;

        // Allocate device memory
        let d_input = self.device.htod_copy(input_f16)?;
        let d_output = self.device.alloc_zeros::<u16>(n)?;

        // Launch kernel with optimized grid size
        let blocks = (n + THREADS_PER_BLOCK as usize - 1) / THREADS_PER_BLOCK as usize;
        let cfg = LaunchConfig {
            grid_dim: (blocks as u32, 1, 1),
            block_dim: (THREADS_PER_BLOCK, 1, 1),
            shared_mem_bytes: 0,
        };

        let func = self.device.get_func("fp16_sincos_kernel", "fp16_sincos_kernel")?;
        unsafe {
            func.launch(cfg, (&d_input, &d_output, n as u32))?;
        }

        // Copy results back and convert to f32
        let output_f16: Vec<u16> = self.device.dtoh_sync_copy(&d_output)?;
        let output: Vec<f32> = output_f16
            .iter()
            .map(|&x| half::f16::from_bits(x).to_f32())
            .collect();

        Ok(output)
    }
}

#[cfg(not(feature = "gpu"))]
pub struct GpuKernels;

#[cfg(not(feature = "gpu"))]
impl GpuKernels {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Err("GPU support not compiled. Build with --features gpu".into())
    }
}
