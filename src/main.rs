mod api;
mod core;
mod state;
mod config;
mod languages;
mod utils;

use std::sync::Arc;

use tracing::info;
use tokio::net::TcpListener;

use state::AppState;
use config::constants::{EPHEMERAL_WORKER_COUNT, PERSISTENT_WORKER_COUNT, HOST, PORT};

#[tokio::main(flavor="multi_thread")]
async fn main() {
    config::bootstrap();

    let state = Arc::new(
        AppState::new(*EPHEMERAL_WORKER_COUNT, *PERSISTENT_WORKER_COUNT)
    );
    let app = api::create_router(state);

    let addr = format!("{}:{}", *HOST, *PORT);
    let listener = TcpListener::bind(&addr).await.unwrap();


    info!("Listening on: {}", addr);

    axum::serve(listener, app).await.unwrap();
}
