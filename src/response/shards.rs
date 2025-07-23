use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ShardResposne {
    pub index: String,
    // pub shard: String,
    // pub prirep: String,
    pub docs: Option<String>,
    // pub state: String,
    // pub node: String,
}
