use anyhow::{Context, Result};
use serde::Serialize;

#[derive(Default, Clone, Serialize, dict_derive::FromPyObject)]
pub struct Config {
    /// Url of the source hypersync instance
    pub url: String,
    /// Optional bearer_token to put into http requests made to source hypersync instance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearer_token: Option<String>,
    /// Timout treshold for a single http request in milliseconds, default is 30 seconds (30_000ms)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_req_timeout_millis: Option<i64>,
}

impl Config {
    pub fn try_convert(&self) -> Result<skar_client_fuel::Config> {
        let json = serde_json::to_vec(self).context("serialize to json")?;
        serde_json::from_slice(&json).context("parse json")
    }
}
