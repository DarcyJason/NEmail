mod config;
mod routes;
mod handlers;
mod models;
mod database;
mod middleware;
mod services;
mod utils;
mod tasks;
mod errors;


use axum::{http::{HeaderValue, Method}, routing::get, serve, Router};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(json)).layer(
        CorsLayer::new()
            .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET]),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await.unwrap();
    serve(listener, app).await.unwrap();
}

async fn json() -> &'static str {
    "Hello, World!"
}
