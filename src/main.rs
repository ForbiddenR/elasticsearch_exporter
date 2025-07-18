use std::sync::Arc;

use anyhow::Result;
use axum::{Router, routing::get};
use elasticsearch_exporter::{exporter, router};
use tokio::net::TcpListener;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/heartbeat", get(router::heartbeat))
        .route("/metrics", get(router::metric))
        .with_state(Arc::new(RwLock::new(exporter::Collect::new())));

    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
