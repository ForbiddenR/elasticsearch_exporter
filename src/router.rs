use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use prometheus::{Encoder, TextEncoder};
use serde::Deserialize;
use tokio::sync::RwLock;

use crate::exporter;

pub async fn heartbeat() -> StatusCode {
    StatusCode::OK
}

#[derive(Debug, Deserialize)]
pub struct Params {
    enabled_exporters: Option<String>,
}

pub async fn metric(
    State(exporter): State<Arc<RwLock<exporter::Collect>>>,
    Query(params): Query<Params>,
) -> Response {
    let metrics;
    {
        metrics = exporter.write().await.collect(&params.enabled_exporters).await;
    }

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
