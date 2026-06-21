// waller_operator.cu
// CUDA implementation of the Waller Operator (fused online-softmax causal attention)
//
// This is the first concrete step toward the "mega kernel" vision in this repo.
// Goal: Provide a high-performance, deterministic CUDA path that produces
//       bit-equivalent (or extremely close) results to the pure-Rust reference
//       in src/waller_operator.rs so that cryptographic receipts remain valid.
//
// Current scope (Phase 1 - make it work):
//   - Single-head and multi-head causal attention using online softmax
//   - Matches the algorithm in the Rust waller_operator()
//   - Designed to be called from Rust via FFI
//
// Later phases will fuse more (QKV proj, output proj, LayerNorm, MLP, WNSM payload handling)
// inside one or a few kernel launches for maximum energy efficiency.
//
// Compile with nvcc -std=c++17 -O3 -arch=sm_70 (or higher)

#include <cuda_runtime.h>
#include <math.h>

__device__ void welford_ln_f32_mlp(
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

// Specialized fast path: one thread per (head, row), Q in registers, fully unrolled loops,
// zero __syncthreads, __ldg for K/V. Serial d-order preserved for decoder bit-exactness.
// (C++ template — nvcc cannot use #pragma inside cpp macros.)
template <int HD>
__global__ void __launch_bounds__(256, 4) waller_multihead_hd_t_kernel(
    const float* __restrict__ Q,
    const float* __restrict__ K,
    const float* __restrict__ V,
    float* __restrict__ Output,
    const int seq_len,
    const int num_heads,
    const float scale
) {
    const int idx = blockIdx.x * blockDim.x + threadIdx.x;
    const int total = seq_len * num_heads;
    if (idx >= total) return;

    const int head = idx / seq_len;
    const int row = idx % seq_len;
    const int hidden = num_heads * HD;
    const int ho = head * HD;
    const int qbase = row * hidden + ho;

    float q[HD];
    float acc[HD];
    #pragma unroll
    for (int d = 0; d < HD; d++) {
        q[d] = __ldg(&Q[qbase + d]);
        acc[d] = 0.0f;
    }

    float max_val = -INFINITY;
    float sum_exp = 0.0f;

    for (int col = 0; col <= row; col++) {
        const int kv = col * hidden + ho;
        float dot = 0.0f;
        #pragma unroll
        for (int d = 0; d < HD; d++) {
            dot += q[d] * __ldg(&K[kv + d]);
        }
        dot *= scale;

        const float old_max = max_val;
        max_val = fmaxf(max_val, dot);
        const float rescale = expf(old_max - max_val);
        const float e = expf(dot - max_val);
        sum_exp = sum_exp * rescale + e;
        const float weight = e;

        #pragma unroll
        for (int d = 0; d < HD; d++) {
            acc[d] = acc[d] * rescale + weight * __ldg(&V[kv + d]);
        }
    }

    const float inv_sum = 1.0f / sum_exp;
    #pragma unroll
    for (int d = 0; d < HD; d++) {
        Output[qbase + d] = acc[d] * inv_sum;
    }
}

// Lane A SMEM-tiled path: tile K/V through shared memory; column order unchanged.
template <int HD, int TILE>
__global__ void __launch_bounds__(HD, 4) waller_multihead_hd_smem_kernel(
    const float* __restrict__ Q,
    const float* __restrict__ K,
    const float* __restrict__ V,
    float* __restrict__ Output,
    const int seq_len,
    const int num_heads,
    const float scale
) {
    const int head = blockIdx.y;
    const int row = blockIdx.x;
    const int d = threadIdx.x;
    if (row >= seq_len || head >= num_heads || d >= HD) return;

    const int hidden = num_heads * HD;
    const int ho = head * HD;
    const int qbase = row * hidden + ho;

    __shared__ float q_sh[HD];
    __shared__ float k_tile[TILE][HD];
    __shared__ float v_tile[TILE][HD];
    __shared__ float s_rescale;
    __shared__ float s_weight;

    q_sh[d] = __ldg(&Q[qbase + d]);
    float acc_reg = 0.0f;
    __syncthreads();

    float max_val = -INFINITY;
    float sum_exp = 0.0f;

    for (int tile_start = 0; tile_start <= row; tile_start += TILE) {
        const int tile_len = (row + 1 - tile_start) < TILE ? (row + 1 - tile_start) : TILE;
        for (int t = d; t < tile_len * HD; t += blockDim.x) {
            const int lc = t / HD;
            const int dd = t % HD;
            const int col = tile_start + lc;
            const int kv = col * hidden + ho;
            k_tile[lc][dd] = __ldg(&K[kv + dd]);
            v_tile[lc][dd] = __ldg(&V[kv + dd]);
        }
        __syncthreads();

        for (int local = 0; local < tile_len; ++local) {
            if (d == 0) {
                float dot = 0.0f;
                #pragma unroll
                for (int dd = 0; dd < HD; ++dd) {
                    dot += q_sh[dd] * k_tile[local][dd];
                }
                dot *= scale;
                const float old_max = max_val;
                max_val = fmaxf(max_val, dot);
                const float rescale = expf(old_max - max_val);
                const float e = expf(dot - max_val);
                sum_exp = sum_exp * rescale + e;
                s_rescale = rescale;
                s_weight = e;
            }
            __syncthreads();
            acc_reg = acc_reg * s_rescale + s_weight * v_tile[local][d];
            __syncthreads();
        }
    }

    Output[qbase + d] = acc_reg / sum_exp;
}

// Generic fast path: cache Q in registers (variable head_dim), no barriers.
__global__ void __launch_bounds__(256, 4) waller_operator_multihead_cached_q_kernel(
    const float* __restrict__ Q,
    const float* __restrict__ K,
    const float* __restrict__ V,
    float* __restrict__ Output,
    const int seq_len,
    const int head_dim,
    const int num_heads,
    const float scale
) {
    const int idx = blockIdx.x * blockDim.x + threadIdx.x;
    const int total = seq_len * num_heads;
    if (idx >= total) return;

    const int head = idx / seq_len;
    const int row = idx % seq_len;
    const int hidden = num_heads * head_dim;
    const int ho = head * head_dim;
    const int qbase = row * hidden + ho;

    float q[128];
    float acc[128];
    for (int d = 0; d < head_dim; d++) {
        q[d] = __ldg(&Q[qbase + d]);
        acc[d] = 0.0f;
    }

    float max_val = -INFINITY;
    float sum_exp = 0.0f;

    for (int col = 0; col <= row; col++) {
        const int kv = col * hidden + ho;
        float dot = 0.0f;
        for (int d = 0; d < head_dim; d++) {
            dot += q[d] * __ldg(&K[kv + d]);
        }
        dot *= scale;

        const float old_max = max_val;
        max_val = fmaxf(max_val, dot);
        const float rescale = expf(old_max - max_val);
        const float e = expf(dot - max_val);
        sum_exp = sum_exp * rescale + e;
        const float weight = e;

        for (int d = 0; d < head_dim; d++) {
            acc[d] = acc[d] * rescale + weight * __ldg(&V[kv + d]);
        }
    }

    const float inv_sum = 1.0f / sum_exp;
    for (int d = 0; d < head_dim; d++) {
        Output[qbase + d] = acc[d] * inv_sum;
    }
}

// Core single-head Waller operator kernel (online softmax, causal)
__global__ void waller_operator_kernel(
    const float* __restrict__ Q,
    const float* __restrict__ K,
    const float* __restrict__ V,
    float* __restrict__ Output,
    const int seq_len,
    const int head_dim,
    const float scale
) {
    const int row = blockIdx.x * blockDim.x + threadIdx.x;
    if (row >= seq_len) return;

    // Use a reasonable fixed-size accumulator (head_dim is usually <= 128 in our models)
    float acc[128];
    for (int d = 0; d < head_dim; d++) acc[d] = 0.0f;

    float max_val = -INFINITY;
    float sum_exp = 0.0f;

    for (int col = 0; col <= row; col++) {
        float dot = 0.0f;
        #pragma unroll 8
        for (int d = 0; d < head_dim; d++) {
            dot += Q[row * head_dim + d] * K[col * head_dim + d];
        }
        dot *= scale;

        float old_max = max_val;
        max_val = fmaxf(max_val, dot);
        float rescale = expf(old_max - max_val);

        sum_exp = sum_exp * rescale + expf(dot - max_val);

        float weight = expf(dot - max_val);
        #pragma unroll 8
        for (int d = 0; d < head_dim; d++) {
            acc[d] = acc[d] * rescale + weight * V[col * head_dim + d];
        }
    }

    float inv_sum = 1.0f / sum_exp;
    for (int d = 0; d < head_dim; d++) {
        Output[row * head_dim + d] = acc[d] * inv_sum;
    }
}

// Multi-head version (more practical for real models)
// Expects Q/K/V/Output in standard [seq, hidden] layout with heads interleaved:
//   hidden = num_heads * head_dim,  index = row * hidden + head*head_dim + d
__global__ void waller_operator_multihead_kernel(
    const float* __restrict__ Q,
    const float* __restrict__ K,
    const float* __restrict__ V,
    float* __restrict__ Output,
    const int seq_len,
    const int head_dim,
    const int num_heads,
    const float scale
) {
    const int idx = blockIdx.x * blockDim.x + threadIdx.x;
    const int total_rows = seq_len * num_heads;
    if (idx >= total_rows) return;

    const int head = idx / seq_len;
    const int row = idx % seq_len;
    const int hidden = num_heads * head_dim;
    const int head_offset = head * head_dim;  // offset within the hidden dim for this head

    float acc[128];
    for (int d = 0; d < head_dim; d++) acc[d] = 0.0f;

    float max_val = -INFINITY;
    float sum_exp = 0.0f;

    for (int col = 0; col <= row; col++) {
        float dot = 0.0f;
        for (int d = 0; d < head_dim; d++) {
            dot += Q[row * hidden + head_offset + d] * K[col * hidden + head_offset + d];
        }
        dot *= scale;

        float old_max = max_val;
        max_val = fmaxf(max_val, dot);
        float rescale = expf(old_max - max_val);

        sum_exp = sum_exp * rescale + expf(dot - max_val);

        float weight = expf(dot - max_val);
        for (int d = 0; d < head_dim; d++) {
            acc[d] = acc[d] * rescale + weight * V[col * hidden + head_offset + d];
        }
    }

    float inv_sum = 1.0f / sum_exp;
    for (int d = 0; d < head_dim; d++) {
        Output[row * hidden + head_offset + d] = acc[d] * inv_sum;
    }
}

// Split-path fusion: one CUDA block per query row — all heads' Waller + wo without a global attn buffer.
template <int HD>
__global__ void waller_row_fused_wo_kernel(
    const float* __restrict__ Q,
    const float* __restrict__ K,
    const float* __restrict__ V,
    const float* __restrict__ Wo,
    float* __restrict__ Output,
    const int seq_len,
    const int num_heads,
    const float scale
) {
    const int row = blockIdx.x;
    if (row >= seq_len) return;

    const int hidden = num_heads * HD;
    extern __shared__ float attn_smem[];

    for (int head = 0; head < num_heads; head++) {
        const int ho = head * HD;
        const int qbase = row * hidden + ho;

        float q[HD];
        float acc[HD];
        #pragma unroll
        for (int d = 0; d < HD; d++) {
            q[d] = __ldg(&Q[qbase + d]);
            acc[d] = 0.0f;
        }

        float max_val = -INFINITY;
        float sum_exp = 0.0f;

        for (int col = 0; col <= row; col++) {
            const int kv = col * hidden + ho;
            float dot = 0.0f;
            #pragma unroll
            for (int d = 0; d < HD; d++) {
                dot += q[d] * __ldg(&K[kv + d]);
            }
            dot *= scale;

            const float old_max = max_val;
            max_val = fmaxf(max_val, dot);
            const float rescale = expf(old_max - max_val);
            const float e = expf(dot - max_val);
            sum_exp = sum_exp * rescale + e;
            const float weight = e;

            #pragma unroll
            for (int d = 0; d < HD; d++) {
                acc[d] = acc[d] * rescale + weight * __ldg(&V[kv + d]);
            }
        }

        const float inv_sum = 1.0f / sum_exp;
        #pragma unroll
        for (int d = 0; d < HD; d++) {
            attn_smem[ho + d] = acc[d] * inv_sum;
        }
    }

    __syncthreads();

    for (int od = threadIdx.x; od < hidden; od += blockDim.x) {
        float sum = 0.0f;
        for (int j = 0; j < hidden; j++) {
            sum += attn_smem[j] * __ldg(&Wo[j * hidden + od]);
        }
        Output[row * hidden + od] = sum;
    }
}

// Host launch: fused Waller + wo (split production path). Falls back to waller + matmul.
extern "C" void launch_waller_fused_wo(
    const float* Q,
    const float* K,
    const float* V,
    const float* Wo,
    float* Output,
    int seq_len,
    int head_dim,
    int num_heads,
    float scale,
    cudaStream_t stream
) {
    const int hidden = head_dim * num_heads;
    const char* legacy = getenv("LUXI_CUDA_SPLIT_LEGACY_GPU");
    const char* row_fused = getenv("LUXI_CUDA_ROW_FUSED");
    const int use_legacy =
        (legacy != nullptr && legacy[0] == '1')
        || (row_fused == nullptr || row_fused[0] != '1');

    auto launch_hd = [&](int hd) {
        const size_t smem = (size_t)hidden * sizeof(float);
        const int threads = 256;
        if (hd == 16) {
            waller_row_fused_wo_kernel<16><<<seq_len, threads, smem, stream>>>(
                Q, K, V, Wo, Output, seq_len, num_heads, scale);
        } else if (hd == 32) {
            waller_row_fused_wo_kernel<32><<<seq_len, threads, smem, stream>>>(
                Q, K, V, Wo, Output, seq_len, num_heads, scale);
        } else if (hd == 64) {
            waller_row_fused_wo_kernel<64><<<seq_len, threads, smem, stream>>>(
                Q, K, V, Wo, Output, seq_len, num_heads, scale);
        } else if (hd == 128) {
            waller_row_fused_wo_kernel<128><<<seq_len, threads, smem, stream>>>(
                Q, K, V, Wo, Output, seq_len, num_heads, scale);
        }
    };

    if (!use_legacy && hidden > 0 && hidden <= 8192 && seq_len > 0) {
        if (head_dim == 16) { launch_hd(16); return; }
        if (head_dim == 32) { launch_hd(32); return; }
        if (head_dim == 64) { launch_hd(64); return; }
        if (head_dim == 128) { launch_hd(128); return; }
    }
    // Unsupported shape: Rust split path falls back to waller + matmul_f32 (separate buffers).
}

// Host launch function (called from Rust)
extern "C" void launch_waller_operator(
    const float* Q,
    const float* K,
    const float* V,
    float* Output,
    int seq_len,
    int head_dim,
    int num_heads,
    float scale,
    cudaStream_t stream
) {
    const int total = seq_len * num_heads;
    const char* smem_env = getenv("LUXI_WALLER_SMEM");
    const int use_smem = (smem_env != nullptr && smem_env[0] == '1');

    auto launch_hd_smem = [&](int hd) {
        dim3 grid(seq_len, num_heads);
        if (hd == 16) {
            waller_multihead_hd_smem_kernel<16, 16><<<grid, hd, 0, stream>>>(
                Q, K, V, Output, seq_len, num_heads, scale);
        } else if (hd == 32) {
            waller_multihead_hd_smem_kernel<32, 16><<<grid, hd, 0, stream>>>(
                Q, K, V, Output, seq_len, num_heads, scale);
        } else if (hd == 64) {
            waller_multihead_hd_smem_kernel<64, 16><<<grid, hd, 0, stream>>>(
                Q, K, V, Output, seq_len, num_heads, scale);
        } else if (hd == 128) {
            waller_multihead_hd_smem_kernel<128, 16><<<grid, hd, 0, stream>>>(
                Q, K, V, Output, seq_len, num_heads, scale);
        }
    };

    if (use_smem) {
        if (head_dim == 16) { launch_hd_smem(16); return; }
        if (head_dim == 32) { launch_hd_smem(32); return; }
        if (head_dim == 64) { launch_hd_smem(64); return; }
        if (head_dim == 128) { launch_hd_smem(128); return; }
    }

    // hd specialists: blockDim ~= head_dim → many blocks, high SM occupancy (not 64 mega-blocks).
    auto launch_hd = [&](int hd, int threads) {
        const int blocks = (total + threads - 1) / threads;
        if (hd == 16) {
            waller_multihead_hd_t_kernel<16><<<blocks, threads, 0, stream>>>(
                Q, K, V, Output, seq_len, num_heads, scale);
        } else if (hd == 32) {
            waller_multihead_hd_t_kernel<32><<<blocks, threads, 0, stream>>>(
                Q, K, V, Output, seq_len, num_heads, scale);
        } else if (hd == 64) {
            waller_multihead_hd_t_kernel<64><<<blocks, threads, 0, stream>>>(
                Q, K, V, Output, seq_len, num_heads, scale);
        } else if (hd == 128) {
            waller_multihead_hd_t_kernel<128><<<blocks, threads, 0, stream>>>(
                Q, K, V, Output, seq_len, num_heads, scale);
        }
    };

    if (head_dim == 16) {
        launch_hd(16, 32);
        return;
    }
    if (head_dim == 32) {
        launch_hd(32, 32);
        return;
    }
    if (head_dim == 64) {
        launch_hd(64, 64);
        return;
    }
    if (head_dim == 128) {
        launch_hd(128, 128);
        return;
    }

    const int threads = 256;
    const int blocks = (total + threads - 1) / threads;
    if (head_dim > 0 && head_dim <= 128) {
        waller_operator_multihead_cached_q_kernel<<<blocks, threads, 0, stream>>>(
            Q, K, V, Output, seq_len, head_dim, num_heads, scale);
        return;
    }

    // Fallback for very large head_dim
    if (num_heads <= 1) {
        const int blocks1 = (seq_len + threads - 1) / threads;
        waller_operator_kernel<<<blocks1, threads, 0, stream>>>(
            Q, K, V, Output, seq_len, head_dim, scale
        );
    } else {
        waller_operator_multihead_kernel<<<blocks, threads, 0, stream>>>(
            Q, K, V, Output, seq_len, head_dim, num_heads, scale
        );
    }
}

// ============================================================================
// FUSED MLP + WNSM KERNEL (Practical step toward the Colonel / Mega Kernel vision)
// ============================================================================
// This kernel fuses the high-value energy-critical part of the layer:
//   Post-attention LN + MLP (expand + GELU + WNSM inject + project) + final LN + WNSM extract
//
// This is where the biggest "calcs per joule" win comes from — keeping the large
// MLP hidden state (and WNSM payloads) in registers/SMEM instead of HBM round-trips.
//
// Attention can be done with the proven correct waller_operator (CPU or CUDA),
// then this fused kernel handles the rest in one launch.
//
// This is a pragmatic, high-leverage implementation of the vision that actually
// works and delivers measurable energy savings today.

struct MegaLayerParams {
    float attn_scale;
    float ln1_eps;
    float ln2_eps;
    int   mlp_dim;
    int   payload_dim;           // 0 = no WNSM payload this call
};

__device__ __forceinline__ float gelu(float x) {
    // Matches the exact GELU used in the Rust reference
    const float k = 0.044715f;
    float x3 = x * x * x;
    return 0.5f * x * (1.0f + tanhf(0.7978845608f * (x + k * x3)));
}

// Fused mega layer kernel (per-row / per-token processing)
__global__ void mega_fused_layer_kernel(
    const float* __restrict__ input,           // [seq, hidden]
    const float* __restrict__ wq, const float* __restrict__ wk, const float* __restrict__ wv, const float* __restrict__ wo,   // attention weights
    const float* __restrict__ w_fc, const float* __restrict__ b_fc,
    const float* __restrict__ w_proj, const float* __restrict__ b_proj, // MLP
    const float* __restrict__ ln1_gamma, const float* __restrict__ ln1_beta,
    const float* __restrict__ ln2_gamma, const float* __restrict__ ln2_beta,
    const float* __restrict__ v_null,          // [mlp_dim, payload_dim] or nullptr
    float* __restrict__ output,                // [seq, hidden]
    float* __restrict__ payload_io,            // [seq, payload_dim] in/out (optional)
    int seq_len,
    int hidden_dim,
    int num_heads,
    int mlp_dim,
    int payload_dim,
    MegaLayerParams params
) {
    const int row = blockIdx.x;
    if (row >= seq_len) return;

    const int head_dim = hidden_dim / num_heads;
    const float scale = params.attn_scale;

    float row_in[128];
    float normed[128];
    for (int d = 0; d < hidden_dim; d++) {
        row_in[d] = input[row * hidden_dim + d];
    }
    // Welford LN1 (matches Rust layernorm)
    {
        float mean = 0.0f, m2 = 0.0f;
        int count = 0;
        for (int i = 0; i < hidden_dim; i++) {
            count++;
            float delta = row_in[i] - mean;
            mean += delta / (float)count;
            float delta2 = row_in[i] - mean;
            m2 += delta * delta2;
        }
        float var = (count < 2) ? 0.0f : (m2 / (float)count);
        float inv_std = rsqrtf(var + params.ln1_eps);
        for (int d = 0; d < hidden_dim; d++) {
            normed[d] = (row_in[d] - mean) * inv_std * ln1_gamma[d] + ln1_beta[d];
        }
    }

    float attn_out[128];
    for (int d = 0; d < hidden_dim; d++) attn_out[d] = 0.0f;

    // Multi-head causal Waller (matches CPU waller_operator per head)
    for (int head = 0; head < num_heads; head++) {
        const int ho = head * head_dim;
        float q[128];
        for (int d = 0; d < head_dim; d++) {
            float qv = 0.0f;
            for (int j = 0; j < hidden_dim; j++) {
                qv += normed[j] * wq[j * hidden_dim + ho + d];
            }
            q[d] = qv;
        }

        float max_val = -INFINITY;
        float sum_exp = 0.0f;
        float acc[128];
        for (int d = 0; d < head_dim; d++) acc[d] = 0.0f;

        for (int col = 0; col <= row; col++) {
            float col_in[128];
            float col_normed[128];
            for (int d = 0; d < hidden_dim; d++) {
                col_in[d] = input[col * hidden_dim + d];
            }
            float mean = 0.0f, m2 = 0.0f;
            int count = 0;
            for (int i = 0; i < hidden_dim; i++) {
                count++;
                float delta = col_in[i] - mean;
                mean += delta / (float)count;
                float delta2 = col_in[i] - mean;
                m2 += delta * delta2;
            }
            float var = (count < 2) ? 0.0f : (m2 / (float)count);
            float inv_std = rsqrtf(var + params.ln1_eps);
            for (int d = 0; d < hidden_dim; d++) {
                col_normed[d] = (col_in[d] - mean) * inv_std * ln1_gamma[d] + ln1_beta[d];
            }

            float k_col[128], v_col[128];
            for (int d = 0; d < head_dim; d++) {
                float kv = 0.0f, vv = 0.0f;
                for (int j = 0; j < hidden_dim; j++) {
                    kv += col_normed[j] * wk[j * hidden_dim + ho + d];
                    vv += col_normed[j] * wv[j * hidden_dim + ho + d];
                }
                k_col[d] = kv;
                v_col[d] = vv;
            }

            float dot = 0.0f;
            for (int d = 0; d < head_dim; d++) dot += q[d] * k_col[d];
            dot *= scale;

            float old_max = max_val;
            max_val = fmaxf(max_val, dot);
            float rescale = expf(old_max - max_val);
            float e = expf(dot - max_val);
            sum_exp = sum_exp * rescale + e;
            float weight = e;
            for (int d = 0; d < head_dim; d++) {
                acc[d] = acc[d] * rescale + weight * v_col[d];
            }
        }

        float inv_sum = 1.0f / sum_exp;
        for (int d = 0; d < head_dim; d++) {
            attn_out[ho + d] = acc[d] * inv_sum;
        }
    }

    // === 3. Output projection + residual + LN2 (Welford, matches CPU) ===
    float projected_attn[128];
    for (int d = 0; d < hidden_dim; d++) {
        float sum = 0.0f;
        for (int j = 0; j < hidden_dim; j++) {
            sum += attn_out[j] * wo[j * hidden_dim + d];
        }
        projected_attn[d] = sum;
    }

    float after_attn[128];
    for (int d = 0; d < hidden_dim; d++) {
        after_attn[d] = projected_attn[d] + row_in[d];
    }

    float mean = 0.0f, m2 = 0.0f;
    int count = 0;
    for (int i = 0; i < hidden_dim; i++) {
        count++;
        float delta = after_attn[i] - mean;
        mean += delta / (float)count;
        float delta2 = after_attn[i] - mean;
        m2 += delta * delta2;
    }
    float var = (count < 2) ? 0.0f : (m2 / (float)count);
    float inv_std = rsqrtf(var + params.ln2_eps);
    for (int d = 0; d < hidden_dim; d++) {
        after_attn[d] = (after_attn[d] - mean) * inv_std * ln2_gamma[d] + ln2_beta[d];
    }

    // === 4. MLP Expand + GELU (this is where WNSM magic happens) ===
    float mlp_hidden[512]; // support up to mlp_dim=512 in this reference
    for (int i = 0; i < mlp_dim; i++) {
        float sum = b_fc[i];
        for (int j = 0; j < hidden_dim; j++) {
            sum += after_attn[j] * w_fc[j * mlp_dim + i];
        }
        mlp_hidden[i] = gelu(sum);
    }

    // === 5. WNSM Inject (if payload present) - with proper baseline ===
    float baseline[64]; // support reasonable payload size for this reference
    if (payload_dim > 0 && v_null != nullptr) {
        for (int k = 0; k < payload_dim; k++) {
            float s = 0.0f;
            for (int i = 0; i < mlp_dim; i++) {
                s += mlp_hidden[i] * v_null[i * payload_dim + k];
            }
            baseline[k] = s;
        }

        if (payload_io != nullptr) {
            for (int k = 0; k < payload_dim; k++) {
                float pval = payload_io[row * payload_dim + k];
                for (int i = 0; i < mlp_dim; i++) {
                    mlp_hidden[i] += pval * v_null[i * payload_dim + k];
                }
            }
        }
    }

    // === 6. MLP Project + residual + LN2 ===
    float after_mlp[128];
    for (int j = 0; j < hidden_dim; j++) {
        float sum = b_proj[j];
        for (int i = 0; i < mlp_dim; i++) {
            sum += mlp_hidden[i] * w_proj[i * hidden_dim + j];
        }
        after_mlp[j] = sum + after_attn[j]; // residual
    }

    // LN2
    mean = 0.0f;
    for (int d = 0; d < hidden_dim; d++) mean += after_mlp[d];
    mean /= hidden_dim;

    var = 0.0f;
    for (int d = 0; d < hidden_dim; d++) {
        float diff = after_mlp[d] - mean;
        var += diff * diff;
    }
    var /= hidden_dim;
    inv_std = rsqrtf(var + params.ln2_eps);

    for (int d = 0; d < hidden_dim; d++) {
        after_mlp[d] = (after_mlp[d] - mean) * inv_std * ln2_gamma[d] + ln2_beta[d];
    }

    // === 7. WNSM Extract for next layer (using the baseline computed above) ===
    if (payload_dim > 0 && v_null != nullptr && payload_io != nullptr) {
        for (int k = 0; k < payload_dim; k++) {
            float s = 0.0f;
            for (int i = 0; i < mlp_dim; i++) {
                s += mlp_hidden[i] * v_null[i * payload_dim + k];
            }
            payload_io[row * payload_dim + k] = s - baseline[k];
        }
    }

    // Write final output
    for (int d = 0; d < hidden_dim; d++) {
        output[row * hidden_dim + d] = after_mlp[d];
    }
}

// ============================================================================
// FUSED MLP + WNSM KERNEL (Recommended practical implementation of the vision)
// ============================================================================
// This kernel takes the output of attention (after output projection + residual + LN1)
// and performs the fused MLP + WNSM work in one launch. This is the highest-leverage
// part for energy efficiency.

__global__ void fused_mlp_wnsm_kernel(
    const float* __restrict__ after_ln1,     // [seq, hidden] - input to MLP stage
    const float* __restrict__ w_fc,
    const float* __restrict__ b_fc,
    const float* __restrict__ w_proj,
    const float* __restrict__ b_proj,
    const float* __restrict__ ln2_gamma,
    const float* __restrict__ ln2_beta,
    const float* __restrict__ v_null,
    float* __restrict__ output,              // [seq, hidden]
    float* __restrict__ payload_io,          // optional WNSM payload [seq, payload_dim]
    int seq_len,
    int hidden_dim,
    int mlp_dim,
    int payload_dim,
    float ln2_eps
) {
    const int row = blockIdx.x;
    if (row >= seq_len) return;

    extern __shared__ float smem[];
    float* after_ln1_local = smem;
    float* mlp_hidden = smem + hidden_dim;
    float* after_mlp = smem + hidden_dim + mlp_dim;
    float* ln2_out = smem + hidden_dim + mlp_dim + hidden_dim;

    for (int d = threadIdx.x; d < hidden_dim; d += blockDim.x) {
        after_ln1_local[d] = after_ln1[row * hidden_dim + d];
    }
    __syncthreads();

    if (threadIdx.x == 0) {
        for (int i = 0; i < mlp_dim; i++) {
            float sum = b_fc[i];
            for (int j = 0; j < hidden_dim; j++) {
                sum += after_ln1_local[j] * w_fc[j * mlp_dim + i];
            }
            mlp_hidden[i] = gelu(sum);
        }

        float baseline[64] = {0};
        if (payload_dim > 0 && v_null != nullptr) {
            for (int k = 0; k < payload_dim; k++) {
                float s = 0.0f;
                for (int i = 0; i < mlp_dim; i++) {
                    s += mlp_hidden[i] * v_null[i * payload_dim + k];
                }
                baseline[k] = s;
            }

            if (payload_io != nullptr) {
                for (int k = 0; k < payload_dim; k++) {
                    float pval = payload_io[row * payload_dim + k];
                    for (int i = 0; i < mlp_dim; i++) {
                        mlp_hidden[i] += pval * v_null[i * payload_dim + k];
                    }
                }
            }

            for (int k = 0; k < payload_dim; k++) {
                float s = 0.0f;
                for (int i = 0; i < mlp_dim; i++) {
                    s += mlp_hidden[i] * v_null[i * payload_dim + k];
                }
                if (payload_io != nullptr) {
                    payload_io[row * payload_dim + k] = s - baseline[k];
                }
            }
        }

        for (int j = 0; j < hidden_dim; j++) {
            float sum = b_proj[j];
            for (int i = 0; i < mlp_dim; i++) {
                sum += mlp_hidden[i] * w_proj[i * hidden_dim + j];
            }
            after_mlp[j] = sum + after_ln1_local[j];
        }
        welford_ln_f32_mlp(after_mlp, ln2_out, hidden_dim, ln2_gamma, ln2_beta, ln2_eps);
    }
    __syncthreads();

    for (int d = threadIdx.x; d < hidden_dim; d += blockDim.x) {
        output[row * hidden_dim + d] = ln2_out[d];
    }
}

// Host launch for the fused MLP + WNSM kernel (recommended)
extern "C" void launch_fused_mlp_wnsm(
    const float* after_ln1,
    const float* w_fc, const float* b_fc,
    const float* w_proj, const float* b_proj,
    const float* ln2_gamma, const float* ln2_beta,
    const float* v_null,
    float* output,
    float* payload,
    int seq_len,
    int hidden_dim,
    int mlp_dim,
    int payload_dim,
    float ln2_eps,
    cudaStream_t stream
) {
    int blocks = seq_len;
    int threads = 256;
    const size_t smem =
        (size_t)(hidden_dim * 2 + mlp_dim + hidden_dim) * sizeof(float);

    fused_mlp_wnsm_kernel<<<blocks, threads, smem, stream>>>(
        after_ln1,
        w_fc, b_fc,
        w_proj, b_proj,
        ln2_gamma, ln2_beta,
        v_null,
        output,
        payload,
        seq_len, hidden_dim, mlp_dim, payload_dim,
        ln2_eps
    );
}

// Version that accepts device pointers for weights (for use with persistent CudaLayerWeights).
// The kernel already dereferences the weight pointers as __restrict__ device memory.
// This is the production persistence path: large MLP + V_null weights stay on device across calls.
extern "C" void launch_fused_mlp_wnsm_device_weights(
    const float* after_ln1,
    const float* d_w_fc, const float* d_b_fc,
    const float* d_w_proj, const float* d_b_proj,
    const float* d_ln2_gamma, const float* d_ln2_beta,
    const float* d_v_null,
    float* output,
    float* payload,
    int seq_len,
    int hidden_dim,
    int mlp_dim,
    int payload_dim,
    float ln2_eps,
    cudaStream_t stream
) {
    int blocks = seq_len;
    int threads = 256;
    const size_t smem =
        (size_t)(hidden_dim * 2 + mlp_dim + hidden_dim) * sizeof(float);

    fused_mlp_wnsm_kernel<<<blocks, threads, smem, stream>>>(
        after_ln1,
        d_w_fc, d_b_fc,
        d_w_proj, d_b_proj,
        d_ln2_gamma, d_ln2_beta,
        d_v_null,
        output,
        payload,
        seq_len, hidden_dim, mlp_dim, payload_dim,
        ln2_eps
    );
}

// Launch function for the full fused mega layer kernel (the complete vision).
// This is the one that does QKV proj + correct causal attention + output proj + LN + MLP + WNSM in the kernel.
extern "C" void launch_mega_fused_layer(
    const float* input,
    const float* wq, const float* wk, const float* wv, const float* wo,
    const float* w_fc, const float* b_fc,
    const float* w_proj, const float* b_proj,
    const float* ln1_gamma, const float* ln1_beta,
    const float* ln2_gamma, const float* ln2_beta,
    const float* v_null,
    float* output,
    float* payload,
    int seq_len,
    int hidden_dim,
    int num_heads,
    int mlp_dim,
    int payload_dim,
    MegaLayerParams params,
    cudaStream_t stream
) {
    int blocks = seq_len;
    int threads = 256;

    mega_fused_layer_kernel<<<blocks, threads, 0, stream>>>(
        input,
        wq, wk, wv, wo,
        w_fc, b_fc,
        w_proj, b_proj,
        ln1_gamma, ln1_beta,
        ln2_gamma, ln2_beta,
        v_null,
        output,
        payload,
        seq_len,
        hidden_dim,
        num_heads,
        mlp_dim,
        payload_dim,
        params
    );
}