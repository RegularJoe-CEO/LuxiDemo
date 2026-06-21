// src/gpu/cuda.rs
// CUDA backend for the Waller Operator (first step toward full mega kernel fusion).
//
// This module is only compiled when the "cuda" feature is enabled.
// It provides a path that must produce identical results + receipts to the
// pure-Rust reference implementation.

#[cfg(feature = "cuda")]
use std::ffi::c_void;

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
#[link(name = "waller_cuda", kind = "static")]
extern "C" {
    fn launch_waller_operator(
        Q: *const f32,
        K: *const f32,
        V: *const f32,
        Output: *mut f32,
        seq_len: i32,
        head_dim: i32,
        num_heads: i32,
        scale: f32,
        stream: *mut c_void,
    );
    fn launch_residual_ln2_rows(
        input: *const f32,
        attn_proj: *const f32,
        out: *mut f32,
        ln2_gamma: *const f32,
        ln2_beta: *const f32,
        seq_len: i32,
        hidden_dim: i32,
        ln_eps: f32,
        stream: *mut c_void,
    );
    fn launch_ln1_rows(
        input: *const f32,
        normed_out: *mut f32,
        ln1_gamma: *const f32,
        ln1_beta: *const f32,
        seq_len: i32,
        hidden_dim: i32,
        ln_eps: f32,
        stream: *mut c_void,
    );
    fn launch_ln_qkv_proj(
        input: *const f32,
        wq: *const f32,
        wk: *const f32,
        wv: *const f32,
        ln1_gamma: *const f32,
        ln1_beta: *const f32,
        Q: *mut f32,
        K: *mut f32,
        V: *mut f32,
        seq_len: i32,
        hidden_dim: i32,
        ln_eps: f32,
        stream: *mut c_void,
    );
    fn launch_waller_kv_decode(
        Q_row: *const f32,
        K_cache: *const f32,
        V_cache: *const f32,
        Output_row: *mut f32,
        row: i32,
        head_dim: i32,
        num_heads: i32,
        scale: f32,
        stream: *mut c_void,
    );
    fn launch_matmul_f32(
        A: *const f32,
        B: *const f32,
        C: *mut f32,
        m: i32,
        k: i32,
        n: i32,
        stream: *mut c_void,
    );
    fn launch_matmul_f32_geodesic(
        A: *const f32,
        B: *const f32,
        C: *mut f32,
        m: i32,
        k: i32,
        n: i32,
        stream: *mut c_void,
    );
    fn launch_mlp_block_geodesic(
        normed: *const f32,
        w_fc: *const f32,
        b_fc: *const f32,
        w_proj: *const f32,
        b_proj: *const f32,
        ln2_gamma: *const f32,
        ln2_beta: *const f32,
        scratch_mlp: *mut f32,
        scratch_h: *mut f32,
        output: *mut f32,
        seq_len: i32,
        hidden_dim: i32,
        mlp_dim: i32,
        ln_eps: f32,
        stream: *mut c_void,
    );
    fn launch_waller_fused_wo(
        Q: *const f32,
        K: *const f32,
        V: *const f32,
        Wo: *const f32,
        Output: *mut f32,
        seq_len: i32,
        head_dim: i32,
        num_heads: i32,
        scale: f32,
        stream: *mut c_void,
    );
}

#[cfg(all(feature = "cuda-quant", not(cuda_compilation_failed)))]
#[cfg(feature = "cuda")]
#[link(name = "waller_cuda", kind = "static")]
extern "C" {
    fn launch_matmul_f32_i8(
        A: *const f32,
        B: *const i8,
        C: *mut f32,
        m: i32,
        k: i32,
        n: i32,
        w_scale: f32,
        stream: *mut c_void,
    );
}

/// Run the Waller Operator on CUDA.
///
/// Only available when the `cuda` feature was enabled at build time **and**
/// the kernels actually compiled (i.e. nvcc was present).
#[cfg(feature = "cuda")]
pub unsafe fn waller_operator_cuda(
    q: *const f32,
    k: *const f32,
    v: *const f32,
    output: *mut f32,
    seq_len: usize,
    head_dim: usize,
    num_heads: usize,
    scale: f32,
) -> Result<(), String> {
    #[cfg(cuda_compilation_failed)]
    {
        return Err(
            "CUDA kernels were not compiled (nvcc not found at build time). \
             Build on a machine with the CUDA Toolkit installed."
                .to_string(),
        );
    }

    #[cfg(not(cuda_compilation_failed))]
    {
        launch_waller_operator(
            q,
            k,
            v,
            output,
            seq_len as i32,
            head_dim as i32,
            num_heads as i32,
            scale,
            std::ptr::null_mut(),
        );
        Ok(())
    }
}

// ============================================================================
// Persistent device buffers for the Waller operator (no per-call cudaMalloc)
// ============================================================================

/// Reusable Q/K/V/output device allocations for repeated Waller launches.
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub struct CudaWallerBuffers {
    d_q: *mut f32,
    d_k: *mut f32,
    d_v: *mut f32,
    d_normed: *mut f32,
    d_out: *mut f32,
    /// Post-attention output projection (wo); same layout as d_out.
    d_proj: *mut f32,
    /// Packed Q||K||V after geodesic GEMM [seq, 3*hidden] (capacity 3× per-tensor cap).
    d_qkv: *mut f32,
    /// Pinned host staging (Q||K||V) for one-shot async H2D.
    h_pinned: *mut f32,
    h_pinned_floats: usize,
    /// Pinned host buffer for async D2H of projected output.
    h_out_pinned: *mut f32,
    h_out_pinned_floats: usize,
    stream: *mut c_void,
    /// If false, stream is borrowed (e.g. shared quant stack) and must not be destroyed here.
    stream_owned: bool,
    capacity_elements: usize,
    /// Set after `upload_inputs`; allows `launch_only` without per-iter H2D.
    pub inputs_on_device: bool,
}

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
impl CudaWallerBuffers {
    pub fn qkv_device_ptrs(&self) -> (*mut f32, *mut f32, *mut f32) {
        (self.d_q, self.d_k, self.d_v)
    }
}

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
impl CudaWallerBuffers {
    pub fn new() -> Self {
        Self {
            d_q: std::ptr::null_mut(),
            d_k: std::ptr::null_mut(),
            d_v: std::ptr::null_mut(),
            d_normed: std::ptr::null_mut(),
            d_out: std::ptr::null_mut(),
            d_proj: std::ptr::null_mut(),
            d_qkv: std::ptr::null_mut(),
            h_pinned: std::ptr::null_mut(),
            h_pinned_floats: 0,
            h_out_pinned: std::ptr::null_mut(),
            h_out_pinned_floats: 0,
            stream: std::ptr::null_mut(),
            stream_owned: true,
            capacity_elements: 0,
            inputs_on_device: false,
        }
    }

    /// Use one CUDA stream for multi-layer TRADE (avoids stream exhaustion on deep stacks).
    pub unsafe fn borrow_stream(&mut self, stream: *mut c_void) {
        if self.stream_owned && !self.stream.is_null() {
            cudaStreamDestroy(self.stream);
        }
        self.stream = stream;
        self.stream_owned = false;
    }

    unsafe fn ensure_stream(&mut self) -> Result<(), String> {
        if !self.stream.is_null() {
            return Ok(());
        }
        self.stream = cuda_create_stream()?;
        self.stream_owned = true;
        Ok(())
    }

    /// Grow device buffers when `total` elements exceeds current capacity (geometric growth).
    pub unsafe fn ensure_capacity(&mut self, total: usize) -> Result<(), String> {
        if total <= self.capacity_elements && !self.d_qkv.is_null() {
            return Ok(());
        }
        self.free_device();
        self.ensure_stream()?;
        let new_cap = total.max(1).next_power_of_two();
        let bytes = new_cap * std::mem::size_of::<f32>();
        let pinned_floats = new_cap * 3;

        if self.h_pinned.is_null() || self.h_pinned_floats < pinned_floats {
            if !self.h_pinned.is_null() {
                cuda_free_host(self.h_pinned as *mut c_void);
                self.h_pinned = std::ptr::null_mut();
            }
            let mut pinned: *mut c_void = std::ptr::null_mut();
            if cuda_malloc_host(
                &mut pinned as *mut *mut c_void,
                pinned_floats * std::mem::size_of::<f32>(),
            ) != CUDA_SUCCESS
            {
                return Err("cudaMallocHost failed for QKV staging".to_string());
            }
            self.h_pinned = pinned as *mut f32;
            self.h_pinned_floats = pinned_floats;
        }

        if self.h_out_pinned.is_null() || self.h_out_pinned_floats < new_cap {
            if !self.h_out_pinned.is_null() {
                cuda_free_host(self.h_out_pinned as *mut c_void);
                self.h_out_pinned = std::ptr::null_mut();
            }
            let mut out_pin: *mut c_void = std::ptr::null_mut();
            if cuda_malloc_host(
                &mut out_pin as *mut *mut c_void,
                new_cap * std::mem::size_of::<f32>(),
            ) != CUDA_SUCCESS
            {
                return Err("cudaMallocHost failed for output staging".to_string());
            }
            self.h_out_pinned = out_pin as *mut f32;
            self.h_out_pinned_floats = new_cap;
        }

        let qkv_bytes = bytes * 3;
        if cudaMalloc(&mut self.d_qkv as *mut _ as *mut *mut c_void, qkv_bytes) != CUDA_SUCCESS {
            return Err("cudaMalloc failed for packed QKV".to_string());
        }
        self.d_q = self.d_qkv;
        self.d_k = unsafe { self.d_q.add(new_cap) };
        self.d_v = unsafe { self.d_k.add(new_cap) };
        if cudaMalloc(&mut self.d_normed as *mut _ as *mut *mut c_void, bytes) != CUDA_SUCCESS {
            cudaFree(self.d_q as *mut c_void);
            cudaFree(self.d_k as *mut c_void);
            cudaFree(self.d_v as *mut c_void);
            self.d_q = std::ptr::null_mut();
            self.d_k = std::ptr::null_mut();
            self.d_v = std::ptr::null_mut();
            return Err("cudaMalloc failed for persistent normed".to_string());
        }
        if cudaMalloc(&mut self.d_out as *mut _ as *mut *mut c_void, bytes) != CUDA_SUCCESS {
            cudaFree(self.d_q as *mut c_void);
            cudaFree(self.d_k as *mut c_void);
            cudaFree(self.d_v as *mut c_void);
            cudaFree(self.d_normed as *mut c_void);
            self.d_q = std::ptr::null_mut();
            self.d_k = std::ptr::null_mut();
            self.d_v = std::ptr::null_mut();
            self.d_normed = std::ptr::null_mut();
            return Err("cudaMalloc failed for persistent output".to_string());
        }
        if cudaMalloc(&mut self.d_proj as *mut _ as *mut *mut c_void, bytes) != CUDA_SUCCESS {
            cudaFree(self.d_q as *mut c_void);
            cudaFree(self.d_k as *mut c_void);
            cudaFree(self.d_v as *mut c_void);
            cudaFree(self.d_normed as *mut c_void);
            cudaFree(self.d_out as *mut c_void);
            self.d_q = std::ptr::null_mut();
            self.d_k = std::ptr::null_mut();
            self.d_v = std::ptr::null_mut();
            self.d_out = std::ptr::null_mut();
            return Err("cudaMalloc failed for persistent proj output".to_string());
        }
        self.capacity_elements = new_cap;
        self.inputs_on_device = false;
        Ok(())
    }

    /// Upload Q/K/V once (pinned staging + async H2D). Call before `launch_only` loops.
    pub unsafe fn upload_inputs(
        &mut self,
        q: &[f32],
        k: &[f32],
        v: &[f32],
        total: usize,
    ) -> Result<(), String> {
        self.ensure_capacity(total)?;
        let byte_len = total * std::mem::size_of::<f32>();
        std::ptr::copy_nonoverlapping(q.as_ptr(), self.h_pinned, total);
        std::ptr::copy_nonoverlapping(k.as_ptr(), self.h_pinned.add(total), total);
        std::ptr::copy_nonoverlapping(v.as_ptr(), self.h_pinned.add(total * 2), total);
        cuda_memcpy_h2d_async(
            self.d_q as *mut c_void,
            self.h_pinned as *const c_void,
            byte_len,
            self.stream,
        );
        cuda_memcpy_h2d_async(
            self.d_k as *mut c_void,
            self.h_pinned.add(total) as *const c_void,
            byte_len,
            self.stream,
        );
        cuda_memcpy_h2d_async(
            self.d_v as *mut c_void,
            self.h_pinned.add(total * 2) as *const c_void,
            byte_len,
            self.stream,
        );
        cuda_stream_synchronize(self.stream);
        self.inputs_on_device = true;
        Ok(())
    }

    unsafe fn free_device(&mut self) {
        if !self.d_qkv.is_null() {
            cuda_free(self.d_qkv as *mut c_void);
            self.d_qkv = std::ptr::null_mut();
            self.d_q = std::ptr::null_mut();
            self.d_k = std::ptr::null_mut();
            self.d_v = std::ptr::null_mut();
        }
        if !self.d_normed.is_null() {
            cuda_free(self.d_normed as *mut c_void);
            self.d_normed = std::ptr::null_mut();
        }
        if !self.d_out.is_null() {
            cuda_free(self.d_out as *mut c_void);
            self.d_out = std::ptr::null_mut();
        }
        if !self.d_proj.is_null() {
            cuda_free(self.d_proj as *mut c_void);
            self.d_proj = std::ptr::null_mut();
        }
        self.capacity_elements = 0;
        if !self.h_pinned.is_null() {
            cuda_free_host(self.h_pinned as *mut c_void);
            self.h_pinned = std::ptr::null_mut();
            self.h_pinned_floats = 0;
        }
        if !self.h_out_pinned.is_null() {
            cuda_free_host(self.h_out_pinned as *mut c_void);
            self.h_out_pinned = std::ptr::null_mut();
            self.h_out_pinned_floats = 0;
        }
        if self.stream_owned && !self.stream.is_null() {
            cudaStreamDestroy(self.stream);
            self.stream = std::ptr::null_mut();
        }
        self.inputs_on_device = false;
    }
}

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
impl Drop for CudaWallerBuffers {
    fn drop(&mut self) {
        unsafe {
            self.free_device();
        }
    }
}

/// Optional per-phase timing (milliseconds) for benchmarking.
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
#[derive(Clone, Copy, Debug, Default)]
pub struct CudaWallerTimings {
    pub h2d_ms: f64,
    pub kernel_ms: f64,
    pub d2h_ms: f64,
}

/// Run Waller on CUDA using pre-allocated device buffers (production / bench path).
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub fn waller_operator_cuda_persistent(
    q: &[f32],
    k: &[f32],
    v: &[f32],
    buffers: &mut CudaWallerBuffers,
    seq_len: usize,
    head_dim: usize,
    num_heads: usize,
    scale: f32,
    timings: Option<&mut CudaWallerTimings>,
) -> Result<Vec<f32>, String> {
    let total = seq_len * head_dim * num_heads;
    if q.len() != total || k.len() != total || v.len() != total {
        return Err("Input sizes do not match expected [seq_len * hidden * num_heads]".to_string());
    }
    unsafe {
        buffers.ensure_capacity(total)?;

        let t0 = std::time::Instant::now();
        buffers.upload_inputs(q, k, v, total)?;
        let h2d_ms = t0.elapsed().as_secs_f64() * 1000.0;

        // Always D2H: callers need results; timings only controls whether phase ms are recorded.
        let (kernel_ms, d2h_ms, host_out) =
            buffers.launch_only(total, seq_len, head_dim, num_heads, scale, true)?;

        if let Some(t) = timings {
            t.h2d_ms = h2d_ms;
            t.kernel_ms = kernel_ms;
            t.d2h_ms = d2h_ms;
        }

        Ok(host_out)
    }
}

/// Kernel + D2H only (inputs must already be on device via [`CudaWallerBuffers::upload_inputs`]).
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
impl CudaWallerBuffers {
    pub unsafe fn launch_only(
        &mut self,
        total: usize,
        seq_len: usize,
        head_dim: usize,
        num_heads: usize,
        scale: f32,
        need_output: bool,
    ) -> Result<(f64, f64, Vec<f32>), String> {
        if !self.inputs_on_device {
            return Err("launch_only called before upload_inputs".to_string());
        }
        self.ensure_stream()?;
        let byte_len = total * std::mem::size_of::<f32>();

        let t1 = std::time::Instant::now();
        launch_waller_operator(
            self.d_q,
            self.d_k,
            self.d_v,
            self.d_out,
            seq_len as i32,
            head_dim as i32,
            num_heads as i32,
            scale,
            self.stream,
        );
        cuda_stream_synchronize(self.stream);
        let kernel_ms = t1.elapsed().as_secs_f64() * 1000.0;

        let mut host_out = if need_output {
            vec![0.0f32; total]
        } else {
            Vec::new()
        };
        let t2 = std::time::Instant::now();
        if need_output {
            cuda_memcpy_d2h_async(
                host_out.as_mut_ptr() as *mut c_void,
                self.d_out as *const c_void,
                byte_len,
                self.stream,
            );
            cuda_stream_synchronize(self.stream);
        }
        let d2h_ms = t2.elapsed().as_secs_f64() * 1000.0;

        Ok((kernel_ms, d2h_ms, host_out))
    }

    /// Waller on device → wo projection on device → single D2H (skips raw-attention host copy).
    pub unsafe fn launch_waller_then_wo(
        &mut self,
        total: usize,
        seq_len: usize,
        hidden_dim: usize,
        head_dim: usize,
        num_heads: usize,
        scale: f32,
        d_wo: *const f32,
    ) -> Result<Vec<f32>, String> {
        if !self.inputs_on_device {
            return Err("launch_waller_then_wo called before upload_inputs".to_string());
        }
        self.ensure_stream()?;
        let byte_len = total * std::mem::size_of::<f32>();

        self.ensure_capacity(total)?;

        let t1 = std::time::Instant::now();
        if cuda_split_fused_wo_supported(head_dim, hidden_dim) {
            launch_waller_fused_wo(
                self.d_q,
                self.d_k,
                self.d_v,
                d_wo,
                self.d_proj,
                seq_len as i32,
                head_dim as i32,
                num_heads as i32,
                scale,
                self.stream,
            );
        } else {
            launch_waller_operator(
                self.d_q,
                self.d_k,
                self.d_v,
                self.d_out,
                seq_len as i32,
                head_dim as i32,
                num_heads as i32,
                scale,
                self.stream,
            );
            launch_matmul_f32(
                self.d_out,
                d_wo,
                self.d_proj,
                seq_len as i32,
                hidden_dim as i32,
                hidden_dim as i32,
                self.stream,
            );
        }
        cuda_stream_synchronize(self.stream);
        let kernel_ms = t1.elapsed().as_secs_f64() * 1000.0;

        let mut host_out = vec![0.0f32; total];
        let t2 = std::time::Instant::now();
        cuda_memcpy_d2h_async(
            self.h_out_pinned as *mut c_void,
            self.d_proj as *const c_void,
            byte_len,
            self.stream,
        );
        cuda_stream_synchronize(self.stream);
        std::ptr::copy_nonoverlapping(self.h_out_pinned, host_out.as_mut_ptr(), total);
        let d2h_ms = t2.elapsed().as_secs_f64() * 1000.0;
        let _ = (kernel_ms, d2h_ms);
        Ok(host_out)
    }

    /// Waller + wo on device; result stays in `d_proj` (no D2H).
    pub unsafe fn launch_waller_then_wo_on_device(
        &mut self,
        total: usize,
        seq_len: usize,
        hidden_dim: usize,
        head_dim: usize,
        num_heads: usize,
        scale: f32,
        d_wo: *const f32,
    ) -> Result<(), String> {
        if !self.inputs_on_device {
            return Err("launch_waller_then_wo_on_device called before upload_inputs".to_string());
        }
        self.ensure_stream()?;
        self.ensure_capacity(total)?;

        if cuda_split_fused_wo_supported(head_dim, hidden_dim) {
            launch_waller_fused_wo(
                self.d_q,
                self.d_k,
                self.d_v,
                d_wo,
                self.d_proj,
                seq_len as i32,
                head_dim as i32,
                num_heads as i32,
                scale,
                self.stream,
            );
        } else {
            launch_waller_operator(
                self.d_q,
                self.d_k,
                self.d_v,
                self.d_out,
                seq_len as i32,
                head_dim as i32,
                num_heads as i32,
                scale,
                self.stream,
            );
            launch_matmul_f32(
                self.d_out,
                d_wo,
                self.d_proj,
                seq_len as i32,
                hidden_dim as i32,
                hidden_dim as i32,
                self.stream,
            );
        }
        Ok(())
    }
}

/// Fused row-wise Waller+wo kernel (hd in {16,32,64,128}, hidden ≤ 8192).
/// TRADE default is parallel waller + wo GEMM (~4 ms @ seq=1024). Row-fused is opt-in (slow).
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub fn cuda_split_fused_wo_supported(head_dim: usize, hidden_dim: usize) -> bool {
    if std::env::var("LUXI_CUDA_SPLIT_LEGACY_GPU")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        return false;
    }
    let row_fused = std::env::var("LUXI_CUDA_ROW_FUSED")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if !row_fused {
        return false;
    }
    matches!(head_dim, 16 | 32 | 64 | 128) && hidden_dim > 0 && hidden_dim <= 8192
}

/// Batched GEMM MLP (quant TRADE default). Row-serial fused kernel: `LUXI_CUDA_FUSED_ROW_MLP=1` or WNSM weights.
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub fn cuda_use_batched_mlp(weights: &CudaLayerWeights) -> bool {
    if std::env::var("LUXI_CUDA_FUSED_ROW_MLP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        return false;
    }
    weights.d_v_null.is_null()
}

/// All geodesic TRADE forwards use the process-wide stream (no per-layer cudaStreamCreate).
pub fn cuda_use_shared_trade_stream() -> bool {
    if cuda_receipt_audit_mode() {
        return false;
    }
    !std::env::var("LUXI_CUDA_OWN_STREAM")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Persistent multi-layer GPU stack (one H2D, layer loop on device, one D2H). `LUXI_CUDA_QUANT_STACK=0` disables.
pub fn cuda_use_quant_stack() -> bool {
    if cuda_receipt_audit_mode() {
        return false;
    }
    !std::env::var("LUXI_CUDA_QUANT_STACK")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .unwrap_or(false)
}

/// Pack Wq,Wk,Wv into [hidden, 3*hidden] for one deterministic GEMM (column = Q|K|V slice).
pub fn pack_w_qkv_host(wq: &[f32], wk: &[f32], wv: &[f32], hidden: usize) -> Vec<f32> {
    let n3 = hidden * 3;
    let mut out = vec![0.0f32; hidden * n3];
    for j in 0..hidden {
        for o in 0..hidden {
            let base = j * n3;
            out[base + o] = wq[j * hidden + o];
            out[base + hidden + o] = wk[j * hidden + o];
            out[base + 2 * hidden + o] = wv[j * hidden + o];
        }
    }
    out
}

/// P0: device LN1 + packed QKV GEMM (default TRADE). Off: `LUXI_CUDA_CPU_QKV=1` or AUDIT.
pub fn cuda_use_geodesic_qkv() -> bool {
    if cuda_receipt_audit_mode() {
        return false;
    }
    !std::env::var("LUXI_CUDA_CPU_QKV")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Per-kernel GPU phase times (ms) from the last geodesic forward. Set `LUXI_CUDA_PHASE_TIMING=1`.
#[derive(Clone, Copy, Debug, Default)]
pub struct GeodesicPhaseMs {
    pub h2d: f64,
    pub ln1: f64,
    pub qkv: f64,
    pub waller_wo: f64,
    pub res_ln2: f64,
    pub mlp: f64,
    pub d2h: f64,
}

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
static GEODESIC_LAST_PHASE: std::sync::OnceLock<std::sync::Mutex<Option<GeodesicPhaseMs>>> =
    std::sync::OnceLock::new();

pub fn cuda_phase_timing_enabled() -> bool {
    std::env::var("LUXI_CUDA_PHASE_TIMING")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Last geodesic phase breakdown (requires `LUXI_CUDA_PHASE_TIMING=1` on prior forward).
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub fn take_geodesic_phase_ms() -> Option<GeodesicPhaseMs> {
    GEODESIC_LAST_PHASE
        .get_or_init(|| std::sync::Mutex::new(None))
        .lock()
        .ok()?
        .take()
}

#[cfg(any(not(feature = "cuda"), cuda_compilation_failed))]
pub fn take_geodesic_phase_ms() -> Option<GeodesicPhaseMs> {
    None
}

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
fn geodesic_sync_phase_ms(stream: *mut c_void, t0: std::time::Instant) -> (f64, std::time::Instant) {
    unsafe {
        cuda_stream_synchronize(stream);
    }
    let ms = t0.elapsed().as_secs_f64() * 1000.0;
    (ms, std::time::Instant::now())
}

/// High-level blocking helper (one-shot: allocates buffers, runs persistent path, frees).
/// Returns a descriptive error on any CUDA failure (allocation, copy, launch, etc.).
///
/// For sustained throughput, use [`CudaWallerBuffers`] + [`waller_operator_cuda_persistent`].
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub fn waller_operator_cuda_blocking(
    q: &[f32],
    k: &[f32],
    v: &[f32],
    seq_len: usize,
    head_dim: usize,
    num_heads: usize,
    scale: f32,
) -> Result<Vec<f32>, String> {
    let mut buffers = CudaWallerBuffers::new();
    waller_operator_cuda_persistent(q, k, v, &mut buffers, seq_len, head_dim, num_heads, scale, None)
}

const CUDA_MEMCPY_HOST_TO_DEVICE: i32 = 1;
const CUDA_MEMCPY_DEVICE_TO_HOST: i32 = 2;
const CUDA_MEMCPY_DEVICE_TO_DEVICE: i32 = 3;

unsafe fn cuda_free(ptr: *mut c_void) -> i32 {
    cudaFree(ptr)
}

unsafe fn cuda_malloc_host(ptr: *mut *mut c_void, size: usize) -> i32 {
    cudaMallocHost(ptr, size)
}

unsafe fn cuda_free_host(ptr: *mut c_void) -> i32 {
    cudaFreeHost(ptr)
}

unsafe fn cuda_memcpy_h2d(dst: *mut c_void, src: *const c_void, size: usize) {
    cudaMemcpy(dst, src, size, CUDA_MEMCPY_HOST_TO_DEVICE);
}

unsafe fn cuda_memcpy_h2d_async(
    dst: *mut c_void,
    src: *const c_void,
    size: usize,
    stream: *mut c_void,
) {
    cudaMemcpyAsync(dst, src, size, CUDA_MEMCPY_HOST_TO_DEVICE, stream);
}

unsafe fn cuda_memcpy_d2h(dst: *mut c_void, src: *const c_void, size: usize) {
    cudaMemcpy(dst, src, size, CUDA_MEMCPY_DEVICE_TO_HOST);
}

unsafe fn cuda_memcpy_d2h_async(
    dst: *mut c_void,
    src: *const c_void,
    size: usize,
    stream: *mut c_void,
) {
    cudaMemcpyAsync(dst, src, size, CUDA_MEMCPY_DEVICE_TO_HOST, stream);
}

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
unsafe fn cuda_memcpy_d2d_async(
    dst: *mut c_void,
    src: *const c_void,
    size: usize,
    stream: *mut c_void,
) {
    cudaMemcpyAsync(dst, src, size, CUDA_MEMCPY_DEVICE_TO_DEVICE, stream);
}

unsafe fn cuda_device_synchronize() {
    let _ = cudaDeviceSynchronize();
}

unsafe fn cuda_stream_synchronize(stream: *mut c_void) {
    let _ = cudaStreamSynchronize(stream);
}

// ============================================================================
// FFI for the Fused MLP + WNSM Kernel (Practical high-value part of the vision)
// ============================================================================
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
extern "C" {
    fn launch_fused_mlp_wnsm(
        after_ln1: *const f32,
        w_fc: *const f32,
        b_fc: *const f32,
        w_proj: *const f32,
        b_proj: *const f32,
        ln2_gamma: *const f32,
        ln2_beta: *const f32,
        v_null: *const f32,
        output: *mut f32,
        payload: *mut f32,
        seq_len: i32,
        hidden_dim: i32,
        mlp_dim: i32,
        payload_dim: i32,
        ln2_eps: f32,
        stream: *mut c_void,
    );
    fn launch_fused_mlp_wnsm_device_weights(
        after_ln1: *const f32,
        d_w_fc: *const f32,
        d_b_fc: *const f32,
        d_w_proj: *const f32,
        d_b_proj: *const f32,
        d_ln2_gamma: *const f32,
        d_ln2_beta: *const f32,
        d_v_null: *const f32,
        output: *mut f32,
        payload: *mut f32,
        seq_len: i32,
        hidden_dim: i32,
        mlp_dim: i32,
        payload_dim: i32,
        ln2_eps: f32,
        stream: *mut c_void,
    );
    fn launch_mega_fused_layer(
        input: *const f32,
        wq: *const f32,
        wk: *const f32,
        wv: *const f32,
        wo: *const f32,
        w_fc: *const f32,
        b_fc: *const f32,
        w_proj: *const f32,
        b_proj: *const f32,
        ln1_gamma: *const f32,
        ln1_beta: *const f32,
        ln2_gamma: *const f32,
        ln2_beta: *const f32,
        v_null: *const f32,
        output: *mut f32,
        payload: *mut f32,
        seq_len: i32,
        hidden_dim: i32,
        num_heads: i32,
        mlp_dim: i32,
        payload_dim: i32,
        params: MegaLayerParams,
        stream: *mut c_void,
    );
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MegaLayerParams {
    pub attn_scale: f32,
    pub ln1_eps: f32,
    pub ln2_eps: f32,
    pub mlp_dim: i32,
    pub payload_dim: i32,
}

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub unsafe fn mega_fused_layer_cuda(
    input: &[f32],
    // Device-side weight pointers (the caller is responsible for uploading weights to the GPU).
    // This is the realistic production pattern for a mega kernel.
    d_wq: *const f32,
    d_wk: *const f32,
    d_wv: *const f32,
    d_wo: *const f32,
    d_w_fc: *const f32,
    d_b_fc: *const f32,
    d_w_proj: *const f32,
    d_b_proj: *const f32,
    d_ln1_gamma: *const f32,
    d_ln1_beta: *const f32,
    d_ln2_gamma: *const f32,
    d_ln2_beta: *const f32,
    d_v_null: *const f32,
    seq_len: usize,
    hidden_dim: usize,
    num_heads: usize,
    mlp_dim: usize,
    payload_dim: usize,
    params: MegaLayerParams,
    d_payload: *mut f32, // optional device buffer for WNSM payload in/out
) -> Result<Vec<f32>, String> {
    let total = seq_len * hidden_dim;

    let mut d_input: *mut f32 = std::ptr::null_mut();
    let mut d_output: *mut f32 = std::ptr::null_mut();

    if cudaMalloc(&mut d_input as *mut _ as *mut *mut c_void, total * 4) != CUDA_SUCCESS {
        return Err("cudaMalloc failed for input".to_string());
    }
    if cudaMalloc(&mut d_output as *mut _ as *mut *mut c_void, total * 4) != CUDA_SUCCESS {
        cudaFree(d_input as *mut c_void);
        return Err("cudaMalloc failed for output".to_string());
    }

    cuda_memcpy_h2d(
        d_input as *mut c_void,
        input.as_ptr() as *const c_void,
        total * 4,
    );

    launch_mega_fused_layer(
        d_input,
        d_wq,
        d_wk,
        d_wv,
        d_wo,
        d_w_fc,
        d_b_fc,
        d_w_proj,
        d_b_proj,
        d_ln1_gamma,
        d_ln1_beta,
        d_ln2_gamma,
        d_ln2_beta,
        d_v_null,
        d_output,
        d_payload,
        seq_len as i32,
        hidden_dim as i32,
        num_heads as i32,
        mlp_dim as i32,
        payload_dim as i32,
        params,
        std::ptr::null_mut(),
    );

    cuda_device_synchronize();

    let mut host_out = vec![0.0f32; total];
    cuda_memcpy_d2h(
        host_out.as_mut_ptr() as *mut c_void,
        d_output as *const c_void,
        total * 4,
    );

    cuda_free(d_input as *mut c_void);
    cuda_free(d_output as *mut c_void);

    Ok(host_out)
}

/// Experimental convenience wrapper for `forward_mega_fused`.
/// This version uploads the layer's host weights to the device automatically.
/// It is convenient for testing and experimentation but is not the most efficient
/// pattern for repeated calls (better to manage device buffers yourself for production).
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub unsafe fn mega_fused_layer_cuda_with_upload(
    layer: &crate::wnsm_transformer::WNSM_GAE_Layer,
    input: &[f32],
    seq_len: usize,
) -> Result<Vec<f32>, String> {
    let cfg = &layer.config;

    // Allocate and upload weights (inefficient but makes the experimental fused path "just work")
    let mut d_wq: *mut f32 = std::ptr::null_mut();
    cudaMalloc(&mut d_wq as *mut _ as *mut *mut c_void, layer.wq.len() * 4);
    cuda_memcpy_h2d(
        d_wq as *mut c_void,
        layer.wq.as_ptr() as *const c_void,
        layer.wq.len() * 4,
    );

    let mut d_wk: *mut f32 = std::ptr::null_mut();
    cudaMalloc(&mut d_wk as *mut _ as *mut *mut c_void, layer.wk.len() * 4);
    cuda_memcpy_h2d(
        d_wk as *mut c_void,
        layer.wk.as_ptr() as *const c_void,
        layer.wk.len() * 4,
    );

    let mut d_wv: *mut f32 = std::ptr::null_mut();
    cudaMalloc(&mut d_wv as *mut _ as *mut *mut c_void, layer.wv.len() * 4);
    cuda_memcpy_h2d(
        d_wv as *mut c_void,
        layer.wv.as_ptr() as *const c_void,
        layer.wv.len() * 4,
    );

    let mut d_wo: *mut f32 = std::ptr::null_mut();
    cudaMalloc(&mut d_wo as *mut _ as *mut *mut c_void, layer.wo.len() * 4);
    cuda_memcpy_h2d(
        d_wo as *mut c_void,
        layer.wo.as_ptr() as *const c_void,
        layer.wo.len() * 4,
    );

    let mut d_w_fc: *mut f32 = std::ptr::null_mut();
    cudaMalloc(
        &mut d_w_fc as *mut _ as *mut *mut c_void,
        layer.w_fc.len() * 4,
    );
    cuda_memcpy_h2d(
        d_w_fc as *mut c_void,
        layer.w_fc.as_ptr() as *const c_void,
        layer.w_fc.len() * 4,
    );

    let mut d_b_fc: *mut f32 = std::ptr::null_mut();
    cudaMalloc(
        &mut d_b_fc as *mut _ as *mut *mut c_void,
        layer.b_fc.len() * 4,
    );
    cuda_memcpy_h2d(
        d_b_fc as *mut c_void,
        layer.b_fc.as_ptr() as *const c_void,
        layer.b_fc.len() * 4,
    );

    let mut d_w_proj: *mut f32 = std::ptr::null_mut();
    cudaMalloc(
        &mut d_w_proj as *mut _ as *mut *mut c_void,
        layer.w_proj.len() * 4,
    );
    cuda_memcpy_h2d(
        d_w_proj as *mut c_void,
        layer.w_proj.as_ptr() as *const c_void,
        layer.w_proj.len() * 4,
    );

    let mut d_b_proj: *mut f32 = std::ptr::null_mut();
    cudaMalloc(
        &mut d_b_proj as *mut _ as *mut *mut c_void,
        layer.b_proj.len() * 4,
    );
    cuda_memcpy_h2d(
        d_b_proj as *mut c_void,
        layer.b_proj.as_ptr() as *const c_void,
        layer.b_proj.len() * 4,
    );

    let mut d_ln1_gamma: *mut f32 = std::ptr::null_mut();
    cudaMalloc(
        &mut d_ln1_gamma as *mut _ as *mut *mut c_void,
        layer.ln1_gamma.len() * 4,
    );
    cuda_memcpy_h2d(
        d_ln1_gamma as *mut c_void,
        layer.ln1_gamma.as_ptr() as *const c_void,
        layer.ln1_gamma.len() * 4,
    );

    let mut d_ln1_beta: *mut f32 = std::ptr::null_mut();
    cudaMalloc(
        &mut d_ln1_beta as *mut _ as *mut *mut c_void,
        layer.ln1_beta.len() * 4,
    );
    cuda_memcpy_h2d(
        d_ln1_beta as *mut c_void,
        layer.ln1_beta.as_ptr() as *const c_void,
        layer.ln1_beta.len() * 4,
    );

    let mut d_ln2_gamma: *mut f32 = std::ptr::null_mut();
    cudaMalloc(
        &mut d_ln2_gamma as *mut _ as *mut *mut c_void,
        layer.ln2_gamma.len() * 4,
    );
    cuda_memcpy_h2d(
        d_ln2_gamma as *mut c_void,
        layer.ln2_gamma.as_ptr() as *const c_void,
        layer.ln2_gamma.len() * 4,
    );

    let mut d_ln2_beta: *mut f32 = std::ptr::null_mut();
    cudaMalloc(
        &mut d_ln2_beta as *mut _ as *mut *mut c_void,
        layer.ln2_beta.len() * 4,
    );
    cuda_memcpy_h2d(
        d_ln2_beta as *mut c_void,
        layer.ln2_beta.as_ptr() as *const c_void,
        layer.ln2_beta.len() * 4,
    );

    let mut d_v_null: *mut f32 = std::ptr::null_mut();
    if let Some(vn) = &layer.v_null {
        cudaMalloc(&mut d_v_null as *mut _ as *mut *mut c_void, vn.len() * 4);
        cuda_memcpy_h2d(
            d_v_null as *mut c_void,
            vn.as_ptr() as *const c_void,
            vn.len() * 4,
        );
    }

    let params = MegaLayerParams {
        attn_scale: 1.0 / (cfg.head_dim as f32).sqrt(),
        ln1_eps: cfg.ln_eps,
        ln2_eps: cfg.ln_eps,
        mlp_dim: cfg.mlp_dim as i32,
        payload_dim: layer.payload_dim as i32,
    };

    // Actually call the real fused mega kernel with the device weight pointers we just uploaded.
    let result = mega_fused_layer_cuda(
        input,
        d_wq as *const f32,
        d_wk as *const f32,
        d_wv as *const f32,
        d_wo as *const f32,
        d_w_fc as *const f32,
        d_b_fc as *const f32,
        d_w_proj as *const f32,
        d_b_proj as *const f32,
        d_ln1_gamma as *const f32,
        d_ln1_beta as *const f32,
        d_ln2_gamma as *const f32,
        d_ln2_beta as *const f32,
        d_v_null as *const f32,
        seq_len,
        cfg.hidden_dim,
        cfg.num_heads,
        cfg.mlp_dim,
        layer.payload_dim,
        params,
        std::ptr::null_mut(),
    );

    // Free device weights (very inefficient for repeated calls, but works for testing the vision)
    cuda_free(d_wq as *mut c_void);
    cuda_free(d_wk as *mut c_void);
    cuda_free(d_wv as *mut c_void);
    cuda_free(d_wo as *mut c_void);
    cuda_free(d_w_fc as *mut c_void);
    cuda_free(d_b_fc as *mut c_void);
    cuda_free(d_w_proj as *mut c_void);
    cuda_free(d_b_proj as *mut c_void);
    cuda_free(d_ln1_gamma as *mut c_void);
    cuda_free(d_ln1_beta as *mut c_void);
    cuda_free(d_ln2_gamma as *mut c_void);
    cuda_free(d_ln2_beta as *mut c_void);
    if !d_v_null.is_null() {
        cuda_free(d_v_null as *mut c_void);
    }

    result
}

/// Version of the fused mega call that uses pre-uploaded persistent device weights from CudaLayerWeights.
/// This enables efficient repeated calls without re-uploading.
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub unsafe fn mega_fused_layer_cuda_with_persistent_weights(
    cfg: &crate::config::Config,
    weights: &CudaLayerWeights,
    input: &[f32],
    seq_len: usize,
) -> Result<Vec<f32>, String> {
    let params = MegaLayerParams {
        attn_scale: 1.0 / (cfg.head_dim as f32).sqrt(),
        ln1_eps: cfg.ln_eps,
        ln2_eps: cfg.ln_eps,
        mlp_dim: cfg.mlp_dim as i32,
        payload_dim: weights.payload_dim as i32,
    };

    let v_null_ptr = if weights.payload_dim > 0 && !weights.d_v_null.is_null() {
        weights.d_v_null as *const f32
    } else {
        std::ptr::null()
    };

    mega_fused_layer_cuda(
        input,
        weights.d_wq as *const f32,
        weights.d_wk as *const f32,
        weights.d_wv as *const f32,
        weights.d_wo as *const f32,
        weights.d_w_fc as *const f32,
        weights.d_b_fc as *const f32,
        weights.d_w_proj as *const f32,
        weights.d_b_proj as *const f32,
        weights.d_ln1_gamma as *const f32,
        weights.d_ln1_beta as *const f32,
        weights.d_ln2_gamma as *const f32,
        weights.d_ln2_beta as *const f32,
        v_null_ptr,
        seq_len,
        cfg.hidden_dim,
        cfg.num_heads,
        cfg.mlp_dim,
        weights.payload_dim,
        params,
        std::ptr::null_mut(),
    )
}

// CUDA runtime FFI declarations (only when the cuda feature is active)
#[cfg(feature = "cuda")]
extern "C" {
    fn cudaMalloc(devPtr: *mut *mut c_void, size: usize) -> i32;
    fn cudaFree(devPtr: *mut c_void) -> i32;
    fn cudaMemcpy(dst: *mut c_void, src: *const c_void, count: usize, kind: i32) -> i32;
    fn cudaMemcpyAsync(
        dst: *mut c_void,
        src: *const c_void,
        count: usize,
        kind: i32,
        stream: *mut c_void,
    ) -> i32;
    fn cudaDeviceSynchronize() -> i32;
    fn cudaStreamCreate(stream: *mut *mut c_void) -> i32;
    fn cudaStreamDestroy(stream: *mut c_void) -> i32;
    fn cudaStreamSynchronize(stream: *mut c_void) -> i32;
    fn cudaMallocHost(ptr: *mut *mut c_void, size: usize) -> i32;
    fn cudaFreeHost(ptr: *mut c_void) -> i32;
}

// ============================================================================
// High-level wrapper for Fused MLP + WNSM (recommended for energy efficiency)
// ============================================================================
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub unsafe fn fused_mlp_wnsm_cuda(
    after_ln1: &[f32],
    w_fc: &[f32],
    b_fc: &[f32],
    w_proj: &[f32],
    b_proj: &[f32],
    ln2_gamma: &[f32],
    ln2_beta: &[f32],
    v_null: *const f32,
    seq_len: usize,
    hidden_dim: usize,
    mlp_dim: usize,
    payload_dim: usize,
    ln2_eps: f32,
    mut payload_host: Option<&mut [f32]>, // optional host payload buffer for WNSM in/out
) -> Result<Vec<f32>, String> {
    let total = seq_len * hidden_dim;

    let mut d_after_ln1: *mut f32 = std::ptr::null_mut();
    let mut d_output: *mut f32 = std::ptr::null_mut();
    let mut d_payload_buf: *mut f32 = std::ptr::null_mut();

    if cudaMalloc(&mut d_after_ln1 as *mut _ as *mut *mut c_void, total * 4) != CUDA_SUCCESS {
        return Err("cudaMalloc failed for after_ln1".to_string());
    }
    if cudaMalloc(&mut d_output as *mut _ as *mut *mut c_void, total * 4) != CUDA_SUCCESS {
        cudaFree(d_after_ln1 as *mut c_void);
        return Err("cudaMalloc failed for output".to_string());
    }

    if payload_dim > 0 && payload_host.is_some() {
        if cudaMalloc(
            &mut d_payload_buf as *mut _ as *mut *mut c_void,
            seq_len * payload_dim * 4,
        ) != CUDA_SUCCESS
        {
            cudaFree(d_after_ln1 as *mut c_void);
            cudaFree(d_output as *mut c_void);
            return Err("cudaMalloc failed for payload".to_string());
        }
        if let Some(ref p) = payload_host {
            cuda_memcpy_h2d(
                d_payload_buf as *mut c_void,
                p.as_ptr() as *const c_void,
                seq_len * payload_dim * 4,
            );
        }
    }

    cuda_memcpy_h2d(
        d_after_ln1 as *mut c_void,
        after_ln1.as_ptr() as *const c_void,
        total * 4,
    );

    launch_fused_mlp_wnsm(
        d_after_ln1,
        w_fc.as_ptr(),
        b_fc.as_ptr(),
        w_proj.as_ptr(),
        b_proj.as_ptr(),
        ln2_gamma.as_ptr(),
        ln2_beta.as_ptr(),
        v_null,
        d_output,
        d_payload_buf,
        seq_len as i32,
        hidden_dim as i32,
        mlp_dim as i32,
        payload_dim as i32,
        ln2_eps,
        std::ptr::null_mut(),
    );

    cuda_device_synchronize();

    let mut host_out = vec![0.0f32; total];
    cuda_memcpy_d2h(
        host_out.as_mut_ptr() as *mut c_void,
        d_output as *const c_void,
        total * 4,
    );

    // Copy payload back if provided
    if payload_dim > 0 {
        if let Some(ref mut p) = payload_host {
            if !d_payload_buf.is_null() {
                cuda_memcpy_d2h(
                    p.as_mut_ptr() as *mut c_void,
                    d_payload_buf as *const c_void,
                    seq_len * payload_dim * 4,
                );
            }
        }
    }

    cuda_free(d_after_ln1 as *mut c_void);
    cuda_free(d_output as *mut c_void);
    if !d_payload_buf.is_null() {
        cuda_free(d_payload_buf as *mut c_void);
    }

    Ok(host_out)
}

/// Production persistence version of fused MLP + WNSM.
/// Uses pre-uploaded device weight pointers from CudaLayerWeights (no re-upload of large MLP/V_null).
/// Only the per-forward activation (after_ln1) and optional payload buffers are transferred this call.
/// This is the high calcs-per-joule path for the MLP stage on CUDA.
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub unsafe fn fused_mlp_wnsm_cuda_with_device_weights(
    after_ln1: &[f32],
    weights: &CudaLayerWeights,
    _ln2_gamma: &[f32],
    _ln2_beta: &[f32],
    seq_len: usize,
    hidden_dim: usize,
    mlp_dim: usize,
    payload_dim: usize,
    ln2_eps: f32,
    mut payload_host: Option<&mut [f32]>,
) -> Result<Vec<f32>, String> {
    let total = seq_len * hidden_dim;

    let mut d_after_ln1: *mut f32 = std::ptr::null_mut();
    let mut d_output: *mut f32 = std::ptr::null_mut();
    let mut d_payload_buf: *mut f32 = std::ptr::null_mut();

    if cudaMalloc(&mut d_after_ln1 as *mut _ as *mut *mut c_void, total * 4) != CUDA_SUCCESS {
        return Err("cudaMalloc failed for after_ln1 (persistent path)".to_string());
    }
    if cudaMalloc(&mut d_output as *mut _ as *mut *mut c_void, total * 4) != CUDA_SUCCESS {
        cudaFree(d_after_ln1 as *mut c_void);
        return Err("cudaMalloc failed for output (persistent path)".to_string());
    }

    if payload_dim > 0 && payload_host.is_some() {
        if cudaMalloc(
            &mut d_payload_buf as *mut _ as *mut *mut c_void,
            seq_len * payload_dim * 4,
        ) != CUDA_SUCCESS
        {
            cudaFree(d_after_ln1 as *mut c_void);
            cudaFree(d_output as *mut c_void);
            return Err("cudaMalloc failed for payload (persistent path)".to_string());
        }
        if let Some(ref p) = payload_host {
            cuda_memcpy_h2d(
                d_payload_buf as *mut c_void,
                p.as_ptr() as *const c_void,
                seq_len * payload_dim * 4,
            );
        }
    }

    cuda_memcpy_h2d(
        d_after_ln1 as *mut c_void,
        after_ln1.as_ptr() as *const c_void,
        total * 4,
    );

    // Use the persistent device pointers for the large weights (the win)
    let v_null_ptr = if weights.payload_dim > 0 {
        weights.d_v_null as *const f32
    } else {
        std::ptr::null()
    };

    launch_fused_mlp_wnsm_device_weights(
        d_after_ln1 as *const f32,
        weights.d_w_fc as *const f32,
        weights.d_b_fc as *const f32,
        weights.d_w_proj as *const f32,
        weights.d_b_proj as *const f32,
        weights.d_ln2_gamma as *const f32,
        weights.d_ln2_beta as *const f32,
        v_null_ptr,
        d_output,
        d_payload_buf,
        seq_len as i32,
        hidden_dim as i32,
        mlp_dim as i32,
        payload_dim as i32,
        ln2_eps,
        std::ptr::null_mut(),
    );

    cuda_device_synchronize();

    let mut host_out = vec![0.0f32; total];
    cuda_memcpy_d2h(
        host_out.as_mut_ptr() as *mut c_void,
        d_output as *const c_void,
        total * 4,
    );

    if payload_dim > 0 {
        if let Some(ref mut p) = payload_host {
            if !d_payload_buf.is_null() {
                cuda_memcpy_d2h(
                    p.as_mut_ptr() as *mut c_void,
                    d_payload_buf as *const c_void,
                    seq_len * payload_dim * 4,
                );
            }
        }
    }

    cuda_free(d_after_ln1 as *mut c_void);
    cuda_free(d_output as *mut c_void);
    if !d_payload_buf.is_null() {
        cuda_free(d_payload_buf as *mut c_void);
    }

    Ok(host_out)
}

const CUDA_SUCCESS: i32 = 0;

// ============================================================================
// Device-side weight persistence for efficiency (no re-upload every call)
// ============================================================================
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub struct CudaLayerWeights {
    // Attention
    pub d_wq: *mut f32,
    pub d_wk: *mut f32,
    pub d_wv: *mut f32,
    pub d_wo: *mut f32,
    /// Packed [hidden, 3*hidden] for geodesic QKV GEMM.
    pub d_w_qkv: *mut f32,
    // MLP
    pub d_w_fc: *mut f32,
    pub d_b_fc: *mut f32,
    pub d_w_proj: *mut f32,
    pub d_b_proj: *mut f32,
    // LayerNorms
    pub d_ln1_gamma: *mut f32,
    pub d_ln1_beta: *mut f32,
    pub d_ln2_gamma: *mut f32,
    pub d_ln2_beta: *mut f32,
    // WNSM
    pub d_v_null: *mut f32,
    pub payload_dim: usize,
}

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
impl CudaLayerWeights {
    pub unsafe fn upload_from_layer(
        layer: &crate::wnsm_transformer::WNSM_GAE_Layer,
    ) -> Result<Self, String> {
        let mut weights = Self {
            d_wq: std::ptr::null_mut(),
            d_wk: std::ptr::null_mut(),
            d_wv: std::ptr::null_mut(),
            d_wo: std::ptr::null_mut(),
            d_w_qkv: std::ptr::null_mut(),
            d_w_fc: std::ptr::null_mut(),
            d_b_fc: std::ptr::null_mut(),
            d_w_proj: std::ptr::null_mut(),
            d_b_proj: std::ptr::null_mut(),
            d_ln1_gamma: std::ptr::null_mut(),
            d_ln1_beta: std::ptr::null_mut(),
            d_ln2_gamma: std::ptr::null_mut(),
            d_ln2_beta: std::ptr::null_mut(),
            d_v_null: std::ptr::null_mut(),
            payload_dim: layer.payload_dim,
        };

        // Upload all weights (called once)
        cudaMalloc(
            &mut weights.d_wq as *mut _ as *mut *mut c_void,
            layer.wq.len() * 4,
        );
        cuda_memcpy_h2d(
            weights.d_wq as *mut c_void,
            layer.wq.as_ptr() as *const c_void,
            layer.wq.len() * 4,
        );

        cudaMalloc(
            &mut weights.d_wk as *mut _ as *mut *mut c_void,
            layer.wk.len() * 4,
        );
        cuda_memcpy_h2d(
            weights.d_wk as *mut c_void,
            layer.wk.as_ptr() as *const c_void,
            layer.wk.len() * 4,
        );

        cudaMalloc(
            &mut weights.d_wv as *mut _ as *mut *mut c_void,
            layer.wv.len() * 4,
        );
        cuda_memcpy_h2d(
            weights.d_wv as *mut c_void,
            layer.wv.as_ptr() as *const c_void,
            layer.wv.len() * 4,
        );

        cudaMalloc(
            &mut weights.d_wo as *mut _ as *mut *mut c_void,
            layer.wo.len() * 4,
        );
        cuda_memcpy_h2d(
            weights.d_wo as *mut c_void,
            layer.wo.as_ptr() as *const c_void,
            layer.wo.len() * 4,
        );

        let h = layer.config.hidden_dim;
        let w_qkv_host = pack_w_qkv_host(&layer.wq, &layer.wk, &layer.wv, h);
        cudaMalloc(
            &mut weights.d_w_qkv as *mut _ as *mut *mut c_void,
            w_qkv_host.len() * 4,
        );
        cuda_memcpy_h2d(
            weights.d_w_qkv as *mut c_void,
            w_qkv_host.as_ptr() as *const c_void,
            w_qkv_host.len() * 4,
        );

        cudaMalloc(
            &mut weights.d_w_fc as *mut _ as *mut *mut c_void,
            layer.w_fc.len() * 4,
        );
        cuda_memcpy_h2d(
            weights.d_w_fc as *mut c_void,
            layer.w_fc.as_ptr() as *const c_void,
            layer.w_fc.len() * 4,
        );

        cudaMalloc(
            &mut weights.d_b_fc as *mut _ as *mut *mut c_void,
            layer.b_fc.len() * 4,
        );
        cuda_memcpy_h2d(
            weights.d_b_fc as *mut c_void,
            layer.b_fc.as_ptr() as *const c_void,
            layer.b_fc.len() * 4,
        );

        cudaMalloc(
            &mut weights.d_w_proj as *mut _ as *mut *mut c_void,
            layer.w_proj.len() * 4,
        );
        cuda_memcpy_h2d(
            weights.d_w_proj as *mut c_void,
            layer.w_proj.as_ptr() as *const c_void,
            layer.w_proj.len() * 4,
        );

        cudaMalloc(
            &mut weights.d_b_proj as *mut _ as *mut *mut c_void,
            layer.b_proj.len() * 4,
        );
        cuda_memcpy_h2d(
            weights.d_b_proj as *mut c_void,
            layer.b_proj.as_ptr() as *const c_void,
            layer.b_proj.len() * 4,
        );

        cudaMalloc(
            &mut weights.d_ln1_gamma as *mut _ as *mut *mut c_void,
            layer.ln1_gamma.len() * 4,
        );
        cuda_memcpy_h2d(
            weights.d_ln1_gamma as *mut c_void,
            layer.ln1_gamma.as_ptr() as *const c_void,
            layer.ln1_gamma.len() * 4,
        );

        cudaMalloc(
            &mut weights.d_ln1_beta as *mut _ as *mut *mut c_void,
            layer.ln1_beta.len() * 4,
        );
        cuda_memcpy_h2d(
            weights.d_ln1_beta as *mut c_void,
            layer.ln1_beta.as_ptr() as *const c_void,
            layer.ln1_beta.len() * 4,
        );

        cudaMalloc(
            &mut weights.d_ln2_gamma as *mut _ as *mut *mut c_void,
            layer.ln2_gamma.len() * 4,
        );
        cuda_memcpy_h2d(
            weights.d_ln2_gamma as *mut c_void,
            layer.ln2_gamma.as_ptr() as *const c_void,
            layer.ln2_gamma.len() * 4,
        );

        cudaMalloc(
            &mut weights.d_ln2_beta as *mut _ as *mut *mut c_void,
            layer.ln2_beta.len() * 4,
        );
        cuda_memcpy_h2d(
            weights.d_ln2_beta as *mut c_void,
            layer.ln2_beta.as_ptr() as *const c_void,
            layer.ln2_beta.len() * 4,
        );

        if let Some(vn) = &layer.v_null {
            cudaMalloc(
                &mut weights.d_v_null as *mut _ as *mut *mut c_void,
                vn.len() * 4,
            );
            cuda_memcpy_h2d(
                weights.d_v_null as *mut c_void,
                vn.as_ptr() as *const c_void,
                vn.len() * 4,
            );
        }

        Ok(weights)
    }
}

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
impl Drop for CudaLayerWeights {
    fn drop(&mut self) {
        unsafe {
            if !self.d_wq.is_null() {
                cudaFree(self.d_wq as *mut c_void);
            }
            if !self.d_wk.is_null() {
                cudaFree(self.d_wk as *mut c_void);
            }
            if !self.d_wv.is_null() {
                cudaFree(self.d_wv as *mut c_void);
            }
            if !self.d_wo.is_null() {
                cudaFree(self.d_wo as *mut c_void);
            }
            if !self.d_w_qkv.is_null() {
                cudaFree(self.d_w_qkv as *mut c_void);
            }
            if !self.d_w_fc.is_null() {
                cudaFree(self.d_w_fc as *mut c_void);
            }
            if !self.d_b_fc.is_null() {
                cudaFree(self.d_b_fc as *mut c_void);
            }
            if !self.d_w_proj.is_null() {
                cudaFree(self.d_w_proj as *mut c_void);
            }
            if !self.d_b_proj.is_null() {
                cudaFree(self.d_b_proj as *mut c_void);
            }
            if !self.d_ln1_gamma.is_null() {
                cudaFree(self.d_ln1_gamma as *mut c_void);
            }
            if !self.d_ln1_beta.is_null() {
                cudaFree(self.d_ln1_beta as *mut c_void);
            }
            if !self.d_ln2_gamma.is_null() {
                cudaFree(self.d_ln2_gamma as *mut c_void);
            }
            if !self.d_ln2_beta.is_null() {
                cudaFree(self.d_ln2_beta as *mut c_void);
            }
            if !self.d_v_null.is_null() {
                cudaFree(self.d_v_null as *mut c_void);
            }
        }
    }
}

// ============================================================================
// GPU layer composer: LN+QKV on device → Waller → host post (wo/MLP)
// ============================================================================

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub struct CudaLayerComposer {
    d_input: *mut f32,
    capacity: usize,
    stream: *mut c_void,
    stream_owned: bool,
}

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
impl CudaLayerComposer {
    pub fn new() -> Self {
        Self {
            d_input: std::ptr::null_mut(),
            capacity: 0,
            stream: std::ptr::null_mut(),
            stream_owned: true,
        }
    }

    /// Share the TRADE stream across decoder layers (see `decoder_forward_cuda_quant_stack`).
    pub unsafe fn borrow_stream(&mut self, stream: *mut c_void) {
        if self.stream_owned && !self.stream.is_null() {
            cudaStreamDestroy(self.stream);
        }
        self.stream = stream;
        self.stream_owned = false;
    }

    unsafe fn ensure(&mut self, total: usize) -> Result<(), String> {
        if total <= self.capacity && !self.d_input.is_null() {
            return Ok(());
        }
        if !self.d_input.is_null() {
            cuda_free(self.d_input as *mut c_void);
            self.d_input = std::ptr::null_mut();
        }
        if self.stream.is_null() {
            self.stream = cuda_create_stream()?;
            self.stream_owned = true;
        }
        let cap = total.max(1).next_power_of_two();
        if cudaMalloc(&mut self.d_input as *mut _ as *mut *mut c_void, cap * 4) != CUDA_SUCCESS {
            return Err("cudaMalloc failed for layer input".into());
        }
        self.capacity = cap;
        Ok(())
    }

    /// Allocate `d_input` [total] without host upload (quant stack handoff between layers).
    pub unsafe fn ensure_device_input(&mut self, total: usize) -> Result<(), String> {
        self.ensure(total)
    }

    pub unsafe fn upload_input(&mut self, input: &[f32], total: usize) -> Result<(), String> {
        self.ensure(total)?;
        cuda_memcpy_h2d_async(
            self.d_input as *mut c_void,
            input.as_ptr() as *const c_void,
            total * 4,
            self.stream,
        );
        cuda_stream_synchronize(self.stream);
        Ok(())
    }
}

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
impl Drop for CudaLayerComposer {
    fn drop(&mut self) {
        unsafe {
            if !self.d_input.is_null() {
                cuda_free(self.d_input as *mut c_void);
            }
            if self.stream_owned && !self.stream.is_null() {
                cudaStreamDestroy(self.stream);
            }
        }
    }
}

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
struct CudaStreamHandle(usize);

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
unsafe impl Send for CudaStreamHandle {}
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
unsafe impl Sync for CudaStreamHandle {}

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
static CUDA_TRADE_STREAM: std::sync::OnceLock<std::sync::Mutex<CudaStreamHandle>> =
    std::sync::OnceLock::new();

/// One process-wide TRADE stream (quant stack + deep decoders). Never destroyed until exit.
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub unsafe fn cuda_trade_stream() -> Result<*mut c_void, String> {
    let slot = CUDA_TRADE_STREAM.get_or_init(|| std::sync::Mutex::new(CudaStreamHandle(0)));
    let mut guard = slot
        .lock()
        .map_err(|_| "cuda_trade_stream lock poisoned".to_string())?;
    if guard.0 == 0 {
        guard.0 = cuda_create_stream()? as usize;
    }
    Ok(guard.0 as *mut c_void)
}

/// Drop borrowed stream handles so the next single-layer forward can allocate its own streams.
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub fn cuda_detach_layer_streams(layer: &mut crate::wnsm_transformer::WNSM_GAE_Layer) {
    if let Some(ref mut bufs) = layer.cuda_waller_buffers {
        bufs.stream = std::ptr::null_mut();
        bufs.stream_owned = true;
    }
    if let Some(ref mut composer) = layer.cuda_layer_composer {
        composer.stream = std::ptr::null_mut();
        composer.stream_owned = true;
    }
}

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
unsafe fn cuda_create_stream() -> Result<*mut c_void, String> {
    let mut stream: *mut c_void = std::ptr::null_mut();
    if cudaStreamCreate(&mut stream as *mut *mut c_void) != CUDA_SUCCESS {
        return Err("cudaStreamCreate failed".to_string());
    }
    Ok(stream)
}

/// CPU-built Q/K/V → device Waller (receipt-safe; matches reference projections).
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub unsafe fn layer_forward_cuda_attn_from_qkv(
    waller_bufs: &mut CudaWallerBuffers,
    q: &[f32],
    k: &[f32],
    v: &[f32],
    seq_len: usize,
    head_dim: usize,
    num_heads: usize,
) -> Result<Vec<f32>, String> {
    let total = q.len();
    waller_bufs.upload_inputs(q, k, v, total)?;
    let scale = 1.0 / (head_dim as f32).sqrt();
    let (_, _, cuda_attn) = waller_bufs.launch_only(
        total,
        seq_len,
        head_dim,
        num_heads,
        scale,
        true,
    )?;
    Ok(cuda_attn)
}

/// Production split path: H2D input → device LN1 → device Q/K/V GEMM → Waller+wo → D2H.
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub unsafe fn layer_forward_cuda_split_device(
    composer: &mut CudaLayerComposer,
    waller_bufs: &mut CudaWallerBuffers,
    weights: &CudaLayerWeights,
    input: &[f32],
    seq_len: usize,
    hidden_dim: usize,
    head_dim: usize,
    num_heads: usize,
    ln_eps: f32,
) -> Result<Vec<f32>, String> {
    let total = seq_len * hidden_dim;
    composer.upload_input(input, total)?;
    waller_bufs.ensure_capacity(total)?;

    launch_ln1_rows(
        composer.d_input as *const f32,
        waller_bufs.d_normed,
        weights.d_ln1_gamma as *const f32,
        weights.d_ln1_beta as *const f32,
        seq_len as i32,
        hidden_dim as i32,
        ln_eps,
        composer.stream,
    );
    launch_matmul_f32(
        waller_bufs.d_normed,
        weights.d_wq as *const f32,
        waller_bufs.d_q,
        seq_len as i32,
        hidden_dim as i32,
        hidden_dim as i32,
        composer.stream,
    );
    launch_matmul_f32(
        waller_bufs.d_normed,
        weights.d_wk as *const f32,
        waller_bufs.d_k,
        seq_len as i32,
        hidden_dim as i32,
        hidden_dim as i32,
        composer.stream,
    );
    launch_matmul_f32(
        waller_bufs.d_normed,
        weights.d_wv as *const f32,
        waller_bufs.d_v,
        seq_len as i32,
        hidden_dim as i32,
        hidden_dim as i32,
        composer.stream,
    );
    cuda_stream_synchronize(composer.stream);
    waller_bufs.inputs_on_device = true;

    let scale = 1.0 / (head_dim as f32).sqrt();
    waller_bufs.launch_waller_then_wo(
        total,
        seq_len,
        hidden_dim,
        head_dim,
        num_heads,
        scale,
        weights.d_wo as *const f32,
    )
}

/// Lane TRADE (GPU): LN1→QKV→Waller→wo→residual+LN2 on device; returns pre-MLP activations for CPU post.
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub unsafe fn layer_forward_cuda_trade(
    composer: &mut CudaLayerComposer,
    waller_bufs: &mut CudaWallerBuffers,
    weights: &CudaLayerWeights,
    input: &[f32],
    seq_len: usize,
    hidden_dim: usize,
    head_dim: usize,
    num_heads: usize,
    ln_eps: f32,
    mlp_dim: usize,
) -> Result<Vec<f32>, String> {
    let total = seq_len * hidden_dim;
    composer.upload_input(input, total)?;
    waller_bufs.ensure_capacity(total)?;

    launch_ln1_rows(
        composer.d_input as *const f32,
        waller_bufs.d_normed,
        weights.d_ln1_gamma as *const f32,
        weights.d_ln1_beta as *const f32,
        seq_len as i32,
        hidden_dim as i32,
        ln_eps,
        composer.stream,
    );
    launch_matmul_f32(
        waller_bufs.d_normed,
        weights.d_wq as *const f32,
        waller_bufs.d_q,
        seq_len as i32,
        hidden_dim as i32,
        hidden_dim as i32,
        composer.stream,
    );
    launch_matmul_f32(
        waller_bufs.d_normed,
        weights.d_wk as *const f32,
        waller_bufs.d_k,
        seq_len as i32,
        hidden_dim as i32,
        hidden_dim as i32,
        composer.stream,
    );
    launch_matmul_f32(
        waller_bufs.d_normed,
        weights.d_wv as *const f32,
        waller_bufs.d_v,
        seq_len as i32,
        hidden_dim as i32,
        hidden_dim as i32,
        composer.stream,
    );
    waller_bufs.inputs_on_device = true;
    let scale = 1.0 / (head_dim as f32).sqrt();
    if cuda_split_fused_wo_supported(head_dim, hidden_dim) {
        launch_waller_fused_wo(
            waller_bufs.d_q,
            waller_bufs.d_k,
            waller_bufs.d_v,
            weights.d_wo as *const f32,
            waller_bufs.d_proj,
            seq_len as i32,
            head_dim as i32,
            num_heads as i32,
            scale,
            composer.stream,
        );
    } else {
        launch_waller_operator(
            waller_bufs.d_q,
            waller_bufs.d_k,
            waller_bufs.d_v,
            waller_bufs.d_out,
            seq_len as i32,
            head_dim as i32,
            num_heads as i32,
            scale,
            composer.stream,
        );
        launch_matmul_f32(
            waller_bufs.d_out,
            weights.d_wo as *const f32,
            waller_bufs.d_proj,
            seq_len as i32,
            hidden_dim as i32,
            hidden_dim as i32,
            composer.stream,
        );
    }

    launch_residual_ln2_rows(
        composer.d_input as *const f32,
        waller_bufs.d_proj,
        waller_bufs.d_normed,
        weights.d_ln2_gamma as *const f32,
        weights.d_ln2_beta as *const f32,
        seq_len as i32,
        hidden_dim as i32,
        ln_eps,
        composer.stream,
    );

    cuda_stream_synchronize(composer.stream);
    cuda_device_synchronize();

    // D2H pre-MLP activations; MLP+LN2 on CPU (receipt-safe, matches forward() post-LN2 path).
    // Do not use fused_mlp_wnsm_kernel here: it used fixed [128] stack arrays (UB at hidden>128).
    let _ = (mlp_dim, weights);
    let mut after_ln2 = vec![0.0f32; total];
    cuda_memcpy_d2h_async(
        after_ln2.as_mut_ptr() as *mut c_void,
        waller_bufs.d_normed as *const c_void,
        total * 4,
        composer.stream,
    );
    cuda_stream_synchronize(composer.stream);
    Ok(after_ln2)
}

/// Lane AUDIT: bit-exact CPU QKV + CPU post (receipt gate). Default = Lane TRADE (GPU-first).
/// Block until all device work completes (for benchmarks).
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub fn cuda_device_sync() {
    unsafe {
        cuda_device_synchronize();
    }
}

pub fn cuda_receipt_audit_mode() -> bool {
    std::env::var("LUXI_RECEIPT_AUDIT")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn cuda_use_cpu_qkv() -> bool {
    cuda_receipt_audit_mode() && !cuda_use_gpu_qkv_force()
}

fn cuda_use_gpu_qkv_force() -> bool {
    std::env::var("LUXI_CUDA_GPU_QKV")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// CPU QKV + device Waller + device wo (one D2H). Receipt-safe when wo matches CPU matmul order.
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub unsafe fn layer_forward_cuda_attn_wo_from_qkv(
    waller_bufs: &mut CudaWallerBuffers,
    weights: &CudaLayerWeights,
    q: &[f32],
    k: &[f32],
    v: &[f32],
    seq_len: usize,
    hidden_dim: usize,
    head_dim: usize,
    num_heads: usize,
) -> Result<Vec<f32>, String> {
    let total = q.len();
    waller_bufs.upload_inputs(q, k, v, total)?;
    let scale = 1.0 / (head_dim as f32).sqrt();
    waller_bufs.launch_waller_then_wo(
        total,
        seq_len,
        hidden_dim,
        head_dim,
        num_heads,
        scale,
        weights.d_wo as *const f32,
    )
}

/// GPU fused MLP post (TRADE default). Off for receipt audit or `LUXI_CUDA_CPU_POST=1`.
pub fn cuda_use_gpu_post() -> bool {
    if cuda_receipt_audit_mode() {
        return false;
    }
    !std::env::var("LUXI_CUDA_CPU_POST")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Geodesic TRADE layer on GPU. Set `upload_input` false when activations already in `composer.d_input`.
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub unsafe fn layer_forward_cuda_geodesic_gpu_post_impl(
    composer: &mut CudaLayerComposer,
    waller_bufs: &mut CudaWallerBuffers,
    weights: &CudaLayerWeights,
    input: Option<&[f32]>,
    seq_len: usize,
    hidden_dim: usize,
    head_dim: usize,
    num_heads: usize,
    ln_eps: f32,
    mlp_dim: usize,
    upload_input: bool,
    download_output: bool,
) -> Result<Option<Vec<f32>>, String> {
    let total = seq_len * hidden_dim;
    let phase_timing = cuda_phase_timing_enabled();
    let mut phases = GeodesicPhaseMs::default();
    let mut t_phase = std::time::Instant::now();

    if cuda_use_shared_trade_stream() {
        let s = cuda_trade_stream()?;
        composer.borrow_stream(s);
        waller_bufs.borrow_stream(s);
    }
    waller_bufs.ensure_capacity(total)?;
    if !cuda_use_shared_trade_stream() {
        waller_bufs.ensure_stream()?;
    }
    let stream = waller_bufs.stream;

    if upload_input {
        let host = input.ok_or("geodesic upload_input requires host slice")?;
        composer.upload_input(host, total)?;
        if phase_timing {
            (phases.h2d, t_phase) = geodesic_sync_phase_ms(composer.stream, t_phase);
        }
    }

    launch_ln1_rows(
        composer.d_input as *const f32,
        waller_bufs.d_normed,
        weights.d_ln1_gamma as *const f32,
        weights.d_ln1_beta as *const f32,
        seq_len as i32,
        hidden_dim as i32,
        ln_eps,
        stream,
    );
    if phase_timing {
        (phases.ln1, t_phase) = geodesic_sync_phase_ms(stream, t_phase);
    }

    launch_matmul_f32_geodesic(
        waller_bufs.d_normed,
        weights.d_w_qkv as *const f32,
        waller_bufs.d_qkv,
        seq_len as i32,
        hidden_dim as i32,
        (hidden_dim * 3) as i32,
        stream,
    );
    waller_bufs.inputs_on_device = true;
    if phase_timing {
        (phases.qkv, t_phase) = geodesic_sync_phase_ms(stream, t_phase);
    }

    let scale = 1.0 / (head_dim as f32).sqrt();
    waller_bufs.launch_waller_then_wo_on_device(
        total,
        seq_len,
        hidden_dim,
        head_dim,
        num_heads,
        scale,
        weights.d_wo as *const f32,
    )?;
    if phase_timing {
        (phases.waller_wo, t_phase) = geodesic_sync_phase_ms(stream, t_phase);
    }

    launch_residual_ln2_rows(
        composer.d_input as *const f32,
        waller_bufs.d_proj,
        waller_bufs.d_normed,
        weights.d_ln2_gamma as *const f32,
        weights.d_ln2_beta as *const f32,
        seq_len as i32,
        hidden_dim as i32,
        ln_eps,
        stream,
    );
    if phase_timing {
        (phases.res_ln2, t_phase) = geodesic_sync_phase_ms(stream, t_phase);
    }

    if cuda_use_batched_mlp(weights) {
        launch_mlp_block_geodesic(
            waller_bufs.d_normed as *const f32,
            weights.d_w_fc as *const f32,
            weights.d_b_fc as *const f32,
            weights.d_w_proj as *const f32,
            weights.d_b_proj as *const f32,
            weights.d_ln2_gamma as *const f32,
            weights.d_ln2_beta as *const f32,
            waller_bufs.d_out,
            waller_bufs.d_q,
            waller_bufs.d_proj,
            seq_len as i32,
            hidden_dim as i32,
            mlp_dim as i32,
            ln_eps,
            stream,
        );
    } else {
        let v_null_ptr = if weights.payload_dim > 0 && !weights.d_v_null.is_null() {
            weights.d_v_null as *const f32
        } else {
            std::ptr::null()
        };
        launch_fused_mlp_wnsm_device_weights(
            waller_bufs.d_normed as *const f32,
            weights.d_w_fc as *const f32,
            weights.d_b_fc as *const f32,
            weights.d_w_proj as *const f32,
            weights.d_b_proj as *const f32,
            weights.d_ln2_gamma as *const f32,
            weights.d_ln2_beta as *const f32,
            v_null_ptr,
            waller_bufs.d_proj,
            std::ptr::null_mut(),
            seq_len as i32,
            hidden_dim as i32,
            mlp_dim as i32,
            weights.payload_dim as i32,
            ln_eps,
            stream,
        );
    }
    if phase_timing {
        (phases.mlp, t_phase) = geodesic_sync_phase_ms(stream, t_phase);
    }

    if !download_output {
        if phase_timing {
            if let Ok(mut slot) = GEODESIC_LAST_PHASE
                .get_or_init(|| std::sync::Mutex::new(None))
                .lock()
            {
                *slot = Some(phases);
            }
        }
        return Ok(None);
    }

    cuda_stream_synchronize(stream);
    let mut host_out = vec![0.0f32; total];
    cuda_memcpy_d2h_async(
        host_out.as_mut_ptr() as *mut c_void,
        waller_bufs.d_proj as *const c_void,
        total * 4,
        stream,
    );
    cuda_stream_synchronize(stream);
    if phase_timing {
        phases.d2h = t_phase.elapsed().as_secs_f64() * 1000.0;
        if let Ok(mut slot) = GEODESIC_LAST_PHASE
            .get_or_init(|| std::sync::Mutex::new(None))
            .lock()
        {
            *slot = Some(phases);
        }
    }
    Ok(Some(host_out))
}

/// Geodesic TRADE: device LN1 → packed QKV → parallel Waller+wo → batched MLP → D2H.
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub unsafe fn layer_forward_cuda_geodesic_gpu_post(
    composer: &mut CudaLayerComposer,
    waller_bufs: &mut CudaWallerBuffers,
    weights: &CudaLayerWeights,
    input: &[f32],
    seq_len: usize,
    hidden_dim: usize,
    head_dim: usize,
    num_heads: usize,
    ln_eps: f32,
    mlp_dim: usize,
) -> Result<Vec<f32>, String> {
    layer_forward_cuda_geodesic_gpu_post_impl(
        composer,
        waller_bufs,
        weights,
        Some(input),
        seq_len,
        hidden_dim,
        head_dim,
        num_heads,
        ln_eps,
        mlp_dim,
        true,
        true,
    )?
    .ok_or_else(|| "geodesic layer expected host output".to_string())
}

/// Multi-layer TRADE forward: one H2D, GPU layer loop (D2D activations), one D2H.
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub unsafe fn decoder_forward_cuda_quant_stack(
    layers: &mut [crate::wnsm_transformer::WNSM_GAE_Layer],
    input: &[f32],
    seq_len: usize,
) -> Result<Vec<f32>, String> {
    if layers.is_empty() {
        return Ok(Vec::new());
    }
    let hidden_dim = layers[0].config.hidden_dim;
    let head_dim = layers[0].config.head_dim;
    let num_heads = layers[0].config.num_heads;
    let mlp_dim = layers[0].config.mlp_dim;
    let ln_eps = layers[0].config.ln_eps;
    let total = seq_len * hidden_dim;
    let n = layers.len();

    let trade_stream = cuda_trade_stream()?;

    for layer in layers.iter_mut() {
        layer.ensure_cuda_weights()?;
        if layer.cuda_waller_buffers.is_none() {
            layer.cuda_waller_buffers = Some(CudaWallerBuffers::new());
        }
        if layer.cuda_layer_composer.is_none() {
            layer.cuda_layer_composer = Some(CudaLayerComposer::new());
        }
        let bufs = layer.cuda_waller_buffers.as_mut().expect("waller bufs");
        let composer = layer.cuda_layer_composer.as_mut().expect("composer");
        bufs.borrow_stream(trade_stream);
        composer.borrow_stream(trade_stream);
        composer.ensure_device_input(total)?;
        bufs.ensure_capacity(total)?;
    }

    for i in 0..n {
        let (head, tail) = layers.split_at_mut(i + 1);
        let layer = &mut head[i];
        let weights = layer.cuda_weights.as_ref().expect("cuda weights");
        let composer = layer.cuda_layer_composer.as_mut().expect("composer");
        let bufs = layer.cuda_waller_buffers.as_mut().expect("waller bufs");
        let upload = i == 0;
        let download = i + 1 == n;
        let host_in = if upload { Some(input) } else { None };
        layer_forward_cuda_geodesic_gpu_post_impl(
            composer,
            bufs,
            weights,
            host_in,
            seq_len,
            hidden_dim,
            head_dim,
            num_heads,
            ln_eps,
            mlp_dim,
            upload,
            download,
        )?;

        if let Some(next_layer) = tail.first_mut() {
            cuda_stream_synchronize(bufs.stream);
            let d_proj = bufs.d_proj;
            let next_composer = next_layer
                .cuda_layer_composer
                .as_mut()
                .expect("next composer");
            cuda_memcpy_d2d_async(
                next_composer.d_input as *mut c_void,
                d_proj as *const c_void,
                total * 4,
                bufs.stream,
            );
            cuda_stream_synchronize(bufs.stream);
        }
    }

    let last = layers.last().expect("non-empty");
    let bufs = last.cuda_waller_buffers.as_ref().expect("bufs");
    let mut host_out = vec![0.0f32; total];
    cuda_memcpy_d2h_async(
        host_out.as_mut_ptr() as *mut c_void,
        bufs.d_proj as *const c_void,
        total * 4,
        bufs.stream,
    );
    cuda_stream_synchronize(trade_stream);
    Ok(host_out)
}

/// CPU QKV → GPU attn+wo → GPU residual+LN2+MLP → one D2H (fallback when geodesic off).
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub unsafe fn layer_forward_cuda_trade_gpu_post(
    composer: &mut CudaLayerComposer,
    waller_bufs: &mut CudaWallerBuffers,
    weights: &CudaLayerWeights,
    input: &[f32],
    q: &[f32],
    k: &[f32],
    v: &[f32],
    seq_len: usize,
    hidden_dim: usize,
    head_dim: usize,
    num_heads: usize,
    ln_eps: f32,
    mlp_dim: usize,
) -> Result<Vec<f32>, String> {
    let total = seq_len * hidden_dim;
    let qkv_total = q.len();
    composer.upload_input(input, total)?;
    waller_bufs.ensure_capacity(qkv_total)?;
    waller_bufs.upload_inputs(q, k, v, qkv_total)?;
    let scale = 1.0 / (head_dim as f32).sqrt();
    waller_bufs.launch_waller_then_wo_on_device(
        qkv_total,
        seq_len,
        hidden_dim,
        head_dim,
        num_heads,
        scale,
        weights.d_wo as *const f32,
    )?;

    let stream = waller_bufs.stream;
    launch_residual_ln2_rows(
        composer.d_input as *const f32,
        waller_bufs.d_proj,
        waller_bufs.d_normed,
        weights.d_ln2_gamma as *const f32,
        weights.d_ln2_beta as *const f32,
        seq_len as i32,
        hidden_dim as i32,
        ln_eps,
        stream,
    );

    let v_null_ptr = if weights.payload_dim > 0 && !weights.d_v_null.is_null() {
        weights.d_v_null as *const f32
    } else {
        std::ptr::null()
    };
    launch_fused_mlp_wnsm_device_weights(
        waller_bufs.d_normed as *const f32,
        weights.d_w_fc as *const f32,
        weights.d_b_fc as *const f32,
        weights.d_w_proj as *const f32,
        weights.d_b_proj as *const f32,
        weights.d_ln2_gamma as *const f32,
        weights.d_ln2_beta as *const f32,
        v_null_ptr,
        waller_bufs.d_proj,
        std::ptr::null_mut(),
        seq_len as i32,
        hidden_dim as i32,
        mlp_dim as i32,
        weights.payload_dim as i32,
        ln_eps,
        stream,
    );

    cuda_stream_synchronize(stream);
    cuda_device_synchronize();

    let mut host_out = vec![0.0f32; total];
    cuda_memcpy_d2h_async(
        host_out.as_mut_ptr() as *mut c_void,
        waller_bufs.d_proj as *const c_void,
        total * 4,
        stream,
    );
    cuda_stream_synchronize(stream);
    Ok(host_out)
}

pub fn cuda_use_cpu_wo() -> bool {
    std::env::var("LUXI_CUDA_CPU_WO")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Experimental: device LN+QKV + Waller (may drift vs CPU Welford; use `LUXI_CUDA_GPU_QKV=1`).
#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub unsafe fn layer_forward_cuda_attn_gpu_qkv_on_device(
    composer: &mut CudaLayerComposer,
    waller_bufs: &mut CudaWallerBuffers,
    weights: &CudaLayerWeights,
    input: &[f32],
    seq_len: usize,
    hidden_dim: usize,
    head_dim: usize,
    num_heads: usize,
    ln_eps: f32,
) -> Result<Vec<f32>, String> {
    let total = seq_len * hidden_dim;
    composer.upload_input(input, total)?;
    waller_bufs.ensure_capacity(total)?;

    let (d_q, d_k, d_v) = waller_bufs.qkv_device_ptrs();
    launch_ln_qkv_proj(
        composer.d_input as *const f32,
        weights.d_wq as *const f32,
        weights.d_wk as *const f32,
        weights.d_wv as *const f32,
        weights.d_ln1_gamma as *const f32,
        weights.d_ln1_beta as *const f32,
        d_q,
        d_k,
        d_v,
        seq_len as i32,
        hidden_dim as i32,
        ln_eps,
        composer.stream,
    );
    cuda_stream_synchronize(composer.stream);
    waller_bufs.inputs_on_device = true;

    let scale = 1.0 / (head_dim as f32).sqrt();
    let (_, _, cuda_attn) = waller_bufs.launch_only(
        total,
        seq_len,
        head_dim,
        num_heads,
        scale,
        true,
    )?;
    Ok(cuda_attn)
}

pub fn cuda_use_mega_layer() -> bool {
    std::env::var("LUXI_CUDA_MEGA")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn cuda_use_gpu_composer() -> bool {
    std::env::var("LUXI_CUDA_GPU_LAYER")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Experimental GPU QKV (naive device GEMM). Default off — CPU batched QKV is faster at seq≥128.
pub fn cuda_use_gpu_qkv() -> bool {
    cuda_use_gpu_qkv_force()
}

pub fn layer_fits_mega(cfg: &crate::config::Config) -> bool {
    cfg.hidden_dim <= 128 && cfg.mlp_dim <= 512
}

// ============================================================================
// KV cache (incremental quant decode)
// ============================================================================

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
pub struct CudaWallerKvCache {
    d_k: *mut f32,
    d_v: *mut f32,
    d_q_row: *mut f32,
    d_out_row: *mut f32,
    capacity_floats: usize,
    pub cached_len: usize,
    stream: *mut c_void,
}

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
impl CudaWallerKvCache {
    pub fn new() -> Self {
        Self {
            d_k: std::ptr::null_mut(),
            d_v: std::ptr::null_mut(),
            d_q_row: std::ptr::null_mut(),
            d_out_row: std::ptr::null_mut(),
            capacity_floats: 0,
            cached_len: 0,
            stream: std::ptr::null_mut(),
        }
    }

    pub unsafe fn ensure(&mut self, max_seq: usize, hidden: usize) -> Result<(), String> {
        let need = max_seq * hidden;
        if need <= self.capacity_floats && !self.d_k.is_null() {
            return Ok(());
        }
        self.reset();
        if self.stream.is_null() {
            let mut s: *mut c_void = std::ptr::null_mut();
            cudaStreamCreate(&mut s as *mut *mut c_void);
            self.stream = s;
        }
        let cap = need.max(1);
        let bytes = cap * 4;
        cudaMalloc(&mut self.d_k as *mut _ as *mut *mut c_void, bytes);
        cudaMalloc(&mut self.d_v as *mut _ as *mut *mut c_void, bytes);
        cudaMalloc(&mut self.d_q_row as *mut _ as *mut *mut c_void, hidden * 4);
        cudaMalloc(&mut self.d_out_row as *mut _ as *mut *mut c_void, hidden * 4);
        self.capacity_floats = cap;
        Ok(())
    }

    pub unsafe fn reset(&mut self) {
        if !self.d_k.is_null() {
            cuda_free(self.d_k as *mut c_void);
            self.d_k = std::ptr::null_mut();
        }
        if !self.d_v.is_null() {
            cuda_free(self.d_v as *mut c_void);
            self.d_v = std::ptr::null_mut();
        }
        if !self.d_q_row.is_null() {
            cuda_free(self.d_q_row as *mut c_void);
            self.d_q_row = std::ptr::null_mut();
        }
        if !self.d_out_row.is_null() {
            cuda_free(self.d_out_row as *mut c_void);
            self.d_out_row = std::ptr::null_mut();
        }
        self.cached_len = 0;
        self.capacity_floats = 0;
    }

    /// Append K/V row and decode attention for Q row at index `row` (0-based).
    pub unsafe fn append_and_decode(
        &mut self,
        q_row: &[f32],
        k_row: &[f32],
        v_row: &[f32],
        row: usize,
        hidden: usize,
        head_dim: usize,
        num_heads: usize,
        scale: f32,
    ) -> Result<Vec<f32>, String> {
        self.ensure(row + 1, hidden)?;
        let off = row * hidden;
        cuda_memcpy_h2d_async(
            (self.d_k as *mut f32).add(off) as *mut c_void,
            k_row.as_ptr() as *const c_void,
            hidden * 4,
            self.stream,
        );
        cuda_memcpy_h2d_async(
            (self.d_v as *mut f32).add(off) as *mut c_void,
            v_row.as_ptr() as *const c_void,
            hidden * 4,
            self.stream,
        );
        cuda_memcpy_h2d_async(
            self.d_q_row as *mut c_void,
            q_row.as_ptr() as *const c_void,
            hidden * 4,
            self.stream,
        );
        cuda_stream_synchronize(self.stream);
        self.cached_len = row + 1;

        launch_waller_kv_decode(
            self.d_q_row,
            self.d_k,
            self.d_v,
            self.d_out_row,
            row as i32,
            head_dim as i32,
            num_heads as i32,
            scale,
            self.stream,
        );
        cuda_stream_synchronize(self.stream);
        let mut out = vec![0.0f32; hidden];
        cuda_memcpy_d2h(
            out.as_mut_ptr() as *mut c_void,
            self.d_out_row as *const c_void,
            hidden * 4,
        );
        Ok(out)
    }
}

#[cfg(feature = "cuda")]
#[cfg(not(cuda_compilation_failed))]
impl Drop for CudaWallerKvCache {
    fn drop(&mut self) {
        unsafe {
            self.reset();
            if !self.stream.is_null() {
                cudaStreamDestroy(self.stream);
            }
        }
    }
}

// ============================================================================
// Lane B: CUDA INT8 GEMM (separate determinism contract from f32 gold)
// ============================================================================

#[cfg(all(feature = "cuda", feature = "cuda-quant"))]
#[cfg(not(cuda_compilation_failed))]
pub unsafe fn matmul_f32_i8_cuda(
    a: &[f32],
    q_data: &[i8],
    q_scale: f32,
    m: usize,
    k: usize,
    n: usize,
) -> Result<Vec<f32>, String> {
    let mut d_a: *mut f32 = std::ptr::null_mut();
    let mut d_b: *mut i8 = std::ptr::null_mut();
    let mut d_c: *mut f32 = std::ptr::null_mut();
    cudaMalloc(&mut d_a as *mut _ as *mut *mut c_void, m * k * 4);
    cudaMalloc(&mut d_b as *mut _ as *mut *mut c_void, k * n);
    cudaMalloc(&mut d_c as *mut _ as *mut *mut c_void, m * n * 4);
    cuda_memcpy_h2d(d_a as *mut c_void, a.as_ptr() as *const c_void, m * k * 4);
    cuda_memcpy_h2d(d_b as *mut c_void, q_data.as_ptr() as *const c_void, k * n);
    launch_matmul_f32_i8(d_a, d_b, d_c, m as i32, k as i32, n as i32, q_scale, std::ptr::null_mut());
    cuda_device_synchronize();
    let mut c = vec![0.0f32; m * n];
    cuda_memcpy_d2h(c.as_mut_ptr() as *mut c_void, d_c as *const c_void, m * n * 4);
    cuda_free(d_a as *mut c_void);
    cuda_free(d_b as *mut c_void);
    cuda_free(d_c as *mut c_void);
    Ok(c)
}

#[cfg(all(feature = "cuda", feature = "cuda-quant"))]
#[cfg(not(cuda_compilation_failed))]
pub fn matmul_f32_i8_cuda_or_cpu(
    a: &[f32],
    q: &crate::linalg::QuantizedMatrix,
    m: usize,
    k: usize,
    n: usize,
) -> Vec<f32> {
    match unsafe { matmul_f32_i8_cuda(a, &q.data, q.scale, m, k, n) } {
        Ok(v) => v,
        Err(_) => crate::linalg::matmul_f32_i8(a, q, m, k, n),
    }
}
