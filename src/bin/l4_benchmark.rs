use warp::Filter;
use std::net::SocketAddr;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use log::{info, warn};
use env_logger;
use rhai::{Engine, Scope, EvalAltResult};

#[derive(Deserialize)]
struct EvalRequest {
    expr: String,
    values: Vec<f32>,
    precision: Option<String>,
}

#[derive(Serialize)]
struct EvalResponse {
    results: Vec<f32>,
    hash: String,
    latency_ms: f64,
    ops_per_sec: f64,
    expr_used: String,
}

fn evaluate_dynamic_expr(engine: &Engine, expr: &str, values: &[f32]) -> Result<Vec<f32>, Box<EvalAltResult>> {
    let ast = engine.compile(expr)?;
    let mut results = Vec::with_capacity(values.len());
    for &x in values {
        let mut scope = Scope::new();
        scope.push("x", x as f64);
        let result: f64 = engine.eval_ast_with_scope(&mut scope, &ast)?;
        results.push(result as f32);
    }
    Ok(results)
}

fn fallback_eval(values: &[f32]) -> Vec<f32> {
    values.iter().map(|&x| x * x).collect()
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("LuxiEdge L4 Server Starting...");
    
    let health = warp::path("health").map(|| warp::reply::json(&"pong"));
    
    let eval_route = warp::post()
        .and(warp::path("eval"))
        .and(warp::body::json())
        .map(|req: EvalRequest| {
            let start = Instant::now();
            let mut engine = Engine::new(); engine.register_fn("erf", |x: f64| libm::erf(x)); engine.register_fn("erfc", |x: f64| libm::erfc(x));
            
            let (results, expr_used) = match evaluate_dynamic_expr(&engine, &req.expr, &req.values) {
                Ok(r) => (r, req.expr.clone()),
                Err(e) => {
                    warn!("Eval failed: {}", e);
                    (fallback_eval(&req.values), "x*x".into())
                }
            };
            
            let latency = start.elapsed().as_millis() as f64;
            let ops = if latency > 0.0 { req.values.len() as f64 / (latency/1000.0) } else { 0.0 };
            
            let mut hasher = Sha256::new();
            for &r in &results {
                hasher.update(r.to_le_bytes());
            }
            let hash = format!("{:x}", hasher.finalize());
            
            warp::reply::json(&EvalResponse {
                results,
                hash,
                latency_ms: latency,
                ops_per_sec: ops,
                expr_used,
            })
        });
    
    let routes = health.or(eval_route);
    let addr: SocketAddr = "0.0.0.0:9090".parse().unwrap();
    info!("Server on {}", addr);
    warp::serve(routes).run(addr).await;
}
