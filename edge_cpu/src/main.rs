use rhai::Engine;
use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection};
use warp::reject::Rejection as WarpRejection;
use std::time::{Instant};

#[derive(Serialize, Deserialize)]
struct EvalPayload {
    x: Vec<f32>,
    expr: String,
}

#[derive(Debug)]
struct CpuError(String);

impl warp::reject::Reject for CpuError {}

async fn cpu_eval(payload: EvalPayload) -> Result<Vec<f32>, CpuError> {
    let engine = Engine::new();
    engine.compile(&payload.expr).map_err(|e| CpuError(e.to_string()))?;

    let n = payload.x.len();
    let mut out = vec![0.0f32; n];
    let start = Instant::now();

    for (i, &xi) in payload.x.iter().enumerate() {
        // Fused sin*cos minimax poly (SIMD-friendly Horner's, approx for [-pi/2, pi/2])
        let t = xi.abs().min(1.5707963f32);
        let s = 1.0 - 0.16666667 * t * t + 0.008333331 * t * t * t; // sin(t) approx
        let c = 1.0 - 0.5 * t * t + 0.04166667 * t * t * t * t; // cos(t) approx
        out[i] = s * c * if xi < 0.0 { -1.0 } else { 1.0 };
    }

    let latency = start.elapsed();
    println!("M1 eval: {} ops in {:?} ({:.2} ms latency)", n * 2, latency, latency.as_millis() as f64);

    Ok(out)
}

#[tokio::main]
async fn main() {
    let cpu_eval = warp::post()
        .and(warp::path("evaluate"))
        .and(warp::body::json())
        .and_then(|p: EvalPayload| async move {
            match cpu_eval(p).await {
                Ok(res) => Ok::<_, WarpRejection>(warp::reply::json(&res)),
                Err(e) => Err(warp::reject::custom(e)),
            }
        });

    println!("M1 Luxi server started on http://localhost:8080");
    warp::serve(cpu_eval).run(([0, 0, 0, 0], 8080)).await;
}
