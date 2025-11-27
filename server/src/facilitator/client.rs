use crate::error::{ServerError, ServerResult};
use crate::facilitator::types::{SettleRequest, SettleResponse, VerifyRequest, VerifyResponse};
use reqwest::Client;
use tracing::{debug, error, info};

pub struct FacilitatorClient {
    base_url: String,
    client: Client,
}

impl FacilitatorClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
        }
    }

    pub async fn verify_payment(
        &self,
        transaction: &str,
        expected_amount: u128,
        expected_recipient: &str,
    ) -> ServerResult<bool> {
        info!("Verifying payment with facilitator");

        let url = format!("{}/verify", self.base_url);
        let request = VerifyRequest {
            transaction: transaction.to_string(),
            expected_amount,
            expected_recipient: expected_recipient.to_string(),
        };

        debug!("Sending verify request to: {}", url);

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to call facilitator verify endpoint: {}", e);
                ServerError::FacilitatorError(format!("Failed to verify payment: {}", e))
            })?;

        if !response.status().is_success() {
            error!("Facilitator returned error status: {}", response.status());
            return Err(ServerError::PaymentVerificationFailed(
                "Facilitator verification failed".to_string(),
            ));
        }

        let verify_response: VerifyResponse = response.json().await.map_err(|e| {
            error!("Failed to parse verify response: {}", e);
            ServerError::FacilitatorError(format!("Invalid response from facilitator: {}", e))
        })?;

        if verify_response.valid {
            info!("Payment verified successfully");
            Ok(true)
        } else {
            info!("Payment verification failed: {}", verify_response.message);
            Err(ServerError::PaymentVerificationFailed(
                verify_response.message,
            ))
        }
    }

    pub async fn settle_payment(&self, transaction: &str) -> ServerResult<String> {
        info!("Settling payment with facilitator");

        let url = format!("{}/settle", self.base_url);
        let request = SettleRequest {
            transaction: transaction.to_string(),
        };

        debug!("Sending settle request to: {}", url);

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to call facilitator settle endpoint: {}", e);
                ServerError::FacilitatorError(format!("Failed to settle payment: {}", e))
            })?;

        let settle_response: SettleResponse = response.json().await.map_err(|e| {
            error!("Failed to parse settle response: {}", e);
            ServerError::FacilitatorError(format!("Invalid response from facilitator: {}", e))
        })?;

        if settle_response.settled {
            let tx_hash = settle_response
                .transaction_hash
                .unwrap_or_else(|| "unknown".to_string());
            info!("Payment settled successfully: {}", tx_hash);
            Ok(tx_hash)
        } else {
            error!("Payment settlement failed: {}", settle_response.message);
            Err(ServerError::PaymentSettlementFailed(
                settle_response.message,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_facilitator_client_creation() {
        let client = FacilitatorClient::new("http://localhost:8080".to_string());
        assert_eq!(client.base_url, "http://localhost:8080");
    }
}
