use std::vec;

use prometheus::{GaugeVec, Opts, core::Collector, proto::MetricFamily};

use crate::{
    client, prefix_gauge_vec, query,
    response::nodes::{
        NodeStatsBreakersResponse, NodeStatsFSDataResponse, NodeStatsFSIOStatsDeviceResponse,
        NodeStatsIndexingPressureResponse, NodeStatsJVMGCCollectorResponse, NodeStatsNodeResponse,
        NodeStatsThreadPoolResponse, NodeStatusResponse,
    },
};

const ENDPOINT: &str = "/_nodes/_local/stats";

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

initializing!(NodeMetric, NodeStatsNodeResponse);
initializing!(IndexingPressureMetric, NodeStatsIndexingPressureResponse);
initializing!(ThreadPoolMetric, NodeStatsThreadPoolResponse);
initializing!(GCCollectionMetric, NodeStatsJVMGCCollectorResponse);
initializing!(BreakerMetric, NodeStatsBreakersResponse);
initializing!(FilesystemDataMetric, NodeStatsFSDataResponse);
initializing!(FilesystemIODeviceMetric, NodeStatsFSIOStatsDeviceResponse);

pub struct Nodes {
    node_metrics: Vec<NodeMetric>,
    gc_collection_metrics: Vec<GCCollectionMetric>,
    breaker_metrics: Vec<BreakerMetric>,
    indexing_pressure_metrics: Vec<IndexingPressureMetric>,
    thread_pool_metrics: Vec<ThreadPoolMetric>,
    filesystem_data_metrics: Vec<FilesystemDataMetric>,
    filesystem_io_device_metrics: Vec<FilesystemIODeviceMetric>,
}

impl Nodes {
    pub fn new() -> Self {
        macro_rules! new {
            ($t:ident, $prefix:ident, $name:literal, $help:literal, $tags:expr, $value_fn:expr) => {
                $t::new(prefix_gauge_vec!($prefix, $name, $help, $tags), $value_fn)
            };
            (node, $prefix:ident, $name:literal, $help:literal, $tags:expr, $value_fn:expr $(,)?) => {
                new!(NodeMetric, $prefix, $name, $help, $tags, $value_fn)
            };
            (gc, $prefix:ident, $name:literal, $help:literal, $tags:expr, $value_fn:expr $(,)?) => {
                new!(GCCollectionMetric, $prefix, $name, $help, $tags, $value_fn)
            };
            (breaker, $prefix:ident, $name:literal, $help:literal, $tags:expr, $value_fn:expr $(,)?) => {
                BreakerMetric::new(prefix_gauge_vec!($prefix, $name, $help, $tags), $value_fn)
            };
            (idx, $prefix:ident, $name:literal, $help:literal, $tags:expr, $value_fn:expr $(,)?) => {
                new!(
                    IndexingPressureMetric,
                    $prefix,
                    $name,
                    $help,
                    $tags,
                    $value_fn
                )
            };
            (thread, $prefix:ident, $name:literal, $help:literal, $tags:expr, $value_fn:expr $(,)?) => {
                ThreadPoolMetric::new(prefix_gauge_vec!($prefix, $name, $help, $tags), $value_fn)
            };
            (file_data, $prefix:ident, $name:literal, $help:literal, $tags:expr, $value_fn:expr $(,)?) => {
                FilesystemDataMetric::new(
                    prefix_gauge_vec!($prefix, $name, $help, $tags),
                    $value_fn,
                )
            };
            (file_io, $prefix:ident, $name:literal, $help:literal, $tags:expr, $value_fn:expr $(,)?) => {
                FilesystemIODeviceMetric::new(
                    prefix_gauge_vec!($prefix, $name, $help, $tags),
                    $value_fn,
                )
            };
        }
        let default_node_labels = ["host", "name"];
        let os = "os";
        let indices = "indices";
        let indices_indexing = "indices_indexing";
        let node_metrics = vec![
            new!(
                node,
                os,
                "load1",
                "Shortterm load average",
                &default_node_labels,
                |n| n.os.cpu.load_average.load1 as f64,
            ),
            new!(
                node,
                os,
                "load5",
                "Midterm load average",
                &default_node_labels,
                |n| n.os.cpu.load_average.load5 as f64,
            ),
            new!(
                node,
                os,
                "load15",
                "Longterm load average",
                &default_node_labels,
                |n| n.os.cpu.load_average.load15 as f64,
            ),
            new!(
                node,
                os,
                "cpu_percent",
                "Percent CPU used by OS",
                &default_node_labels,
                |n| n.os.cpu.percent as f64,
            ),
            new!(
                node,
                os,
                "mem_free_bytes",
                "Amount of free physical memory in bytes",
                &default_node_labels,
                |n| n.os.mem.free_in_bytes as f64,
            ),
            new!(
                node,
                os,
                "mem_used_bytes",
                "Amount of used physical memory in bytes",
                &default_node_labels,
                |n| n.os.mem.used_in_bytes as f64,
            ),
            new!(
                node,
                indices,
                "docs",
                "Count of documents on this node",
                &default_node_labels,
                |n| n.indices.docs.count as f64,
            ),
            new!(
                node,
                indices,
                "docs_deleted",
                "Count of deleted documents on this node",
                &default_node_labels,
                |n| n.indices.docs.deleted as f64,
            ),
            new!(
                node,
                indices_indexing,
                "index_total",
                "Total index calls",
                &default_node_labels,
                |n| n.indices.indexing.index_total as f64,
            ),
            new!(
                node,
                indices_indexing,
                "delete_time_seconds_total",
                "Total time indexing delete in seconds",
                &default_node_labels,
                |n| n.indices.indexing.delete_time_in_millis as f64 / 1000.0,
            ),
            new!(
                node,
                indices_indexing,
                "delete_total",
                "Total indexing deletes",
                &default_node_labels,
                |n| n.indices.indexing.delete_total as f64,
            ),
            new!(
                node,
                indices_indexing,
                "is_throttled",
                "Indexing throttling",
                &default_node_labels,
                |n| {
                    if n.indices.indexing.is_throttled {
                        1.0
                    } else {
                        0.0
                    }
                },
            ),
            new!(
                node,
                indices_indexing,
                "throttle_time_seconds_total",
                "Cumulative indexing throttling time",
                &default_node_labels,
                |n| n.indices.indexing.throttle_time_in_millis as f64 / 1000.0,
            ),
        ];

        let jvm_gc = "jvm_gc";
        let default_gc_collection_labels = ["host", "name", "gc"];
        let gc_collection_metrics = vec![
            new!(
                gc,
                jvm_gc,
                "collection_seconds_count",
                "Count of JVM GC runs",
                &default_gc_collection_labels,
                |g| g.collection_count as f64,
            ),
            new!(
                gc,
                jvm_gc,
                "collection_seconds_sum",
                "GC run time in seconds",
                &default_gc_collection_labels,
                |g| g.collection_time_in_millis as f64 / 1000.0,
            ),
        ];

        let breakers = "breakers";
        let default_breaker_labels = ["host", "name", "breaker"];
        let breaker_metrics = vec![
            new!(
                breaker,
                breakers,
                "estimated_size_bytes",
                "Estimated size in bytes of breaker",
                &default_breaker_labels,
                |b| b.estimated_size_in_bytes as f64,
            ),
            new!(
                breaker,
                breakers,
                "limit_size_bytes",
                "Limit size in bytes for breaker",
                &default_gc_collection_labels,
                |b| b.limit_size_in_bytes as f64,
            ),
            new!(
                breaker,
                breakers,
                "tripped",
                "tripped for breaker",
                &default_breaker_labels,
                |b| b.tripped as f64,
            ),
            new!(
                breaker,
                breakers,
                "overhead",
                "Overhead of circuit breakers",
                &default_gc_collection_labels,
                |b| b.overhead,
            ),
        ];

        let indexing_pressure = "indexing_pressure";
        let default_indexing_pressure_labels = ["host", "name", "indexing_pressure"];
        let indexing_pressure_metrics = vec![
            new!(
                idx,
                indexing_pressure,
                "current_all_in_bytes",
                "Memory consumed, in bytes, by indexing requests in the coordinating, primary, or replica stage.",
                &default_indexing_pressure_labels,
                |n| n.current.all_in_bytes as f64
            ),
            new!(
                idx,
                indexing_pressure,
                "limit_in_bytes",
                "Configured memory limit, in bytes, for the indexing requests",
                &default_indexing_pressure_labels,
                |n| n.current.all_in_bytes as f64
            ),
        ];

        let thread_pool = "thread_pool";
        let default_thread_pool_labels = ["host", "name", "type"];
        let thread_pool_metrics = vec![
            new!(
                thread,
                thread_pool,
                "completed_count",
                "Thread Pool operatios completed",
                &default_thread_pool_labels,
                |n| n.completed as f64,
            ),
            new!(
                thread,
                thread_pool,
                "rejected_count",
                "Thread Pool operations rejected",
                &default_thread_pool_labels,
                |n| n.rejected as f64,
            ),
            new!(
                thread,
                thread_pool,
                "active_count",
                "Thread Pool threads active",
                &default_thread_pool_labels,
                |n| n.active as f64,
            ),
            new!(
                thread,
                thread_pool,
                "largest_count",
                "Thread Pool largest threads count",
                &default_thread_pool_labels,
                |n| n.largest as f64,
            ),
            new!(
                thread,
                thread_pool,
                "queue_count",
                "Thread Pool operations queued",
                &default_thread_pool_labels,
                |n| n.queue as f64,
            ),
            new!(
                thread,
                thread_pool,
                "threads_count",
                "Thread Pool current threads count",
                &default_thread_pool_labels,
                |n| n.threads as f64,
            ),
        ];

        let filesystem_data = "filesystem_data";
        let default_filesystem_data_labels = ["host", "name", "mount", "path"];
        let filesystem_data_metrics = vec![
            new!(
                file_data,
                filesystem_data,
                "available_bytes",
                "Available space on block device in bytes",
                &default_filesystem_data_labels,
                |f| f.available_in_bytes as f64,
            ),
            new!(
                file_data,
                filesystem_data,
                "free_bytes",
                "Free space on block device in bytes",
                &default_filesystem_data_labels,
                |f| f.free_in_bytes as f64,
            ),
            new!(
                file_data,
                filesystem_data,
                "size_bytes",
                "Size of block device in bytes",
                &default_filesystem_data_labels,
                |f| f.total_in_bytes as f64,
            ),
        ];

        let filesystem_io = "filesystem_io_stats_device";
        let default_filesystem_io_device_labels = ["host", "name", "device"];
        let filesystem_io_device_metrics = vec![
            new!(
                file_io,
                filesystem_io,
                "operations_count",
                "Count of disk operations",
                &default_filesystem_io_device_labels,
                |f| f.operations as f64,
            ),
            new!(
                file_io,
                filesystem_io,
                "read_operations_count",
                "Count of disk read operations",
                &default_filesystem_io_device_labels,
                |f| f.read_operations as f64,
            ),
            new!(
                file_io,
                filesystem_io,
                "write_operations_count",
                "Count of disk write operations",
                &default_filesystem_io_device_labels,
                |f| f.write_operations as f64,
            ),
            new!(
                file_io,
                filesystem_io,
                "read_size_kilobytes_sum",
                "Total kilobytes read from disk",
                &default_filesystem_io_device_labels,
                |f| f.read_kilobytes as f64,
            ),
            new!(
                file_io,
                filesystem_io,
                "write_size_kilobytes_sum",
                "Total kilobytes written from disk",
                &default_filesystem_io_device_labels,
                |f| f.write_kilobytes as f64,
            ),
        ];

        Self {
            node_metrics,
            gc_collection_metrics,
            breaker_metrics,
            indexing_pressure_metrics,
            thread_pool_metrics,
            filesystem_data_metrics,
            filesystem_io_device_metrics,
        }
    }

    pub async fn collect(&self, base: &str) -> crate::error::Result<Vec<MetricFamily>> {
        macro_rules! collect {
            ($v:ident, $($name:ident),+) => {
                $(
                    self.$name
                    .iter()
                    .for_each(|f| $v.extend(f.gauge_vec.collect()));
                )+
            };
            ($m:ident, $tags:expr, $node:ident) => {
                self.$m.iter().for_each(|f| {
                    f.gauge_vec
                        .with_label_values($tags)
                        .set((f.value_fn)($node))
                });
            }
        }

        let resp: NodeStatusResponse = query!(base);
        println!("{:?}", &resp);
        let mut result = vec![];

        for (.., node) in &resp.nodes {
            collect!(node_metrics, &[&node.host, &node.name], node);

            for (collector, gc_stats) in &node.jvm.gc.collectors {
                collect!(
                    gc_collection_metrics,
                    &[&node.host, &node.name, collector],
                    gc_stats
                );
            }

            for (breaker, bstats) in &node.breakers {
                collect!(breaker_metrics, &[&node.host, &node.name, breaker], bstats);
            }

            for (indexing_pressure, ipstats) in &node.indexing_pressure {
                collect!(
                    indexing_pressure_metrics,
                    &[&node.host, &node.name, indexing_pressure],
                    ipstats
                );
            }

            for (pool, pstats) in &node.thread_pool {
                collect!(thread_pool_metrics, &[&node.host, &node.name, pool], pstats);
            }

            for fs_data_stats in &node.fs.data {
                collect!(
                    filesystem_data_metrics,
                    &[
                        &node.host,
                        &node.name,
                        &fs_data_stats.mount,
                        &fs_data_stats.path
                    ],
                    fs_data_stats
                );
            }

            for fs_io_device_stats in &node.fs.io_stats.devices {
                collect!(
                    filesystem_io_device_metrics,
                    &[&node.host, &node.name],
                    fs_io_device_stats
                );
            }
        }

        collect!(
            result,
            node_metrics,
            breaker_metrics,
            gc_collection_metrics,
            indexing_pressure_metrics,
            thread_pool_metrics,
            filesystem_data_metrics,
            filesystem_io_device_metrics
        );

        Ok(result)
    }
}
