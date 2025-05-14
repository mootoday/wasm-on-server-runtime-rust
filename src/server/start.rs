use std::net::SocketAddr;

use anyhow::Result;
use axum::{Router, extract::DefaultBodyLimit, routing::post};
use tracing::info;

use crate::AppState;
use crate::server::run::post::handle as run_post_handle;

pub async fn start(app_state: AppState) -> Result<()> {
    let app = Router::new()
        .route(
            "/run",
            post(run_post_handle).layer(DefaultBodyLimit::max(50 * 1024 * 1024)),
        )
        .with_state(app_state);

    let addr_str = "127.0.0.1:8080";
    let addr: SocketAddr = addr_str.parse()?;

    info!("Starting server on {}", addr_str);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}
