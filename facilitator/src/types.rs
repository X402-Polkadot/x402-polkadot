use serde::{Deserialize, Serialize};

/// Verification request from server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyRequest {
    pub transaction: String,  // Hex-encoded signed transaction
    pub expected_recipient: String,
    pub expected_amount: u128,
    pub network: String,
}

/// Verification response to server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyResponse {
    pub is_valid: bool,
    pub error: Option<String>,
}

/// Settlement request from server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettleRequest {
    pub transaction: String,  // Hex-encoded signed transaction
}

/// Settlement response to server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettleResponse {
    pub signature: String,
    pub timestamp: u64,
}
