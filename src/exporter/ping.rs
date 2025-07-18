use anyhow::Result;
use prometheus::proto::MetricFamily;

use crate::query;

const ENDPOINT: &str = "/";

pub struct Ping {}

impl Ping {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn collect(&self, base: &str) -> Result<Vec<MetricFamily>> {
        Ok(query!(ok base))
    }
}
