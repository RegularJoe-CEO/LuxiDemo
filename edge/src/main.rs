/*
SPDX-FileCopyrightText: 2025 Eric Waller
SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0
*/

use std::{collections::HashMap, convert::Infallible, net::SocketAddr};

use erock::luxi_eval::{interpreter, lexer, parser};
use hyper::{
    body::to_bytes,
    header::CONTENT_TYPE,
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, StatusCode,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};

mod jit_health;

const VERSION: &str = env!("CARGO_PKG_VERSION");

// ---------- shared helpers ----------
fn json_response(status: StatusCode, payload: &Value) -> Response<Body> {
    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(payload).unwrap()))
        .unwrap()
}

fn json_ok<T: Serialize>(payload: &T) -> Response<Body> {
    let body = serde_json::to_value(payload).unwrap();
    json_response(StatusCode::OK, &body)
}

fn bad_request(message: impl Into<String>) -> Response<Body> {
    let body = json!({ "error": message.into() });
    json_response(StatusCode::BAD_REQUEST, &body)
}

fn method_not_allowed() -> Response<Body> {
    let body = json!({ "error": "method not allowed" });
    json_response(StatusCode::METHOD_NOT_ALLOWED, &body)
}

fn not_found() -> Response<Body> {
    let body = json!({ "error": "not found" });
    json_response(StatusCode::NOT_FOUND, &body)
}

async fn parse_body<T: DeserializeOwned>(body: Body) -> Result<T, Response<Body>> {
    let bytes = to_bytes(body)
        .await
        .map_err(|_| bad_request("failed to read request body"))?;
    serde_json::from_slice(&bytes).map_err(|err| bad_request(format!("invalid json: {err}")))
}

// ---------- /evaluate ----------
#[derive(Deserialize)]
struct EvalReq {
    expr: String,
    x: Vec<f64>,
    vars: Option<HashMap<String, f64>>,
}

#[derive(Serialize)]
struct EvalResp {
    y: Vec<f64>,
}

fn handle_evaluate(req: EvalReq) -> Result<EvalResp, String> {
    let tokens = lexer::tokenize(&req.expr);
    let (arena, root) = parser::parse(tokens)?;
    let fixed = req.vars.unwrap_or_default();
    let y = interpreter::simd_eval_over_x_inplace(root, &arena, &fixed, req.x);
    Ok(EvalResp { y })
}

#[derive(Deserialize)]
struct EvalDerivativeReq {
    expr: String,
    x: Vec<f64>,
    vars: Option<HashMap<String, f64>>,
    step: Option<f64>,
}

#[derive(Serialize)]
struct EvalDerivativeResp {
    y: Vec<f64>,
    dy_dx: Vec<f64>,
}

fn handle_evaluate_derivative(req: EvalDerivativeReq) -> Result<EvalDerivativeResp, String> {
    let tokens = lexer::tokenize(&req.expr);
    let (arena, root) = parser::parse(tokens)?;
    let fixed = req.vars.unwrap_or_default();
    let step = req.step.unwrap_or(1e-6);
    let y = interpreter::simd_eval_over_x_inplace(root, &arena, &fixed, req.x);
    let mut dy_dx = Vec::with_capacity(y.len());
    for &x in y.iter() {
        let deriv = interpreter::derivative_with_var(&arena, &fixed, "x", x, step);
        dy_dx.push(deriv);
    }
    Ok(EvalDerivativeResp { y, dy_dx })
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

fn handle_bisect(req: BisectReq) -> Result<BisectResp, String> {
    let tokens = lexer::tokenize(&req.expr);
    let (arena, root) = parser::parse(tokens)?;
    let fixed = req.vars.unwrap_or_default();

    let eval_at =
        |t: f64| -> f64 { interpreter::simd_eval_over_x_inplace(root, &arena, &fixed, vec![t])[0] };

    let mut lo = req.lo;
    let mut hi = req.hi;
    let mut flo = eval_at(lo);
    let fhi = eval_at(hi);

    let bracket_ok = (flo <= 0.0 && fhi >= 0.0) || (flo >= 0.0 && fhi <= 0.0);
    if !bracket_ok {
        return Ok(BisectResp {
            root: f64::NAN,
            f: f64::NAN,
            iters: 0,
            bracket_ok,
        });
    }

    let tol = req.tol.unwrap_or(1e-9);
    let max_iter = req.max_iter.unwrap_or(60);
    let mut iters = 0usize;

    for _ in 0..max_iter {
        let mid = 0.5 * (lo + hi);
        let fm = eval_at(mid);
        iters += 1;

        if (hi - lo).abs() <= tol {
            return Ok(BisectResp {
                root: mid,
                f: fm,
                iters,
                bracket_ok: true,
            });
        }
        if (flo <= 0.0 && fm <= 0.0) || (flo >= 0.0 && fm >= 0.0) {
            lo = mid;
            flo = fm;
        } else {
            hi = mid;
        }
    }

    let mid = 0.5 * (lo + hi);
    let fm = eval_at(mid);
    Ok(BisectResp {
        root: mid,
        f: fm,
        iters,
        bracket_ok: true,
    })
}

// ---------- /bisect_auto ----------
#[derive(Deserialize)]
struct BisectAutoReq {
    expr: String,
    guess: f64,
    step: Option<f64>,
    max_expand: Option<usize>,
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

fn handle_bisect_auto(req: BisectAutoReq) -> Result<BisectAutoResp, String> {
    let tokens = lexer::tokenize(&req.expr);
    let (arena, root) = parser::parse(tokens)?;
    let fixed = req.vars.unwrap_or_default();

    let eval_at =
        |t: f64| -> f64 { interpreter::simd_eval_over_x_inplace(root, &arena, &fixed, vec![t])[0] };

    let g = req.guess;
    let mut s = req.step.unwrap_or(1.0).abs().max(1e-6);
    let max_expand = req.max_expand.unwrap_or(20);
    let f0 = eval_at(g);

    if f0.abs() == 0.0 {
        return Ok(BisectAutoResp {
            root: g,
            f: f0,
            lo: g,
            hi: g,
            iters: 0,
            bracket_ok: true,
            expansions: 0,
        });
    }

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
        return Ok(BisectAutoResp {
            root: f64::NAN,
            f: f64::NAN,
            lo: f64::NAN,
            hi: f64::NAN,
            iters: 0,
            bracket_ok: false,
            expansions,
        });
    }

    let tol = req.tol.unwrap_or(1e-9);
    let max_iter = req.max_iter.unwrap_or(60);
    let mut iters = 0usize;
    let mut flo = eval_at(lo);

    for _ in 0..max_iter {
        let mid = 0.5 * (lo + hi);
        let fm = eval_at(mid);
        iters += 1;

        if (hi - lo).abs() <= tol {
            return Ok(BisectAutoResp {
                root: mid,
                f: fm,
                lo,
                hi,
                iters,
                bracket_ok: true,
                expansions,
            });
        }
        if same_sign(fm, flo) {
            lo = mid;
            flo = fm;
        } else {
            hi = mid;
        }
    }

    let mid = 0.5 * (lo + hi);
    let fm = eval_at(mid);
    Ok(BisectAutoResp {
        root: mid,
        f: fm,
        lo,
        hi,
        iters,
        bracket_ok: true,
        expansions,
    })
}

#[derive(Deserialize)]
struct GradientReq {
    expr: String,
    vars: HashMap<String, f64>,
    variables: Option<Vec<String>>,
    step: Option<f64>,
}

#[derive(Serialize)]
struct GradientResp {
    value: f64,
    gradient: HashMap<String, f64>,
}

fn handle_gradient(req: GradientReq) -> Result<GradientResp, String> {
    if req.vars.is_empty() {
        return Err("vars must include at least one variable".into());
    }

    let tokens = lexer::tokenize(&req.expr);
    let (arena, _root) = parser::parse(tokens)?;
    let step = req.step.unwrap_or(1e-6);
    let point = req.vars;
    let mut variables = if let Some(mut vars) = req.variables {
        if vars.is_empty() {
            return Err("variables list must not be empty".into());
        }
        vars.sort();
        vars.dedup();
        vars
    } else {
        point.keys().cloned().collect::<Vec<_>>()
    };
    variables.sort();
    variables.dedup();

    for var in &variables {
        if !point.contains_key(var) {
            return Err(format!("variable '{var}' missing from vars"));
        }
    }

    let value = interpreter::eval_scalar(&arena, &point);
    let gradient = interpreter::gradient(&arena, &point, &variables, step);
    Ok(GradientResp { value, gradient })
}

#[derive(Deserialize)]
struct NewtonReq {
    expr: String,
    guesses: Vec<f64>,
    vars: Option<HashMap<String, f64>>,
    tol: Option<f64>,
    max_iter: Option<usize>,
    step: Option<f64>,
    fallback_step: Option<f64>,
    fallback_expand: Option<usize>,
    fallback_tol: Option<f64>,
    fallback_max_iter: Option<usize>,
}

#[derive(Serialize)]
struct NewtonItem {
    guess: f64,
    root: f64,
    f: f64,
    newton_iters: usize,
    converged: bool,
    used_fallback: bool,
    fallback_iters: usize,
    fallback_expansions: usize,
    bracket_ok: bool,
}

#[derive(Serialize)]
struct NewtonResp {
    results: Vec<NewtonItem>,
}

fn fallback_bisect_auto<F>(
    eval_at: &F,
    guess: f64,
    step: f64,
    max_expand: usize,
    tol: f64,
    max_iter: usize,
) -> (f64, f64, usize, usize, bool)
where
    F: Fn(f64) -> f64,
{
    let base_step = step.abs().max(1e-6);
    let f0 = eval_at(guess);
    if !f0.is_finite() {
        return (f64::NAN, f64::NAN, 0, 0, false);
    }
    if f0.abs() <= tol {
        return (guess, f0, 0, 0, true);
    }

    let mut s = base_step;
    let mut lo = f64::NAN;
    let mut hi = f64::NAN;
    let mut expansions = 0usize;

    for i in 0..=max_expand {
        expansions = i;
        let a = guess - s;
        let fa = eval_at(a);
        if fa.is_finite() && !same_sign(fa, f0) {
            lo = a.min(guess);
            hi = a.max(guess);
            break;
        }

        let b = guess + s;
        let fb = eval_at(b);
        if fb.is_finite() && !same_sign(fb, f0) {
            lo = guess.min(b);
            hi = guess.max(b);
            break;
        }

        s *= 2.0;
    }

    if !lo.is_finite() || !hi.is_finite() {
        return (f64::NAN, f64::NAN, 0, expansions, false);
    }

    let mut iters = 0usize;
    let mut left = lo;
    let mut right = hi;
    let mut flo = eval_at(left);

    for _ in 0..max_iter {
        let mid = 0.5 * (left + right);
        let fm = eval_at(mid);
        iters += 1;

        if !fm.is_finite() {
            return (mid, fm, iters, expansions, false);
        }

        if (right - left).abs() <= tol || fm.abs() <= tol {
            return (mid, fm, iters, expansions, true);
        }

        if same_sign(fm, flo) {
            left = mid;
            flo = fm;
        } else {
            right = mid;
        }
    }

    let mid = 0.5 * (left + right);
    let fm = eval_at(mid);
    (mid, fm, iters, expansions, true)
}

fn handle_newton(req: NewtonReq) -> Result<NewtonResp, String> {
    if req.guesses.is_empty() {
        return Err("guesses must not be empty".into());
    }

    let tokens = lexer::tokenize(&req.expr);
    let (arena, _root) = parser::parse(tokens)?;
    let fixed = req.vars.unwrap_or_default();
    let tol = req.tol.unwrap_or(1e-9).abs().max(1e-12);
    let max_iter = req.max_iter.unwrap_or(25).max(1);
    let step = req.step.unwrap_or(1e-6);
    let fallback_step = req.fallback_step.unwrap_or(1.0);
    let fallback_expand = req.fallback_expand.unwrap_or(20);
    let fallback_tol = req.fallback_tol.unwrap_or(tol);
    let fallback_max_iter = req.fallback_max_iter.unwrap_or(60);

    let eval_at = |x: f64| interpreter::eval_with_var(&arena, &fixed, "x", x);

    let mut results = Vec::with_capacity(req.guesses.len());

    for guess in req.guesses {
        let mut current = guess;
        let mut value = eval_at(current);
        let mut iters = 0usize;
        let mut converged = value.is_finite() && value.abs() <= tol;
        let mut used_fallback = false;
        let mut fallback_iters = 0usize;
        let mut fallback_expansions = 0usize;
        let mut bracket_ok = true;

        if !converged {
            for _ in 0..max_iter {
                let deriv = interpreter::derivative_with_var(&arena, &fixed, "x", current, step);
                if !deriv.is_finite() || deriv.abs() < 1e-12 {
                    break;
                }

                let next = current - value / deriv;
                if !next.is_finite() {
                    break;
                }

                let next_val = eval_at(next);
                iters += 1;

                if !next_val.is_finite() {
                    current = next;
                    value = next_val;
                    break;
                }

                if next_val.abs() <= tol || (next - current).abs() <= tol {
                    current = next;
                    value = next_val;
                    converged = true;
                    break;
                }

                current = next;
                value = next_val;
            }
        }

        if !converged {
            let (root, f, fb_iters, fb_expansions, ok) = fallback_bisect_auto(
                &eval_at,
                guess,
                fallback_step,
                fallback_expand,
                fallback_tol,
                fallback_max_iter,
            );

            if ok {
                used_fallback = true;
                converged = true;
                current = root;
                value = f;
                fallback_iters = fb_iters;
                fallback_expansions = fb_expansions;
                bracket_ok = true;
            } else {
                bracket_ok = false;
                current = root;
                value = f;
            }
        }

        results.push(NewtonItem {
            guess,
            root: current,
            f: value,
            newton_iters: iters,
            converged,
            used_fallback,
            fallback_iters,
            fallback_expansions,
            bracket_ok,
        });
    }

    Ok(NewtonResp { results })
}

// ---------- /health ----------
fn health_payload() -> Value {
    let mut payload = jit_health::payload();
    payload["version"] = json!(VERSION);
    payload
}

async fn router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let (parts, body) = req.into_parts();
    let method = parts.method;
    let path = parts.uri.path().to_owned();

    let response = match (method, path.as_str()) {
        (Method::POST, "/evaluate") => match parse_body::<EvalReq>(body).await {
            Ok(req) => match handle_evaluate(req) {
                Ok(resp) => json_ok(&resp),
                Err(err) => bad_request(err),
            },
            Err(resp) => resp,
        },
        (Method::POST, "/evaluate_derivative") => match parse_body::<EvalDerivativeReq>(body).await
        {
            Ok(req) => match handle_evaluate_derivative(req) {
                Ok(resp) => json_ok(&resp),
                Err(err) => bad_request(err),
            },
            Err(resp) => resp,
        },
        (Method::POST, "/gradient") => match parse_body::<GradientReq>(body).await {
            Ok(req) => match handle_gradient(req) {
                Ok(resp) => json_ok(&resp),
                Err(err) => bad_request(err),
            },
            Err(resp) => resp,
        },
        (Method::POST, "/newton") => match parse_body::<NewtonReq>(body).await {
            Ok(req) => match handle_newton(req) {
                Ok(resp) => json_ok(&resp),
                Err(err) => bad_request(err),
            },
            Err(resp) => resp,
        },
        (Method::POST, "/bisect") => match parse_body::<BisectReq>(body).await {
            Ok(req) => match handle_bisect(req) {
                Ok(resp) => json_ok(&resp),
                Err(err) => bad_request(err),
            },
            Err(resp) => resp,
        },
        (Method::POST, "/bisect_auto") => match parse_body::<BisectAutoReq>(body).await {
            Ok(req) => match handle_bisect_auto(req) {
                Ok(resp) => json_ok(&resp),
                Err(err) => bad_request(err),
            },
            Err(resp) => resp,
        },
        (Method::GET, "/ping") => Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "text/plain; charset=utf-8")
            .body(Body::from("pong"))
            .unwrap(),
        (Method::GET, "/health") => json_ok(&health_payload()),
        (Method::GET, _) | (Method::POST, _) | (Method::PUT, _) | (Method::DELETE, _) => {
            not_found()
        }
        _ => method_not_allowed(),
    };

    Ok(response)
}

// ---------- /ping ----------
async fn ping() -> &'static str {
    "pong"
}

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(router)) });
    if let Err(err) = hyper::Server::bind(&addr).serve(make_svc).await {
        eprintln!("server error: {err}");
    }
}
