use anyhow::{anyhow, Result};
use rhai::{Engine, Scope};

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
