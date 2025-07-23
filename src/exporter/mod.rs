use prometheus::{Gauge, core::Collector, proto::MetricFamily};

use crate::{
    config::Conf,
    exporter::{cluster_health::ClusterHealth, ping::Ping, shards::Shards},
};
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
    ($name: literal, $help:literal, $tags:expr) => {
        GaugeVec::new(Opts::new($name, $help), $tags)
            .expect("Could not create gauge")
    };
}

#[macro_export]
macro_rules! initializing {
    ($name:ident, $t:ty) => {
        #[derive(Debug)]
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

pub struct Collect {
    config: Conf,
    cluster_health: ClusterHealth,
    ping: Ping,
    shards: Shards,
    up: Gauge,
}

impl Collect {
    pub fn new(config: Conf) -> Self {
        Self {
            cluster_health: ClusterHealth::new(),
            shards: Shards::new(),
            ping: Ping::new(),
            up: prefix_gauge!("node_status", "Was the last scrape of rabbitmq successful."),
            config,
        }
    }

    async fn all(&self) -> Result<Vec<MetricFamily>> {
        let mut result = Vec::from(self.cluster_health.collect(&self.config).await?);
        result.extend(self.shards.collect(&self.config).await?);
        Ok(result)
    }

    async fn ping(&self) -> Result<Vec<MetricFamily>> {
        Ok(Vec::from(self.ping.collect(&self.config).await?))
    }

    pub async fn collect(&self, all: bool) -> Vec<MetricFamily> {
        match if all || self.config.all_exporters() {
            self.all().await
        } else {
            self.ping().await
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
