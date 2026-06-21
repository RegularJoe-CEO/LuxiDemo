//! Multiply-free kernel probe — the "slide rule" lever.
//!
//! GOAL: measure whether approximating float multiply with INTEGER ADDITION of
//! IEEE-754 bit patterns (Mitchell's logarithmic multiplication) buys us speed,
//! and what accuracy we pay for it versus the exact f32 reference.
//!
//! This is a STANDALONE probe. It touches NO production code path and produces
//! NO receipt. It exists purely to generate honest before/after numbers so we
//! can decide if the multiply-free path is worth wiring in.
//!
//! Run:  cargo run --release --example multiply_free_probe

use std::time::Instant;

/// Mitchell approximate multiply: treat the IEEE-754 bits as an integer (which
/// is approximately log2 of the value), ADD them, subtract one exponent bias,
/// reinterpret. Sign handled separately. No hardware multiply on the value path.
#[inline(always)]
fn approx_mul(x: f32, y: f32) -> f32 {
    if x == 0.0 || y == 0.0 {
        return 0.0;
    }
    let sign = (x.is_sign_negative() ^ y.is_sign_negative()) as u32;
    let bx = (x.abs().to_bits()) as i64;
    let by = (y.abs().to_bits()) as i64;
    // 0x3f800000 == bits of 1.0f32; subtracting one bias keeps the exponent sane.
    let mut bits = (bx + by - 0x3f80_0000) as u32;
    bits |= sign << 31;
    f32::from_bits(bits)
}

/// Exact f32 dot product (left-to-right, two roundings) — the reference.
#[inline(always)]
fn dot_exact(a: &[f32], b: &[f32]) -> f32 {
    let mut s = 0.0f32;
    for i in 0..a.len() {
        s += a[i] * b[i];
    }
    s
}

/// Multiply-free dot product: approx multiply, exact add.
#[inline(always)]
fn dot_approx(a: &[f32], b: &[f32]) -> f32 {
    let mut s = 0.0f32;
    for i in 0..a.len() {
        s += approx_mul(a[i], b[i]);
    }
    s
}

fn main() {
    // GPT-2-ish dimensions: matmul [M x K] @ [K x N].
    let m = 128usize; // rows (tokens)
    let k = 768usize; // inner dim (hidden)
    let n = 768usize; // output cols
    let iters = 20usize;

    // Deterministic pseudo-random inputs in a realistic range.
    let mut seed = 0x1234_5678u64;
    let mut rng = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        ((seed >> 40) as f32 / 16_777_216.0 - 0.5) * 2.0 // ~[-1, 1]
    };
    let a: Vec<f32> = (0..m * k).map(|_| rng()).collect();
    let b: Vec<f32> = (0..k * n).map(|_| rng()).collect();

    // Pre-transpose B columns into contiguous vectors for clean dot products.
    let mut bt = vec![0.0f32; n * k];
    for p in 0..k {
        for j in 0..n {
            bt[j * k + p] = b[p * n + j];
        }
    }

    // ---- Accuracy: compare every output element ----
    let mut max_abs_err = 0.0f32;
    let mut sum_abs_err = 0.0f64;
    let mut sum_abs_ref = 0.0f64;
    for i in 0..m {
        let a_row = &a[i * k..i * k + k];
        for j in 0..n {
            let bcol = &bt[j * k..j * k + k];
            let r = dot_exact(a_row, bcol);
            let x = dot_approx(a_row, bcol);
            let e = (r - x).abs();
            if e > max_abs_err {
                max_abs_err = e;
            }
            sum_abs_err += e as f64;
            sum_abs_ref += r.abs() as f64;
        }
    }
    let mean_rel_err = sum_abs_err / sum_abs_ref.max(1e-12);

    // ---- Timing: exact ----
    let mut sink = 0.0f32;
    let t0 = Instant::now();
    for _ in 0..iters {
        for i in 0..m {
            let a_row = &a[i * k..i * k + k];
            for j in 0..n {
                sink += dot_exact(a_row, &bt[j * k..j * k + k]);
            }
        }
    }
    let exact_ms = t0.elapsed().as_secs_f64() * 1000.0 / iters as f64;

    // ---- Timing: multiply-free ----
    let t1 = Instant::now();
    for _ in 0..iters {
        for i in 0..m {
            let a_row = &a[i * k..i * k + k];
            for j in 0..n {
                sink += dot_approx(a_row, &bt[j * k..j * k + k]);
            }
        }
    }
    let approx_ms = t1.elapsed().as_secs_f64() * 1000.0 / iters as f64;

    println!("MULTIPLY-FREE PROBE  (matmul {m}x{k} @ {k}x{n})");
    println!("  (sink={sink:.3})");
    println!("--- Accuracy vs exact f32 reference ---");
    println!("  max abs error   : {max_abs_err:.6}");
    println!("  mean rel error  : {:.4}%", mean_rel_err * 100.0);
    println!("--- Speed (per full matmul pass, avg of {iters}) ---");
    println!("  exact f32       : {exact_ms:.2} ms");
    println!("  multiply-free   : {approx_ms:.2} ms");
    println!("  speedup         : {:.2}x", exact_ms / approx_ms.max(1e-9));
}
