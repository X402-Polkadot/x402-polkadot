use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub polkadot_network: String,
    pub polkadot_rpc_url: String,
    pub facilitator_host: String,
    pub facilitator_port: u16,
    pub signer_seed: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            polkadot_network: env::var("POLKADOT_NETWORK")
                .context("POLKADOT_NETWORK must be set")?,
            polkadot_rpc_url: env::var("POLKADOT_RPC_URL")
                .context("POLKADOT_RPC_URL must be set")?,
            facilitator_host: env::var("FACILITATOR_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            facilitator_port: env::var("FACILITATOR_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .context("FACILITATOR_PORT must be a valid u16")?,
            signer_seed: env::var("SIGNER_SEED").ok(),
        })
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.facilitator_host, self.facilitator_port)
    }
}
