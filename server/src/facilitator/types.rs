use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct VerifyRequest {
    pub transaction: String,
    pub expected_amount: u128,
    pub expected_recipient: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyResponse {
    pub valid: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SettleRequest {
    pub transaction: String,
}

#[derive(Debug, Deserialize)]
pub struct SettleResponse {
    pub settled: bool,
    pub transaction_hash: Option<String>,
    pub message: String,
}
