use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use tracing::{info, warn};

use crate::{
    api::models::{HealthResponse, SettleRequest, SettleResponse, VerifyRequest, VerifyResponse},
    config::Config,
    error::FacilitatorResult,
    polkadot::PolkadotClient,
};

pub type AppState = Arc<AppStateInner>;

pub struct AppStateInner {
    pub config: Config,
    pub polkadot_client: PolkadotClient,
}

pub async fn health(State(state): State<AppState>) -> FacilitatorResult<Json<HealthResponse>> {
    info!("Health check requested");

    let connected = state.polkadot_client.is_connected().await;

    Ok(Json(HealthResponse {
        status: "ok".to_string(),
        network: state.config.polkadot_network.clone(),
        connected,
    }))
}

pub async fn verify(
    State(state): State<AppState>,
    Json(payload): Json<VerifyRequest>,
) -> FacilitatorResult<Json<VerifyResponse>> {
    info!(
        "Verify request - amount={}, recipient={}",
        payload.expected_amount, payload.expected_recipient
    );

    match state
        .polkadot_client
        .verify_transaction(
            &payload.transaction,
            payload.expected_amount,
            &payload.expected_recipient,
        )
        .await
    {
        Ok(()) => {
            info!("Transaction verified successfully");
            Ok(Json(VerifyResponse {
                valid: true,
                message: "Transaction verified successfully".to_string(),
            }))
        }
        Err(e) => {
            warn!("Transaction verification failed: {}", e);
            Ok(Json(VerifyResponse {
                valid: false,
                message: format!("Verification failed: {}", e),
            }))
        }
    }
}

pub async fn settle(
    State(state): State<AppState>,
    Json(payload): Json<SettleRequest>,
) -> FacilitatorResult<(StatusCode, Json<SettleResponse>)> {
    info!("Settle request for transaction");

    match state
        .polkadot_client
        .submit_transaction(&payload.transaction)
        .await
    {
        Ok(tx_hash) => {
            info!("Transaction settled - Hash: {}", tx_hash);
            Ok((
                StatusCode::OK,
                Json(SettleResponse {
                    settled: true,
                    transaction_hash: Some(tx_hash.clone()),
                    message: format!("Transaction settled - Hash: {}", tx_hash),
                }),
            ))
        }
        Err(e) => {
            warn!("Transaction settlement failed: {}", e);
            Ok((
                StatusCode::BAD_REQUEST,
                Json(SettleResponse {
                    settled: false,
                    transaction_hash: None,
                    message: format!("Settlement failed: {}", e),
                }),
            ))
        }
    }
}
