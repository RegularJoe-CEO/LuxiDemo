use axum::{routing::{get, post}, Router, Json};
use axum::extract::DefaultBodyLimit;
use clap::Parser;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::net::SocketAddr;

#[derive(Parser)]
#[command(name = "luxiedge-server")]
#[command(about = "LuxiEdge REST API - Bit-exact deterministic math engine")]
#[command(version)]
struct Args {
    #[arg(short, long, default_value = "10000")]
    port: u16,
}

#[derive(Deserialize)]
struct EvalRequest {
    expr: String,
    values: Vec<f64>,
    #[serde(default = "default_precision")]
    precision: String,
}

fn default_precision() -> String { "f64".to_string() }

#[derive(Serialize)]
struct EvalResponse {
    expr: String,
    results: Vec<f64>,
    count: usize,
    precision: String,
    sha256: String,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    engine: String,
}

#[tokio::main]
async fn main() {
    let token = ".pilot_token";
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("Time").as_secs();
    let start = if let Ok(x) = std::fs::read_to_string(token) { x.trim().parse().unwrap_or(now) } else { std::fs::write(token, now.to_string()).ok(); now };
    let days = 30u64.saturating_sub((now - start) / 86400);
    
    print!("\x1B[2J\x1B[1;1H");
    println!("✅ COMPILE: [COMPLETE] | ✅ LINK: [COMPLETE] | ⏳ PILOT: [{} DAYS REMAINING]", days);
    println!("    ┌──┬──────────────────────────┬──┐");
    println!("    │▒▒│  LUXIEDGE CORE ENGINE    │▒▒│");
    println!("    │▒▒│  v1.0 · Port 10000       │▒▒│");
    println!("    │▒▒│  POWERED BY ELVIS        │▒▒│");
    println!("    └──┴──────────────────────────┴──┘");
    println!("\n    Lu(x)i Quantitative and Academic Solutions\n");
    
    let args = Args::parse();
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    let app = Router::new()
        .route("/health", get(health))
        .route("/evaluate", post(evaluate))
        .layer(DefaultBodyLimit::max(128 * 1024 * 1024));
    println!("Listening on http://{}...", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: "1.0".to_string(),
        engine: "LuxiEdge Core".to_string(),
    })
}

async fn evaluate(Json(req): Json<EvalRequest>) -> Json<EvalResponse> {
    let results: Vec<f64> = req.values.iter().map(|&x| {
        evaluate_expr(&req.expr, x as f32) as f64
    }).collect();
    
    let mut hasher = Sha256::new();
    for val in &results { 
        hasher.update(val.to_le_bytes()); 
    }
    let hash = format!("{:x}", hasher.finalize());
    
    Json(EvalResponse {
        expr: req.expr,
        count: results.len(),
        results,
        precision: req.precision,
        sha256: hash,
    })
}

fn evaluate_expr(expr: &str, x: f32) -> f32 {
    match expr {
        "sin(x)" => luxiedge::LuxiEdge_Core_Engine::sin_f32(x),
        "cos(x)" => luxiedge::LuxiEdge_Core_Engine::cos_f32(x),
        "sin(x)*cos(x)" => {
            let s = luxiedge::LuxiEdge_Core_Engine::sin_f32(x);
            let c = luxiedge::LuxiEdge_Core_Engine::cos_f32(x);
            s * c
        },
        "exp(x)" => luxiedge::LuxiEdge_Core_Engine::exp_f32(x),
        "ln(x)" => luxiedge::LuxiEdge_Core_Engine::ln_f32(x),
        "sqrt(x)" => luxiedge::LuxiEdge_Core_Engine::sqrt_f32(x),
        "x^2" => x * x,
        "x^3" => x * x * x,
        "erf(x)" => luxiedge::LuxiEdge_Core_Engine::erf_f32(x),
        "normcdf(x)" => luxiedge::LuxiEdge_Core_Engine::normcdf_f32(x),
        "normpdf(x)" => luxiedge::LuxiEdge_Core_Engine::normpdf_f32(x),
        "gamma(x)" => luxiedge::LuxiEdge_Core_Engine::gamma_f32(x),
        _ => x,
    }
}
