use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Payment required: {0}")]
    PaymentRequired(String),

    #[error("Payment verification failed: {0}")]
    PaymentVerificationFailed(String),

    #[error("Payment settlement failed: {0}")]
    PaymentSettlementFailed(String),

    #[error("Facilitator error: {0}")]
    FacilitatorError(String),

    #[error("Invalid payment header: {0}")]
    InvalidPaymentHeader(String),

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

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, error_type) = match &self {
            ServerError::PaymentRequired(_) => (StatusCode::PAYMENT_REQUIRED, "PaymentRequired"),
            ServerError::PaymentVerificationFailed(_) => (StatusCode::UNPROCESSABLE_ENTITY, "PaymentVerificationFailed"),
            ServerError::PaymentSettlementFailed(_) => (StatusCode::BAD_GATEWAY, "PaymentSettlementFailed"),
            ServerError::FacilitatorError(_) => (StatusCode::BAD_GATEWAY, "FacilitatorError"),
            ServerError::InvalidPaymentHeader(_) => (StatusCode::BAD_REQUEST, "InvalidPaymentHeader"),
            ServerError::ConfigError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "ConfigError"),
            ServerError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "InternalError"),
        };

        let body = Json(ErrorResponse {
            error: error_type.to_string(),
            message: self.to_string(),
        });

        (status, body).into_response()
    }
}

pub type ServerResult<T> = Result<T, ServerError>;
