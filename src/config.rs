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
}

impl Conf {
    pub fn build() -> Result<Conf> {
        Ok(envy::from_env()?)
    }

    pub fn has_anthentication(&self) -> bool {
        !&self.password.is_empty() && !&self.username.is_empty()
    }
}
