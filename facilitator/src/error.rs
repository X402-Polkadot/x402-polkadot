use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FacilitatorError {
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),

    #[error("Transaction verification failed: {0}")]
    VerificationFailed(String),

    #[error("Transaction submission failed: {0}")]
    SubmissionFailed(String),

    #[error("Polkadot RPC error: {0}")]
    PolkadotRpcError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Internal server error: {0}")]
    InternalError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl IntoResponse for FacilitatorError {
    fn into_response(self) -> Response {
        let (status, error_type) = match &self {
            FacilitatorError::InvalidTransaction(_) => (StatusCode::BAD_REQUEST, "InvalidTransaction"),
            FacilitatorError::VerificationFailed(_) => (StatusCode::UNPROCESSABLE_ENTITY, "VerificationFailed"),
            FacilitatorError::SubmissionFailed(_) => (StatusCode::BAD_GATEWAY, "SubmissionFailed"),
            FacilitatorError::PolkadotRpcError(_) => (StatusCode::BAD_GATEWAY, "PolkadotRpcError"),
            FacilitatorError::ConfigError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "ConfigError"),
            FacilitatorError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "InternalError"),
        };

        let body = Json(ErrorResponse {
            error: error_type.to_string(),
            message: self.to_string(),
        });

        (status, body).into_response()
    }
}

pub type FacilitatorResult<T> = Result<T, FacilitatorError>;
