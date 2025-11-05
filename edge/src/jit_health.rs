use axum::Json;
use serde_json::{json, Value};

#[allow(dead_code)]
pub async fn health_handler() -> Json<Value> {
    let (jit_avail, jit_en, jit_reason) = erock::health_fields();
    let service = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    Json(json!({
        "service": service,
        "status": "ok",
        "version": version,
        "jit_available": jit_avail,
        "jit_enabled": jit_en,
        "jit_reason": jit_reason
    }))
}
