use std::net::SocketAddr;

use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct Hello {
    message: &'static str,
}

async fn hello() -> Json<Hello> {
    Json(Hello {
        message: "Hello, world!",
    })
}

fn app() -> Router {
    Router::new().route("/api/hello", get(hello))
}

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Keepsake server listening on http://{addr}");
    axum::serve(listener, app()).await.unwrap();
}
