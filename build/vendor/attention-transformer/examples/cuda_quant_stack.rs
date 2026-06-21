//! Demonstrates all four quant-oriented CUDA features (RunPod / CUDA host).
//!
//! ```bash
//! cargo run --release --features "cuda cuda-quant" --example cuda_quant_stack
//! ```
//!
//! Environment toggles:
//! - `LUXI_CUDA_MEGA=1`        — full mega-fused layer on GPU
//! - `LUXI_CUDA_GPU_LAYER=1`   — CPU QKV + device Waller + CPU post (receipt-safe)
//! - `LUXI_CUDA_GPU_QKV=1`     — experimental device LN+QKV (may drift receipts)
//! - `LUXI_WALLER_SMEM=1`      — SMEM-tiled Lane A attention kernel
//! - `cuda-quant` feature      — Lane B INT8 GEMM on GPU

#[cfg(all(feature = "cuda", not(cuda_compilation_failed)))]
fn main() {
    use attention_transformer::config::Config;
    use attention_transformer::gpu::cuda::matmul_f32_i8_cuda;
    use attention_transformer::linalg::{matmul_f32_i8, quantize_symmetric_i8};
    use attention_transformer::wnsm_transformer::WNSM_GAE_Layer;

    println!("=== CUDA quant stack demo ===\n");

    // 4) Lane B INT8 GEMM
    #[cfg(feature = "cuda-quant")]
    {
        let a = vec![0.1f32, 0.2, 0.3, 0.4];
        let w = vec![1.0f32, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0];
        let q = quantize_symmetric_i8(&w, 2, 2);
        let cpu = matmul_f32_i8(&a, &q, 2, 2, 2);
        let gpu = unsafe { matmul_f32_i8_cuda(&a, &q.data, q.scale, 2, 2, 2).expect("int8 cuda") };
        let max_d: f32 = cpu
            .iter()
            .zip(gpu.iter())
            .map(|(x, y)| (x - y).abs())
            .fold(0.0f32, f32::max);
        println!("Lane B INT8 GEMM max_abs(cpu,gpu): {:.3e}", max_d);
    }
    #[cfg(not(feature = "cuda-quant"))]
    {
        println!("Lane B: rebuild with --features cuda-quant");
    }

    // 2) KV cache step
    let cfg = Config::new(64, 4, 256, 8);
    let mut layer = WNSM_GAE_Layer::new(cfg);
    let h = 64usize;
    let q = vec![0.01f32; h];
    let k = vec![0.02f32; h];
    let v = vec![0.03f32; h];
    let out0 = layer.forward_cuda_kv_step(&q, &k, &v, 0).expect("kv step");
    println!("KV cache step out[0]: {:.6}", out0[0]);

    // 1/3) Layer forward (respects LUXI_CUDA_MEGA / LUXI_CUDA_GPU_LAYER)
    let input: Vec<f32> = (0..8 * h).map(|i| (i as f32 * 0.01).sin()).collect();
    let y = layer.forward_cuda(&input, 8).expect("forward_cuda");
    println!("forward_cuda len={} y[0]={:.6}", y.len(), y[0]);

    println!("\nSet LUXI_CUDA_GPU_LAYER=1 (safe) or LUXI_CUDA_MEGA=1 or LUXI_WALLER_SMEM=1 to exercise other paths.");
}

#[cfg(not(all(feature = "cuda", not(cuda_compilation_failed))))]
fn main() {
    eprintln!("Build with: cargo run --release --features \"cuda cuda-quant\" --example cuda_quant_stack");
}