use std::fs;
use std::net::SocketAddr;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) http: Http,
}

impl Config {
    pub(crate) fn new(filename: &str) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(filename)?;
        let cfg = toml::from_str(&contents)?;
        Ok(cfg)
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct Http {
    address: String,
    port: u16,
}

impl Http {
    pub(crate) fn socketaddr(&self) -> anyhow::Result<SocketAddr> {
        let str = format!("{}:{}", self.address, self.port);
        let sock = str.parse::<SocketAddr>()?;
        Ok(sock)
    }
}