use crate::error::{FacilitatorError, FacilitatorResult};
use crate::polkadot::types::{TransactionData, ValidationParams};
use tracing::{debug, warn};

pub struct TransactionValidator;

impl TransactionValidator {
    pub fn validate(
        tx_data: &TransactionData,
        params: &ValidationParams,
    ) -> FacilitatorResult<()> {
        debug!("Validating transaction: {:?}", tx_data);

        Self::validate_amount(tx_data.amount, params.expected_amount)?;
        Self::validate_recipient(&tx_data.to, &params.expected_recipient)?;
        Self::validate_signature(&tx_data.signature)?;

        debug!("Transaction validation successful");
        Ok(())
    }

    fn validate_amount(actual: u128, expected: u128) -> FacilitatorResult<()> {
        if actual < expected {
            warn!(
                "Amount validation failed: actual={}, expected={}",
                actual, expected
            );
            return Err(FacilitatorError::VerificationFailed(format!(
                "Insufficient amount: expected {}, got {}",
                expected, actual
            )));
        }
        Ok(())
    }

    fn validate_recipient(actual: &str, expected: &str) -> FacilitatorResult<()> {
        if actual != expected {
            warn!(
                "Recipient validation failed: actual={}, expected={}",
                actual, expected
            );
            return Err(FacilitatorError::VerificationFailed(format!(
                "Invalid recipient: expected {}, got {}",
                expected, actual
            )));
        }
        Ok(())
    }

    fn validate_signature(signature: &str) -> FacilitatorResult<()> {
        if signature.is_empty() {
            warn!("Signature validation failed: empty signature");
            return Err(FacilitatorError::VerificationFailed(
                "Missing transaction signature".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_amount_success() {
        assert!(TransactionValidator::validate_amount(100, 100).is_ok());
        assert!(TransactionValidator::validate_amount(150, 100).is_ok());
    }

    #[test]
    fn test_validate_amount_failure() {
        assert!(TransactionValidator::validate_amount(50, 100).is_err());
    }

    #[test]
    fn test_validate_recipient_success() {
        assert!(
            TransactionValidator::validate_recipient("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty").is_ok()
        );
    }

    #[test]
    fn test_validate_recipient_failure() {
        assert!(
            TransactionValidator::validate_recipient("invalid", "valid").is_err()
        );
    }

    #[test]
    fn test_validate_signature_success() {
        assert!(TransactionValidator::validate_signature("0x123abc").is_ok());
    }

    #[test]
    fn test_validate_signature_failure() {
        assert!(TransactionValidator::validate_signature("").is_err());
    }
}
