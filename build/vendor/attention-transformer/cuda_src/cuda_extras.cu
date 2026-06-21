// cuda_extras.cu — KV-cache Waller, Lane-B INT8 GEMM, LN+QKV projection
#include <cuda_runtime.h>
#include <cstdlib>
#include <math.h>
#include <stdint.h>

#ifndef LUXI_CUDA_MIN
#define LUXI_CUDA_MIN(a, b) ((a) < (b) ? (a) : (b))
#endif

// Welford LN (matches Rust WelfordState + layernorm)
__device__ void welford_ln_f32(
    const float* __restrict__ in,
    float* __restrict__ out,
    int n,
    const float* __restrict__ gamma,
    const float* __restrict__ beta,
    float eps
) {
    float mean = 0.0f;
    float m2 = 0.0f;
    int count = 0;
    for (int i = 0; i < n; i++) {
        count++;
        float delta = in[i] - mean;
        mean += delta / (float)count;
        float delta2 = in[i] - mean;
        m2 += delta * delta2;
    }
    float var = (count < 2) ? 0.0f : (m2 / (float)count);
    float inv_std = rsqrtf(var + eps);
    for (int i = 0; i < n; i++) {
        out[i] = (in[i] - mean) * inv_std * gamma[i] + beta[i];
    }
}

__device__ float proj_dot(
    const float* __restrict__ x,
    const float* __restrict__ w,
    int in_dim,
    int out_dim,
    int out_idx
) {
    float s = 0.0f;
    for (int i = 0; i < in_dim; i++) {
        s += x[i] * w[i * out_dim + out_idx];
    }
    return s;
}

// Batch LN1 + Q/K/V projection for all sequence positions (deterministic row order).
__global__ void ln_qkv_proj_kernel(
    const float* __restrict__ input,
    const float* __restrict__ wq,
    const float* __restrict__ wk,
    const float* __restrict__ wv,
    const float* __restrict__ ln1_gamma,
    const float* __restrict__ ln1_beta,
    float* __restrict__ Q,
    float* __restrict__ K,
    float* __restrict__ V,
    int seq_len,
    int hidden_dim,
    float ln_eps
) {
    const int row = blockIdx.x;
    if (row >= seq_len) return;

    float row_in[128];
    float normed[128];
    for (int d = threadIdx.x; d < hidden_dim; d += blockDim.x) {
        row_in[d] = input[row * hidden_dim + d];
    }
    __syncthreads();
    if (threadIdx.x == 0) {
        welford_ln_f32(row_in, normed, hidden_dim, ln1_gamma, ln1_beta, ln_eps);
    }
    __syncthreads();
    if (threadIdx.x == 0) {
        for (int o = 0; o < hidden_dim; o++) {
            Q[row * hidden_dim + o] = proj_dot(normed, wq, hidden_dim, hidden_dim, o);
            K[row * hidden_dim + o] = proj_dot(normed, wk, hidden_dim, hidden_dim, o);
            V[row * hidden_dim + o] = proj_dot(normed, wv, hidden_dim, hidden_dim, o);
        }
    }
}

// LN1 per row (any hidden_dim ≤ 8192); output [seq, hidden].
__global__ void ln1_rows_kernel(
    const float* __restrict__ input,
    float* __restrict__ normed_out,
    const float* __restrict__ gamma,
    const float* __restrict__ beta,
    int seq_len,
    int hidden_dim,
    float ln_eps
) {
    const int row = blockIdx.x;
    if (row >= seq_len) return;

    extern __shared__ float smem[];
    float* row_in = smem;
    float* normed = smem + hidden_dim;

    for (int d = threadIdx.x; d < hidden_dim; d += blockDim.x) {
        row_in[d] = input[row * hidden_dim + d];
    }
    __syncthreads();
    if (threadIdx.x == 0) {
        welford_ln_f32(row_in, normed, hidden_dim, gamma, beta, ln_eps);
    }
    __syncthreads();
    for (int d = threadIdx.x; d < hidden_dim; d += blockDim.x) {
        normed_out[row * hidden_dim + d] = normed[d];
    }
}

// residual + Welford LN2 per row (matches CPU cuda_post_one_row pre-MLP).
__global__ void residual_ln2_rows_kernel(
    const float* __restrict__ input,
    const float* __restrict__ attn_proj,
    float* __restrict__ out,
    const float* __restrict__ gamma,
    const float* __restrict__ beta,
    int seq_len,
    int hidden_dim,
    float ln_eps
) {
    const int row = blockIdx.x;
    if (row >= seq_len) return;

    extern __shared__ float smem[];
    float* combined = smem;
    float* normed = smem + hidden_dim;

    for (int d = threadIdx.x; d < hidden_dim; d += blockDim.x) {
        combined[d] = attn_proj[row * hidden_dim + d] + input[row * hidden_dim + d];
    }
    __syncthreads();
    if (threadIdx.x == 0) {
        welford_ln_f32(combined, normed, hidden_dim, gamma, beta, ln_eps);
    }
    __syncthreads();
    for (int d = threadIdx.x; d < hidden_dim; d += blockDim.x) {
        out[row * hidden_dim + d] = normed[d];
    }
}

extern "C" void launch_residual_ln2_rows(
    const float* input,
    const float* attn_proj,
    float* out,
    const float* ln2_gamma,
    const float* ln2_beta,
    int seq_len,
    int hidden_dim,
    float ln_eps,
    cudaStream_t stream
) {
    const size_t smem = (size_t)hidden_dim * 2u * sizeof(float);
    residual_ln2_rows_kernel<<<seq_len, 256, smem, stream>>>(
        input, attn_proj, out, ln2_gamma, ln2_beta, seq_len, hidden_dim, ln_eps);
}

extern "C" void launch_ln1_rows(
    const float* input,
    float* normed_out,
    const float* ln1_gamma,
    const float* ln1_beta,
    int seq_len,
    int hidden_dim,
    float ln_eps,
    cudaStream_t stream
) {
    const size_t smem = (size_t)hidden_dim * 2u * sizeof(float);
    ln1_rows_kernel<<<seq_len, 256, smem, stream>>>(
        input, normed_out, ln1_gamma, ln1_beta, seq_len, hidden_dim, ln_eps);
}

extern "C" void launch_ln_qkv_proj(
    const float* input,
    const float* wq,
    const float* wk,
    const float* wv,
    const float* ln1_gamma,
    const float* ln1_beta,
    float* Q,
    float* K,
    float* V,
    int seq_len,
    int hidden_dim,
    float ln_eps,
    cudaStream_t stream
) {
    ln_qkv_proj_kernel<<<seq_len, 256, 0, stream>>>(
        input, wq, wk, wv, ln1_gamma, ln1_beta, Q, K, V, seq_len, hidden_dim, ln_eps);
}

// Incremental decode: append K/V row, run causal Waller for one query row against cache.
__global__ void waller_kv_decode_kernel(
    const float* __restrict__ Q_row,
    const float* __restrict__ K_cache,
    const float* __restrict__ V_cache,
    float* __restrict__ Output_row,
    int row,
    int head_dim,
    int num_heads,
    float scale
) {
    const int idx = blockIdx.x * blockDim.x + threadIdx.x;
    const int total = num_heads;
    if (idx >= total) return;

    const int head = idx;
    const int hidden = num_heads * head_dim;
    const int ho = head * head_dim;

    float q[128];
    float acc[128];
    for (int d = 0; d < head_dim; d++) {
        q[d] = Q_row[ho + d];
        acc[d] = 0.0f;
    }

    float max_val = -INFINITY;
    float sum_exp = 0.0f;

    for (int col = 0; col <= row; col++) {
        const int kv = col * hidden + ho;
        float dot = 0.0f;
        for (int d = 0; d < head_dim; d++) {
            dot += q[d] * K_cache[kv + d];
        }
        dot *= scale;

        const float old_max = max_val;
        max_val = fmaxf(max_val, dot);
        const float rescale = expf(old_max - max_val);
        const float e = expf(dot - max_val);
        sum_exp = sum_exp * rescale + e;
        const float weight = e;

        for (int d = 0; d < head_dim; d++) {
            acc[d] = acc[d] * rescale + weight * V_cache[kv + d];
        }
    }

    const float inv_sum = 1.0f / sum_exp;
    for (int d = 0; d < head_dim; d++) {
        Output_row[ho + d] = acc[d] * inv_sum;
    }
}

extern "C" void launch_waller_kv_decode(
    const float* Q_row,
    const float* K_cache,
    const float* V_cache,
    float* Output_row,
    int row,
    int head_dim,
    int num_heads,
    float scale,
    cudaStream_t stream
) {
    const int threads = 32;
    const int blocks = (num_heads + threads - 1) / threads;
    waller_kv_decode_kernel<<<blocks, threads, 0, stream>>>(
        Q_row, K_cache, V_cache, Output_row, row, head_dim, num_heads, scale);
}

// Lane B: deterministic int8 weight matmul (activation f32, weight symmetric int8).
__global__ void matmul_f32_i8_kernel(
    const float* __restrict__ A,
    const int8_t* __restrict__ B,
    float* __restrict__ C,
    int m,
    int k,
    int n,
    float w_scale
) {
    const int row = blockIdx.y * blockDim.y + threadIdx.y;
    const int col = blockIdx.x * blockDim.x + threadIdx.x;
    if (row >= m || col >= n) return;

    int32_t sum = 0;
    for (int p = 0; p < k; p++) {
        const int32_t aq = (int32_t)lrintf(A[row * k + p] * w_scale);
        const int32_t bq = (int32_t)B[p * n + col];
        sum += aq * bq;
    }
    const float inv = 1.0f / (w_scale * w_scale);
    C[row * n + col] = (float)sum * inv;
}

extern "C" void launch_matmul_f32_i8(
    const float* A,
    const int8_t* B,
    float* C,
    int m,
    int k,
    int n,
    float w_scale,
    cudaStream_t stream
) {
    dim3 block(16, 16);
    dim3 grid((n + block.x - 1) / block.x, (m + block.y - 1) / block.y);
    matmul_f32_i8_kernel<<<grid, block, 0, stream>>>(A, B, C, m, k, n, w_scale);
}

// Deterministic f32 GEMM: C = A @ B, row-major [m×k] × [k×n].
// Each output element uses strict p=0..k-1 accumulation (matches CPU linear_projection / matmul).
__global__ void matmul_f32_kernel(
    const float* __restrict__ A,
    const float* __restrict__ B,
    float* __restrict__ C,
    int m,
    int k,
    int n
) {
    const int row = blockIdx.y * blockDim.y + threadIdx.y;
    const int col = blockIdx.x * blockDim.x + threadIdx.x;
    if (row >= m || col >= n) return;

    float sum = 0.0f;
    for (int p = 0; p < k; p++) {
        sum += A[row * k + p] * B[p * n + col];
    }
    C[row * n + col] = sum;
}

// P2: cache-blocked GEMM — same math as matmul_f32_kernel (fixed kt, fixed kk order per thread).
#ifndef LUXI_GEMM_TILE
#define LUXI_GEMM_TILE 32
#endif

template <int TILE>
__global__ void matmul_f32_tiled_kernel(
    const float* __restrict__ A,
    const float* __restrict__ B,
    float* __restrict__ C,
    int m,
    int k,
    int n
) {
    const int row = blockIdx.y * TILE + threadIdx.y;
    const int col = blockIdx.x * TILE + threadIdx.x;

    __shared__ float As[TILE][TILE];
    __shared__ float Bs[TILE][TILE];

    float sum = 0.0f;
    for (int kt = 0; kt < k; kt += TILE) {
        const int a_col = kt + threadIdx.x;
        const int b_row = kt + threadIdx.y;

        As[threadIdx.y][threadIdx.x] =
            (row < m && a_col < k) ? A[row * k + a_col] : 0.0f;
        Bs[threadIdx.y][threadIdx.x] =
            (b_row < k && col < n) ? B[b_row * n + col] : 0.0f;
        __syncthreads();

        for (int kk = 0; kk < TILE; kk++) {
            sum += As[threadIdx.y][kk] * Bs[kk][threadIdx.x];
        }
        __syncthreads();
    }

    if (row < m && col < n) {
        C[row * n + col] = sum;
    }
}

extern "C" void launch_matmul_f32_tiled(
    const float* A,
    const float* B,
    float* C,
    int m,
    int k,
    int n,
    cudaStream_t stream
) {
    const int tile = LUXI_GEMM_TILE;
    dim3 block(tile, tile);
    dim3 grid((n + tile - 1) / tile, (m + tile - 1) / tile);
    if (tile == 32) {
        matmul_f32_tiled_kernel<32><<<grid, block, 0, stream>>>(A, B, C, m, k, n);
    } else if (tile == 16) {
        matmul_f32_tiled_kernel<16><<<grid, block, 0, stream>>>(A, B, C, m, k, n);
    } else {
        matmul_f32_tiled_kernel<32><<<grid, block, 0, stream>>>(A, B, C, m, k, n);
    }
}

extern "C" void launch_matmul_f32(
    const float* A,
    const float* B,
    float* C,
    int m,
    int k,
    int n,
    cudaStream_t stream
) {
    dim3 block(16, 16);
    dim3 grid((n + block.x - 1) / block.x, (m + block.y - 1) / block.y);
    matmul_f32_kernel<<<grid, block, 0, stream>>>(A, B, C, m, k, n);
}

// Dispatcher: tiled for large geodesic QKV GEMM, naive for small shapes.
extern "C" void launch_matmul_f32_geodesic(
    const float* A,
    const float* B,
    float* C,
    int m,
    int k,
    int n,
    cudaStream_t stream
) {
    const char* force_naive = getenv("LUXI_CUDA_NAIVE_GEMM");
    if (force_naive != nullptr && force_naive[0] == '1') {
        launch_matmul_f32(A, B, C, m, k, n, stream);
        return;
    }
    if (m >= 64 && k >= 64 && n >= 64) {
        launch_matmul_f32_tiled(A, B, C, m, k, n, stream);
    } else {
        launch_matmul_f32(A, B, C, m, k, n, stream);
    }
}

// Deterministic GELU (matches src/activations.rs).
__device__ __forceinline__ float gelu_det_f32(float x) {
    return 0.5f * x * (1.0f + tanhf(0.7978845608025684f * (x + 0.044715f * x * x * x)));
}

// Bias + GELU on row-major [seq_len × width] (after fc GEMM).
__global__ void bias_gelu_rows_kernel(
    float* __restrict__ data,
    const float* __restrict__ bias,
    int seq_len,
    int width
) {
    const int row = blockIdx.x;
    if (row >= seq_len) return;
    for (int col = threadIdx.x; col < width; col += blockDim.x) {
        const int idx = row * width + col;
        data[idx] = gelu_det_f32(data[idx] + bias[col]);
    }
}

extern "C" void launch_bias_gelu_rows(
    float* data,
    const float* bias,
    int seq_len,
    int width,
    cudaStream_t stream
) {
    bias_gelu_rows_kernel<<<seq_len, 256, 0, stream>>>(data, bias, seq_len, width);
}

__global__ void bias_add_rows_kernel(
    float* __restrict__ data,
    const float* __restrict__ bias,
    int seq_len,
    int width
) {
    const int row = blockIdx.x;
    if (row >= seq_len) return;
    for (int col = threadIdx.x; col < width; col += blockDim.x) {
        const int idx = row * width + col;
        data[idx] += bias[col];
    }
}

extern "C" void launch_bias_add_rows(
    float* data,
    const float* bias,
    int seq_len,
    int width,
    cudaStream_t stream
) {
    bias_add_rows_kernel<<<seq_len, 256, 0, stream>>>(data, bias, seq_len, width);
}

// Quant TRADE MLP: tiled GEMM fc → GELU → tiled GEMM proj → residual+Welford LN2.
// `normed` = post-attn LN2 input [seq×hidden]; scratch_mlp [seq×mlp_dim]; scratch_h [seq×hidden].
extern "C" void launch_mlp_block_geodesic(
    const float* normed,
    const float* w_fc,
    const float* b_fc,
    const float* w_proj,
    const float* b_proj,
    const float* ln2_gamma,
    const float* ln2_beta,
    float* scratch_mlp,
    float* scratch_h,
    float* output,
    int seq_len,
    int hidden_dim,
    int mlp_dim,
    float ln_eps,
    cudaStream_t stream
) {
    launch_matmul_f32_geodesic(
        normed, w_fc, scratch_mlp,
        seq_len, hidden_dim, mlp_dim, stream);
    launch_bias_gelu_rows(scratch_mlp, b_fc, seq_len, mlp_dim, stream);
    launch_matmul_f32_geodesic(
        scratch_mlp, w_proj, scratch_h,
        seq_len, mlp_dim, hidden_dim, stream);
    launch_bias_add_rows(scratch_h, b_proj, seq_len, hidden_dim, stream);
    launch_residual_ln2_rows(
        normed, scratch_h, output,
        ln2_gamma, ln2_beta,
        seq_len, hidden_dim, ln_eps, stream);
}