mod exe;
mod runner;
mod loader;
mod models;

use std::env;
use dotenvy::dotenv;
use axum::http::StatusCode;
use tokio::net::TcpListener;
use axum::{Router, routing::{get, post}, Json};

use crate::models::{Resp, Req};
use crate::exe::execute_code;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or("8000".into());
    let host = env::var("HOST").unwrap_or("127.0.0.1".into());

    let app = Router::new()
        .route("/", get(handler))
        .route("/execute/", post(execution_handler));

    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr).await.unwrap();

    println!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "UP!"
}

async fn execution_handler(Json(req): Json<Req>) -> Result<Json<Resp>, StatusCode> {
    let result = execute_code(req).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
}
