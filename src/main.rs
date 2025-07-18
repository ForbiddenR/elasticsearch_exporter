use std::sync::Arc;

use anyhow::Result;
use axum::{Router, routing::get};
use elasticsearch_exporter::config::Conf;
use elasticsearch_exporter::{exporter, router};
use tokio::net::TcpListener;
use tokio::sync::RwLock;

#[cfg(all(
    not(windows),
    not(target_os = "android"),
    not(target_os = "macos"),
    not(target_os = "freebsd"),
    not(target_os = "openbsd"),
    not(target_os = "illumos"),
    not(all(target_env = "musl", target_pointer_width = "32")),
    not(target_arch = "riscv64"),
    feature = "use-jemalloc"
))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/heartbeat", get(router::heartbeat))
        .route("/metrics", get(router::metric))
        .with_state(Arc::new(RwLock::new(
            exporter::Collect::new(Conf::build()?),
        )));

    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
