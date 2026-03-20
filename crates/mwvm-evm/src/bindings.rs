//! Morpheum EVM precompile definitions.
//!
//! These precompile addresses and call structures define how Solidity/Vyper
//! contracts interact with Morpheum's native modules. The EVM engine routes
//! calls to these addresses through `NativeOp` dispatch on the L1 side.
//!
//! Precompile addresses are in the `0x0800`–`0x08FF` range, reserved for
//! Morpheum-specific extensions.

use serde::{Deserialize, Serialize};

/// Precompile address for MintTo (bank mint).
pub const PRECOMPILE_MINT: &str = "0x0000000000000000000000000000000000000800";
/// Precompile address for BurnFrom (bank burn).
pub const PRECOMPILE_BURN: &str = "0x0000000000000000000000000000000000000801";
/// Precompile address for SettleX402 (x402 payment settlement).
pub const PRECOMPILE_SETTLE_X402: &str = "0x0000000000000000000000000000000000000802";
/// Precompile address for Transfer (bank transfer).
pub const PRECOMPILE_TRANSFER: &str = "0x0000000000000000000000000000000000000803";
/// Precompile address for BankBalance query.
pub const PRECOMPILE_BANK_QUERY: &str = "0x0000000000000000000000000000000000000810";
/// Precompile address for AssetInfo query.
pub const PRECOMPILE_ASSET_QUERY: &str = "0x0000000000000000000000000000000000000811";

/// Morpheum precompile calls that map to native module operations.
///
/// Each variant corresponds to a precompile at a reserved address.
/// On the L1 side, these are converted to `NativeOp` for dispatch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MorpheumPrecompile {
    MintTo {
        recipient: String,
        asset_index: u64,
        amount: u128,
    },
    BurnFrom {
        sender: String,
        asset_index: u64,
        amount: u128,
    },
    Transfer {
        from: String,
        to: String,
        asset_index: u64,
        amount: u128,
    },
    SettleX402 {
        protocol_id: String,
        raw_envelope: Vec<u8>,
    },
}

/// Morpheum EVM query precompile calls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MorpheumEvmQuery {
    BankBalance { address: String, asset_index: u64 },
    AssetInfo { asset_index: u64 },
}

/// Response for BankBalance query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BankBalanceResponse {
    pub balance: u128,
}

/// Response for AssetInfo query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetInfoResponse {
    pub asset_index: u64,
    pub symbol: String,
    pub total_supply: u128,
}

/// Returns the precompile address for a given call.
pub fn precompile_address(call: &MorpheumPrecompile) -> &'static str {
    match call {
        MorpheumPrecompile::MintTo { .. } => PRECOMPILE_MINT,
        MorpheumPrecompile::BurnFrom { .. } => PRECOMPILE_BURN,
        MorpheumPrecompile::Transfer { .. } => PRECOMPILE_TRANSFER,
        MorpheumPrecompile::SettleX402 { .. } => PRECOMPILE_SETTLE_X402,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precompile_address_mapping() {
        let mint = MorpheumPrecompile::MintTo {
            recipient: "0xabc".into(),
            asset_index: 0,
            amount: 100,
        };
        assert_eq!(precompile_address(&mint), PRECOMPILE_MINT);

        let burn = MorpheumPrecompile::BurnFrom {
            sender: "0xabc".into(),
            asset_index: 0,
            amount: 50,
        };
        assert_eq!(precompile_address(&burn), PRECOMPILE_BURN);
    }

    #[test]
    fn test_precompile_serde_roundtrip() {
        let call = MorpheumPrecompile::Transfer {
            from: "0xaaa".into(),
            to: "0xbbb".into(),
            asset_index: 1,
            amount: 500,
        };
        let json = serde_json::to_string(&call).unwrap();
        let recovered: MorpheumPrecompile = serde_json::from_str(&json).unwrap();
        assert_eq!(call, recovered);
    }
}
