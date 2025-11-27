use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::x402::types::{PaymentHeader, PaymentRequirements};
use crate::error::ServerError;

pub const PAYMENT_HEADER_NAME: &str = "x-payment";

pub fn extract_payment_header(headers: &HeaderMap) -> Option<PaymentHeader> {
    headers
        .get(PAYMENT_HEADER_NAME)
        .and_then(|v| v.to_str().ok())
        .and_then(PaymentHeader::from_header)
}

pub fn create_payment_required_response(requirements: PaymentRequirements) -> Response {
    let body = Json(json!({
        "error": "PaymentRequired",
        "message": "Payment is required to access this resource",
        "paymentRequirements": requirements
    }));

    (StatusCode::PAYMENT_REQUIRED, body).into_response()
}

pub fn create_payment_error_response(error: ServerError) -> Response {
    error.into_response()
}
