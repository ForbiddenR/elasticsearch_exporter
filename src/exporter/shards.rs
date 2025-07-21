use crate::{config::Conf, prefix_gauge_vec, query, response::shards::ShardResposne};
use anyhow::Result;
use prometheus::{GaugeVec, Opts, core::Collector, proto::MetricFamily};

const ENDPOINT: &str = "/_cat/shards";
const QUERY: &[(&str, &str); 1] = &[("format", "json")];

macro_rules! initializing {
    ($name:ident, $t:ty) => {
        struct $name {
            gauge_vec: GaugeVec,
            value_fn: fn(&$t) -> f64,
        }

        impl $name {
            fn new(gauge_vec: GaugeVec, value_fn: fn(&$t) -> f64) -> Self {
                Self {
                    gauge_vec,
                    value_fn,
                }
            }
        }
    };
}

initializing!(NodeShardMetric, ShardResposne);

pub struct Shards {
    metrics: Vec<NodeShardMetric>,
    // total_metrics: Vec<NodeShardMetric>,
}

impl Shards {
    pub fn new() -> Self {
        let default_labels = &["node", "index", "state", "prirep"];
        // let default_labels = &["index", "state"];
        let metrics = vec![
            NodeShardMetric::new(
                prefix_gauge_vec!(
                    "",
                    "shard",
                    "The shard number in target index",
                    default_labels
                ),
                |_| 1.0,
            ),
            NodeShardMetric::new(
                prefix_gauge_vec!(
                    "",
                    "docs",
                    "The docs number in target index",
                    default_labels
                ),
                |s| s.docs.as_ref().and_then(|f| f.parse().ok()).unwrap_or(0.0),
            ),
        ];

        // let default_labels: &[&str; 0] = &[];

        // let total_metrics = vec![
        //     NodeShardMetric::new(
        //         prefix_gauge_vec!("", "shard_global", "The total shard number", default_labels),
        //         |_| 1.0,
        //     ),
        //     NodeShardMetric::new(
        //         prefix_gauge_vec!("", "docs_global", "The total docs number", default_labels),
        //         |s| s.docs.as_ref().and_then(|f| f.parse().ok()).unwrap_or(0.0),
        //     ),
        // ];
        Self {
            metrics,
            // total_metrics,
        }
    }

    pub async fn collect(&self, conf: &Conf) -> Result<Vec<MetricFamily>> {
        self.metrics
            .iter()
            // .chain(self.total_metrics.iter())
            .for_each(|f| f.gauge_vec.reset());

        let resp: Vec<ShardResposne> =
            query!(json conf.addr, &conf.username, &conf.password, QUERY);
        for r in &resp {
            if !r.index.starts_with(".") {
                self.metrics.iter().for_each(|f| {
                    f.gauge_vec
                        .with_label_values(&[&r.node, &r.index, &r.state, &r.prirep])
                        .add((f.value_fn)(r));
                });
                // self.total_metrics.iter().for_each(|f| {
                //     f.gauge_vec
                //         .with_label_values(EMPTY_LABELS)
                //         .add((f.value_fn)(r));
                // });
            }
        }

        Ok(self
            .metrics
            .iter()
            .flat_map(|f| f.gauge_vec.collect())
            .collect())
    }
}
