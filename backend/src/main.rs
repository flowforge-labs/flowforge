use axum::{routing::get, Router, serve};
use std::net::SocketAddr;
use tokio::net::TcpListener;


mod models;
mod ingest;

#[tokio::main]
async fn main() {
    // ingest.rs test
    let test_event = ingest::ingest_file("samples/test.json").unwrap();
    println!("Ingested {} events", test_event.len());

    let app = Router::new()
        .route("/health", get(|| async { "OK" }));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("FlowForge running at http://127.0.0.1:8080");

    serve(listener, app).await.unwrap();
}
