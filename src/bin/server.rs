use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use erock::{evaluate, find_root};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use std::time::Instant;

#[derive(Deserialize)]
struct EvaluateRequest {
    expression: String,
}

#[derive(Deserialize)]
struct FindRootRequest {
    expression: String,
    interval_start: f64,
    interval_end: f64,
    tolerance: f64,
}

#[derive(Serialize)]
struct ApiResponse {
    result: f64,
    execution_time_ms: f64,
    success: bool,
    error: Option<String>,
}

// GET /health
async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "healthy"})))
}

// POST /evaluate
async fn evaluate_handler(Json(payload): Json<EvaluateRequest>) -> impl IntoResponse {
    let start = Instant::now();

    if payload.expression.trim().is_empty() {
        let duration = start.elapsed().as_millis() as f64;
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                result: 0.0,
                execution_time_ms: duration,
                success: false,
                error: Some("Expression cannot be empty".to_string()),
            }),
        );
    }

    match evaluate(&payload.expression) {
        Ok(result) => {
            let duration = start.elapsed().as_millis() as f64;
            println!("SUCCESS: {} = {}", payload.expression, result);
            (
                StatusCode::OK,
                Json(ApiResponse {
                    result,
                    execution_time_ms: duration,
                    success: true,
                    error: None,
                }),
            )
        }
        Err(e) => {
            let duration = start.elapsed().as_millis() as f64;
            println!("ERROR: {} - {}", payload.expression, e);
            (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    result: 0.0,
                    execution_time_ms: duration,
                    success: false,
                    error: Some(format!("Evaluation error: {}", e)),
                }),
            )
        }
    }
}

// POST /find_root
async fn find_root_handler(Json(payload): Json<FindRootRequest>) -> impl IntoResponse {
    let start = Instant::now();

    if payload.interval_start >= payload.interval_end {
        let duration = start.elapsed().as_millis() as f64;
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                result: 0.0,
                execution_time_ms: duration,
                success: false,
                error: Some("interval_start must be less than interval_end".to_string()),
            }),
        );
    }

    if payload.tolerance <= 0.0 {
        let duration = start.elapsed().as_millis() as f64;
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                result: 0.0,
                execution_time_ms: duration,
                success: false,
                error: Some("tolerance must be > 0.0".to_string()),
            }),
        );
    }

    match find_root(&payload.expression, payload.interval_start, payload.interval_end, payload.tolerance) {
        Ok(result) => {
            let duration = start.elapsed().as_millis() as f64;
            println!("ROOT FOUND: {} = {}", payload.expression, result);
            (
                StatusCode::OK,
                Json(ApiResponse {
                    result,
                    execution_time_ms: duration,
                    success: true,
                    error: None,
                }),
            )
        }
        Err(e) => {
            let duration = start.elapsed().as_millis() as f64;
            println!("ROOT ERROR: {} - {}", payload.expression, e);
            (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    result: 0.0,
                    execution_time_ms: duration,
                    success: false,
                    error: Some(format!("Root finding error: {}", e)),
                }),
            )
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/evaluate", post(evaluate_handler))
        .route("/find_root", post(find_root_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Starting eRock Server on http://0.0.0.0:3000");
    println!("📊 Endpoints: GET /health, POST /evaluate, POST /find_root");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
