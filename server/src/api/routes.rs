use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use tracing::{info, warn};

use crate::{
    api::models::{FreeResponse, HealthResponse, PaidResponse},
    config::Config,
    error::ServerResult,
    facilitator::FacilitatorClient,
    x402::{create_payment_required_response, extract_payment_header, PaymentRequirements},
};

pub type AppState = Arc<AppStateInner>;

pub struct AppStateInner {
    pub config: Config,
    pub facilitator_client: FacilitatorClient,
}

pub async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    info!("Health check requested");

    Json(HealthResponse {
        status: "ok".to_string(),
        network: state.config.polkadot_network.clone(),
        facilitator_url: state.config.facilitator_url.clone(),
    })
}

pub async fn free() -> Json<FreeResponse> {
    info!("Free endpoint accessed");

    Json(FreeResponse {
        message: "This is a free endpoint".to_string(),
        data: "No payment required to access this data".to_string(),
    })
}

pub async fn paid(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<PaidResponse>), axum::response::Response> {
    info!("Paid endpoint accessed");

    match extract_payment_header(&headers) {
        Some(payment_header) => {
            info!("Payment header found, verifying payment");

            match verify_and_settle_payment(&state, &payment_header.transaction).await {
                Ok(tx_hash) => {
                    info!("Payment successful - Transaction Hash: {}", tx_hash);
                    Ok((
                        StatusCode::OK,
                        Json(PaidResponse {
                            message: "Payment successful".to_string(),
                            data: "This is protected content that requires payment".to_string(),
                            transaction_hash: tx_hash,
                        }),
                    ))
                }
                Err(e) => {
                    warn!("Payment verification/settlement failed: {}", e);
                    Err(e.into_response())
                }
            }
        }
        None => {
            info!("No payment header found, returning 402 Payment Required");

            let payment_requirements = PaymentRequirements::new(
                state.config.default_price,
                state.config.receiver_wallet_address.clone(),
                state.config.polkadot_network.clone(),
            );

            Err(create_payment_required_response(payment_requirements))
        }
    }
}

async fn verify_and_settle_payment(
    state: &AppState,
    transaction: &str,
) -> ServerResult<String> {
    info!("Verifying payment");

    let is_valid = state
        .facilitator_client
        .verify_payment(
            transaction,
            state.config.default_price,
            &state.config.receiver_wallet_address,
        )
        .await?;

    if !is_valid {
        warn!("Payment verification failed");
        return Err(crate::error::ServerError::PaymentVerificationFailed(
            "Invalid payment".to_string(),
        ));
    }

    info!("Payment verified, settling transaction");

    let tx_hash = state
        .facilitator_client
        .settle_payment(transaction)
        .await?;

    info!("Payment settled successfully - TX Hash: {}", tx_hash);
    Ok(tx_hash)
}
