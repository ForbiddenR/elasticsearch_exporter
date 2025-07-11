use axum::http::StatusCode;

pub async fn heartbeat() -> StatusCode {
    StatusCode::OK
}
