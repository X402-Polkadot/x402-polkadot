use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequirements {
    pub amount: u128,
    pub recipient: String,
    pub network: String,
    pub currency: String,
}

impl PaymentRequirements {
    pub fn new(amount: u128, recipient: String, network: String) -> Self {
        Self {
            amount,
            recipient,
            network,
            currency: "DOT".to_string(),
        }
    }

    pub fn to_header_value(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
pub struct PaymentHeader {
    pub transaction: String,
}

impl PaymentHeader {
    pub fn from_header(header_value: &str) -> Option<Self> {
        Some(Self {
            transaction: header_value.to_string(),
        })
    }
}
