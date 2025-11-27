use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub facilitator_url: String,
    pub receiver_wallet_address: String,
    pub default_price: u128,
    pub polkadot_network: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .context("SERVER_PORT must be a valid u16")?,
            facilitator_url: env::var("FACILITATOR_URL")
                .context("FACILITATOR_URL must be set")?,
            receiver_wallet_address: env::var("RECEIVER_WALLET_ADDRESS")
                .context("RECEIVER_WALLET_ADDRESS must be set")?,
            default_price: env::var("DEFAULT_PRICE")
                .unwrap_or_else(|_| "1000000000000".to_string())
                .parse()
                .context("DEFAULT_PRICE must be a valid u128")?,
            polkadot_network: env::var("POLKADOT_NETWORK")
                .unwrap_or_else(|_| "westend".to_string()),
        })
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}
