// src/lambert.rs — Lambert's problem utilities

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
    // For elliptical orbits (a > 0)
    if a <= 0.0 {
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
    let tof = (a.powi(3) / mu).sqrt() * (alpha - alpha.sin() - (beta - beta.sin()));
    
    tof
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
}
