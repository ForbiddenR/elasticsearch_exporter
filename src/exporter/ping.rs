use anyhow::Result;
use prometheus::proto::MetricFamily;

use crate::{config::Conf, query};

const ENDPOINT: &str = "/";

pub struct Ping {}

impl Ping {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn collect(&self, conf: &Conf) -> Result<Vec<MetricFamily>> {
        Ok(query!(ok conf.addr, &conf.username, &conf.password))
    }
}
