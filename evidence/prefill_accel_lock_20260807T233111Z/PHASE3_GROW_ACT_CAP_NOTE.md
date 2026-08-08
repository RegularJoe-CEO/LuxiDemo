# Phase 3 — grow_act_cap (status)

**Date:** 2026-08-08  

## Intent

Replace `next_power_of_two` activation capacity with `grow_act_cap` (need + 12.5%, 256-align) to remove the B72→B76 **2× HBM cliff**.

## Local code

`attention-transformer-v2/src/gpu/cuda.rs` — `grow_act_cap` applied to:

- `CudaWallerBuffers::ensure_capacity` / MLP scratch / payload  
- Quant stack layer input  
- Post-attn pack paths  

## Pod rebuild attempt

1. Remote tree had **SwiGLU / true-AR drift** vs thr binary (Aug 4): `d_w_up` args, `append_and_decode_layer_host_qkv` missing.  
2. Drift patched enough to **compile** thr example (`cuda,gpt2`).  
3. Resulting binary **regressed**:
   - B72 dual_gemm OOM on `cudaMalloc failed for layer input`  
   - WNSM free-ride residual ~7.8 (was ~0 on champion thr binary)  
   - Trade map log: `swiglu_gold` path differs from thr-lock binary  
4. **Champion thr binary restored** from `cuda_qwen7b_trade.pre_exactcap` (8205464 bytes, Aug 4 thr-lock).  
5. Broken rebuild kept as `cuda_qwen7b_trade.growcap_broken`.

## Next work (do not rush)

1. Apply **only** `grow_act_cap` on a clean checkout of the **exact thr-lock tree** (no SwiGLU/true_ar partial merge).  
2. Rebuild, B72 sanity thr+J must match champion within noise.  
3. Then ladder B76–B112 dual_gemm; stop when thr flattens or J/pos worsens.  
4. Do not replace product thr binary until B72 multi-run re-locks.

## Product impact

**None yet.** Absolute champion remains dual_gemm B72 multi-run **44,860 / 0.01532** on pre-exactcap thr binary.
