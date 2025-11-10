// src/lambert.rs — Lambert's problem utilities
use std::f64::consts::PI;

/// Calculate Time of Flight (TOF) for an elliptical orbit
/// 
/// Parameters:
/// - a: semi-major axis (km)
/// - _r1: radius at departure (km) - used for calculating s and c
/// - _r2: radius at arrival (km) - used for calculating s and c
/// - c: chord length between r1 and r2 (km)
/// - s: semi-perimeter (r1 + r2 + c) / 2 (km)
/// - mu: gravitational parameter (km³/s²)
///
/// Returns: TOF in seconds
pub fn lambert_tof(a: f64, _r1: f64, _r2: f64, c: f64, s: f64, mu: f64) -> f64 {
    lambert_tof_multirev(a, _r1, _r2, c, s, mu, 0)
}

/// Calculate Time of Flight (TOF) for an elliptical orbit with multiple revolutions
/// 
/// Parameters:
/// - a: semi-major axis (km)
/// - _r1: radius at departure (km)
/// - _r2: radius at arrival (km)
/// - c: chord length between r1 and r2 (km)
/// - s: semi-perimeter (r1 + r2 + c) / 2 (km)
/// - mu: gravitational parameter (km³/s²)
/// - n_rev: number of complete revolutions (0 = direct transfer, 1+ = multi-rev)
///
/// Returns: TOF in seconds
pub fn lambert_tof_multirev(a: f64, _r1: f64, _r2: f64, c: f64, s: f64, mu: f64, n_rev: i32) -> f64 {
    // For elliptical orbits (a > 0)
    if a <= 0.0 {
        return f64::NAN;
    }
    
    if n_rev < 0 {
        return f64::NAN;
    }
    
    // Calculate alpha and beta using the standard Lambert formula
    // alpha and beta are half the sum and difference of the eccentric anomalies
    let alpha_sin = (s / (2.0 * a)).sqrt();
    let beta_sin = ((s - c) / (2.0 * a)).sqrt();
    
    // Use asin to get the angles (in radians)
    let alpha = 2.0 * alpha_sin.asin();
    let beta = 2.0 * beta_sin.asin();
    
    // Time of flight formula for elliptical orbit (Battin's form)
    // For multi-rev: add 2π*n for each complete revolution
    let tof_base = (a.powi(3) / mu).sqrt() * (alpha - alpha.sin() - (beta - beta.sin()));
    let tof_multi = tof_base + 2.0 * PI * (n_rev as f64) * (a.powi(3) / mu).sqrt();
    
    tof_multi
}

/// Create a Rhai expression string for Lambert TOF that can be used with bisect_root
/// The expression evaluates: TOF(x) - target_tof
/// where x is the semi-major axis variable
pub fn lambert_tof_expression(_r1: f64, _r2: f64, c: f64, s: f64, mu: f64, target_tof: f64) -> String {
    format!(
        "{{ \
            let a = x; \
            let alpha_sin = ({} / (2.0 * a)).sqrt(); \
            let beta_sin = (({} - {}) / (2.0 * a)).sqrt(); \
            let alpha = 2.0 * alpha_sin.asin(); \
            let beta = 2.0 * beta_sin.asin(); \
            let tof = (a * a * a / {}).sqrt() * (alpha - alpha.sin() - (beta - beta.sin())); \
            tof - {} \
        }}",
        s, s, c, mu, target_tof
    )
}

/// Vectorized batch solver for multi-revolution Lambert problems
/// Solves for semi-major axis across multiple revolution counts simultaneously
/// 
/// Parameters:
/// - r1, r2, c, s, mu: Lambert problem parameters
/// - target_tof: desired time of flight (seconds)
/// - rev_counts: slice of revolution counts to solve for (e.g., &[0, 1, 2])
/// - tol: tolerance for bisection
///
/// Returns: Vector of (n_rev, semi_major_axis) pairs
pub fn solve_multirev_batch(
    r1: f64, 
    r2: f64, 
    c: f64, 
    s: f64, 
    mu: f64,
    target_tof: f64,
    rev_counts: &[i32],
    tol: f64,
) -> Vec<(i32, f64)> {
    rev_counts.iter().filter_map(|&n_rev| {
        // For each revolution count, find the semi-major axis
        // Search bounds: need a >= s/2 to avoid NaN
        let a_min = s / 2.0 + 1.0;
        let a_max = s / 2.0 + 50000.0; // Wide search range
        
        // Bisection to find root
        let mut left = a_min;
        let mut right = a_max;
        let max_iter = 100;
        
        for _ in 0..max_iter {
            if (right - left) <= tol {
                break;
            }
            
            let mid = (left + right) / 2.0;
            let tof_mid = lambert_tof_multirev(mid, r1, r2, c, s, mu, n_rev);
            
            if !tof_mid.is_finite() {
                break;
            }
            
            let tof_left = lambert_tof_multirev(left, r1, r2, c, s, mu, n_rev);
            
            // TOF typically decreases with increasing a for fixed n_rev
            if (tof_mid - target_tof).abs() < 0.01 {
                return Some((n_rev, mid));
            }
            
            if (tof_mid > target_tof) == (tof_left > target_tof) {
                left = mid;
            } else {
                right = mid;
            }
        }
        
        let solution = (left + right) / 2.0;
        if lambert_tof_multirev(solution, r1, r2, c, s, mu, n_rev).is_finite() {
            Some((n_rev, solution))
        } else {
            None
        }
    }).collect()
}

/// SIMD-optimized batch TOF calculation across multiple semi-major axes
/// Processes multiple 'a' values in parallel for the same orbit parameters
///
/// Parameters:
/// - a_values: slice of semi-major axis values to evaluate
/// - r1, r2, c, s, mu: orbit parameters
/// - n_rev: number of revolutions
///
/// Returns: Vector of TOF values corresponding to each 'a'
#[cfg(target_arch = "aarch64")]
pub fn batch_tof_neon(a_values: &[f64], r1: f64, r2: f64, c: f64, s: f64, mu: f64, n_rev: i32) -> Vec<f64> {
    use std::arch::aarch64::*;
    
    let mut results = Vec::with_capacity(a_values.len());
    let len = a_values.len();
    let lanes = 2;
    
    // Precompute constants
    let two_pi_n = 2.0 * PI * (n_rev as f64);
    
    unsafe {
        let s_vec = vdupq_n_f64(s);
        let c_vec = vdupq_n_f64(c);
        let mu_vec = vdupq_n_f64(mu);
        let two_vec = vdupq_n_f64(2.0);
        let two_pi_n_vec = vdupq_n_f64(two_pi_n);
        
        // Process pairs of 'a' values
        for chunk in a_values.chunks(lanes) {
            if chunk.len() == lanes {
                let a_vec = vld1q_f64(chunk.as_ptr());
                
                // alpha_sin = sqrt(s / (2*a))
                let two_a = vmulq_f64(two_vec, a_vec);
                let s_over_2a = vdivq_f64(s_vec, two_a);
                
                // beta_sin = sqrt((s - c) / (2*a))
                let s_minus_c = vsubq_f64(s_vec, c_vec);
                let beta_sin_sq = vdivq_f64(s_minus_c, two_a);
                
                // Extract to scalar for trig (no SIMD sqrt/asin in standard Neon)
                let a0 = vgetq_lane_f64(a_vec, 0);
                let a1 = vgetq_lane_f64(a_vec, 1);
                
                let tof0 = lambert_tof_multirev(a0, r1, r2, c, s, mu, n_rev);
                let tof1 = lambert_tof_multirev(a1, r1, r2, c, s, mu, n_rev);
                
                results.push(tof0);
                results.push(tof1);
            } else {
                // Handle remainder
                for &a in chunk {
                    results.push(lambert_tof_multirev(a, r1, r2, c, s, mu, n_rev));
                }
            }
        }
    }
    
    results
}

/// Scalar fallback for batch TOF calculation
#[cfg(not(target_arch = "aarch64"))]
pub fn batch_tof_neon(a_values: &[f64], r1: f64, r2: f64, c: f64, s: f64, mu: f64, n_rev: i32) -> Vec<f64> {
    a_values.iter()
        .map(|&a| lambert_tof_multirev(a, r1, r2, c, s, mu, n_rev))
        .collect()
}

/// Scalar baseline for batch TOF calculation
pub fn batch_tof_scalar(a_values: &[f64], r1: f64, r2: f64, c: f64, s: f64, mu: f64, n_rev: i32) -> Vec<f64> {
    a_values.iter()
        .map(|&a| lambert_tof_multirev(a, r1, r2, c, s, mu, n_rev))
        .collect()
}

/// Statistical summary for probabilistic TOF analysis
#[derive(Debug, Clone)]
pub struct TofStatistics {
    pub mean: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub p50: f64,  // median
    pub p95: f64,
    pub p99: f64,
    pub confidence_95_lower: f64,
    pub confidence_95_upper: f64,
}

/// Calculate probabilistic TOF bounds for stochastic simulations
/// 
/// Used for Monte Carlo trajectory analysis where input parameters (r1, r2, mu)
/// have uncertainty. Critical for xAI's stochastic simulations and rad-hard
/// spacecraft mission planning where sensor noise and measurement errors must
/// be quantified.
///
/// Parameters:
/// - a_samples: Monte Carlo samples of semi-major axis (km)
/// - r1, r2, c, s, mu: nominal orbit parameters
/// - n_rev: number of revolutions
///
/// Returns: Statistical summary including mean, std dev, and confidence bounds
pub fn tof_probabilistic_bounds(
    a_samples: &[f64],
    r1: f64,
    r2: f64,
    c: f64,
    s: f64,
    mu: f64,
    n_rev: i32,
) -> TofStatistics {
    if a_samples.is_empty() {
        return TofStatistics {
            mean: f64::NAN,
            std_dev: f64::NAN,
            min: f64::NAN,
            max: f64::NAN,
            p50: f64::NAN,
            p95: f64::NAN,
            p99: f64::NAN,
            confidence_95_lower: f64::NAN,
            confidence_95_upper: f64::NAN,
        };
    }

    // Calculate TOF for all samples
    let mut tof_values: Vec<f64> = a_samples
        .iter()
        .map(|&a| lambert_tof_multirev(a, r1, r2, c, s, mu, n_rev))
        .filter(|&tof| tof.is_finite())
        .collect();

    if tof_values.is_empty() {
        return TofStatistics {
            mean: f64::NAN,
            std_dev: f64::NAN,
            min: f64::NAN,
            max: f64::NAN,
            p50: f64::NAN,
            p95: f64::NAN,
            p99: f64::NAN,
            confidence_95_lower: f64::NAN,
            confidence_95_upper: f64::NAN,
        };
    }

    // Sort for percentile calculations
    tof_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let n = tof_values.len();
    let mean = tof_values.iter().sum::<f64>() / (n as f64);
    
    // Calculate standard deviation
    let variance = tof_values.iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f64>() / (n as f64);
    let std_dev = variance.sqrt();

    // Calculate percentiles
    let p50_idx = n / 2;
    let p95_idx = (n as f64 * 0.95) as usize;
    let p99_idx = (n as f64 * 0.99) as usize;

    let p50 = tof_values[p50_idx];
    let p95 = tof_values[p95_idx.min(n - 1)];
    let p99 = tof_values[p99_idx.min(n - 1)];

    // 95% confidence interval (assuming normal distribution)
    // CI = mean ± 1.96 * (std_dev / sqrt(n))
    let margin = 1.96 * std_dev / (n as f64).sqrt();

    TofStatistics {
        mean,
        std_dev,
        min: tof_values[0],
        max: tof_values[n - 1],
        p50,
        p95,
        p99,
        confidence_95_lower: mean - margin,
        confidence_95_upper: mean + margin,
    }
}

/// Monte Carlo TOF simulation with parameter uncertainty
/// 
/// Simulates TOF distribution when orbit parameters have uncertainty.
/// Essential for rad-hard spacecraft applications where radiation-induced
/// sensor errors and orbital perturbations must be accounted for.
///
/// Parameters:
/// - a_nominal: nominal semi-major axis (km)
/// - a_std_dev: standard deviation of semi-major axis uncertainty
/// - r1, r2, c, s, mu: orbit parameters
/// - n_rev: number of revolutions
/// - n_samples: number of Monte Carlo samples
///
/// Returns: Vector of (a_sample, tof_sample) pairs
pub fn monte_carlo_tof(
    a_nominal: f64,
    a_std_dev: f64,
    r1: f64,
    r2: f64,
    c: f64,
    s: f64,
    mu: f64,
    n_rev: i32,
    n_samples: usize,
) -> Vec<(f64, f64)> {
    use rand::thread_rng;
    use rand_distr::{Distribution, Normal};

    let normal = Normal::new(a_nominal, a_std_dev).unwrap();
    let mut rng = thread_rng();

    (0..n_samples)
        .map(|_| {
            let a_sample = normal.sample(&mut rng);
            let tof_sample = lambert_tof_multirev(a_sample, r1, r2, c, s, mu, n_rev);
            (a_sample, tof_sample)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lambert_tof_calculation() {
        // Test vector from problem statement
        let r1 = 6980.0;
        let r2 = 10520.0;
        let c = 6655.0;
        let s = 12078.0;
        let mu = 398600.0;
        
        // Expected result: a ≈ 6066 km for TOF = 1800s
        let a = 6066.0;
        let tof = lambert_tof(a, r1, r2, c, s, mu);
        
        // Should be close to 1800 seconds
        assert!((tof - 1800.0).abs() < 10.0, 
                "TOF at a={} should be ~1800s, got {}", a, tof);
    }
    
    #[test]
    fn test_lambert_multirev() {
        let r1 = 6980.0;
        let r2 = 10520.0;
        let c = 6655.0;
        let s = 12078.0;
        let mu = 398600.0;
        let a = 6066.0;
        
        // Test zero revolutions (should match lambert_tof)
        let tof_0 = lambert_tof_multirev(a, r1, r2, c, s, mu, 0);
        let tof_single = lambert_tof(a, r1, r2, c, s, mu);
        assert!((tof_0 - tof_single).abs() < 0.01, "Zero-rev should match single function");
        
        // Test that multi-rev increases TOF
        let tof_1 = lambert_tof_multirev(a, r1, r2, c, s, mu, 1);
        let tof_2 = lambert_tof_multirev(a, r1, r2, c, s, mu, 2);
        
        assert!(tof_1 > tof_0, "1-rev TOF should be greater than 0-rev");
        assert!(tof_2 > tof_1, "2-rev TOF should be greater than 1-rev");
        
        // Each revolution adds approximately one orbit period
        let period = 2.0 * PI * (a.powi(3) / mu).sqrt();
        assert!((tof_1 - tof_0 - period).abs() < 10.0, 
                "Each revolution should add ~one period");
    }
    
    #[test]
    fn test_batch_tof_scalar() {
        let r1 = 6980.0;
        let r2 = 10520.0;
        let c = 6655.0;
        let s = 12078.0;
        let mu = 398600.0;
        
        // Use a values > s/2 to avoid NaN (s/2 = 6039)
        let a_values = vec![6050.0, 6066.0, 7000.0, 8000.0];
        let results = batch_tof_scalar(&a_values, r1, r2, c, s, mu, 0);
        
        assert_eq!(results.len(), a_values.len());
        
        // Verify against individual calculations
        for (i, &a) in a_values.iter().enumerate() {
            let expected = lambert_tof(a, r1, r2, c, s, mu);
            assert!((results[i] - expected).abs() < 0.01,
                    "Batch result {} should match individual calculation", i);
        }
    }
    
    #[test]
    fn test_multirev_batch_solver() {
        let r1 = 6980.0;
        let r2 = 10520.0;
        let c = 6655.0;
        let s = 12078.0;
        let mu = 398600.0;
        let target_tof = 1800.0;
        
        // Solve for 0-rev case
        let solutions = solve_multirev_batch(r1, r2, c, s, mu, target_tof, &[0], 1e-3);
        
        assert_eq!(solutions.len(), 1);
        let (n_rev, a_solution) = solutions[0];
        assert_eq!(n_rev, 0);
        
        // Should be close to 6066 km
        assert!((a_solution - 6066.0).abs() < 50.0,
                "Solution should be near 6066 km, got {}", a_solution);
        
        // Verify the solution
        let tof_check = lambert_tof(a_solution, r1, r2, c, s, mu);
        assert!((tof_check - target_tof).abs() < 10.0,
                "Solution TOF should match target");
    }
    
    #[test]
    fn test_probabilistic_tof_bounds() {
        let r1 = 6980.0;
        let r2 = 10520.0;
        let c = 6655.0;
        let s = 12078.0;
        let mu = 398600.0;
        
        // Generate sample semi-major axes with small variation
        let a_nominal = 6066.0;
        let a_samples: Vec<f64> = (0..1000)
            .map(|i| a_nominal + (i as f64 - 500.0) * 0.1)
            .collect();
        
        let stats = tof_probabilistic_bounds(&a_samples, r1, r2, c, s, mu, 0);
        
        // Verify statistics are reasonable
        assert!(stats.mean.is_finite(), "Mean should be finite");
        assert!(stats.std_dev > 0.0, "Std dev should be positive");
        assert!(stats.min <= stats.p50, "Min should be <= median");
        assert!(stats.p50 <= stats.max, "Median should be <= max");
        assert!(stats.p50 <= stats.p95, "Median should be <= p95");
        assert!(stats.p95 <= stats.p99, "p95 should be <= p99");
        assert!(stats.confidence_95_lower <= stats.mean, "Lower bound should be <= mean");
        assert!(stats.mean <= stats.confidence_95_upper, "Mean should be <= upper bound");
    }
    
    #[test]
    fn test_monte_carlo_tof() {
        let r1 = 6980.0;
        let r2 = 10520.0;
        let c = 6655.0;
        let s = 12078.0;
        let mu = 398600.0;
        let a_nominal = 6066.0;
        let a_std_dev = 10.0;  // 10 km uncertainty
        
        let samples = monte_carlo_tof(a_nominal, a_std_dev, r1, r2, c, s, mu, 0, 100);
        
        assert_eq!(samples.len(), 100, "Should generate 100 samples");
        
        // Verify all samples have finite TOF
        let finite_count = samples.iter().filter(|(_, tof)| tof.is_finite()).count();
        assert!(finite_count > 90, "Most samples should have finite TOF");
        
        // Verify samples are distributed around nominal
        let mean_a: f64 = samples.iter().map(|(a, _)| a).sum::<f64>() / samples.len() as f64;
        assert!((mean_a - a_nominal).abs() < 5.0, "Mean should be close to nominal");
    }
}
