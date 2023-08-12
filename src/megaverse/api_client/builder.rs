use std::time::Duration;

use crate::megaverse::config::handler::Config;
use crate::megaverse::api_client::client::ApiClient;
use reqwest::blocking::Client;

#[derive(Default)]
pub struct ApiClientBuilder {
    cfg: Config,
    retries: u32,
    timeout: Duration,
}

// Builder pattern!
impl ApiClientBuilder {
    pub fn new() -> ApiClientBuilder {
        ApiClientBuilder{
            timeout: Duration::from_secs(1), 
            ..Default::default()
        }
    }

    pub fn with_config(mut self, c: &Config) -> Self {
        self.cfg = c.clone();
        self
    }

    pub fn with_retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn build(&self) -> ApiClient {
        ApiClient {
            cfg: self.cfg.clone(),
            client: Client::builder().timeout(self.timeout).build().unwrap(),
            retries: self.retries,
        }
    }
}