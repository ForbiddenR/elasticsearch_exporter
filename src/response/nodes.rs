use std::collections::HashMap;

use serde::Deserialize;
use visible::data;

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatusResponse {
    cluster_name: String,
    nodes: HashMap<String, NodeStatsNodeResponse>,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsNodeResponse {
    name: String,
    host: String,
    timestamp: i64,
    roles: Option<Vec<String>>,
    attributes: HashMap<String, String>,
    transport_address: String,
    indices: NodeStatsIndicesResponse,
    os: NodeStatsOSResponse,
    fs: NodeStatsFSResponse,
    thread_pool: HashMap<String, NodeStatsThreadPoolResponse>,
    jvm: NodeStatsJVMResponse,
    breakers: HashMap<String, NodeStatsBreakersResponse>,
    indexing_pressure: HashMap<String, NodeStatsIndexingPressureResponse>,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsBreakersResponse {
    estimated_size_in_bytes: i64,
    limit_size_in_bytes: i64,
    overhead: f64,
    tripped: i64,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsIndexingPressureResponse {
    current: NodeStatsIndexingPressureCurrentResponse,
    limit_in_bytes: i64,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsIndexingPressureCurrentResponse {
    all_in_bytes: i64,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsJVMResponse {
    mem: NodeStatsJVMMemResponse,
    gc: NodeStatsJVMGCResponse,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsJVMGCResponse {
    collectors: HashMap<String, NodeStatsJVMGCCollectorResponse>,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsJVMGCCollectorResponse {
    collection_count: i64,
    collection_time_in_millis: i64,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsJVMMemResponse {
    heap_committed_in_bytes: i64,
    heap_used_in_bytes: i64,
    heap_max_in_bytes: i64,
    non_heap_committed_in_bytes: i64,
    non_heap_used_in_bytes: i64,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsThreadPoolResponse {
    threads: i64,
    queue: i64,
    active: i64,
    rejected: i64,
    largest: i64,
    completed: i64,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsIndicesResponse {
    docs: NodeStatsIndicesDocsResponse,
    indexing: NodeStatsIndicesIndexingResponse,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsIndicesDocsResponse {
    count: i64,
    deleted: i64,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsIndicesIndexingResponse {
    index_total: i64,
    index_time_in_millis: i64,
    index_current: i64,
    delete_total: i64,
    delete_time_in_millis: i64,
    delete_current: i64,
    is_throttled: bool,
    throttle_time_in_millis: i64,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsOSResponse {
    timestamp: i64,
    // uptime_in_millis: i64,
    cpu: NodeStatsOSCPUResponse,
    mem: NodeStatsOSMemResponse,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsOSMemResponse {
    free_in_bytes: i64,
    used_in_bytes: i64,
    // actual_free_in_bytes: i64,
    // actual_used_in_bytes: i64,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsOSCPUResponse {
    load_average: NodeStatsOSCPULoadResponse,
    percent: i64,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsOSCPULoadResponse {
    #[serde(rename = "1m")]
    load1: f64,
    #[serde(rename = "5m")]
    load5: f64,
    #[serde(rename = "15m")]
    load15: f64,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsFSResponse {
    timestamp: i64,
    #[serde(default)]
    data: Vec<NodeStatsFSDataResponse>,
    io_stats: NodeStatsFSIOStatsResponse,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsFSDataResponse {
    path: String,
    mount: String,
    #[serde(default)]
    dev: String,
    total_in_bytes: i64,
    free_in_bytes: i64,
    available_in_bytes: i64,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsFSIOStatsResponse {
    #[serde(default)]
    devices: Vec<NodeStatsFSIOStatsDeviceResponse>,
}

#[data]
#[derive(Debug, Deserialize)]
pub struct NodeStatsFSIOStatsDeviceResponse {
    device_name: String,
    operations: i64,
    read_operations: i64,
    write_operations: i64,
    read_kilobytes: i64,
    write_kilobytes: i64,
}
