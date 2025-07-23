use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ShardResposne {
    pub index: String,
    pub docs: Option<String>,
}
