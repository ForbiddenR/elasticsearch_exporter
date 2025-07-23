use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ClusterHealthResponse {
    pub status: String,
}
