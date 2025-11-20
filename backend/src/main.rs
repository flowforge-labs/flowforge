use axum::{Router, routing::{get, post}, serve};
use tokio::net::TcpListener;
// consider bacon for code watching/testing

mod ingest;
mod models;
mod handlers;

#[tokio::main]
async fn main() {
    // ingest.rs test
    let test_event = ingest::ingest_file("samples/test.txt").unwrap();
    println!("Ingested {} events", test_event.len());

    let app = Router::new()
    .route("/health", get(|| async { "OK" }))
    .route("/api/reload", post(handlers::reload_logs_handler));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("FlowForge running at http://127.0.0.1:8080");

    serve(listener, app).await.unwrap();
}
