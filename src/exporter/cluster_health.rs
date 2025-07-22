use crate::{config::Conf, response::cluster_health::ClusterHealthResponse};
use anyhow::Result;
use prometheus::{Gauge, GaugeVec, Opts, core::Collector, proto::MetricFamily};

use crate::{prefix_gauge, prefix_gauge_vec, query};

const COLORS: [&'static str; 3] = ["green", "yellow", "red"];
const ENDPOINT: &str = "/_cluster/health";

struct ClusterHealthMetric {
    gauge: Gauge,
    value_fn: fn(&ClusterHealthResponse) -> f64,
}

struct ClusterHealthStatusMetric {
    gauge_vec: GaugeVec,
    value_fn: fn(&ClusterHealthResponse, &str) -> f64,
}

impl ClusterHealthMetric {
    fn new(gauge: Gauge, value_fn: fn(&ClusterHealthResponse) -> f64) -> Self {
        Self { gauge, value_fn }
    }
}

impl ClusterHealthStatusMetric {
    fn new(gauge_vec: GaugeVec, value_fn: fn(&ClusterHealthResponse, &str) -> f64) -> Self {
        Self {
            gauge_vec,
            value_fn,
        }
    }
}

pub struct ClusterHealth {
    metrics: Vec<ClusterHealthMetric>,
    status_metric: ClusterHealthStatusMetric,
}

impl ClusterHealth {
    pub fn new() -> Self {
        let prefix = "cluster_health";
        let metrics = vec![
            ClusterHealthMetric::new(
                prefix_gauge!(
                    prefix,
                    "active_primary_shards",
                    "The number of primary shards in your cluster. This is an aggregate total across all indices."
                ),
                |n| n.active_primary_shards as f64,
            ),
            ClusterHealthMetric::new(
                prefix_gauge!(
                    prefix,
                    "active_shards",
                    "Aggregate total of all shards across all indices, which includes replica shards."
                ),
                |n| n.active_shards as f64,
            ),
            ClusterHealthMetric::new(
                prefix_gauge!(
                    prefix,
                    "delayed_unassigned_shards",
                    "Shards delayed to reduce reallocation overhead."
                ),
                |n| n.delayed_unassigned_shards as f64,
            ),
            ClusterHealthMetric::new(
                prefix_gauge!(
                    prefix,
                    "initializing_shards",
                    "Count of shards that are being freshly created."
                ),
                |n| n.initializing_shards as f64,
            ),
            ClusterHealthMetric::new(
                prefix_gauge!(
                    prefix,
                    "number_of_data_nodes",
                    "Number of data nodes in the cluster."
                ),
                |n| n.number_of_data_nodes as f64,
            ),
            ClusterHealthMetric::new(
                prefix_gauge!(
                    prefix,
                    "number_of_in_flight_fetch",
                    "The number of ongoing shard info requests."
                ),
                |n| n.number_of_in_flight_fetch as f64,
            ),
            ClusterHealthMetric::new(
                prefix_gauge!(
                    prefix,
                    "task_max_waiting_in_queue_millis",
                    "Tasks max time waiting in queue."
                ),
                |n| n.task_max_waiting_in_queue_millis as f64,
            ),
            ClusterHealthMetric::new(
                prefix_gauge!(prefix, "number_of_nodes", "Number of nodes in the cluster."),
                |n| n.number_of_nodes as f64,
            ),
            ClusterHealthMetric::new(
                prefix_gauge!(
                    prefix,
                    "number_of_pending_tasks",
                    "Cluster level changes which have not yet been executed."
                ),
                |n| n.number_of_pending_tasks as f64,
            ),
            ClusterHealthMetric::new(
                prefix_gauge!(
                    prefix,
                    "relocating_shards",
                    "The number of shards that are currently moving from one node to another node."
                ),
                |n| n.relocating_shards as f64,
            ),
            ClusterHealthMetric::new(
                prefix_gauge!(
                    prefix,
                    "unassigned_shards",
                    "The number of shards that exist in the cluster state, but cannot be found in the cluster itself."
                ),
                |n| n.unassigned_shards as f64,
            ),
        ];

        let status_metric = ClusterHealthStatusMetric::new(
            prefix_gauge_vec!(
                prefix,
                "status",
                "Whether all primary and replica shards are allocated.",
                &["color"]
            ),
            |n, c| (n.status == c) as u8 as f64,
        );
        Self {
            metrics,
            status_metric,
        }
    }

    pub async fn collect(&self, conf: &Conf) -> Result<Vec<MetricFamily>> {
        let resp: ClusterHealthResponse = query!(json conf.addr, &conf.username, &conf.password);

        let mut result = vec![];

        for ClusterHealthMetric { gauge, value_fn } in &self.metrics {
            gauge.set(value_fn(&resp));
            result.extend(gauge.collect());
        }

        for color in COLORS {
            self.status_metric
                .gauge_vec
                .with_label_values(&[color])
                .set((self.status_metric.value_fn)(&resp, color));
        }

        result.extend(self.status_metric.gauge_vec.collect());

        Ok(result)
    }
}
