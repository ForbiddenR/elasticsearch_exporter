use crate::{config::Conf, initializing, response::cluster_health::ClusterHealthResponse};
use anyhow::Result;
use prometheus::{GaugeVec, Opts, core::Collector, proto::MetricFamily};

use crate::{prefix_gauge_vec, query};

const ACTIVE_COLOR: &str = "green";
const ENDPOINT: &str = "/_cluster/health";
const DEFAULT_LABEL: &[&str; 0] = &[];

initializing!(ClusterHealthStatusMetric, ClusterHealthResponse);

pub struct ClusterHealth {
    status_metric: ClusterHealthStatusMetric,
}

impl ClusterHealth {
    pub fn new() -> Self {
        let prefix = "cluster_health";

        let status_metric = ClusterHealthStatusMetric::new(
            prefix_gauge_vec!(
                prefix,
                "ok",
                "Elasticsearch cluster status is green (1 if green, 0 otherwise)",
                DEFAULT_LABEL
            ),
            |n| (n.status == ACTIVE_COLOR) as u8 as f64,
        );
        Self {
            // metrics,
            status_metric,
        }
    }

    // get cluster info from http://ip:port/_cluster/health
    pub async fn collect(&self, conf: &Conf) -> Result<Vec<MetricFamily>> {
        let resp: ClusterHealthResponse = query!(json conf.addr, &conf.username, &conf.password);

        Ok({
            self.status_metric
                .gauge_vec
                .with_label_values(DEFAULT_LABEL)
                .set((self.status_metric.value_fn)(&resp));
            self.status_metric.gauge_vec.collect()
        })

        // Ok(result)
    }
}
