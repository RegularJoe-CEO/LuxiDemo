// src/math.rs — Luxi
use anyhow::{anyhow, bail, Result};
use rhai::{Dynamic, Engine, Scope};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct BisectReq {
    pub func: String,
    pub a: f64,
    pub b: f64,
    #[serde(default = "default_tol")]
    pub tol: f64,
    #[serde(default = "default_max_iter")]
    pub max_iter: usize,
}

#[derive(Serialize)]
pub struct BisectResp {
    pub root: f64,
    pub iterations: usize,
}

pub fn default_tol() -> f64 {
    1e-6
}
pub fn default_max_iter() -> usize {
    64
}

/// Evaluate `func` at x using Rhai, but **do not** use `?` on Rhai errors
/// (Rhai's EvalAltResult is not Send/Sync). Convert explicitly to anyhow::Error.
fn eval_func(engine: &Engine, scope: &mut Scope, func: &str, x: f64) -> Result<f64> {
    scope.set_value("x", x);

    let out = engine
        .eval_with_scope::<Dynamic>(scope, func)
        .map_err(|e| anyhow!("eval error at x={}: {}", x, e))?;

    // Rhai Dynamic -> f64 (will error if not a number)
    let num = out.cast::<f64>();
    if !num.is_finite() {
        bail!("function evaluated to non-finite at x={}", x);
    }
    Ok(num)
}

/// Classic bisection with deterministic bounds/iterations.
pub fn bisect(engine: &Engine, req: &BisectReq) -> Result<BisectResp> {
    if req.a >= req.b {
        bail!("invalid interval: a must be < b");
    }
    if !(req.tol > 0.0) {
        bail!("tol must be > 0");
    }

    let mut scope = Scope::new();

    let mut left = req.a;
    let mut right = req.b;

    let mut fa = eval_func(engine, &mut scope, &req.func, left)?;
    let mut fb = eval_func(engine, &mut scope, &req.func, right)?;
    if fa == 0.0 {
        return Ok(BisectResp {
            root: left,
            iterations: 0,
        });
    }
    if fb == 0.0 {
        return Ok(BisectResp {
            root: right,
            iterations: 0,
        });
    }
    if fa.signum() == fb.signum() {
        bail!("f(a) and f(b) must have opposite signs (got f(a)={} f(b)={})", fa, fb);
    }

    let mut iters = 0usize;
    while (right - left) > req.tol && iters < req.max_iter {
        let mid = 0.5 * (left + right);
        let fm = eval_func(engine, &mut scope, &req.func, mid)?;

        if fm == 0.0 {
            return Ok(BisectResp {
                root: mid,
                iterations: iters + 1,
            });
        }

        // Keep the sign change interval
        if fa.signum() != fm.signum() {
            right = mid;
            fb = fm;
        } else {
            left = mid;
            fa = fm;
        }
        iters += 1;
    }

    Ok(BisectResp {
        root: 0.5 * (left + right),
        iterations: iters,
    })
}

/// Auto-tune for sub-1ms target by capping iterations and loosening tol if not set.
pub fn bisect_auto(engine: &Engine, mut req: BisectReq) -> Result<BisectResp> {
    req.max_iter = req.max_iter.min(20).max(16);
    if (req.tol - default_tol()).abs() < f64::EPSILON {
        req.tol = 1e-5;
    }
    bisect(engine, &req)
}
