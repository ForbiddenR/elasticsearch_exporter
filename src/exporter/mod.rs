use prometheus::{Gauge, core::Collector, proto::MetricFamily};

use crate::exporter::{cluster_health::ClusterHealth, ping::Ping, shards::Shards};
use anyhow::Result;

mod cluster_health;
mod ping;
mod shards;
mod stats;

#[macro_export]
macro_rules! prefix_gauge {
    ($name: expr, $help:literal) => {
        Gauge::new($name, $help).expect("Could not create gauge")
    };
    ($prefix: expr, $name: literal, $help:literal) => {
        prefix_gauge!(&format!("{}_{}", $prefix, $name), $help)
    };
}

#[macro_export]
macro_rules! prefix_gauge_vec {
    ($prefix: expr, $name: literal, $help:literal, $tags:expr) => {
        GaugeVec::new(Opts::new($name, $help).subsystem($prefix), $tags)
            .expect("Could not create gauge")
    };
}

pub struct Collect {
    cluster_health: ClusterHealth,
    ping: Ping,
    shards: Shards,
    up: Gauge,
}

impl Collect {
    pub fn new() -> Self {
        Self {
            cluster_health: ClusterHealth::new(),
            shards: Shards::new(),
            ping: Ping::new(),
            up: prefix_gauge!("node_status", "Was the last scrape of rabbitmq successful."),
        }
    }

    async fn all(&self) -> Result<Vec<MetricFamily>> {
        let base = "http://10.43.0.30:9200";
        let mut result = Vec::from(self.cluster_health.collect(base).await?);
        result.extend(self.shards.collect(base).await?);
        Ok(result)
    }

    async fn ping(&self) -> Result<Vec<MetricFamily>> {
        let base = "http://10.43.0.30:9200";
        Ok(Vec::from(self.ping.collect(base).await?))
    }

    pub async fn collect(&self, all: &Option<String>) -> Vec<MetricFamily> {
        // let result =
        match match all {
            None => self.ping().await,
            _ => self.all().await,
        } {
            Ok(mut m) => {
                self.up.set(1.0);
                m.extend(self.up.collect());
                m
            }
            Err(e) => {
                println!("{e}");
                self.up.set(0.0);
                self.up.collect()
            }
        }
        .into_iter()
        .filter(|f| !f.get_metric().is_empty())
        .collect()
    }
}
