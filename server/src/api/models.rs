use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub network: String,
    pub facilitator_url: String,
}

#[derive(Debug, Serialize)]
pub struct FreeResponse {
    pub message: String,
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct PaidResponse {
    pub message: String,
    pub data: String,
    pub transaction_hash: String,
}
