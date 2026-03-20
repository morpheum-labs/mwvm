//! Morpheum SVM system program definitions.
//!
//! These system program IDs and CPI (cross-program invocation) structures
//! define how SVM programs interact with Morpheum's native modules.
//! The SVM engine routes CPIs to these program IDs through `NativeOp`
//! dispatch on the L1 side.
//!
//! System program IDs use Morpheum's base58-encoded address format.

use serde::{Deserialize, Serialize};

/// System program ID for MintTo (bank mint).
pub const PROGRAM_MORPHEUM_MINT: &str = "MormMint1111111111111111111111111111111111";
/// System program ID for BurnFrom (bank burn).
pub const PROGRAM_MORPHEUM_BURN: &str = "MormBurn1111111111111111111111111111111111";
/// System program ID for Transfer (bank transfer).
pub const PROGRAM_MORPHEUM_TRANSFER: &str = "MormXfer1111111111111111111111111111111111";
/// System program ID for SettleX402 (x402 payment settlement).
pub const PROGRAM_MORPHEUM_SETTLE: &str = "MormSettle11111111111111111111111111111111";
/// System program ID for BankBalance query.
pub const PROGRAM_MORPHEUM_BANK_QUERY: &str = "MormBankQ111111111111111111111111111111111";

/// Morpheum CPI calls that map to native module operations.
///
/// Each variant corresponds to a system program CPI instruction.
/// On the L1 side, these are converted to `NativeOp` for dispatch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MorpheumCpi {
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

/// Morpheum SVM query instructions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MorpheumSvmQuery {
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

/// SVM account metadata for CPI instructions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountMeta {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

/// Returns the system program ID for a given CPI call.
pub fn program_id(cpi: &MorpheumCpi) -> &'static str {
    match cpi {
        MorpheumCpi::MintTo { .. } => PROGRAM_MORPHEUM_MINT,
        MorpheumCpi::BurnFrom { .. } => PROGRAM_MORPHEUM_BURN,
        MorpheumCpi::Transfer { .. } => PROGRAM_MORPHEUM_TRANSFER,
        MorpheumCpi::SettleX402 { .. } => PROGRAM_MORPHEUM_SETTLE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_id_mapping() {
        let mint = MorpheumCpi::MintTo {
            recipient: "mormPK1abc".into(),
            asset_index: 0,
            amount: 100,
        };
        assert_eq!(program_id(&mint), PROGRAM_MORPHEUM_MINT);

        let settle = MorpheumCpi::SettleX402 {
            protocol_id: "hyperlane".into(),
            raw_envelope: vec![],
        };
        assert_eq!(program_id(&settle), PROGRAM_MORPHEUM_SETTLE);
    }

    #[test]
    fn test_cpi_serde_roundtrip() {
        let cpi = MorpheumCpi::Transfer {
            from: "mormPK1aaa".into(),
            to: "mormPK1bbb".into(),
            asset_index: 1,
            amount: 500,
        };
        let json = serde_json::to_string(&cpi).unwrap();
        let recovered: MorpheumCpi = serde_json::from_str(&json).unwrap();
        assert_eq!(cpi, recovered);
    }

    #[test]
    fn test_account_meta() {
        let meta = AccountMeta {
            pubkey: "mormPK1xyz".to_string(),
            is_signer: true,
            is_writable: false,
        };
        let json = serde_json::to_string(&meta).unwrap();
        let recovered: AccountMeta = serde_json::from_str(&json).unwrap();
        assert_eq!(meta, recovered);
    }
}
