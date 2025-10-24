use rhai::{Engine, Scope, EvalAltResult, Dynamic, Position};
use std::collections::HashMap;

#[inline(always)]
pub fn evaluate(expr: &str, x: &[f64], vars: &HashMap<String, f64>) -> Result<Vec<f64>, Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.disable_symbol("^");
    engine.register_custom_operator("^", 160).unwrap();
    engine.register_fn("^", |a: f64, b: f64| a.powf(b));
    engine.register_fn("^", |a: f64, b: i64| a.powf(b as f64));
    let mut scope = Scope::new();
    for (k, v) in vars {
        scope.push(k.clone(), *v);
    }
    let ast = engine.compile(expr)?;
    let mut y = Vec::with_capacity(x.len());
    for &val in x {
        scope.set_value("x", val);
        let res = engine.eval_ast_with_scope(&mut scope, &ast)?;
        y.push(res);
    }
    Ok(y)
}

#[inline(always)]
pub fn find_root(expr: &str, lo: f64, hi: f64, tol: f64, vars: &HashMap<String, f64>) -> Result<f64, Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.disable_symbol("^");
    engine.register_custom_operator("^", 160).unwrap();
    engine.register_fn("^", |a: f64, b: f64| a.powf(b));
    engine.register_fn("^", |a: f64, b: i64| a.powf(b as f64));
    let ast = engine.compile(expr)?;
    let mut a = lo;
    let mut b = hi;
    let mut scope_a = Scope::new();
    let mut scope_b = Scope::new();
    for (k, v) in vars {
        scope_a.push(k.clone(), *v);
        scope_b.push(k.clone(), *v);
    }
    for _ in 0..64 {
        let mid = (a + b) / 2.0;
        scope_a.set_value("x", mid);
        let f_mid: f64 = engine.eval_ast_with_scope(&mut scope_a, &ast)?;
        if f_mid.abs() < tol {
            return Ok(mid);
        }
        scope_b.set_value("x", a);
        let f_a: f64 = engine.eval_ast_with_scope(&mut scope_b, &ast)?;
        if f_a * f_mid < 0.0 {
            b = mid;
        } else {
            a = mid;
        }
    }
    Err(Box::new(EvalAltResult::ErrorRuntime(Dynamic::from("No convergence"), Position::NONE)))
}
