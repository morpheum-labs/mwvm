//! Simulated Morpheum environment for local CosmWasm contract testing.
//!
//! `MockMorpheumApp` provides a local execution environment that simulates
//! Morpheum's native modules (bank balances, asset registry, x402 policies)
//! so that contracts can be tested without a running node.

use std::collections::HashMap;

use cosmwasm_std::Uint128;
use serde::{Deserialize, Serialize};

use crate::bindings::{MorpheumMsg, MorpheumQuery};

/// Simulated bank state for testing.
#[derive(Debug, Clone, Default)]
pub struct MockBankState {
    /// (address, asset_index) → balance
    balances: HashMap<(String, u64), u128>,
    /// asset_index → supply
    supplies: HashMap<u64, u128>,
    /// asset_index → symbol
    assets: HashMap<u64, String>,
}

impl MockBankState {
    pub fn new() -> Self {
        let mut state = Self::default();
        state.assets.insert(0, "MORM".to_string());
        state.assets.insert(1, "USDC".to_string());
        state
    }

    pub fn balance(&self, address: &str, asset_index: u64) -> u128 {
        self.balances
            .get(&(address.to_string(), asset_index))
            .copied()
            .unwrap_or(0)
    }

    pub fn mint(&mut self, recipient: &str, asset_index: u64, amount: u128) {
        *self
            .balances
            .entry((recipient.to_string(), asset_index))
            .or_default() += amount;
        *self.supplies.entry(asset_index).or_default() += amount;
    }

    pub fn burn(
        &mut self,
        sender: &str,
        asset_index: u64,
        amount: u128,
    ) -> Result<(), String> {
        let bal = self
            .balances
            .get_mut(&(sender.to_string(), asset_index))
            .ok_or_else(|| format!("no balance for {sender} asset {asset_index}"))?;
        if *bal < amount {
            return Err(format!(
                "insufficient balance: have {bal}, need {amount}"
            ));
        }
        *bal -= amount;
        if let Some(supply) = self.supplies.get_mut(&asset_index) {
            *supply = supply.saturating_sub(amount);
        }
        Ok(())
    }

    pub fn supply(&self, asset_index: u64) -> u128 {
        self.supplies.get(&asset_index).copied().unwrap_or(0)
    }

    pub fn asset_symbol(&self, asset_index: u64) -> Option<&str> {
        self.assets.get(&asset_index).map(|s| s.as_str())
    }
}

/// Mock Morpheum application for local testing.
///
/// Processes `MorpheumMsg` and `MorpheumQuery` against simulated state,
/// enabling full contract testing without a live node.
#[derive(Debug, Clone)]
pub struct MockMorpheumApp {
    pub bank: MockBankState,
    /// Tracks x402 settlements for assertion in tests.
    pub x402_settlements: Vec<X402SettlementRecord>,
}

/// Record of an x402 settlement for test assertions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct X402SettlementRecord {
    pub protocol_id: String,
    pub raw_envelope: Vec<u8>,
}

impl Default for MockMorpheumApp {
    fn default() -> Self {
        Self::new()
    }
}

impl MockMorpheumApp {
    pub fn new() -> Self {
        Self {
            bank: MockBankState::new(),
            x402_settlements: Vec::new(),
        }
    }

    /// Process a `MorpheumMsg` against the simulated state.
    pub fn execute_custom(&mut self, msg: MorpheumMsg) -> Result<(), String> {
        match msg {
            MorpheumMsg::MintTo {
                recipient,
                asset_index,
                amount,
            } => {
                self.bank.mint(&recipient, asset_index, amount.u128());
                Ok(())
            }
            MorpheumMsg::BurnFrom {
                sender,
                asset_index,
                amount,
            } => self.bank.burn(&sender, asset_index, amount.u128()),
            MorpheumMsg::SettleX402 {
                protocol_id,
                raw_envelope,
            } => {
                self.x402_settlements.push(X402SettlementRecord {
                    protocol_id,
                    raw_envelope: raw_envelope.to_vec(),
                });
                Ok(())
            }
        }
    }

    /// Execute a `MorpheumQuery` against the simulated state.
    pub fn query_custom(
        &self,
        query: MorpheumQuery,
    ) -> Result<Vec<u8>, String> {
        match query {
            MorpheumQuery::BankBalance {
                address,
                asset_index,
            } => {
                let balance = self.bank.balance(&address, asset_index);
                let resp = crate::bindings::BankBalanceResponse {
                    balance: Uint128::new(balance),
                };
                serde_json::to_vec(&resp).map_err(|e| e.to_string())
            }
            MorpheumQuery::AssetInfo { asset_index } => {
                let symbol = self
                    .bank
                    .asset_symbol(asset_index)
                    .unwrap_or("UNKNOWN")
                    .to_string();
                let supply = self.bank.supply(asset_index);
                let resp = crate::bindings::AssetInfoResponse {
                    asset_index,
                    symbol,
                    total_supply: Uint128::new(supply),
                };
                serde_json::to_vec(&resp).map_err(|e| e.to_string())
            }
            MorpheumQuery::X402Policy { agent_address } => {
                let resp = serde_json::json!({
                    "agent_address": agent_address,
                    "is_active": true,
                    "daily_cap": "1000000000",
                });
                serde_json::to_vec(&resp).map_err(|e| e.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use morpheum_primitives::address::module_address;

    #[test]
    fn test_mock_mint_and_balance() {
        let alice = module_address("test-alice");
        let mut app = MockMorpheumApp::new();

        app.execute_custom(MorpheumMsg::MintTo {
            recipient: alice.clone(),
            asset_index: 1,
            amount: Uint128::new(1_000_000),
        })
        .unwrap();

        assert_eq!(app.bank.balance(&alice, 1), 1_000_000);
        assert_eq!(app.bank.supply(1), 1_000_000);
    }

    #[test]
    fn test_mock_burn() {
        let bob = module_address("test-bob");
        let mut app = MockMorpheumApp::new();
        app.bank.mint(&bob, 1, 500_000);

        app.execute_custom(MorpheumMsg::BurnFrom {
            sender: bob.clone(),
            asset_index: 1,
            amount: Uint128::new(200_000),
        })
        .unwrap();

        assert_eq!(app.bank.balance(&bob, 1), 300_000);
    }

    #[test]
    fn test_mock_burn_insufficient() {
        let charlie = module_address("test-charlie");
        let mut app = MockMorpheumApp::new();
        app.bank.mint(&charlie, 1, 100);

        let result = app.execute_custom(MorpheumMsg::BurnFrom {
            sender: charlie,
            asset_index: 1,
            amount: Uint128::new(200),
        });

        assert!(result.is_err());
    }

    #[test]
    fn test_mock_x402_settlement() {
        let mut app = MockMorpheumApp::new();

        app.execute_custom(MorpheumMsg::SettleX402 {
            protocol_id: "hyperlane".to_string(),
            raw_envelope: cosmwasm_std::Binary::from(b"test_envelope".to_vec()),
        })
        .unwrap();

        assert_eq!(app.x402_settlements.len(), 1);
        assert_eq!(app.x402_settlements[0].protocol_id, "hyperlane");
    }

    #[test]
    fn test_mock_query_balance() {
        let dave = module_address("test-dave");
        let mut app = MockMorpheumApp::new();
        app.bank.mint(&dave, 0, 42);

        let result = app
            .query_custom(MorpheumQuery::BankBalance {
                address: dave,
                asset_index: 0,
            })
            .unwrap();

        let resp: crate::bindings::BankBalanceResponse =
            serde_json::from_slice(&result).unwrap();
        assert_eq!(resp.balance, Uint128::new(42));
    }
}
