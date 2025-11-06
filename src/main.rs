use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use env_logger::Env;
use rhai::{Engine, Scope};
use serde::{Deserialize, Serialize};
use std::env;

mod math;
use math::{bisect, bisect_auto, BisectReq};

#[derive(Deserialize)]
struct EvalReq {
    expr: String,
    #[serde(flatten)]
    vars: serde_json::Map<String, serde_json::Value>,
}

#[derive(Serialize)]
struct EvalResp { result: f64 }

#[derive(Serialize)]
struct ErrMsg { error: String }
fn err_json(msg: &str) -> ErrMsg { ErrMsg { error: msg.to_string() } }

#[get("/ping")]
async fn ping() -> impl Responder { HttpResponse::Ok().body("pong") }

#[post("/evaluate")]
async fn evaluate(payload: web::Json<EvalReq>) -> impl Responder {
    let engine = Engine::new();
    let mut scope = Scope::new();
    for (k, v) in payload.vars.iter() {
        if let Some(n) = v.as_f64() { scope.set_value(k.to_string(), n); }
    }
    match engine.eval_with_scope::<rhai::Dynamic>(&mut scope, &payload.expr) {
        Ok(d) => {
            let n = d.cast::<f64>();
            if !n.is_finite() { return HttpResponse::BadRequest().json(err_json("non-finite result")); }
            HttpResponse::Ok().json(EvalResp { result: n })
        }
        Err(e) => HttpResponse::BadRequest().json(err_json(&format!("eval error: {e}"))),
    }
}

#[post("/bisect")]
async fn bisect_handler(payload: web::Json<BisectReq>) -> impl Responder {
    let engine = Engine::new();
    match bisect(&engine, &payload.into_inner()) {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::BadRequest().json(err_json(&e.to_string())),
    }
}

#[post("/bisect_auto")]
async fn bisect_auto_handler(payload: web::Json<BisectReq>) -> impl Responder {
    let engine = Engine::new();
    match bisect_auto(&engine, payload.into_inner()) {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::BadRequest().json(err_json(&e.to_string())),
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let port = env::var("PORT").ok().and_then(|p| p.parse::<u16>().ok()).unwrap_or(8080);
    let bind = format!("0.0.0.0:{port}");
    println!("✅ Starting Luxi microservice on http://{bind}");
    HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(evaluate)
            .service(bisect_handler)
            .service(bisect_auto_handler)
    })
    .bind(bind)?
    .run()
    .await?;
    Ok(())
}
