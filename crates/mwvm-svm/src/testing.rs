//! Testing helpers for SVM programs targeting Morpheum.
//!
//! Provides utilities for:
//! - Generating test public keys in SVM format
//! - Creating mock execution contexts
//! - Validating CPI call results

/// Generates a deterministic SVM test public key.
///
/// Returns a base58-style address string.
pub fn test_pubkey(index: u32) -> String {
    format!("mormPK1test{:032x}", index)
}

/// Generates a deterministic SVM program ID for testing.
///
/// Uses a distinct prefix range from `test_pubkey` to avoid collisions.
pub fn test_program_id(index: u32) -> String {
    format!("mormProg1test{:029x}", index)
}

/// Mock SVM execution context for testing.
#[derive(Debug, Clone)]
pub struct MockExecContext {
    pub signer: String,
    pub program_id: String,
    pub compute_limit: u64,
}

impl MockExecContext {
    pub fn new(signer: &str, program_id: &str) -> Self {
        Self {
            signer: signer.to_string(),
            program_id: program_id.to_string(),
            compute_limit: 200_000,
        }
    }

    pub fn with_compute_limit(mut self, limit: u64) -> Self {
        self.compute_limit = limit;
        self
    }
}

/// Validates that a set of CPI calls includes a specific type.
pub fn assert_has_cpi_call(
    calls: &[crate::bindings::MorpheumCpi],
    expected_type: &str,
) {
    let found = calls.iter().any(|cpi| {
        let cpi_type = match cpi {
            crate::bindings::MorpheumCpi::MintTo { .. } => "mint_to",
            crate::bindings::MorpheumCpi::BurnFrom { .. } => "burn_from",
            crate::bindings::MorpheumCpi::Transfer { .. } => "transfer",
            crate::bindings::MorpheumCpi::SettleX402 { .. } => "settle_x402",
        };
        cpi_type == expected_type
    });
    assert!(
        found,
        "expected MorpheumCpi::{expected_type} in CPI calls"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pubkey_generation() {
        let pk = test_pubkey(0);
        assert!(pk.starts_with("mormPK1test"));
        let pk2 = test_pubkey(1);
        assert_ne!(pk, pk2);
    }

    #[test]
    fn test_program_id_generation() {
        let pid = test_program_id(0);
        assert!(pid.starts_with("mormProg1test"));
        assert_ne!(test_pubkey(0), test_program_id(0));
    }

    #[test]
    fn test_mock_exec_context() {
        let ctx = MockExecContext::new("mormPK1signer", "mormProg1myprogram")
            .with_compute_limit(500_000);
        assert_eq!(ctx.signer, "mormPK1signer");
        assert_eq!(ctx.program_id, "mormProg1myprogram");
        assert_eq!(ctx.compute_limit, 500_000);
    }

    #[test]
    fn test_assert_has_cpi_call() {
        let calls = vec![
            crate::bindings::MorpheumCpi::MintTo {
                recipient: "mormPK1abc".into(),
                asset_index: 0,
                amount: 100,
            },
        ];
        assert_has_cpi_call(&calls, "mint_to");
    }

    #[test]
    #[should_panic(expected = "expected MorpheumCpi::burn_from")]
    fn test_assert_has_cpi_call_missing() {
        let calls = vec![
            crate::bindings::MorpheumCpi::MintTo {
                recipient: "mormPK1abc".into(),
                asset_index: 0,
                amount: 100,
            },
        ];
        assert_has_cpi_call(&calls, "burn_from");
    }
}
