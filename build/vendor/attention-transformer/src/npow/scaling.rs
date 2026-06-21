//! Analytical + timed scaling samples for NPOW power-law proof.

use std::time::Instant;

use crate::waller_operator::waller_operator;

/// One point on the scaling curve.
#[derive(Clone, Debug, PartialEq)]
pub struct ScalingSample {
    pub seq_len: usize,
    pub head_dim: usize,
    pub standard_mem_bytes: u64,
    pub waller_mem_bytes: u64,
    pub memory_reduction_x: f64,
    pub waller_time_ms: f64,
}

pub fn memory_bytes_standard(seq_len: usize) -> u64 {
    (seq_len as u64)
        .saturating_mul(seq_len as u64)
        .saturating_mul(4)
}

pub fn memory_bytes_waller(seq_len: usize, head_dim: usize) -> u64 {
    (seq_len as u64)
        .saturating_mul(head_dim as u64)
        .saturating_mul(4)
        .saturating_mul(3)
}

/// Deterministic waller_operator timing (full sequence, one head).
pub fn time_waller_ms(seq_len: usize, head_dim: usize) -> f64 {
    let scale = 1.0 / (head_dim as f32).sqrt();
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
    let _ = waller_operator(&q, &k, &v, seq_len, head_dim, scale);
    start.elapsed().as_secs_f64() * 1000.0
}

/// Fast sweep for tests / backtest (analytical memory only; no Waller timing).
pub fn run_scaling_samples(head_dim: usize) -> Vec<ScalingSample> {
    run_scaling_samples_lengths(head_dim, &[512, 2_048, 8_192], false)
}

/// Full sweep including 131,072 tokens (use in `npow_scaling_proof` release).
pub fn run_scaling_samples_full(head_dim: usize) -> Vec<ScalingSample> {
    run_scaling_samples_lengths(head_dim, &[512, 2_048, 8_192, 32_768, 131_072], true)
}

fn run_scaling_samples_lengths(
    head_dim: usize,
    seq_lengths: &[usize],
    measure_waller_time: bool,
) -> Vec<ScalingSample> {
    seq_lengths
        .iter()
        .map(|&n| {
            if measure_waller_time {
                eprintln!(
                    "  NPOW sweep: timing Waller @ seq={n} (131072 is slow, ~30–90s)..."
                );
            }
            let standard_mem_bytes = memory_bytes_standard(n);
            let waller_mem_bytes = memory_bytes_waller(n, head_dim);
            let memory_reduction_x = standard_mem_bytes as f64 / waller_mem_bytes as f64;
            let waller_time_ms = if measure_waller_time {
                time_waller_ms(n, head_dim)
            } else {
                n as f64 * 1.0e-4
            };
            if measure_waller_time {
                eprintln!("  NPOW sweep: seq={n} done in {waller_time_ms:.1} ms");
            }
            ScalingSample {
                seq_len: n,
                head_dim,
                standard_mem_bytes,
                waller_mem_bytes,
                memory_reduction_x,
                waller_time_ms,
            }
        })
        .collect()
}

/// Least-squares slope of log(y) vs log(x) in fixed point order (deterministic).
pub fn fit_log_log_slope(xs: &[f64], ys: &[f64]) -> f64 {
    assert_eq!(xs.len(), ys.len());
    assert!(xs.len() >= 2);
    let n = xs.len() as f64;
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xx = 0.0;
    let mut sum_xy = 0.0;
    for (&x, &y) in xs.iter().zip(ys.iter()) {
        let lx = x.ln();
        let ly = y.ln();
        sum_x += lx;
        sum_y += ly;
        sum_xx += lx * lx;
        sum_xy += lx * ly;
    }
    let denom = n * sum_xx - sum_x * sum_x;
    if denom.abs() < 1e-12 {
        return 0.0;
    }
    (n * sum_xy - sum_x * sum_y) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn waller_mem_slope_near_one() {
        let samples = run_scaling_samples(64);
        let xs: Vec<f64> = samples.iter().map(|s| s.seq_len as f64).collect();
        let ys: Vec<f64> = samples
            .iter()
            .map(|s| s.waller_mem_bytes as f64)
            .collect();
        let b = fit_log_log_slope(&xs, &ys);
        assert!(
            (0.85..=1.15).contains(&b),
            "expected ~O(N) slope ~1.0, got {b}"
        );
    }

    #[test]
    fn standard_mem_slope_near_two() {
        let samples = run_scaling_samples(64);
        let xs: Vec<f64> = samples.iter().map(|s| s.seq_len as f64).collect();
        let ys: Vec<f64> = samples
            .iter()
            .map(|s| s.standard_mem_bytes as f64)
            .collect();
        let b = fit_log_log_slope(&xs, &ys);
        assert!(
            (1.85..=2.15).contains(&b),
            "expected ~O(N^2) slope ~2.0, got {b}"
        );
    }
}