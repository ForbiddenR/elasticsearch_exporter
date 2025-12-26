use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ShardResposne {
    pub index: String,
    pub docs: Option<String>,
}
