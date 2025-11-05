/*
SPDX-FileCopyrightText: 2025 Eric Waller
SPDX-License-Identifier: LicenseRef-eRock-Business-1.0
*/

use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use erock::luxi_eval::{lexer, parser, interpreter};
mod jit_health;
#[allow(dead_code)]

const VERSION: &str = env!("CARGO_PKG_VERSION");

// ---------- /evaluate ----------
#[derive(Deserialize)]
struct EvalReq {
    expr: String,
    x: Vec<f64>,
    vars: Option<HashMap<String, f64>>,
}
#[derive(Serialize)]
struct EvalResp { y: Vec<f64> }

async fn evaluate(Json(req): Json<EvalReq>) -> Json<EvalResp> {
    let tokens = lexer::tokenize(&req.expr);
    let (arena, root) = parser::parse(tokens).expect("parse error");
    let fixed = req.vars.unwrap_or_default();
    let y = interpreter::simd_eval_over_x(root, &arena, &fixed, &req.x);
    Json(EvalResp { y })
}

// ---------- /bisect ----------
#[derive(Deserialize)]
struct BisectReq {
    expr: String,
    lo: f64,
    hi: f64,
    vars: Option<HashMap<String, f64>>,
    tol: Option<f64>,
    max_iter: Option<usize>,
}
#[derive(Serialize)]
struct BisectResp {
    root: f64,
    f: f64,
    iters: usize,
    bracket_ok: bool,
}

async fn bisect(Json(req): Json<BisectReq>) -> Json<BisectResp> {
    let tokens = lexer::tokenize(&req.expr);
    let (arena, root) = parser::parse(tokens).expect("parse error");
    let fixed = req.vars.unwrap_or_default();

    let eval_at = |t: f64| -> f64 {
        interpreter::simd_eval_over_x(root, &arena, &fixed, &vec![t])[0]
    };

    let mut lo = req.lo;
    let mut hi = req.hi;
    let mut flo = eval_at(lo);
    let fhi = eval_at(hi);

    let bracket_ok = (flo <= 0.0 && fhi >= 0.0) || (flo >= 0.0 && fhi <= 0.0);
    if !bracket_ok {
        return Json(BisectResp { root: f64::NAN, f: f64::NAN, iters: 0, bracket_ok });
    }

    let tol = req.tol.unwrap_or(1e-9);
    let max_iter = req.max_iter.unwrap_or(60);
    let mut iters = 0usize;

    for _ in 0..max_iter {
        let mid = 0.5 * (lo + hi);
        let fm = eval_at(mid);
        iters += 1;

        if (hi - lo).abs() <= tol {
            return Json(BisectResp { root: mid, f: fm, iters, bracket_ok: true });
        }
        if (flo <= 0.0 && fm <= 0.0) || (flo >= 0.0 && fm >= 0.0) {
            lo = mid; flo = fm;
        } else {
            hi = mid;
        }
    }

    let mid = 0.5 * (lo + hi);
    let fm = eval_at(mid);
    Json(BisectResp { root: mid, f: fm, iters, bracket_ok: true })
}

// ---------- /bisect_auto ----------
#[derive(Deserialize)]
struct BisectAutoReq {
    expr: String,
    guess: f64,                 // starting point (seconds)
    step: Option<f64>,          // initial half-interval (default 1.0s)
    max_expand: Option<usize>,  // expansions of step (default 20)
    vars: Option<HashMap<String, f64>>,
    tol: Option<f64>,
    max_iter: Option<usize>,
}
#[derive(Serialize)]
struct BisectAutoResp {
    root: f64,
    f: f64,
    lo: f64,
    hi: f64,
    iters: usize,
    bracket_ok: bool,
    expansions: usize,
}

fn same_sign(a: f64, b: f64) -> bool {
    (a >= 0.0 && b >= 0.0) || (a <= 0.0 && b <= 0.0)
}

async fn bisect_auto(Json(req): Json<BisectAutoReq>) -> Json<BisectAutoResp> {
    let tokens = lexer::tokenize(&req.expr);
    let (arena, root) = parser::parse(tokens).expect("parse error");
    let fixed = req.vars.unwrap_or_default();

    let eval_at = |t: f64| -> f64 {
        interpreter::simd_eval_over_x(root, &arena, &fixed, &vec![t])[0]
    };

    let g = req.guess;
    let mut s = req.step.unwrap_or(1.0).abs().max(1e-6);
    let max_expand = req.max_expand.unwrap_or(20);
    let f0 = eval_at(g);

    if f0.abs() == 0.0 {
        return Json(BisectAutoResp { root: g, f: f0, lo: g, hi: g, iters: 0, bracket_ok: true, expansions: 0 });
    }

    // Exponential outward search
    let mut lo = f64::NAN;
    let mut hi = f64::NAN;
    let mut expansions = 0usize;

    for i in 0..=max_expand {
        expansions = i;

        let a = g - s;
        let fa = eval_at(a);
        if !same_sign(fa, f0) {
            lo = a.min(g);
            hi = a.max(g);
            break;
        }

        let b = g + s;
        let fb = eval_at(b);
        if !same_sign(fb, f0) {
            lo = g.min(b);
            hi = g.max(b);
            break;
        }

        s *= 2.0;
    }

    if !lo.is_finite() || !hi.is_finite() {
        return Json(BisectAutoResp { root: f64::NAN, f: f64::NAN, lo: f64::NAN, hi: f64::NAN, iters: 0, bracket_ok: false, expansions });
    }

    // Bisection on the found bracket
    let tol = req.tol.unwrap_or(1e-9);
    let max_iter = req.max_iter.unwrap_or(60);
    let mut iters = 0usize;
    let mut flo = eval_at(lo);

    for _ in 0..max_iter {
        let mid = 0.5 * (lo + hi);
        let fm = eval_at(mid);
        iters += 1;

        if (hi - lo).abs() <= tol {
            return Json(BisectAutoResp { root: mid, f: fm, lo, hi, iters, bracket_ok: true, expansions });
        }
        if same_sign(fm, flo) {
            lo = mid; flo = fm;
        } else {
            hi = mid;
        }
    }

    let mid = 0.5 * (lo + hi);
    let fm = eval_at(mid);
    Json(BisectAutoResp { root: mid, f: fm, lo, hi, iters, bracket_ok: true, expansions })
}

// ---------- /health ----------
#[allow(dead_code)]
async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "service": "erock_edge",
        "version": VERSION,
        "status": "ok"
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/evaluate", post(evaluate))
        .route("/bisect", post(bisect))
        .route("/bisect_auto", post(bisect_auto))
        .route("/health", axum::routing::get(crate::jit_health::health_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
