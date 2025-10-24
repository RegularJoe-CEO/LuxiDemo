use reqwest::Client;
use serde_json::json;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "http://127.0.0.1:3000/evaluate";
    let body = json!({
        "expression": "x^2 - 4",
        "x": [3.0, 4.0],
        "vars": {}
    });
    let start = Instant::now();
    let response = client.post(url).json(&body).send().await?;
    let result: serde_json::Value = response.json().await?;
    let duration = start.elapsed().as_secs_f64();
    println!("Evaluate: {:?}, Time: {:.6} s", result["y"], duration);

    let bisect_url = "http://127.0.0.1:3000/bisect";
    let bisect_body = json!({
        "expression": "x^2 - 4",
        "x": [1.0, 3.0], // lo, hi
        "vars": {}
    });
    let start = Instant::now();
    let response = client.post(bisect_url).json(&bisect_body).send().await?;
    let result: serde_json::Value = response.json().await?;
    let duration = start.elapsed().as_secs_f64();
    println!("Root: {}, Iterations: {}, Time: {:.6} s", result["root"], result["iterations"], duration);

    Ok(())
}
