//! Testing helpers for EVM contracts targeting Morpheum.
//!
//! Provides utilities for:
//! - Generating test addresses in EVM hex format
//! - Creating mock transaction contexts
//! - Validating precompile call results

/// Generates a deterministic EVM test address.
///
/// Returns a checksummed 40-hex-char address prefixed with `0x`.
pub fn test_address(index: u32) -> String {
    format!("0x{:040x}", index)
}

/// Generates a deterministic EVM contract address.
///
/// Uses a distinct prefix range from `test_address` to avoid collisions.
pub fn test_contract_address(index: u32) -> String {
    format!("0x{:040x}", 0xC0000000_u64 + u64::from(index))
}

/// Mock EVM transaction context for testing.
#[derive(Debug, Clone)]
pub struct MockTxContext {
    pub sender: String,
    pub gas_limit: u64,
    pub value: u128,
    pub nonce: u64,
}

impl MockTxContext {
    pub fn new(sender: &str) -> Self {
        Self {
            sender: sender.to_string(),
            gas_limit: 3_000_000,
            value: 0,
            nonce: 0,
        }
    }

    pub fn with_gas_limit(mut self, gas_limit: u64) -> Self {
        self.gas_limit = gas_limit;
        self
    }

    pub fn with_value(mut self, value: u128) -> Self {
        self.value = value;
        self
    }

    pub fn with_nonce(mut self, nonce: u64) -> Self {
        self.nonce = nonce;
        self
    }
}

/// Validates that a set of precompile calls includes a specific type.
pub fn assert_has_precompile_call(
    calls: &[crate::bindings::MorpheumPrecompile],
    expected_type: &str,
) {
    let found = calls.iter().any(|call| {
        let call_type = match call {
            crate::bindings::MorpheumPrecompile::MintTo { .. } => "mint_to",
            crate::bindings::MorpheumPrecompile::BurnFrom { .. } => "burn_from",
            crate::bindings::MorpheumPrecompile::Transfer { .. } => "transfer",
            crate::bindings::MorpheumPrecompile::SettleX402 { .. } => "settle_x402",
        };
        call_type == expected_type
    });
    assert!(
        found,
        "expected MorpheumPrecompile::{expected_type} in calls"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_generation() {
        let addr = test_address(0);
        assert!(addr.starts_with("0x"));
        assert_eq!(addr.len(), 42); // 0x + 40 hex chars
    }

    #[test]
    fn test_contract_address_generation() {
        let addr = test_contract_address(0);
        assert!(addr.starts_with("0x"));
        assert_eq!(addr.len(), 42);
        assert_ne!(test_address(0), test_contract_address(0));
    }

    #[test]
    fn test_mock_tx_context() {
        let ctx = MockTxContext::new("0xsender")
            .with_gas_limit(5_000_000)
            .with_value(100)
            .with_nonce(3);
        assert_eq!(ctx.sender, "0xsender");
        assert_eq!(ctx.gas_limit, 5_000_000);
        assert_eq!(ctx.value, 100);
        assert_eq!(ctx.nonce, 3);
    }

    #[test]
    fn test_assert_has_precompile_call() {
        let calls = vec![
            crate::bindings::MorpheumPrecompile::MintTo {
                recipient: "0xabc".into(),
                asset_index: 0,
                amount: 100,
            },
        ];
        assert_has_precompile_call(&calls, "mint_to");
    }

    #[test]
    #[should_panic(expected = "expected MorpheumPrecompile::burn_from")]
    fn test_assert_has_precompile_call_missing() {
        let calls = vec![
            crate::bindings::MorpheumPrecompile::MintTo {
                recipient: "0xabc".into(),
                asset_index: 0,
                amount: 100,
            },
        ];
        assert_has_precompile_call(&calls, "burn_from");
    }
}
