mod lang_config;
mod exe;
mod models;
mod runner;

use tokio::net::TcpListener;
use axum::{Router, routing::{get, post}, Json};
use std::net::SocketAddr;
use models::Req;
use crate::exe::execute_code;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(handler))
        .route("/execute/", post(execution_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "Hello world!"
}

async fn execution_handler(Json(req): Json<Req>) -> String {
    println!("received");
    execute_code(req).await.unwrap_or_else(|e| e)
}
