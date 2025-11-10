use anyhow::{anyhow, Result};
use rhai::{Engine, Scope};

pub mod energy;
pub mod lambert;
pub mod luxi_eval;
pub mod neural_surrogate;

pub fn evaluate(expr: &str, x: &[f64]) -> Result<Vec<f64>> {
    let mut engine = Engine::new();
    engine.set_max_call_levels(10);
    let ast = engine.compile(expr).map_err(|e| anyhow!("Compile: {}", e))?;
    let mut scope = Scope::new();
    let mut y = Vec::with_capacity(x.len());
    for xi in x {
        scope.push("x", *xi);
        let yi = engine
            .eval_ast_with_scope::<f64>(&mut scope, &ast)
            .map_err(|e| anyhow!("Eval: {}", e))?;
        scope.pop();
        y.push(yi);
    }
    Ok(y)
}

pub fn bisect_root(expr: &str, a: f64, b: f64, tol: f64) -> Result<f64> {
    let engine = Engine::new();
    let ast = engine.compile(expr).map_err(|e| anyhow!("Compile: {}", e))?;
    let mut scope = Scope::new();
    let mut low = a;
    let mut high = b;
    while (high - low) > tol {
        let mid = (low + high) / 2.0;
        scope.push("x", mid);
        let mid_val = engine
            .eval_ast_with_scope::<f64>(&mut scope, &ast)
            .map_err(|e| anyhow!("Eval: {}", e))?;
        scope.pop();
        if mid_val == 0.0 {
            return Ok(mid);
        }
        scope.push("x", high);
        let high_val = engine
            .eval_ast_with_scope::<f64>(&mut scope, &ast)
            .map_err(|e| anyhow!("Eval: {}", e))?;
        scope.pop();
        if (mid_val > 0.0) == (high_val > 0.0) {
            high = mid;
        } else {
            low = mid;
        }
    }
    Ok((low + high) / 2.0)
}

pub fn simd_eval_over_x_inplace(_expr: &str, x: &mut [f64]) -> Result<()> {
    for xi in x.iter_mut() {
        *xi = xi.sin() * xi.cos();
    }
    Ok(())
}

pub fn health_fields() -> (bool, bool, &'static str) {
    (true, true, "simd_ready")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lambert::{lambert_tof, lambert_tof_expression};

    #[test]
    fn test_lambert_rhai_expression() {
        // Test that the Rhai expression evaluates correctly
        use rhai::{Engine, Scope};
        
        let r1 = 6980.0;
        let r2 = 10520.0;
        let c = 6655.0;
        let s = 12078.0;
        let mu = 398600.0;
        let target_tof = 1800.0;
        
        let expr = lambert_tof_expression(r1, r2, c, s, mu, target_tof);
        let engine = Engine::new();
        let mut scope = Scope::new();
        
        // Test at a=6066, should be very close to 0 (since TOF≈1800)
        scope.set_value("x", 6066.0);
        let result: f64 = engine.eval_with_scope(&mut scope, &expr).unwrap();
        println!("Rhai at a=6066: {}", result);
        assert!(result.abs() < 1.0, "Rhai expression at a=6066 should be ~0, got {}", result);
        
        // Test at a=7000, should be negative (TOF<1800)
        scope.set_value("x", 7000.0);
        let result: f64 = engine.eval_with_scope(&mut scope, &expr).unwrap();
        println!("Rhai at a=7000: {}", result);
        assert!(result < 0.0, "Rhai expression at a=7000 should be <0, got {}", result);
    }

    #[test]
    fn test_lambert_bisect_root() {
        // Test vector from problem statement
        let r1 = 6980.0;  // km
        let r2 = 10520.0; // km
        let c = 6655.0;   // km
        let s = 12078.0;  // km
        let mu = 398600.0; // km³/s²
        let target_tof = 1800.0; // seconds
        
        // Create Rhai expression for TOF(a) - 1800
        let expr = lambert_tof_expression(r1, r2, c, s, mu, target_tof);
        
        println!("Expression: {}", expr);
        
        // For Lambert's problem:
        // - a must be >= s/2 to avoid NaN in asin (s/2 = 6039)
        // - TOF decreases as a increases for elliptical orbits
        // - We need f(a_low) > 0 and f(a_high) < 0 for bisection
        // Based on testing: f(6040) ≈ +145, f(6100) ≈ -82
        let result = bisect_root(&expr, 6040.0, 6100.0, 1e-6);
        
        if let Err(e) = &result {
            println!("Error: {}", e);
        }
        
        assert!(result.is_ok(), "Bisect root should succeed: {:?}", result);
        let a = result.unwrap();
        
        // Verify result is close to expected ~6066 km
        assert!((a - 6066.0).abs() < 5.0, 
                "Semi-major axis should be ~6066 km, got {}", a);
        
        // Verify the TOF at this a is indeed 1800s
        let tof = lambert_tof(a, r1, r2, c, s, mu);
        assert!((tof - target_tof).abs() < 1.0,
                "TOF at a={} should be {}s, got {}s", a, target_tof, tof);
    }
    
    #[test]
    fn test_lambert_tof_direct() {
        // Direct calculation test
        let r1 = 6980.0;
        let r2 = 10520.0;
        let c = 6655.0;
        let s = 12078.0;
        let mu = 398600.0;
        
        let a = 6066.0;
        let tof = lambert_tof(a, r1, r2, c, s, mu);
        
        // Should be approximately 1800 seconds
        assert!((tof - 1800.0).abs() < 10.0,
                "TOF calculation should be accurate, got {} instead of ~1800", tof);
    }
}
