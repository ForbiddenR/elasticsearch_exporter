use crate::{config::Conf, initializing, prefix_gauge_vec, query, response::shards::ShardResposne};
use anyhow::Result;
use prometheus::{GaugeVec, Opts, core::Collector, proto::MetricFamily};

const ENDPOINT: &str = "/_cat/shards";
const QUERY: &[(&str, &str); 1] = &[("format", "json")];

initializing!(NodeShardMetric, ShardResposne);


pub struct Shards {
    metrics: Vec<NodeShardMetric>,
}

impl Shards {
    pub fn new() -> Self {
        let default_labels = &["index"];
        let metrics = vec![NodeShardMetric::new(
            prefix_gauge_vec!("docs", "The docs number in target index", default_labels),
            |s| s.docs.as_ref().and_then(|f| f.parse().ok()).unwrap_or(0.0),
        )];

        Self { metrics }
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
                        .with_label_values(&[&r.index])
                        .add((f.value_fn)(r));
                });
            }
        }

        Ok(self
            .metrics
            .iter()
            .flat_map(|f| f.gauge_vec.collect())
            .collect())
    }
}
