use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ClusterHealthResponse {
    // cluster_name: String,
    pub status: String,
    // pub timed_out: bool,
    // pub number_of_nodes: i64,
    // pub number_of_data_nodes: i64,
    // pub active_primary_shards: i64,
    // pub active_shards: i64,
    // pub relocating_shards: i64,
    // pub initializing_shards: i64,
    // pub unassigned_shards: i64,
    // pub delayed_unassigned_shards: i64,
    // pub number_of_pending_tasks: i64,
    // pub number_of_in_flight_fetch: i64,
    // pub task_max_waiting_in_queue_millis: i64,
    // pub active_shards_percent_as_number: f64,
}
