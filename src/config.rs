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
    pub exporter_mode: Mode,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Mode {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "simple")]
    Simple,
}

impl TryFrom<&axum::http::HeaderValue> for Mode {
    type Error = &'static str;

    fn try_from(value: &axum::http::HeaderValue) -> Result<Self, Self::Error> {
        Ok(value
            .to_str()
            .map_err(|_| "Invalid header value")?
            .try_into()?)
    }
}

impl TryFrom<&str> for Mode {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "standard" => Ok(Mode::Standard),
            "simple" => Ok(Mode::Simple),
            _ => Err("Invalid mode value"),
        }
    }
}

impl Conf {
    pub fn build() -> Result<Conf> {
        Ok(envy::from_env()?)
    }
}
