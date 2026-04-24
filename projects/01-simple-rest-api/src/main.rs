use axum::{
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct Status {
    status: String,
    versao: String,
    sistema: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Bem-vindo ao Lab de Sistemas Distribuídos!" }))
        .route("/health", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Servidor rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<Status> {
    let feedback = Status {
        status: "UP".to_string(),
        versao: "v1".to_string(),
        sistema: "Rust Distributed Lab".to_string(),
    };
    Json(feedback)
}
