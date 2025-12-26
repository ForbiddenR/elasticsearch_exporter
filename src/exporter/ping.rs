use anyhow::Result;
use prometheus::proto::MetricFamily;

use crate::{config::Conf, query};

const ENDPOINT: &str = "/";

pub struct Ping {}

impl Ping {
    pub(super) fn new() -> Self {
        Self {}
    }

    // only check the node status from http://ip:port/. the node is health when
    // the status of the json resposne is ok
    pub(super) async fn collect(&self, conf: &Conf) -> Result<Vec<MetricFamily>> {
        Ok(query!(ok conf.addr, &conf.username, &conf.password))
    }
}
