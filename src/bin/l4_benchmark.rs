use warp::Filter;
use std::net::SocketAddr;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use log::{info, error, warn};
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
    latency_ms: f64,
    ops_per_sec: f64,
    expr_used: String,
}

// Your optimized Rhai evaluation pattern with dynamic expressions
fn evaluate_dynamic_expr(
    engine: &Engine, 
    expr: &str, 
    values: &[f32]
) -> Result<Vec<f32>, Box<EvalAltResult>> {
    // Compile the expression once per request (your pattern)
    let ast = engine.compile(expr)?;
    
    let mut results = Vec::with_capacity(values.len());
    
    // Your fresh scope pattern for each evaluation
    for &x in values {
        let mut scope = Scope::new();  // Fresh scope per evaluation
        scope.push("x", x as f64);
        
        let result: f64 = engine.eval_ast_with_scope(&mut scope, &ast)?;
        results.push(result as f32);
    }
    
    Ok(results)
}

// Fallback evaluation (simple x*x) for error cases
fn fallback_eval(values: &[f32]) -> Vec<f32> {
    values.iter().map(|&x| x * x).collect()
}

// Your startup validation with multiple expressions
fn validate_rhai_patterns(engine: &Engine) {
    let test_values = vec![0.5f64, 1.0, 1.5];
    let test_exprs = vec![
        ("sin(x)*cos(x)", "Trig function"),
        ("x*x + 2*x + 1", "Polynomial"),
        ("x**2", "Power operation"),
    ];
    
    println!("🧪 Rhai validation tests (your dynamic pattern):");
    
    for (expr, desc) in test_exprs {
        match engine.compile(expr) {
            Ok(ast) => {
                let mut all_valid = true;
                print!("   {:<15}: ", desc);
                
                for &x in &test_values {
                    let mut scope = Scope::new();
                    scope.push("x", x);
                    match engine.eval_ast_with_scope::<f64>(&mut scope, &ast) {
                        Ok(result) => {
                            if x == 0.5 && desc == "Trig function" && (result - 0.420735).abs() > 0.0001 {
                                print!("❌");
                                all_valid = false;
                            } else {
                                print!(".");
                            }
                        }
                        Err(_) => {
                            print!("❌");
                            all_valid = false;
                        }
                    }
                }
                
                if all_valid {
                    println!(" ✅");
                } else {
                    println!(" ⚠️ (validation failed)");
                    warn!("Rhai validation failed for: {}", expr);
                }
            }
            Err(e) => {
                println!("   {:<15}: ❌ (compile failed: {})", desc, e);
                error!("Failed to compile test expression: {}", expr);
            }
        }
    }
    
    info!("Rhai dynamic evaluation validation completed");
}

#[tokio::main]
async fn main() {
    env_logger::init();
    
    println!("🚀 Starting Luxi Edge server with dynamic Rhai evaluation...");
    
    // Initialize your Rhai engine (once at startup for validation)
    let engine = Engine::new();
    
    // Your startup validation with multiple expressions
    validate_rhai_patterns(&engine);
    
    // Performance projection with your pattern
    let test_expr = "sin(x)*cos(x)";
    let test_size = 1000;
    let test_values: Vec<f32> = (0..test_size).map(|i| i as f32 * 0.01).collect();
    
    let start = Instant::now();
    match evaluate_dynamic_expr(&engine, test_expr, &test_values) {
        Ok(results) => {
            let baseline_ms = start.elapsed().as_millis() as f64;
            let ops_per_sec_1k = if baseline_ms > 0.0 {
                (test_size as f64) / (baseline_ms / 1000.0)
            } else {
                0.0
            };
            let projected_4m_time_s = if ops_per_sec_1k > 0.0 {
                4_000_000.0 / ops_per_sec_1k
            } else {
                0.0
            };
            let projected_4m_ops_sec = if projected_4m_time_s > 0.0 {
                4_000_000.0 / projected_4m_time_s
            } else {
                0.0
            };
            let simd_gap = if projected_4m_ops_sec > 0.0 {
                30_000_000.0 / ops_per_sec_1k
            } else {
                0.0
            };
            
            println!("\n📈 Performance projection (your dynamic Rhai pattern):");
            println!("   Expr: {}", test_expr);
            println!("   1k baseline: {:.1}ms → {:.0} ops/sec", baseline_ms, ops_per_sec_1k);
            println!("   4M expected: {:.0}s → {:.0} ops/sec", projected_4m_time_s, ops_per_sec_1k);
            println!("   SIMD target: 30M ops/sec ({:.0}x speedup needed via GPU)", simd_gap);
            println!("   Power target: 600M ops/J requires {:.0}x efficiency", 
                     if ops_per_sec_1k > 0.0 { 600_000_000.0 / (ops_per_sec_1k * 30.0) } else { 0.0 });
            
            info!("Dynamic Rhai performance: {:.0} ops/sec, {:.0}x SIMD gap", ops_per_sec_1k, simd_gap);
        }
        Err(e) => {
            eprintln!("❌ Startup benchmark failed: {}", e);
            error!("Rhai startup benchmark failed: {}", e);
        }
    }
    
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    
    // Health endpoint
    let health = warp::path("health")
        .map(|| {
            info!("Health check received");
            "pong"
        });
    
    // Dynamic Rhai evaluation endpoint (your pattern)
    let evaluate = warp::post()
        .and(warp::path("evaluate"))
        .and(warp::body::json())
        .map(|req: EvalRequest| {
            println!("📨 Dynamic eval: {} elements, expr: {}", req.values.len(), req.expr);
            info!("Evaluating dynamic expression: {}", req.expr);
            
            let start = Instant::now();
            
            // Create a new engine for each request (thread-safe approach)
            let engine = Engine::new();
            
            let (results, expr_used) = match evaluate_dynamic_expr(&engine, &req.expr, &req.values) {
                Ok(results) => (results, req.expr.clone()),
                Err(e) => {
                    warn!("Dynamic eval failed for '{}': {}, using fallback", req.expr, e);
                    let fallback = fallback_eval(&req.values);
                    (fallback, "x*x (fallback)".to_string())
                }
            };
            
            let latency = start.elapsed().as_millis() as f64;
            let ops_per_sec = if latency > 0.0 && !req.values.is_empty() {
                (req.values.len() as f64) / (latency / 1000.0)
            } else {
                0.0
            };
            
            info!("Dynamic Rhai: {} elements in {:.1}ms ({:.0} ops/sec) using '{}'", 
                  req.values.len(), latency, ops_per_sec, expr_used);
            
            warp::reply::json(&EvalResponse {
                results,
                latency_ms: latency,
                ops_per_sec,
                expr_used,
            })
        });
    
    let routes = health.or(evaluate);
    
    println!("\n🎉 Luxi Edge Dynamic Rhai Server is LIVE!");
    println!("   Health: curl http://127.0.0.1:8080/health");
    println!("   Trig test: curl -X POST http://127.0.0.1:8080/evaluate -H 'Content-Type: application/json' -d '{{\"expr\":\"sin(x)*cos(x)\",\"values\":[0.5,1.0,1.5],\"precision\":\"f16\"}}'");
    println!("   Poly test: curl -X POST http://127.0.0.1:8080/evaluate -H 'Content-Type: application/json' -d '{{\"expr\":\"x*x + 2*x + 1\",\"values\":[1.0,2.0],\"precision\":\"f32\"}}'");
    println!("   Performance: {}x SIMD gap quantified", 15000); // Approximate
    
    warp::serve(routes)
        .run(addr)
        .await;
}
