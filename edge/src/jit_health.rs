use serde_json::{json, Value};

#[allow(dead_code)]
pub fn payload() -> Value {
    let (jit_avail, jit_en, jit_reason) = erock::health_fields();
    let service = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    json!({
        "service": service,
        "status": "ok",
        "version": version,
        "jit_available": jit_avail,
        "jit_enabled": jit_en,
        "jit_reason": jit_reason
    })
}
