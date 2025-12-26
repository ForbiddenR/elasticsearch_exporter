use prometheus::{Gauge, core::Collector, proto::MetricFamily};

use crate::{
    config::{Conf, Mode},
    exporter::{cluster_health::ClusterHealth, ping::Ping, shards::Shards},
};
use anyhow::Result;

mod cluster_health;
mod ping;
mod shards;

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
        GaugeVec::new(Opts::new($name, $help), $tags).expect("Could not create gauge")
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

    // A cheif collector collects all metrics from different endpoints.
    // If some errors occur in any http request, it will just return node_status with 0 whether
    // exporter_mode is set to standard.
    pub(crate) async fn collect(&self, mode: &Option<Mode>) -> Vec<MetricFamily> {
        match match mode.as_ref().unwrap_or(&self.config.exporter_mode) {
            Mode::Standard => self.all().await,
            Mode::Simple => self.ping().await,
        } {
            Ok(m) => {
                self.up.set(1.0);
                m
            }
            Err(e) => {
                eprintln!("{e}");
                self.up.set(0.0);
                return self.up.collect();
            }
        }
        .into_iter()
        .filter(|f| !f.get_metric().is_empty())
        .chain(self.up.collect())
        .collect()
    }
}
