pub mod client;
pub mod config;
pub mod exporter;
pub mod header;
pub mod response;
pub mod router;

pub async fn run() -> anyhow::Result<()> {
    use axum::{Router, routing::get};
    use config::Conf;
    use std::sync::Arc;
    use tokio::net::TcpListener;
    use tokio::sync::RwLock;

    let app = Router::new()
        .route("/heartbeat", get(router::heartbeat))
        .route("/metrics", get(router::metric))
        .with_state(Arc::new(RwLock::new(
            exporter::Collect::new(Conf::build()?),
        )));

    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    Ok(axum::serve(listener, app).await?)
}
