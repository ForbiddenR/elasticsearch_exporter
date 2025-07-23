use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use prometheus::{Encoder, TextEncoder};
use tokio::sync::RwLock;

use crate::{exporter, header::ExplicitHeader};

pub async fn heartbeat() -> StatusCode {
    StatusCode::OK
}

pub async fn metric(
    State(exporter): State<Arc<RwLock<exporter::Collect>>>,
    header: ExplicitHeader,
) -> Response {
    let metrics = { exporter.write().await.collect(&header).await };

    let encoder = TextEncoder::new();
    let mut buffer = vec![];

    match encoder.encode(&metrics, &mut buffer) {
        Err(e) => {
            println!("{e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
        _ => Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", encoder.format_type())
            .body(buffer.into())
            .unwrap(),
    }
}
