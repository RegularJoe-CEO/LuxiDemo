// Fallback evaluator for Luxi Edge until the real SIMD modules are wired.

pub mod lexer {
    pub fn tokenize(input: &str) -> String { input.to_string() }
}

pub mod parser {
    #[derive(Clone)]
    pub struct Arena { pub expr: String }
    pub fn parse(tokens: String) -> Result<(Arena, usize), String> {
        if tokens.trim().is_empty() { return Err("empty expression".into()); }
        Ok((Arena { expr: tokens }, 0))
    }
}

pub mod interpreter {
    use super::parser::Arena;
    use std::collections::HashMap;
    use rhai::{Dynamic, Engine, Scope};

    pub fn interpret(_root: usize, arena: &Arena, vars: &mut HashMap<String, f64>) -> f64 {
        let eng = Engine::new();
        let mut scope = Scope::new();
        for (k, v) in vars.iter() { scope.push_dynamic(k.as_str(), Dynamic::from(*v)); }
        match eng.eval_with_scope::<Dynamic>(&mut scope, &arena.expr) {
            Ok(val) => val.as_float().unwrap_or(f64::NAN),
            Err(_) => f64::NAN,
        }
    }

    pub fn simd_eval_over_x(
        _root: usize,
        arena: &Arena,
        fixed: &HashMap<String, f64>,
        xs: &Vec<f64>,
    ) -> Vec<f64> {
        let eng = Engine::new();
        let mut out = Vec::with_capacity(xs.len());
        for &x in xs {
            let mut scope = Scope::new();
            scope.push_dynamic("x", Dynamic::from(x));
            for (k, v) in fixed.iter() {
                if k != "x" { scope.push_dynamic(k.as_str(), Dynamic::from(*v)); }
            }
            match eng.eval_with_scope::<Dynamic>(&mut scope, &arena.expr) {
                Ok(val) => out.push(val.as_float().unwrap_or(f64::NAN)),
                Err(_) => out.push(f64::NAN),
            }
        }
        out
    }
}
