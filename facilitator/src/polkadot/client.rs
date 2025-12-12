use crate::error::{FacilitatorError, FacilitatorResult};
use crate::polkadot::networks::{find_healthy_node, NetworkConfig};
use crate::polkadot::types::{TransactionData, ValidationParams};
use crate::polkadot::validator::TransactionValidator;
use serde_json;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::Keypair;

const MAX_RETRIES: u32 = 3;

pub struct PolkadotClient {
    network: String,
    network_config: NetworkConfig,
    connected: Arc<RwLock<bool>>,
    current_rpc: Arc<RwLock<Option<String>>>,
    api: Arc<RwLock<Option<OnlineClient<PolkadotConfig>>>>,
    signer: Option<Keypair>,
}

impl PolkadotClient {
    pub async fn new(_rpc_url: String, network: String, _signer_seed: Option<String>) -> FacilitatorResult<Self> {
        info!("Initializing Polkadot client for network: {}", network);
        info!("Mode: Broadcast only (signing done in frontend)");

        let network_config = NetworkConfig::from_network_name(&network);
        info!("Loaded {} RPC endpoints for {}", network_config.nodes.len(), network_config.name);

        let client = Self {
            network,
            network_config,
            connected: Arc::new(RwLock::new(false)),
            current_rpc: Arc::new(RwLock::new(None)),
            api: Arc::new(RwLock::new(None)),
            signer: None,
        };

        client.connect().await?;
        Ok(client)
    }

    async fn connect(&self) -> FacilitatorResult<()> {
        // Find a healthy node
        let node = find_healthy_node(&self.network_config).await
            .ok_or_else(|| FacilitatorError::PolkadotRpcError("No healthy RPC nodes available".to_string()))?;

        info!("Connecting to Polkadot RPC: {}", node.url);

        let api = OnlineClient::<PolkadotConfig>::from_url(&node.url)
            .await
            .map_err(|e| FacilitatorError::PolkadotRpcError(format!("Failed to connect: {}", e)))?;

        *self.api.write().await = Some(api);
        *self.current_rpc.write().await = Some(node.url.clone());
        *self.connected.write().await = true;
        info!("Successfully connected to Polkadot network via {}", node.name);
        Ok(())
    }

    /// Mark connection as disconnected (called when RPC errors occur)
    async fn mark_disconnected(&self) {
        *self.connected.write().await = false;
        *self.api.write().await = None;
    }

    /// Force reconnection to a healthy RPC node
    pub async fn reconnect(&self) -> FacilitatorResult<()> {
        warn!("Forcing reconnection to RPC node...");
        self.mark_disconnected().await;
        self.connect().await
    }

    /// Ensure we have a healthy connection, reconnect if needed
    pub async fn ensure_connected(&self) -> FacilitatorResult<()> {
        if self.is_connected().await {
            return Ok(());
        }

        warn!("Connection lost, attempting to reconnect...");
        self.connect().await
    }

    pub async fn is_connected(&self) -> bool {
        *self.connected.read().await
    }

    /// Check if an error message indicates a connection issue that requires reconnection
    fn is_connection_error(error_msg: &str) -> bool {
        let connection_error_patterns = [
            "connection closed",
            "restart required",
            "background task closed",
            "websocket",
            "transport",
            "disconnected",
            "connection reset",
            "broken pipe",
            "connection refused",
            "timeout",
        ];

        let lower_msg = error_msg.to_lowercase();
        connection_error_patterns.iter().any(|pattern| lower_msg.contains(pattern))
    }

    pub async fn verify_transaction(
        &self,
        transaction: &str,
        expected_amount: u128,
        expected_recipient: &str,
    ) -> FacilitatorResult<()> {
        // Ensure we have a healthy connection
        self.ensure_connected().await?;

        let tx_hex = transaction.trim_start_matches("0x");
        if hex::decode(tx_hex).is_err() {
            return Err(FacilitatorError::InvalidTransaction(
                "Invalid hex-encoded transaction".to_string()
            ));
        }

        info!("Transaction format validated");
        Ok(())
    }

    pub async fn submit_transaction(&self, transaction: &str) -> FacilitatorResult<String> {
        info!("Broadcasting signed transaction");

        let tx_hex = transaction.trim_start_matches("0x");
        let tx_bytes = hex::decode(tx_hex).map_err(|e| {
            FacilitatorError::InvalidTransaction(format!("Invalid hex transaction: {}", e))
        })?;

        // Retry loop for connection errors
        let mut last_error: Option<FacilitatorError> = None;

        for attempt in 1..=MAX_RETRIES {
            if attempt > 1 {
                info!("Retry attempt {} of {}", attempt, MAX_RETRIES);
            }

            // Ensure we have a healthy connection
            if let Err(e) = self.ensure_connected().await {
                error!("Failed to ensure connection: {}", e);
                last_error = Some(e);
                continue;
            }

            match self.submit_transaction_inner(&tx_bytes).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    let error_msg = e.to_string();
                    if Self::is_connection_error(&error_msg) {
                        warn!("Connection error detected: {}. Reconnecting...", error_msg);
                        self.mark_disconnected().await;

                        // Try to reconnect before next attempt
                        if let Err(reconnect_err) = self.connect().await {
                            error!("Reconnection failed: {}", reconnect_err);
                        }

                        last_error = Some(e);
                        continue;
                    } else {
                        // Non-connection error, don't retry
                        return Err(e);
                    }
                }
            }
        }

        // All retries exhausted
        Err(last_error.unwrap_or_else(|| {
            FacilitatorError::PolkadotRpcError("Failed after max retries".to_string())
        }))
    }

    /// Internal transaction submission (called by submit_transaction with retry logic)
    async fn submit_transaction_inner(&self, tx_bytes: &[u8]) -> FacilitatorResult<String> {
        let api_guard = self.api.read().await;
        let api = api_guard.as_ref().ok_or_else(|| {
            FacilitatorError::PolkadotRpcError("API client not initialized".to_string())
        })?;

        info!("Submitting transaction to blockchain");

        use futures::StreamExt;
        let mut submit_progress = api.backend()
            .submit_transaction(tx_bytes)
            .await
            .map_err(|e| FacilitatorError::PolkadotRpcError(
                format!("Failed to submit transaction: {}", e)
            ))?;

        info!("Transaction submitted, waiting for block inclusion");

        use blake2::{Blake2b512, Digest};
        let tx_hash_bytes = Blake2b512::digest(tx_bytes);
        let tx_hash_hex = format!("0x{}", hex::encode(&tx_hash_bytes[..32]));

        let mut block_hash = String::new();

        while let Some(status) = submit_progress.next().await {
            match status {
                Ok(status) => {
                    match status {
                        subxt::backend::TransactionStatus::Validated => {
                            info!("Transaction validated in transaction pool");
                        }
                        subxt::backend::TransactionStatus::Broadcasted { .. } => {
                            info!("Transaction broadcasted to network");
                        }
                        subxt::backend::TransactionStatus::InBestBlock { hash } => {
                            info!("Transaction included in best block (hash: 0x{})", hex::encode(hash.hash()));
                            info!("Waiting for finalization...");
                        }
                        subxt::backend::TransactionStatus::NoLongerInBestBlock => {
                            info!("Transaction no longer in best block, waiting for finalization...");
                        }
                        subxt::backend::TransactionStatus::InFinalizedBlock { hash } => {
                            info!("Transaction finalized in block");
                            block_hash = format!("0x{}", hex::encode(hash.hash()));
                            break;
                        }
                        subxt::backend::TransactionStatus::Error { message } => {
                            return Err(FacilitatorError::PolkadotRpcError(
                                format!("Transaction error: {}", message)
                            ));
                        }
                        subxt::backend::TransactionStatus::Invalid { message } => {
                            return Err(FacilitatorError::PolkadotRpcError(
                                format!("Transaction invalid: {}", message)
                            ));
                        }
                        subxt::backend::TransactionStatus::Dropped { message } => {
                            return Err(FacilitatorError::PolkadotRpcError(
                                format!("Transaction dropped: {}", message)
                            ));
                        }
                    }
                }
                Err(e) => {
                    let error_msg = format!("Transaction status error: {}", e);
                    return Err(FacilitatorError::PolkadotRpcError(error_msg));
                }
            }
        }

        info!("Transaction confirmed on-chain");
        info!("Transaction hash: {}", tx_hash_hex);
        info!("Block hash: {}", block_hash);

        let response = serde_json::json!({
            "transaction_hash": tx_hash_hex,
            "block_hash": block_hash,
            "explorer_url": format!("https://paseo.subscan.io/extrinsic/{}", tx_hash_hex),
            "network": "paseo",
            "status": "confirmed"
        });

        Ok(response.to_string())
    }

    fn decode_transaction(&self, transaction: &str) -> FacilitatorResult<TransactionData> {
        let tx_hex = transaction.trim_start_matches("0x");

        if hex::decode(tx_hex).is_err() {
            return Err(FacilitatorError::InvalidTransaction(
                "Invalid hex-encoded transaction".to_string()
            ));
        }

        Ok(TransactionData {
            from: "signed_transaction".to_string(),
            to: "will_be_verified_on_chain".to_string(),
            amount: 0,
            signature: transaction.to_string(),
            nonce: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = PolkadotClient::new(
            "wss://westend-rpc.polkadot.io".to_string(),
            "westend".to_string(),
            None,
        )
        .await;
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_is_connected() {
        let client = PolkadotClient::new(
            "wss://westend-rpc.polkadot.io".to_string(),
            "westend".to_string(),
            None,
        )
        .await
        .unwrap();
        assert!(client.is_connected().await);
    }
}
