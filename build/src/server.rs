use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct EvalRequest {
    pub expr: String,
    pub values: Vec<f64>,
    #[serde(default = "default_precision")]
    pub precision: String,
}

fn default_precision() -> String {
    "f64".to_string()
}

#[derive(Serialize, Clone)]
pub struct EvalResponse {
    pub expr: String,
    pub results: Vec<f64>,
    pub count: usize,
    pub precision: String,
    pub sha256: String,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub engine: String,
    pub mode: String,
    pub operators: usize,
}

pub type EvalFn = Arc<dyn Fn(&str, &[f64], bool) -> (Vec<f64>, String) + Send + Sync>;

pub fn print_banner(port: u16, title: &str, mode: &str, operators: usize) {
    println!();
    println!("    +--+--------------------------+--+");
    println!("    |  |  {}  |  |", pad_center(title, 24));
    println!("    |  |  v3.0.0 · Port {:<5}     |  |", port);
    println!("    |  |  {}  |  |", pad_center(mode, 24));
    println!("    |  |  SHA-256 Verified        |  |");
    println!("    +--+--------------------------+--+");
    println!("       {} operators available", operators);
    println!();
}

fn pad_center(s: &str, width: usize) -> String {
    if s.len() >= width {
        return s[..width].to_string();
    }
    let pad = width - s.len();
    let left = pad / 2;
    let right = pad - left;
    format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
}

pub async fn run_server(port: u16, title: &str, mode: &str, operators: usize, eval_fn: EvalFn) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    print_banner(port, title, mode, operators);

    let mode_health = mode.to_string();
    let app = Router::new()
        .route(
            "/health",
            get(move || {
                let mode = mode_health.clone();
                async move {
                    Json(HealthResponse {
                        status: "ok".to_string(),
                        version: "3.0.0".to_string(),
                        engine: "LuxiEdge Core".to_string(),
                        mode,
                        operators,
                    })
                }
            }),
        )
        .route(
            "/evaluate",
            post({
                let eval_fn = eval_fn.clone();
                move |Json(req): Json<EvalRequest>| {
                    let eval_fn = eval_fn.clone();
                    async move {
                        let use_f64 = req.precision == "f64";
                        let precision = if use_f64 {
                            "f64".to_string()
                        } else {
                            "f32".to_string()
                        };
                        let (results, sha256) = eval_fn(&req.expr, &req.values, use_f64);
                        Json(EvalResponse {
                            expr: req.expr,
                            results,
                            count: req.values.len(),
                            precision,
                            sha256,
                        })
                    }
                }
            }),
        )
        .layer(DefaultBodyLimit::max(128 * 1024 * 1024));

    println!("    Listening on http://{}...", addr);
    println!();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn make_eval_handler<F>(f: F) -> EvalFn
where
    F: Fn(&str, &[f64], bool) -> (Vec<f64>, String) + Send + Sync + 'static,
{
    Arc::new(f)
}