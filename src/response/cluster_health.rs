use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ClusterHealthResponse {
    pub status: String,
}
