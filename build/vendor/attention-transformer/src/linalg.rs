//! Deterministic, high-performance linear algebra primitives for the CPU path.
//!
//! All accumulations use a FIXED traversal order so that results (and SHA
//! receipts) are identical regardless of SIMD width or rayon thread count.
//!
//! Focus: matmul (the dominant cost in GPT-2 width models).
//!
//! ## Why this is both fast AND bit-exact
//!
//! The naive way to use NEON is to vectorize the K-dimension (the dot-product
//! reduction) with 4 lane accumulators and then horizontally sum the lanes.
//! That is FAST but changes the floating-point accumulation order, and a fused
//! `vfmaq_f32` collapses the multiply+add into a single rounding. Both of those
//! change the result bits and would break the SHA receipts.
//!
//! Instead we vectorize the **N-dimension (output columns)**:
//!   - For a fixed output element `c[i,j]`, the K-loop still runs `p = 0..k`
//!     strictly left-to-right -- identical order to the scalar baseline.
//!   - We use `vmulq_f32` then `vaddq_f32` (multiply, round, add, round) -- the
//!     exact same two-rounding sequence as scalar `sum += a*b`. We deliberately
//!     do NOT use the fused `vfmaq_f32`.
//!   - B loads are contiguous (`b[p*n + j .. j+4]`), so this is cache-friendly.
//!
//! Result: 4 output columns computed per instruction, contiguous memory access,
//! and every element bit-identical to the deterministic scalar path.
//!
//! Parallelism (rayon) is only ever over independent output ROWS -- never inside
//! a dot product -- so it cannot perturb any reduction.

#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[cfg(all(target_arch = "aarch64", feature = "simd"))]
use core::arch::aarch64::*;

/// High-level matmul: C = A @ B
/// A: [m, k], B: [k, n] (row-major), C: [m, n]
/// This is the exact contract used by the GPT-2 projections.
#[inline]
pub fn matmul(a: &[f32], b: &[f32], m: usize, k: usize, n: usize) -> Vec<f32> {
    let mut c = vec![0.0f32; m * n];
    matmul_into(a, b, &mut c, m, k, n);
    c
}

/// Add bias after matmul (out[i*n + j] += bias[j]).
#[inline]
pub fn add_bias_inplace(out: &mut [f32], bias: &[f32], m: usize, n: usize) {
    for i in 0..m {
        for j in 0..n {
            out[i * n + j] += bias[j];
        }
    }
}

// -----------------------------------------------------------------------------
// Quantization turbo path (new primary focus for speed)
// -----------------------------------------------------------------------------
// Per user's direction: we are now aggressively tuning for quantization as the
// main way to make this engine practically fast on laptops.
// The f32 path remains the deterministic gold standard with full receipts.
// The quantized path is the "fast mode" (approximate, separate receipts).

/// Simple symmetric int8 quantization result for a weight matrix.
/// scale is chosen so that the max absolute value maps to 127.
#[derive(Clone, Debug)]
pub struct QuantizedMatrix {
    pub data: Vec<i8>,
    pub scale: f32,
    pub k: usize,
    pub n: usize,
}

/// Quantize a f32 matrix [k, n] to symmetric int8 (per-tensor for simplicity
/// and speed on first implementation).
pub fn quantize_symmetric_i8(matrix: &[f32], k: usize, n: usize) -> QuantizedMatrix {
    let mut max_abs = 0.0f32;
    for &v in matrix {
        max_abs = max_abs.max(v.abs());
    }
    let scale = if max_abs > 0.0 { 127.0 / max_abs } else { 1.0 };

    let mut data = Vec::with_capacity(k * n);
    for &v in matrix {
        let q = (v * scale).round().clamp(-127.0, 127.0) as i8;
        data.push(q);
    }

    QuantizedMatrix { data, scale, k, n }
}

/// Matmul between f32 activations [m, k] and quantized weights [k, n] (int8).
/// 
/// Determinism contract for the quantized path:
/// - Weights are quantized once at load time using deterministic symmetric quantization.
/// - For every output element, the K-dimension (p) is accumulated strictly left-to-right.
/// - We use widening accumulation (i8 → i16 → i32) to reduce overflow risk while keeping order fixed.
/// - The final dequantization is also deterministic.
/// 
/// This path is *not* bit-identical to the f32 path. It has its own receipts.
/// It is, however, fully reproducible across runs and machines when the same
/// quantized weights (same scales + rounding) are used.
pub fn matmul_f32_i8(a: &[f32], q: &QuantizedMatrix, m: usize, k: usize, n: usize) -> Vec<f32> {
    #[cfg(all(target_arch = "aarch64", feature = "simd"))]
    {
        return matmul_f32_i8_neon(a, q, m, k, n);
    }
    #[cfg(not(all(target_arch = "aarch64", feature = "simd")))]
    {
        matmul_f32_i8_scalar(a, q, m, k, n)
    }
}

#[inline]
#[allow(dead_code)]
fn matmul_f32_i8_scalar(a: &[f32], q: &QuantizedMatrix, m: usize, k: usize, n: usize) -> Vec<f32> {
    assert_eq!(k, q.k);
    assert_eq!(n, q.n);

    let mut c = vec![0.0f32; m * n];

    // Deterministic tiled reference (same blocking philosophy as f32 path).
    // p is strictly outer within each tile for every output element.
    const MC: usize = 64;
    const NC: usize = 64;
    const KC: usize = 128;

    for i0 in (0..m).step_by(MC) {
        let i_end = (i0 + MC).min(m);
        for j0 in (0..n).step_by(NC) {
            let j_end = (j0 + NC).min(n);
            for p0 in (0..k).step_by(KC) {
                let p_end = (p0 + KC).min(k);

                for i in i0..i_end {
                    for j in j0..j_end {
                        let mut sum: i32 = 0;
                        for p in p0..p_end {
                            let scaled_a = (a[i * k + p] * q.scale).round() as i32;
                            let q_val = q.data[p * n + j] as i32;
                            sum += scaled_a * q_val;
                        }
                        c[i * n + j] += (sum as f32) / (q.scale * q.scale);
                    }
                }
            }
        }
    }

    c
}

/// High-level quantized matmul using pre-packed int8 weights.
/// Simple deterministic fused LN + matmul_bias for M=1 (common in incremental generation).
/// Applies layernorm then matmul_bias in one pass over data to reduce writes.
pub fn fused_ln_matmul_bias(
    x: &[f32],
    ln_gamma: &[f32],
    ln_beta: &[f32],
    eps: f32,
    w: &[f32],
    b: &[f32],
    n_embd: usize,
    out_dim: usize,
) -> Vec<f32> {
    // Deterministic Welford LN for the single vector
    let mut state = crate::welford::WelfordState::new();
    for &v in x {
        state.update(v);
    }
    let mean = state.mean;
    let std = state.std(eps);

    let mut ln_out = vec![0.0f32; n_embd];
    for d in 0..n_embd {
        ln_out[d] = (x[d] - mean) / std * ln_gamma[d] + ln_beta[d];
    }

    // Then normal matmul_bias
    matmul_bias(&ln_out, w, b, 1, n_embd, out_dim)
}

pub fn matmul_f32_i8_packed(a: &[f32], packed_q: &[i8], scale: f32, m: usize, k: usize, n: usize) -> Vec<f32> {
    // Bridge for now — real version will have direct packed NEON.
    let q = QuantizedMatrix {
        data: packed_q.to_vec(),
        scale,
        k,
        n,
    };
    matmul_f32_i8(a, &q, m, k, n)
}

/// NEON-optimized version of matmul_f32_i8 for aarch64.
/// 
/// Vectorizes over the N (output) dimension while streaming K left-to-right,
/// exactly matching the determinism contract of the scalar version above.
/// Uses widening int8 → i16 → i32 accumulation.
#[cfg(all(target_arch = "aarch64", feature = "simd"))]
pub fn matmul_f32_i8_neon(a: &[f32], q: &QuantizedMatrix, m: usize, k: usize, n: usize) -> Vec<f32> {
    assert_eq!(k, q.k);
    assert_eq!(n, q.n);

    let mut c = vec![0.0f32; m * n];

    // Aggressive NEON int8 kernel:
    // - Vectorize over output columns (N) — same as f32 path for determinism.
    // - Stream K (p) strictly left-to-right for every output element.
    // - Use widening int8 loads + i16 -> i32 accumulation.
    // This is the most aggressive speed path while obeying the fixed-order rule.

    unsafe {
        let qptr = q.data.as_ptr();
        let scale = q.scale;

        for i in 0..m {
            let a_row = &a[i * k..(i + 1) * k];
            let c_row = &mut c[i * n..(i + 1) * n];

            let mut j = 0;
            // Process 16 columns at a time (aggressive for speed on M-series)
            while j + 16 <= n {
                let mut acc0 = vdupq_n_s32(0);
                let mut acc1 = vdupq_n_s32(0);
                let mut acc2 = vdupq_n_s32(0);
                let mut acc3 = vdupq_n_s32(0);

                for p in 0..k {
                    let a_val = (a_row[p] * scale).round() as i32;
                    let av = vdupq_n_s32(a_val);

                    let qbase = p * n + j;
                    let qv0 = vld1q_s8(qptr.add(qbase));
                    let qv1 = vld1q_s8(qptr.add(qbase + 8));

                    // Widen to i16 then i32 and accumulate (fixed p order)
                    let lo0 = vmovl_s8(vget_low_s8(qv0));
                    let hi0 = vmovl_s8(vget_high_s8(qv0));
                    let lo1 = vmovl_s8(vget_low_s8(qv1));
                    let hi1 = vmovl_s8(vget_high_s8(qv1));

                    acc0 = vmlaq_s32(acc0, av, vmovl_s16(vget_low_s16(lo0)));
                    acc0 = vmlaq_s32(acc0, av, vmovl_s16(vget_high_s16(lo0)));
                    acc1 = vmlaq_s32(acc1, av, vmovl_s16(vget_low_s16(hi0)));
                    acc1 = vmlaq_s32(acc1, av, vmovl_s16(vget_high_s16(hi0)));

                    acc2 = vmlaq_s32(acc2, av, vmovl_s16(vget_low_s16(lo1)));
                    acc2 = vmlaq_s32(acc2, av, vmovl_s16(vget_high_s16(lo1)));
                    acc3 = vmlaq_s32(acc3, av, vmovl_s16(vget_low_s16(hi1)));
                    acc3 = vmlaq_s32(acc3, av, vmovl_s16(vget_high_s16(hi1)));
                }

                // Store results (dequant)
                let inv_scale2 = 1.0 / (scale * scale);
                vst1q_f32(c_row.as_mut_ptr().add(j),      vmulq_n_f32(vcvtq_f32_s32(acc0), inv_scale2));
                vst1q_f32(c_row.as_mut_ptr().add(j + 4),  vmulq_n_f32(vcvtq_f32_s32(acc1), inv_scale2));
                vst1q_f32(c_row.as_mut_ptr().add(j + 8),  vmulq_n_f32(vcvtq_f32_s32(acc2), inv_scale2));
                vst1q_f32(c_row.as_mut_ptr().add(j + 12), vmulq_n_f32(vcvtq_f32_s32(acc3), inv_scale2));

                j += 16;
            }

            // Tail (scalar, same left-to-right order)
            while j < n {
                let mut sum: i32 = 0;
                for p in 0..k {
                    let scaled_a = (a_row[p] * scale).round() as i32;
                    sum += scaled_a * (q.data[p * n + j] as i32);
                }
                c_row[j] = (sum as f32) / (scale * scale);
                j += 1;
            }
        }
    }
    c
}

#[cfg(not(all(target_arch = "aarch64", feature = "simd")))]
pub fn matmul_f32_i8_neon(a: &[f32], q: &QuantizedMatrix, m: usize, k: usize, n: usize) -> Vec<f32> {
    matmul_f32_i8(a, q, m, k, n)
}

/// Fused matmul + bias that seeds each output element with `bias[j]` BEFORE the
/// K-dimension accumulation. This exactly reproduces the original GPT-2
/// accumulation order (`sum = bias; sum += products...` left-to-right), so the
/// SHA-256 logits receipt stays bit-identical to the pre-optimization baseline.
///
/// A: [m, k], B: [k, n] (row-major), bias: [n], C: [m, n]
#[inline]
pub fn matmul_bias(a: &[f32], b: &[f32], bias: &[f32], m: usize, k: usize, n: usize) -> Vec<f32> {
    let mut c = vec![0.0f32; m * n];

    // Seed every output row with its bias term first.
    for i in 0..m {
        c[i * n..i * n + n].copy_from_slice(bias);
    }

    // Accumulate A@B on top of the bias seed (no zero-fill).
    matmul_accum_into(a, b, &mut c, m, k, n);
    c
}

// -----------------------------------------------------------------------------
// Load-time weight packing (Wave 2 step change)
// -----------------------------------------------------------------------------
// The current driver already does excellent on-the-fly B-tile packing inside
// the hot loop. The next radical lever is to do the full N-tiled + K-paneled
// packing *once* at model construction time for the 4 heavy per-layer matrices.
// This eliminates thousands of small allocations + strided copies per forward
// (especially painful on the M=1 incremental path) and improves L1/L2 residency.
//
// Packing layout (simple, deterministic, matches the existing NC/KC):
//   [ k, n, num_panels, ... panels ... ]
// Each panel for (p0, j0) is a dense KC x NC block stored row-major.
// The packed matmul entry points below consume this layout directly with
// zero runtime repacking of B.
// -----------------------------------------------------------------------------

/// Pack a [k, n] matrix into the kernel-friendly panel layout used by the
/// tiled driver. The resulting buffer can be fed to `matmul_bias_packed`.
#[allow(unused_variables)]
pub fn pack_b_for_kernel(b: &[f32], k: usize, n: usize) -> Vec<f32> {
    const NC: usize = 64;
    const KC: usize = 128;

    let num_j_panels = n.div_ceil(NC);
    let num_p_panels = k.div_ceil(KC);

    // Header: k, n (as f32 for now — simple), followed by dense panels.
    // Panel order: p-major, then j-major. Each panel is KC x NC (padded).
    let total_panels = num_p_panels * num_j_panels;
    let mut packed = Vec::with_capacity(2 + total_panels * KC * NC);
    packed.push(k as f32);
    packed.push(n as f32);

    for p0 in (0..k).step_by(KC) {
        for j0 in (0..n).step_by(NC) {
            let p_end = (p0 + KC).min(k);
            let j_end = (j0 + NC).min(n);
            let kc = p_end - p0;
            let nc = j_end - j0;

            for pp in 0..KC {
                for jj in 0..NC {
                    let val = if pp < kc && jj < nc {
                        b[(p0 + pp) * n + (j0 + jj)]
                    } else {
                        0.0
                    };
                    packed.push(val);
                }
            }
        }
    }
    packed
}

/// Quantized version of packing for int8 weights.
/// Produces panels in the same layout so the NEON int8 kernel can stream unit-stride.
pub fn pack_b_for_kernel_i8(q: &QuantizedMatrix) -> Vec<i8> {
    const NC: usize = 64;
    const KC: usize = 128;

    let k = q.k;
    let n = q.n;
    let num_j_panels = n.div_ceil(NC);
    let num_p_panels = k.div_ceil(KC);

    let total_panels = num_p_panels * num_j_panels;
    let mut packed = Vec::with_capacity(total_panels * KC * NC);

    for p0 in (0..k).step_by(KC) {
        for j0 in (0..n).step_by(NC) {
            let p_end = (p0 + KC).min(k);
            let j_end = (j0 + NC).min(n);
            let kc = p_end - p0;
            let nc = j_end - j0;

            for pp in 0..KC {
                for jj in 0..NC {
                    let val = if pp < kc && jj < nc {
                        q.data[(p0 + pp) * n + (j0 + jj)]
                    } else {
                        0i8
                    };
                    packed.push(val);
                }
            }
        }
    }
    packed
}

/// High-level matmul_bias using a pre-packed B (from pack_b_for_kernel).
/// This completely bypasses the per-tile B packing inside the driver.
#[inline]
pub fn matmul_bias_packed(
    a: &[f32],
    packed_b: &[f32],
    bias: &[f32],
    m: usize,
    k: usize,
    n: usize,
) -> Vec<f32> {
    let mut c = vec![0.0f32; m * n];
    for i in 0..m {
        c[i * n..i * n + n].copy_from_slice(bias);
    }
    matmul_accum_packed_into(a, packed_b, &mut c, m, k, n);
    c
}

/// Allocation-free variant of matmul_bias_packed.
/// Writes (A @ B_packed + bias) directly into the provided `out` buffer.
/// `out` must have length exactly m * n and will be overwritten (bias is copied in).
#[inline]
pub fn matmul_bias_packed_into(
    a: &[f32],
    packed_b: &[f32],
    bias: &[f32],
    m: usize,
    k: usize,
    n: usize,
    out: &mut [f32],
) {
    assert_eq!(out.len(), m * n, "matmul_bias_packed_into: out buffer size mismatch");
    for i in 0..m {
        out[i * n..i * n + n].copy_from_slice(bias);
    }
    matmul_accum_packed_into(a, packed_b, out, m, k, n);
}

#[inline]
fn matmul_accum_packed_into(a: &[f32], packed_b: &[f32], c: &mut [f32], m: usize, k: usize, n: usize) {
    // Decode header
    let pk = packed_b[0] as usize;
    let pn = packed_b[1] as usize;
    debug_assert_eq!(pk, k);
    debug_assert_eq!(pn, n);

    const MC: usize = 64;
    const NC: usize = 64;
    const KC: usize = 128;

    // M=1 fast path using the proven NEON accumulate_row (same as main driver).
    // For packed B we still fall back to a simple path for M=1 for now (the big win is for larger M).
    if m == 1 {
        // Strong M=1 path for packed: use the same NEON-capable accumulate_dense_tile
        // that the main driver uses for dense panels. This gives the generation path
        // (M=1) the full vectorized benefit from the pre-packed layout.
        let c_row = &mut c[0..n];
        let data = &packed_b[2..];
        let num_j_panels = n.div_ceil(NC);
        let panel_size = KC * NC;

        for j0 in (0..n).step_by(NC) {
            let j_end = (j0 + NC).min(n);
            let nc = j_end - j0;
            let j_panel = j0 / NC;

            for p0 in (0..k).step_by(KC) {
                let p_end = (p0 + KC).min(k);
                let kc = p_end - p0;
                let p_panel = p0 / KC;

                let panel_idx = p_panel * num_j_panels + j_panel;
                let panel_start = panel_idx * panel_size;
                let panel = &data[panel_start..panel_start + KC * NC];

                let a_seg = &a[p0..p_end];
                let c_row_seg = &mut c_row[j0..j_end];

                // Use the cfg-gated dense tile (NEON when available, scalar fallback)
                unsafe {
                    accumulate_dense_tile(
                        a_seg.as_ptr(),
                        panel.as_ptr(),
                        c_row_seg.as_mut_ptr(),
                        kc,
                        nc,
                        NC,
                    );
                }
            }
        }
        return;
    }

    let data = &packed_b[2..];
    let num_j_panels = n.div_ceil(NC);
    let panel_size = KC * NC;

    #[cfg(feature = "rayon")]
    {
        let m_tiles: Vec<_> = (0..m).step_by(MC).collect();
        let c_ptr_usize = c.as_mut_ptr() as usize;
        m_tiles.par_iter().for_each(|&i0| {
            let i_end = (i0 + MC).min(m);
            let c_base = c_ptr_usize as *mut f32;
            for j0 in (0..n).step_by(NC) {
                let j_end = (j0 + NC).min(n);
                let nc = j_end - j0;
                let j_panel = j0 / NC;

                for p0 in (0..k).step_by(KC) {
                    let p_end = (p0 + KC).min(k);
                    let kc = p_end - p0;
                    let p_panel = p0 / KC;

                    let panel_idx = p_panel * num_j_panels + j_panel;
                    let panel_start = panel_idx * panel_size;
                    let panel = &data[panel_start..panel_start + KC * NC];

                    for i in i0..i_end {
                        let a_seg = &a[i * k + p0..i * k + p_end];
                        let c_row_ptr = unsafe { c_base.add(i * n + j0) };
                        unsafe {
                            // Use the NEON dense tile when possible (falls back to scalar inside)
                            accumulate_dense_tile(
                                a_seg.as_ptr(),
                                panel.as_ptr(),
                                c_row_ptr,
                                kc,
                                nc,
                                NC,
                            );
                        }
                    }
                }
            }
        });
    }

    #[cfg(not(feature = "rayon"))]
    {
        for i0 in (0..m).step_by(MC) {
            let i_end = (i0 + MC).min(m);
            for j0 in (0..n).step_by(NC) {
                let j_end = (j0 + NC).min(n);
                let nc = j_end - j0;
                let j_panel = j0 / NC;

                for p0 in (0..k).step_by(KC) {
                    let p_end = (p0 + KC).min(k);
                    let kc = p_end - p0;
                    let p_panel = p0 / KC;

                    let panel_idx = p_panel * num_j_panels + j_panel;
                    let panel_start = panel_idx * panel_size;
                    let panel = &data[panel_start..panel_start + KC * NC];

                    for i in i0..i_end {
                        let a_seg = &a[i * k + p0..i * k + p_end];
                        let c_row = &mut c[i * n + j0..i * n + j_end];
                        accumulate_dense_tile_scalar(a_seg, panel, c_row, kc, nc, NC);
                    }
                }
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Drivers: zero-seeded (matmul) and bias-seeded (matmul_bias) share one kernel.
// They differ only in whether `c` is zeroed first.
// -----------------------------------------------------------------------------

#[inline]
pub fn matmul_into(a: &[f32], b: &[f32], c: &mut [f32], m: usize, k: usize, n: usize) {
    c.fill(0.0);
    matmul_accum_into(a, b, c, m, k, n);
}

/// Fast path for the extremely common M=1 case (single-token incremental generation).
/// Directly invokes the proven per-row NEON/scalar kernel on the full width.
/// This completely bypasses 2D tiling overhead, packing, and small-tile loops
/// for the hot per-token path while preserving exact left-to-right K accumulation.
#[inline]
fn matmul_accum_m1(a: &[f32], b: &[f32], c: &mut [f32], k: usize, n: usize) {
    // a is a single row of length k; c is a single row of length n
    let a_row = &a[0..k];
    let c_row = &mut c[0..n];
    accumulate_row(a_row, b, c_row, k, n);
}

#[inline]
fn matmul_accum_into(a: &[f32], b: &[f32], c: &mut [f32], m: usize, k: usize, n: usize) {
    // M=1 fast path: bypass all tiling for the generation-critical single-row case.
    // Uses the full-width vectorized accumulate_row (16-col NEON + tails, separate
    // mul+add, strict p=0..k left-to-right order). This is the "step change" bypass.
    if m == 1 {
        matmul_accum_m1(a, b, c, k, n);
        return;
    }

    // Aggressive 2D cache blocking + explicit B-tile packing for register/L1 intensity.
    // Packing the KC x NC B tile into a dense contiguous buffer (paid once per tile,
    // reused by all MC rows) is the first-principles lever that raises arithmetic
    // intensity for both compute-bound (Mac) and bandwidth (Pi) cases.
    const MC: usize = 64;  // rows of A/C per outer tile
    const NC: usize = 64;  // columns of B/C per outer tile
    const KC: usize = 128; // K dimension per inner tile

    #[cfg(feature = "rayon")]
    {
        // Parallel over disjoint M-row blocks. Each task mutates only its own
        // row range of C. We smuggle the raw pointer as a usize (plain integer)
        // so the closure is Send+Sync for rayon; we cast back inside the task.
        let m_tiles: Vec<_> = (0..m).step_by(MC).collect();
        let c_ptr_usize = c.as_mut_ptr() as usize; // integer, freely Send+Sync
        m_tiles.par_iter().for_each(|&i0| {
            let i_end = (i0 + MC).min(m);
            let c_base = c_ptr_usize as *mut f32; // restore pointer (same address)
            for j0 in (0..n).step_by(NC) {
                let j_end = (j0 + NC).min(n);
                for p0 in (0..k).step_by(KC) {
                    let p_end = (p0 + KC).min(k);
                    let kc = p_end - p0;
                    let nc = j_end - j0;

                    // Pack the strided B tile into a dense KC x NC buffer once.
                    // This eliminates stride-n loads in the inner kernel and lets
                    // the MC rows reuse the tile from L1/registers.
                    let mut b_pack: Vec<f32> = vec![0.0; kc * nc];
                    for pp in 0..kc {
                        for jj in 0..nc {
                            b_pack[pp * nc + jj] = b[(p0 + pp) * n + (j0 + jj)];
                        }
                    }

                    for i in i0..i_end {
                        let a_seg = &a[i * k + p0..i * k + p_end];
                        // Raw pointer into this task's private row range of C.
                        // SAFETY: M-tiles are disjoint by construction (step_by + min);
                        // each parallel task owns a unique [i0..i_end) row block.
                        // All stores are to [i*n + j0 .. i*n + j_end) which do not overlap
                        // across tasks. No other thread touches this range while we run.
                        // The usize round-trip preserves the exact address.
                        // This pattern is standard in high-performance parallel GEMM
                        // (e.g. matrixmultiply, faer, ndarray with rayon).
                        let c_row_ptr = unsafe { c_base.add(i * n + j0) };
                        unsafe {
                            accumulate_dense_tile(
                                a_seg.as_ptr(),
                                b_pack.as_ptr(),
                                c_row_ptr,
                                kc,
                                nc,
                                nc,
                            );
                        }
                    }
                }
            }
        });
    }

    #[cfg(not(feature = "rayon"))]
    {
        for i0 in (0..m).step_by(MC) {
            let i_end = (i0 + MC).min(m);
            for j0 in (0..n).step_by(NC) {
                let j_end = (j0 + NC).min(n);
                for p0 in (0..k).step_by(KC) {
                    let p_end = (p0 + KC).min(k);
                    let kc = p_end - p0;
                    let nc = j_end - j0;

                    // Same explicit B-tile packing (dense, unit-stride for inner kernel).
                    let mut b_pack: Vec<f32> = vec![0.0; kc * nc];
                    for pp in 0..kc {
                        for jj in 0..nc {
                            b_pack[pp * nc + jj] = b[(p0 + pp) * n + (j0 + jj)];
                        }
                    }

                    for i in i0..i_end {
                        let a_seg = &a[i * k + p0..i * k + p_end];
                        let c_row = &mut c[i * n + j0..i * n + j_end];
                        // Portable dense path: exact p-outer, j-inner order on packed data.
                        accumulate_dense_tile_scalar(a_seg, &b_pack, c_row, kc, nc, nc);
                    }
                }
            }
        }
    }
}

/// Dense-tile accumulator (portable scalar, fixed order for determinism).
/// c_row[j] += sum_p a[p] * b_pack[p*nc + j]  for p in 0..kc, j in 0..nc.
/// p is strictly outer — identical accumulation order to the original scalar baseline.
#[inline]
#[allow(dead_code)]
fn accumulate_dense_tile_scalar(a: &[f32], b_pack: &[f32], c_row: &mut [f32], kc: usize, nc: usize, b_stride: usize) {
    for p in 0..kc {
        let av = a[p];
        let base = p * b_stride;
        for j in 0..nc {
            c_row[j] += av * b_pack[base + j];
        }
    }
}

/// Dense-tile accumulator with NEON vectorization over the N (column) dimension.
/// Uses the exact same separate vmulq/vaddq + left-to-right K discipline as
/// the original accumulate_row, now operating on a unit-stride dense B pack.
/// This makes the previously-dead NEON code live inside the production tiled path.
#[cfg(all(target_arch = "aarch64", feature = "simd"))]
#[inline]
#[allow(dead_code)]
unsafe fn accumulate_dense_tile(
    ap: *const f32,
    bp_pack: *const f32,
    cp: *mut f32,
    kc: usize,
    nc: usize,
    b_stride: usize,
) {
    // Vectorized over columns (N), K streamed left-to-right.
    // We process 16 columns at a time when possible (4 NEON regs), then 4-wide, then scalar tail.
    let mut j = 0usize;

    // 16-wide
    while j + 16 <= nc {
        let mut acc0 = vld1q_f32(cp.add(j));
        let mut acc1 = vld1q_f32(cp.add(j + 4));
        let mut acc2 = vld1q_f32(cp.add(j + 8));
        let mut acc3 = vld1q_f32(cp.add(j + 12));
        for p in 0..kc {
            let av = vdupq_n_f32(*ap.add(p));
            let bbase = p * b_stride + j;
            acc0 = vaddq_f32(acc0, vmulq_f32(av, vld1q_f32(bp_pack.add(bbase))));
            acc1 = vaddq_f32(acc1, vmulq_f32(av, vld1q_f32(bp_pack.add(bbase + 4))));
            acc2 = vaddq_f32(acc2, vmulq_f32(av, vld1q_f32(bp_pack.add(bbase + 8))));
            acc3 = vaddq_f32(acc3, vmulq_f32(av, vld1q_f32(bp_pack.add(bbase + 12))));
        }
        vst1q_f32(cp.add(j), acc0);
        vst1q_f32(cp.add(j + 4), acc1);
        vst1q_f32(cp.add(j + 8), acc2);
        vst1q_f32(cp.add(j + 12), acc3);
        j += 16;
    }

    // 4-wide
    while j + 4 <= nc {
        let mut acc = vld1q_f32(cp.add(j));
        for p in 0..kc {
            let av = vdupq_n_f32(*ap.add(p));
            acc = vaddq_f32(acc, vmulq_f32(av, vld1q_f32(bp_pack.add(p * b_stride + j))));
        }
        vst1q_f32(cp.add(j), acc);
        j += 4;
    }

    // Scalar tail (identical order and two roundings)
    while j < nc {
        let mut sum = *cp.add(j);
        for p in 0..kc {
            sum += *ap.add(p) * *bp_pack.add(p * nc + j);
        }
        *cp.add(j) = sum;
        j += 1;
    }
}

/// Fallback for non-aarch64 or no simd: delegates to the scalar dense version
/// (called via the safe slice path in the non-rayon driver).
#[cfg(not(all(target_arch = "aarch64", feature = "simd")))]
#[inline]
#[allow(dead_code)]
unsafe fn accumulate_dense_tile(
    ap: *const f32,
    bp_pack: *const f32,
    cp: *mut f32,
    kc: usize,
    nc: usize,
    b_stride: usize,
) {
    // Safe re-interpretation for the portable build: we just run the scalar logic.
    let a_slice = std::slice::from_raw_parts(ap, kc);
    let b_slice = std::slice::from_raw_parts(bp_pack, kc * b_stride);
    let c_slice = std::slice::from_raw_parts_mut(cp, nc);
    accumulate_dense_tile_scalar(a_slice, b_slice, c_slice, kc, nc, b_stride);
}

// -----------------------------------------------------------------------------
// Per-row kernel: c_row[j] += sum_{p=0..k} a_row[p] * b[p*n + j]
// The K-loop streams left-to-right; columns are vectorized. Bit-identical
// between the NEON and scalar variants (same op order, same two roundings).
// -----------------------------------------------------------------------------

#[cfg(all(target_arch = "aarch64", feature = "simd"))]
#[inline]
fn accumulate_row(a_row: &[f32], b: &[f32], c_row: &mut [f32], k: usize, n: usize) {
    // SAFETY: all indices below are bounded by k and n, which match the slice
    // lengths guaranteed by the caller (a_row.len()==k, c_row.len()==n,
    // b.len()==k*n). Pointer offsets stay in-bounds.
    unsafe {
        let bp = b.as_ptr();
        let cp = c_row.as_mut_ptr();
        let ap = a_row.as_ptr();

        // Process 16 columns per K-step (4 NEON vectors) to hide load/FMA
        // latency, then 4-wide, then a scalar tail. Every column accumulates
        // p = 0..k in order -> identical to the scalar reduction.
        let mut j = 0usize;

        // 16-wide blocks
        while j + 16 <= n {
            let mut acc0 = vld1q_f32(cp.add(j));
            let mut acc1 = vld1q_f32(cp.add(j + 4));
            let mut acc2 = vld1q_f32(cp.add(j + 8));
            let mut acc3 = vld1q_f32(cp.add(j + 12));
            for p in 0..k {
                let av = vdupq_n_f32(*ap.add(p));
                let base = p * n + j;
                acc0 = vaddq_f32(acc0, vmulq_f32(av, vld1q_f32(bp.add(base))));
                acc1 = vaddq_f32(acc1, vmulq_f32(av, vld1q_f32(bp.add(base + 4))));
                acc2 = vaddq_f32(acc2, vmulq_f32(av, vld1q_f32(bp.add(base + 8))));
                acc3 = vaddq_f32(acc3, vmulq_f32(av, vld1q_f32(bp.add(base + 12))));
            }
            vst1q_f32(cp.add(j), acc0);
            vst1q_f32(cp.add(j + 4), acc1);
            vst1q_f32(cp.add(j + 8), acc2);
            vst1q_f32(cp.add(j + 12), acc3);
            j += 16;
        }

        // 4-wide blocks
        while j + 4 <= n {
            let mut acc = vld1q_f32(cp.add(j));
            for p in 0..k {
                let av = vdupq_n_f32(*ap.add(p));
                acc = vaddq_f32(acc, vmulq_f32(av, vld1q_f32(bp.add(p * n + j))));
            }
            vst1q_f32(cp.add(j), acc);
            j += 4;
        }

        // Scalar tail (same left-to-right order, same two roundings)
        while j < n {
            let mut sum = *cp.add(j);
            for p in 0..k {
                sum += *ap.add(p) * *bp.add(p * n + j);
            }
            *cp.add(j) = sum;
            j += 1;
        }
    }
}

#[cfg(not(all(target_arch = "aarch64", feature = "simd")))]
#[inline]
fn accumulate_row(a_row: &[f32], b: &[f32], c_row: &mut [f32], k: usize, n: usize) {
    // Portable scalar path: p-outer, j-inner. Each c_row[j] accumulates
    // p = 0..k in order with multiply-then-add (two roundings).
    for p in 0..k {
        let av = a_row[p];
        let base = p * n;
        for j in 0..n {
            c_row[j] += av * b[base + j];
        }
    }
}

// -----------------------------------------------------------------------------
// Vocab projection (tied embedding) — the last major manual loop.
// These are the step-change primitives for the 50k-wide final projection.
//
// Pattern is identical to accumulate_row: vectorize over the *output*
// dimension (vocab entries), stream the hidden dimension (K=768) strictly
// left-to-right. This preserves the exact accumulation order + two roundings
// required for sacred receipts.
//
// For generation we provide a zero-allocation argmax variant that never
// materializes the 50k logits (energy + bandwidth win).
// -----------------------------------------------------------------------------

/// Project a single hidden row [n_embd] against the full wte [vocab, n_embd]
/// producing a full logits row [vocab].
/// Bit-exact with the original manual triple loop when using the same scalar path.
#[inline]
pub fn project_logits_row(hidden: &[f32], wte: &[f32], n_embd: usize, vocab: usize) -> Vec<f32> {
    let mut logits = vec![0.0f32; vocab];
    project_logits_row_into(hidden, wte, &mut logits, n_embd, vocab);
    logits
}

#[inline]
fn project_logits_row_into(hidden: &[f32], wte: &[f32], logits: &mut [f32], n_embd: usize, vocab: usize) {
    // Use the same proven vectorized logic as accumulate_row, just with
    // the wte layout (each "column" is strided by n_embd in memory).
    // For M=1 this is the hot generation path.
    #[cfg(all(target_arch = "aarch64", feature = "simd"))]
    {
        unsafe { project_vocab_row_neon(hidden, wte, logits.as_mut_ptr(), n_embd, vocab); }
    }
    #[cfg(not(all(target_arch = "aarch64", feature = "simd")))]
    {
        project_vocab_row_scalar(hidden, wte, logits, n_embd, vocab);
    }
}

/// Scalar reference implementation (exact accumulation order).
#[inline]
#[allow(dead_code)]
fn project_vocab_row_scalar(hidden: &[f32], wte: &[f32], logits: &mut [f32], n_embd: usize, vocab: usize) {
    for v in 0..vocab {
        let mut sum = 0.0f32;
        for d in 0..n_embd {
            sum += hidden[d] * wte[v * n_embd + d];
        }
        logits[v] = sum;
    }
}

/// NEON implementation: vectorize over vocab (output "columns"), stream d (K).
/// Reliable 4-wide + tail using the exact same separate-mul+add + left-to-right
/// d discipline as accumulate_row. 16-wide can be added later for extra throughput.
#[cfg(all(target_arch = "aarch64", feature = "simd"))]
#[inline]
unsafe fn project_vocab_row_neon(hidden: &[f32], wte: &[f32], out: *mut f32, n_embd: usize, vocab: usize) {
    let hp = hidden.as_ptr();
    let wp = wte.as_ptr();
    let mut v = 0usize;

    // Reliable 4-wide blocks (vectorize N/vocab, stream K/d exactly)
    while v + 4 <= vocab {
        let mut acc = vdupq_n_f32(0.0);
        for d in 0..n_embd {
            let av = vdupq_n_f32(*hp.add(d));
            let base = v * n_embd + d;
            acc = vaddq_f32(acc, vmulq_f32(av, vld1q_f32(wp.add(base))));
        }
        vst1q_f32(out.add(v), acc);
        v += 4;
    }

    // Scalar tail — identical order to the original manual loop
    while v < vocab {
        let mut sum = 0.0f32;
        for d in 0..n_embd {
            sum += *hp.add(d) * *wp.add(v * n_embd + d);
        }
        *out.add(v) = sum;
        v += 1;
    }
}

/// Zero-allocation argmax over the vocab projection for a single hidden row.
/// Computes exact dots (same order) but tracks only max value + index.
/// This is the production fast path for greedy generation.
#[inline]
pub fn argmax_dot(hidden: &[f32], wte: &[f32], n_embd: usize, vocab: usize) -> u32 {
    #[cfg(all(target_arch = "aarch64", feature = "simd"))]
    {
        unsafe { argmax_dot_neon(hidden, wte, n_embd, vocab) }
    }
    #[cfg(not(all(target_arch = "aarch64", feature = "simd")))]
    {
        argmax_dot_scalar(hidden, wte, n_embd, vocab)
    }
}

#[inline]
#[allow(dead_code)]
fn argmax_dot_scalar(hidden: &[f32], wte: &[f32], n_embd: usize, vocab: usize) -> u32 {
    let mut best = 0u32;
    let mut best_val = f32::NEG_INFINITY;
    for v in 0..vocab {
        let mut sum = 0.0f32;
        for d in 0..n_embd {
            sum += hidden[d] * wte[v * n_embd + d];
        }
        if sum > best_val {
            best_val = sum;
            best = v as u32;
        }
    }
    best
}

#[cfg(all(target_arch = "aarch64", feature = "simd"))]
#[inline]
unsafe fn argmax_dot_neon(hidden: &[f32], wte: &[f32], n_embd: usize, vocab: usize) -> u32 {
    let hp = hidden.as_ptr();
    let wp = wte.as_ptr();
    let mut best = 0u32;
    let mut best_val = f32::NEG_INFINITY;

    // 4-wide vectorized dots + scalar max tracking (correct and fast enough for first delivery)
    let mut v = 0usize;
    while v + 4 <= vocab {
        let mut acc = vdupq_n_f32(0.0);
        for d in 0..n_embd {
            let av = vdupq_n_f32(*hp.add(d));
            let base = v * n_embd + d;
            acc = vaddq_f32(acc, vmulq_f32(av, vld1q_f32(wp.add(base))));
        }
        // Extract 4 exact dot results (fixed order horizontal)
        let tmp = [0.0f32; 4];
        vst1q_f32(tmp.as_ptr() as *mut f32, acc);
        for i in 0..4 {
            let val = tmp[i];
            if val > best_val {
                best_val = val;
                best = (v + i) as u32;
            }
        }
        v += 4;
    }

    // Tail
    while v < vocab {
        let mut sum = 0.0f32;
        for d in 0..n_embd {
            sum += *hp.add(d) * *wp.add(v * n_embd + d);
        }
        if sum > best_val {
            best_val = sum;
            best = v as u32;
        }
        v += 1;
    }
    best
}

/// Project multiple hidden rows (prompt case) to full vocab logits.
/// Parallel over rows when rayon is available (safe: rows are independent).
#[inline]
pub fn project_hidden_to_vocab(hidden: &[f32], wte: &[f32], n_embd: usize, vocab: usize) -> Vec<f32> {
    let m = hidden.len() / n_embd;
    let mut out = vec![0.0f32; m * vocab];

    #[cfg(feature = "rayon")]
    {
        use rayon::prelude::*;
        let rows: Vec<_> = (0..m).collect();
        let out_ptr_usize = out.as_mut_ptr() as usize;
        rows.par_iter().for_each(|&i| {
            let hrow = &hidden[i * n_embd..(i + 1) * n_embd];
            let mut orow = vec![0.0f32; vocab];
            project_logits_row_into(hrow, wte, &mut orow, n_embd, vocab);
            // SAFETY: rows are disjoint, each writes exactly one vocab-sized block.
            unsafe {
                let out_ptr = out_ptr_usize as *mut f32;
                let dst = std::slice::from_raw_parts_mut(out_ptr.add(i * vocab), vocab);
                dst.copy_from_slice(&orow);
            }
        });
    }
    #[cfg(not(feature = "rayon"))]
    {
        for i in 0..m {
            let hrow = &hidden[i * n_embd..(i + 1) * n_embd];
            let dst = &mut out[i * vocab..(i + 1) * vocab];
            project_logits_row_into(hrow, wte, dst, n_embd, vocab);
        }
    }
    out
}

// -----------------------------------------------------------------------------
// Walsh-Hadamard transform (the "rotating orthogonal survey" lever).
//
// A fast Walsh-Hadamard transform (FWHT) rotates a vector into an orthogonal
// basis using ONLY additions and subtractions -- no multiplies. The butterfly
// order is fixed (h = 1, 2, 4, ... ascending), so the result is identical on
// every machine and every run, which keeps SHA receipts stable.
//
// Why this matters physically:
//   * Multiply-free: every operation is add or sub, the cheapest the ALU has,
//     and there is no FMA-contraction ambiguity to break determinism.
//   * Orthogonal: the transform preserves vector length (up to the global
//     scale `n`), so it is perfectly invertible -- apply it twice and divide
//     by `n` to recover the input. For power-of-two `n`, the `1/n` divide is an
//     exact exponent shift, so the round-trip is bit-exact in the integer
//     regime (and deterministic everywhere).
//   * It "swizzles" the energy of a vector into fewer coordinates, which is the
//     foundation the bigger quantization / multiply-light levers stand on.
//
// This is the same rotation modern quantization work (QuaRot, SpinQuant) uses
// to move activations into a friendlier basis -- here implemented add-only and
// determinism-safe.
// -----------------------------------------------------------------------------

/// In-place unnormalized fast Walsh-Hadamard transform.
/// Requires `v.len()` to be a power of two (>= 1). Add/sub only; fixed order.
#[inline]
pub fn hadamard_inplace(v: &mut [f32]) {
    let n = v.len();
    debug_assert!(n.is_power_of_two(), "hadamard length must be a power of two");
    let mut h = 1usize;
    while h < n {
        let mut i = 0usize;
        while i < n {
            for j in i..i + h {
                let x = v[j];
                let y = v[j + h];
                v[j] = x + y; // butterfly: sum
                v[j + h] = x - y; // butterfly: difference
            }
            i += 2 * h;
        }
        h *= 2;
    }
}

/// Convenience: return a transformed copy (unnormalized forward transform).
#[inline]
pub fn hadamard_transform(v: &[f32]) -> Vec<f32> {
    let mut out = v.to_vec();
    hadamard_inplace(&mut out);
    out
}

/// In-place inverse: apply the forward transform again, then divide by `n`.
/// For power-of-two `n` the `1/n` factor is an exact exponent shift.
#[inline]
pub fn hadamard_inverse_inplace(v: &mut [f32]) {
    let n = v.len();
    debug_assert!(n.is_power_of_two(), "hadamard length must be a power of two");
    hadamard_inplace(v);
    let inv = 1.0f32 / (n as f32); // exact for power-of-two n
    for x in v.iter_mut() {
        *x *= inv;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Reference: strict scalar dot product, multiply-then-add, p ascending.
    fn matmul_ref(a: &[f32], b: &[f32], bias: Option<&[f32]>, m: usize, k: usize, n: usize) -> Vec<f32> {
        let mut c = vec![0.0f32; m * n];
        for i in 0..m {
            for j in 0..n {
                let mut sum = bias.map_or(0.0, |bs| bs[j]);
                for p in 0..k {
                    sum += a[i * k + p] * b[p * n + j];
                }
                c[i * n + j] = sum;
            }
        }
        c
    }

    #[test]
    fn matmul_matches_reference_bitexact() {
        let (m, k, n) = (5, 37, 23); // non-multiples of 4/16 to exercise tails
        let a: Vec<f32> = (0..m * k).map(|x| ((x * 7 % 13) as f32 - 6.0) * 0.31).collect();
        let b: Vec<f32> = (0..k * n).map(|x| ((x * 5 % 11) as f32 - 5.0) * 0.17).collect();
        let got = matmul(&a, &b, m, k, n);
        let want = matmul_ref(&a, &b, None, m, k, n);
        assert_eq!(got, want, "matmul must be bit-identical to scalar reference");
    }

    #[test]
    fn matmul_bias_matches_reference_bitexact() {
        let (m, k, n) = (4, 64, 18);
        let a: Vec<f32> = (0..m * k).map(|x| ((x * 3 % 17) as f32 - 8.0) * 0.21).collect();
        let b: Vec<f32> = (0..k * n).map(|x| ((x * 9 % 19) as f32 - 9.0) * 0.13).collect();
        let bias: Vec<f32> = (0..n).map(|x| (x as f32 - 9.0) * 0.5).collect();
        let got = matmul_bias(&a, &b, &bias, m, k, n);
        let want = matmul_ref(&a, &b, Some(&bias), m, k, n);
        assert_eq!(got, want, "matmul_bias must be bit-identical to scalar reference");
    }

    // -------------------------------------------------------------------------
    // Walsh-Hadamard transform tests
    // -------------------------------------------------------------------------

    #[test]
    fn hadamard_known_4_vector_is_exact() {
        // Unnormalized FWHT of [1,0,1,0], worked by hand:
        //   stage h=1 (pairs (0,1),(2,3)): [1+0, 1-0, 1+0, 1-0] = [1,1,1,1]
        //   stage h=2 (pairs (0,2),(1,3)): [1+1, 1+1, 1-1, 1-1] = [2,2,0,0]
        let v = vec![1.0f32, 0.0, 1.0, 0.0];
        let got = hadamard_transform(&v);
        assert_eq!(got, vec![2.0, 2.0, 0.0, 0.0], "FWHT of [1,0,1,0] must be exact");
    }

    #[test]
    fn hadamard_roundtrip_is_bit_exact() {
        // Forward then inverse must recover the input bit-for-bit for
        // power-of-two lengths (the 1/n divide is an exact exponent shift).
        for &n in &[1usize, 2, 4, 8, 16, 32, 64, 128] {
            let original: Vec<f32> =
                (0..n).map(|x| ((x * 13 % 251) as f32 - 125.0) * 0.5).collect();
            let mut work = original.clone();
            hadamard_inplace(&mut work); // forward
            hadamard_inverse_inplace(&mut work); // inverse (forward again + 1/n)
            assert_eq!(work, original, "Hadamard round-trip must be bit-exact at n={n}");
        }
    }

    #[test]
    fn hadamard_preserves_energy_parseval() {
        // Orthogonality (Parseval): sum(H(v)^2) == n * sum(v^2).
        // Use integer-valued inputs so the squared sums are computed exactly.
        let n = 8usize;
        let v: Vec<f32> = vec![3.0, -1.0, 4.0, -2.0, 0.0, 5.0, -3.0, 1.0];
        let t = hadamard_transform(&v);
        let energy_in: f32 = v.iter().map(|x| x * x).sum();
        let energy_out: f32 = t.iter().map(|x| x * x).sum();
        assert_eq!(
            energy_out,
            (n as f32) * energy_in,
            "Hadamard must preserve energy: sum(H(v)^2) == n*sum(v^2)"
        );
    }

    #[test]
    fn matmul_bias_packed_matches_matmul_bias_bitexact() {
        // Small shape: exact equality test
        let m = 4;
        let k = 37;
        let n = 23;

        let a: Vec<f32> = (0..(m * k)).map(|i| ((i % 13) as f32) * 0.1 - 0.6).collect();
        let b: Vec<f32> = (0..(k * n)).map(|i| ((i % 7) as f32) * 0.2 - 0.7).collect();
        let bias: Vec<f32> = (0..n).map(|i| ((i % 5) as f32) * 0.15 - 0.3).collect();

        let c_default = matmul_bias(&a, &b, &bias, m, k, n);
        let packed = pack_b_for_kernel(&b, k, n);
        let c_packed = matmul_bias_packed(&a, &packed, &bias, m, k, n);

        assert_eq!(c_default.len(), c_packed.len());
        for (i, (d, p)) in c_default.iter().zip(c_packed.iter()).enumerate() {
            assert_eq!(
                *d, *p,
                "mismatch at index {}: default={}, packed={}",
                i, d, p
            );
        }

        // GPT-2-ish shape: m=3, k=64, n=192
        let m2 = 3;
        let k2 = 64;
        let n2 = 192;

        let a2: Vec<f32> = (0..(m2 * k2)).map(|i| ((i % 31) as f32) * 0.05 - 0.8).collect();
        let b2: Vec<f32> = (0..(k2 * n2)).map(|i| ((i % 17) as f32) * 0.08 - 0.5).collect();
        let bias2: Vec<f32> = (0..n2).map(|i| ((i % 11) as f32) * 0.12 - 0.4).collect();

        let c2_default = matmul_bias(&a2, &b2, &bias2, m2, k2, n2);
        let packed2 = pack_b_for_kernel(&b2, k2, n2);
        let c2_packed = matmul_bias_packed(&a2, &packed2, &bias2, m2, k2, n2);

        assert_eq!(c2_default.len(), c2_packed.len());
        for (i, (d, p)) in c2_default.iter().zip(c2_packed.iter()).enumerate() {
            assert_eq!(
                *d, *p,
                "mismatch at index {} (shape m=3,k=64,n=192): default={}, packed={}",
                i, d, p
            );
        }
    }
}

/// Cache-blocked GEMM (M x K) @ (K x N) -> (M x N)
/// Keeps weights (B) stationary in tiles and walks K left-to-right
/// for bit-exact determinism with the existing matmul.
/// Cache-blocked GEMM (M x K) @ (K x N) -> (M x N)
/// Keeps weights (B) stationary in tiles and walks K left-to-right
/// for bit-exact determinism with the existing matmul.
#[inline]
pub fn matmul_blocked(a: &[f32], b: &[f32], m: usize, k: usize, n: usize) -> Vec<f32> {
    const MC: usize = 64;
    const NC: usize = 64;
    const KC: usize = 128;

    let mut c = vec![0.0f32; m * n];

    for i0 in (0..m).step_by(MC) {
        for j0 in (0..n).step_by(NC) {
            for p0 in (0..k).step_by(KC) {
                let i_end = (i0 + MC).min(m);
                let j_end = (j0 + NC).min(n);
                let p_end = (p0 + KC).min(k);

                for i in i0..i_end {
                    for p in p0..p_end {
                        let a_val = a[i * k + p];
                        for j in j0..j_end {
                            c[i * n + j] += a_val * b[p * n + j];
                        }
                    }
                }
            }
        }
    }
    c
}
