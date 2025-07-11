use axum::{Router, routing::get};
use elasticsearch_exporter::error::Result;
use elasticsearch_exporter::router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/heartbeat", get(router::heartbeat));

    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
