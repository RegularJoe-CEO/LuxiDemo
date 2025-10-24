use reqwest::Client;
use serde_json::json;
use std::time::Instant;
use tokio::time::sleep;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let url = "http://127.0.0.1:50051/evaluate";
    let body = json!({
        "type": "evaluate",
        "expr": "x^2 - 4",
        "x": [3.0, 4.0, 5.0],
        "vars": {}
    });

    let start = Instant::now();
    let mut handles = vec![];

    for i in 0..1000 {
        let client = client.clone();
        let body = body.clone();
        let handle = tokio::spawn(async move {
            let _ = client.post(url).json(&body).send().await;
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    let duration = start.elapsed();
    println!("1000 concurrent requests: {} ms", duration.as_millis());
    println!("Throughput: {} req/s", 1000.0 / duration.as_secs_f64());
}
