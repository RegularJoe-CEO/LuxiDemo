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

    pub fn simd_eval_over_x(
        _root: usize,
        arena: &Arena,
        fixed: &HashMap<String, f64>,
        xs: &Vec<f64>,
    ) -> Vec<f64> {
        let mut out = Vec::with_capacity(xs.len());
        for &x in xs {
            out.push(eval_with_var(arena, fixed, "x", x));
        }
        out
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
