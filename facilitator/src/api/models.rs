use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    pub transaction: String,
    pub expected_amount: u128,
    pub expected_recipient: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyResponse {
    pub valid: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct SettleRequest {
    pub transaction: String,
}

#[derive(Debug, Serialize)]
pub struct SettleResponse {
    pub settled: bool,
    pub transaction_hash: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub network: String,
    pub connected: bool,
}
