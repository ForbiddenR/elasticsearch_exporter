use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Conf {
    // form: http://ip:port
    pub addr: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    pub enabled_exporters: Option<String>
}

impl Conf {
    pub fn build() -> Result<Conf> {
        Ok(envy::from_env()?)
    }
}
