//! Scaling sweep: demonstrates O(N) Waller memory vs O(N^2) standard attention.
//!
//! Runs the single-pass Waller Operator attention across a range of sequence
//! lengths and emits a clean CSV to stdout. Each row reports, for one sequence
//! length N:
//!   - standard attention memory (the N*N score matrix, in bytes) -> O(N^2)
//!   - Waller Operator memory (Q,K,V streaming buffers, in bytes) -> O(N)
//!   - the memory reduction factor (standard / waller)
//!   - wall-clock time to attend over the full context from the last position
//!
//! Pipe the CSV into a file to plot the linear-vs-quadratic story:
//!     cargo run --release --example scaling_sweep > scaling.csv
//!
//! Then chart `standard_mem_bytes` and `waller_mem_bytes` against `seq_len`:
//! standard climbs as a parabola, Waller climbs as a straight line.

#![allow(clippy::needless_range_loop)] // Intentional indexing in the micro-benchmark kernels for clarity and performance.

use std::time::Instant;

/// Run one single-pass Waller attention over `seq_len` tokens and return the
/// wall-clock duration in milliseconds. Mirrors the inline kernel used in
/// `production_demo.rs` so the timing reflects the real streaming path.
fn run_waller(seq_len: usize, head_dim: usize) -> f64 {
    let scale = 1.0 / (head_dim as f32).sqrt();

    // Deterministic, reproducible inputs (same generators as production_demo).
    let q: Vec<f32> = (0..seq_len * head_dim)
        .map(|i| ((i as f32) * 0.01).sin() * 0.1)
        .collect();
    let k: Vec<f32> = (0..seq_len * head_dim)
        .map(|i| ((i as f32) * 0.01).cos() * 0.1)
        .collect();
    let v: Vec<f32> = (0..seq_len * head_dim)
        .map(|i| (((i % 1000) as f32) * 0.1).sin())
        .collect();

    let start = Instant::now();

    // Attend from the final position over the entire context — single pass,
    // online-softmax rescaling, O(head_dim) running state, no N*N matrix.
    let pos = seq_len - 1;
    let mut max_score: f32 = f32::NEG_INFINITY;
    let mut sum_exp: f32 = 0.0;
    let mut acc = vec![0.0f32; head_dim];

    for j in 0..=pos {
        let mut score: f32 = 0.0;
        for d in 0..head_dim {
            score += q[pos * head_dim + d] * k[j * head_dim + d];
        }
        score *= scale;

        if score > max_score {
            let correction = (max_score - score).exp();
            sum_exp = sum_exp * correction + 1.0;
            for d in 0..head_dim {
                acc[d] *= correction;
            }
            max_score = score;
        } else {
            sum_exp += (score - max_score).exp();
        }

        let weight = (score - max_score).exp();
        for d in 0..head_dim {
            acc[d] += weight * v[j * head_dim + d];
        }
    }

    let inv_sum = 1.0 / sum_exp;
    for d in 0..head_dim {
        acc[d] *= inv_sum;
    }

    // Touch the accumulator so the optimizer cannot delete the whole loop.
    std::hint::black_box(&acc);

    start.elapsed().as_secs_f64() * 1000.0
}

fn main() {
    let head_dim: usize = 128;

    // Sequence lengths to sweep. Powers of two make the parabola obvious.
    let seq_lengths: [usize; 7] = [128, 512, 2_048, 8_192, 32_768, 65_536, 131_072];

    // CSV header.
    println!("seq_len,standard_mem_bytes,waller_mem_bytes,memory_reduction_x,waller_time_ms");

    for &n in seq_lengths.iter() {
        // Standard attention must materialize the full N*N score matrix: O(N^2).
        let standard_mem = (n as u128) * (n as u128) * 4;
        // Waller streams Q, K, V only: 3 * N * head_dim floats: O(N).
        let waller_mem = (n as u128) * (head_dim as u128) * 4 * 3;
        let reduction = standard_mem as f64 / waller_mem as f64;

        let time_ms = run_waller(n, head_dim);

        println!(
            "{},{},{},{:.1},{:.3}",
            n, standard_mem, waller_mem, reduction, time_ms
        );
    }
}
