use tokio::net::TcpStream;
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Evaluate
    let mut stream = TcpStream::connect("127.0.0.1:50051").await?;
    let request = json!({
        "type": "evaluate",
        "expr": "x^2 - 4",
        "x": [3.0, 4.0],
        "vars": {}
    });
    stream.write_all(request.to_string().as_bytes()).await?;
    stream.flush().await?;

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let response: serde_json::Value = serde_json::from_slice(&buffer[..n]).unwrap();
    println!("Evaluate: {:?}", response["y"]);

    // Root finding (new connection)
    let mut stream = TcpStream::connect("127.0.0.1:50051").await?;
    let root_request = json!({
        "type": "bisect",
        "expr": "x^2 - 4",
        "lo": 1.0,
        "hi": 3.0,
        "tol": 1e-6,
        "vars": {}
    });
    stream.write_all(root_request.to_string().as_bytes()).await?;
    stream.flush().await?;

    let n = stream.read(&mut buffer).await?;
    let root_response: serde_json::Value = serde_json::from_slice(&buffer[..n]).unwrap();
    println!("Root: {}", root_response["root"]);

    Ok(())
}
