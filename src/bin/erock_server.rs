use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;
use rhai::{Engine, Scope};
use serde_json::{json, Value};
use std::collections::HashMap;

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:50051".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr).await?;
    println!("Server running on {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut engine = Engine::new();
        engine.disable_symbol("^");
        engine.register_custom_operator("^", 160).unwrap();
        engine.register_fn("^", |a: f64, b: f64| a.powf(b));
        engine.register_fn("^", |a: f64, b: i64| a.powf(b as f64));

        let mut buffer = [0; 1024];
        let n = socket.read(&mut buffer).await.unwrap();
        let request: Value = serde_json::from_slice(&buffer[..n]).unwrap();
        let type_ = request["type"].as_str().unwrap_or("evaluate");
        let expr = request["expr"].as_str().unwrap().to_string();
        let vars: HashMap<String, f64> = request["vars"].as_object().unwrap_or(&serde_json::Map::new()).iter()
            .map(|(k, v)| (k.clone(), v.as_f64().unwrap())).collect();

        let mut scope = Scope::new();
        for (k, v) in &vars {
            scope.push(k.clone(), *v);
        }

        let response = if type_ == "evaluate" {
            let x: Vec<f64> = request["x"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
            let mut y = Vec::new();
            for &val in &x {
                scope.set_value("x", val);
                let res: f64 = engine.eval_with_scope(&mut scope, &expr).unwrap();
                y.push(res);
            }
            json!({ "y": y })
        } else if type_ == "bisect" {
            let lo = request["lo"].as_f64().unwrap_or(0.0);
            let hi = request["hi"].as_f64().unwrap_or(1.0);
            let tol = request["tol"].as_f64().unwrap_or(1e-6);
            let mut a = lo;
            let mut b = hi;
            let mut scope_a = Scope::new();
            let mut scope_b = Scope::new();
            for (k, v) in &vars {
                scope_a.push(k.clone(), *v);
                scope_b.push(k.clone(), *v);
            }
            let mut mid = 0.0;
            for _ in 0..64 {
                mid = (a + b) / 2.0;
                scope_a.set_value("x", mid);
                let f_mid: f64 = engine.eval_with_scope(&mut scope_a, &expr).unwrap();
                if f_mid.abs() < tol {
                    break;
                }
                scope_b.set_value("x", a);
                let f_a: f64 = engine.eval_with_scope(&mut scope_b, &expr).unwrap();
                if f_a * f_mid < 0.0 {
                    b = mid;
                } else {
                    a = mid;
                }
            }
            json!({ "root": mid })
        } else {
            json!({ "error": "Unknown type" })
        };
        let response_str = serde_json::to_string(&response).unwrap();
        socket.write_all(response_str.as_bytes()).await.unwrap();
        socket.flush().await.unwrap();
    }
}
