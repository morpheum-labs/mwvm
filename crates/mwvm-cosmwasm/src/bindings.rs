//! Morpheum-specific CosmWasm message and query bindings.
//!
//! These types are identical to `mormcore/crates/modules/wasm/src/types/custom.rs`
//! and must be kept in sync. They are duplicated here so that CosmWasm contract
//! developers can import them without depending on mormcore.
//!
//! When the separate `morpheum-cosmwasm-bindings` crate is published, both
//! this module and mormcore's types will be replaced by imports from that crate.

use cosmwasm_std::{Binary, CustomMsg, CustomQuery, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Custom messages for interacting with Morpheum's native modules.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MorpheumMsg {
    /// Mint tokens to a recipient via the bank module.
    MintTo {
        recipient: String,
        asset_index: u64,
        amount: Uint128,
    },
    /// Burn tokens from a sender via the bank module.
    BurnFrom {
        sender: String,
        asset_index: u64,
        amount: Uint128,
    },
    /// Settle an x402 payment via the native x402 module.
    SettleX402 {
        protocol_id: String,
        raw_envelope: Binary,
    },
}

impl CustomMsg for MorpheumMsg {}

/// Custom queries against Morpheum's native module state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MorpheumQuery {
    /// Query a bank balance.
    BankBalance { address: String, asset_index: u64 },
    /// Query asset metadata.
    AssetInfo { asset_index: u64 },
    /// Query an x402 policy.
    X402Policy { agent_address: String },
}

impl CustomQuery for MorpheumQuery {}

/// Response for BankBalance query.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BankBalanceResponse {
    pub balance: Uint128,
}

/// Response for AssetInfo query.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AssetInfoResponse {
    pub asset_index: u64,
    pub symbol: String,
    pub total_supply: Uint128,
}
