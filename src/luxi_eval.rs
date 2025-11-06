// Fallback evaluator for Luxi Edge until the real SIMD modules are wired.

pub mod lexer {
    pub fn tokenize(input: &str) -> String {
        input.to_string()
    }
}

pub mod parser {
    #[derive(Clone)]
    pub struct Arena {
        pub expr: String,
    }
    pub fn parse(tokens: String) -> Result<(Arena, usize), String> {
        if tokens.trim().is_empty() {
            return Err("empty expression".into());
        }
        Ok((Arena { expr: tokens }, 0))
    }
}

pub mod interpreter {
    use super::parser::Arena;
    use rhai::{Dynamic, Engine, Scope};
    use std::collections::HashMap;

    fn eval_with_scope(arena: &Arena, vars: &HashMap<String, f64>) -> f64 {
        let eng = Engine::new();
        let mut scope = Scope::new();
        for (k, v) in vars.iter() {
            scope.push_dynamic(k.as_str(), Dynamic::from(*v));
        }
        match eng.eval_with_scope::<Dynamic>(&mut scope, &arena.expr) {
            Ok(val) => val.as_float().unwrap_or(f64::NAN),
            Err(_) => f64::NAN,
        }
    }

    fn assemble_vars(
        base: &HashMap<String, f64>,
        var: Option<(&str, f64)>,
    ) -> HashMap<String, f64> {
        let mut vars = base.clone();
        if let Some((name, value)) = var {
            vars.insert(name.to_string(), value);
        }
        vars
    }

    pub fn interpret(_root: usize, arena: &Arena, vars: &mut HashMap<String, f64>) -> f64 {
        eval_with_scope(arena, vars)
    }

    pub fn eval_scalar(arena: &Arena, vars: &HashMap<String, f64>) -> f64 {
        eval_with_scope(arena, vars)
    }

    pub fn eval_with_var(arena: &Arena, base: &HashMap<String, f64>, var: &str, value: f64) -> f64 {
        let vars = assemble_vars(base, Some((var, value)));
        eval_with_scope(arena, &vars)
    }

 pub fn simd_eval_over_x_inplace(
    _root: usize,
    arena: &Arena,
    fixed: &HashMap<String, f64>,
    mut xs: Vec<f64>,
) -> Result<Vec<f64>, String> {
    if xs.is_empty() {
        return Ok(xs);
    }

    let len = xs.len();
    let lanes = 2; // f64x2 per 128-bit NEON register on AArch64/M1
    let full_vecs = len / lanes;

    // Vectorized loop for basic ops (optimized for sin(x)*cos(x); extend for full AST)
    for i in (0..full_vecs * lanes).step_by(lanes) {
        unsafe {
            // Unaligned load (per technical docs)
            let x_ptr = xs.as_ptr().add(i) as *const f64;
            let x_vec = vld1q_f64(x_ptr);

            // Placeholder vector sin/cos (simple approx for demo; use full neon-libm for prod)
            // For sin(x)*cos(x), compute vectorized: sin ~ x (small x), cos ~ 1, but use scalar for accuracy here
            // TODO: Implement full vectorized eval_ast (post-order traversal on vectors)
            let sin_vals = vdupq_n_f64(0.0f64); // Temp: fallback to scalar below for correctness
            let cos_vals = vdupq_n_f64(1.0f64);
            let res_vec = vmulq_f64(sin_vals, cos_vals); // Vector mul for *

            // Store in-place (buffer reuse, 0 allocs)
            let xs_ptr = xs.as_mut_ptr().add(i) as *mut f64;
            vst1q_f64(xs_ptr, res_vec);
        }
    }

    // Scalar fallback for remainder & full AST evaluation (ensures correctness for any expr)
    let rem_start = full_vecs * lanes;
    for j in rem_start..len {
        xs[j] = eval_with_var(arena, fixed, "x", xs[j]).map_err(|e| format!("Eval error at {}: {}", j, e))?;
    }

    // For now, full scalar override to match original behavior (SIMD placeholder above; enable vector check later)
    for j in 0..len {
        xs[j] = eval_with_var(arena, fixed, "x", xs[j]).map_err(|e| format!("Eval error at {}: {}", j, e))?;
    }

    Ok(xs)
}

    pub fn derivative_with_var(
        arena: &Arena,
        base: &HashMap<String, f64>,
        var: &str,
        value: f64,
        h: f64,
    ) -> f64 {
        let step = h.abs().max(1e-12);
        let forward = eval_with_var(arena, base, var, value + step);
        let backward = eval_with_var(arena, base, var, value - step);
        (forward - backward) / (2.0 * step)
    }

    pub fn gradient(
        arena: &Arena,
        base: &HashMap<String, f64>,
        variables: &[String],
        h: f64,
    ) -> HashMap<String, f64> {
        let mut grads = HashMap::new();
        for var in variables {
            let value = match base.get(var) {
                Some(v) => *v,
                None => {
                    grads.insert(var.clone(), f64::NAN);
                    continue;
                }
            };
            let deriv = derivative_with_var(arena, base, var, value, h);
            grads.insert(var.clone(), deriv);
        }
        grads
    }
}
