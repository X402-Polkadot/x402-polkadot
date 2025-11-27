use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    pub from: String,
    pub to: String,
    pub amount: u128,
    pub signature: String,
    pub nonce: u64,
}

#[derive(Debug, Clone)]
pub struct ValidationParams {
    pub expected_amount: u128,
    pub expected_recipient: String,
}

impl ValidationParams {
    pub fn new(expected_amount: u128, expected_recipient: String) -> Self {
        Self {
            expected_amount,
            expected_recipient,
        }
    }
}
