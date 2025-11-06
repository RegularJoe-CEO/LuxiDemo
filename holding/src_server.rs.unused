// Copyright 2025 RegularJoe-CEO. All rights reserved. eRock is a protected product—commercial use requires licensing.
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use rhai::{Engine, Scope};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Deserialize)]
struct Payload {
    expression: String,
    x: Vec<f64>,
    vars: HashMap<String, f64>,
}

#[derive(Serialize)]
struct Response {
    y: Vec<f64>,
    root: f64,
    iterations: u32,
    duration: f64,
}

async fn evaluate(Json(payload): Json<Payload>) -> Json<Response> {
    let mut engine = Engine::new();
    engine.disable_symbol("^");
    engine.register_custom_operator("^", 160).unwrap();
    engine.register_fn("^", |a: f64, b: f64| a.powf(b));
    engine.register_fn("^", |a: f64, b: i64| a.powf(b as f64));

    let mut scope = Scope::new();
    for (k, v) in &payload.vars {
        scope.push(k.clone(), *v);
    }
    let mut y = Vec::new();
    for &val in &payload.x {
        scope.set_value("x", val);
        let res: f64 = engine.eval_with_scope(&mut scope, &payload.expression).unwrap();
        y.push(res);
    }

    let response = Response {
        y,
        root: 0.0,
        iterations: 0,
        duration: 0.0,
    };
    Json(response)
}

async fn bisect(Json(payload): Json<Payload>) -> Json<Response> {
    let mut engine = Engine::new();
    engine.disable_symbol("^");
    engine.register_custom_operator("^", 160).unwrap();
    engine.register_fn("^", |a: f64, b: f64| a.powf(b));
    engine.register_fn("^", |a: f64, b: i64| a.powf(b as f64));

    let mut scope_a = Scope::new();
    let mut scope_b = Scope::new();
    for (k, v) in &payload.vars {
        scope_a.push(k.clone(), *v);
        scope_b.push(k.clone(), *v);
    }

    let mut a = payload.x[0]; // lo
    let mut b = payload.x[1]; // hi
    let mut iterations = 0u32;
    let start = Instant::now();
    let mut current_root = 0.0;

    for i in 0..64 {
        let mid = (a + b) / 2.0;
        scope_a.set_value("x", mid);
        let f_mid: f64 = engine.eval_with_scope(&mut scope_a, &payload.expression).unwrap();
        if f_mid.abs() < 1e-6 {
            current_root = mid;
            iterations = i + 1;
            break;
        }
        scope_b.set_value("x", a);
        let f_a: f64 = engine.eval_with_scope(&mut scope_b, &payload.expression).unwrap();
        if f_a * f_mid < 0.0 {
            b = mid;
        } else {
            a = mid;
        }
        iterations = i + 1;
        current_root = mid;
    }

    let duration = start.elapsed().as_secs_f64();

    let response = Response {
        y: vec![],
        root: current_root,
        iterations,
        duration,
    };
    Json(response)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/evaluate", post(evaluate))
        .route("/bisect", post(bisect));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 eRock server starting on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
