//! Simulated Morpheum environment for local EVM contract testing.
//!
//! `MockEvmApp` provides a local execution environment that simulates
//! Morpheum's precompile calls (bank balances, x402 settlement)
//! so that Solidity/Vyper contracts can be tested without a running node.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::bindings::{
    BankBalanceResponse, AssetInfoResponse, MorpheumEvmQuery, MorpheumPrecompile,
};

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

    pub fn transfer(
        &mut self,
        from: &str,
        to: &str,
        asset_index: u64,
        amount: u128,
    ) -> Result<(), String> {
        self.burn(from, asset_index, amount)?;
        self.mint(to, asset_index, amount);
        Ok(())
    }

    pub fn supply(&self, asset_index: u64) -> u128 {
        self.supplies.get(&asset_index).copied().unwrap_or(0)
    }

    pub fn asset_symbol(&self, asset_index: u64) -> Option<&str> {
        self.assets.get(&asset_index).map(|s| s.as_str())
    }
}

/// Record of an x402 settlement for test assertions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct X402SettlementRecord {
    pub protocol_id: String,
    pub raw_envelope: Vec<u8>,
}

/// Mock Morpheum EVM application for local testing.
///
/// Processes `MorpheumPrecompile` calls and `MorpheumEvmQuery` queries
/// against simulated state, enabling Solidity/Vyper contract testing
/// without a live node.
#[derive(Debug, Clone)]
pub struct MockEvmApp {
    pub bank: MockBankState,
    pub x402_settlements: Vec<X402SettlementRecord>,
}

impl Default for MockEvmApp {
    fn default() -> Self {
        Self::new()
    }
}

impl MockEvmApp {
    pub fn new() -> Self {
        Self {
            bank: MockBankState::new(),
            x402_settlements: Vec::new(),
        }
    }

    /// Simulate a precompile call against the mock state.
    pub fn call_precompile(&mut self, call: MorpheumPrecompile) -> Result<(), String> {
        match call {
            MorpheumPrecompile::MintTo {
                recipient,
                asset_index,
                amount,
            } => {
                self.bank.mint(&recipient, asset_index, amount);
                Ok(())
            }
            MorpheumPrecompile::BurnFrom {
                sender,
                asset_index,
                amount,
            } => self.bank.burn(&sender, asset_index, amount),
            MorpheumPrecompile::Transfer {
                from,
                to,
                asset_index,
                amount,
            } => self.bank.transfer(&from, &to, asset_index, amount),
            MorpheumPrecompile::SettleX402 {
                protocol_id,
                raw_envelope,
            } => {
                self.x402_settlements.push(X402SettlementRecord {
                    protocol_id,
                    raw_envelope,
                });
                Ok(())
            }
        }
    }

    /// Execute a query precompile call against the mock state.
    pub fn query_precompile(
        &self,
        query: MorpheumEvmQuery,
    ) -> Result<Vec<u8>, String> {
        match query {
            MorpheumEvmQuery::BankBalance {
                address,
                asset_index,
            } => {
                let balance = self.bank.balance(&address, asset_index);
                let resp = BankBalanceResponse { balance };
                serde_json::to_vec(&resp).map_err(|e| e.to_string())
            }
            MorpheumEvmQuery::AssetInfo { asset_index } => {
                let symbol = self
                    .bank
                    .asset_symbol(asset_index)
                    .unwrap_or("UNKNOWN")
                    .to_string();
                let supply = self.bank.supply(asset_index);
                let resp = AssetInfoResponse {
                    asset_index,
                    symbol,
                    total_supply: supply,
                };
                serde_json::to_vec(&resp).map_err(|e| e.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_mint_and_balance() {
        let mut app = MockEvmApp::new();

        app.call_precompile(MorpheumPrecompile::MintTo {
            recipient: "0xalice".to_string(),
            asset_index: 1,
            amount: 1_000_000,
        })
        .unwrap();

        assert_eq!(app.bank.balance("0xalice", 1), 1_000_000);
        assert_eq!(app.bank.supply(1), 1_000_000);
    }

    #[test]
    fn test_mock_burn() {
        let mut app = MockEvmApp::new();
        app.bank.mint("0xbob", 1, 500_000);

        app.call_precompile(MorpheumPrecompile::BurnFrom {
            sender: "0xbob".to_string(),
            asset_index: 1,
            amount: 200_000,
        })
        .unwrap();

        assert_eq!(app.bank.balance("0xbob", 1), 300_000);
    }

    #[test]
    fn test_mock_transfer() {
        let mut app = MockEvmApp::new();
        app.bank.mint("0xalice", 0, 1000);

        app.call_precompile(MorpheumPrecompile::Transfer {
            from: "0xalice".to_string(),
            to: "0xbob".to_string(),
            asset_index: 0,
            amount: 400,
        })
        .unwrap();

        assert_eq!(app.bank.balance("0xalice", 0), 600);
        assert_eq!(app.bank.balance("0xbob", 0), 400);
    }

    #[test]
    fn test_mock_burn_insufficient() {
        let mut app = MockEvmApp::new();
        app.bank.mint("0xcharlie", 1, 100);

        let result = app.call_precompile(MorpheumPrecompile::BurnFrom {
            sender: "0xcharlie".to_string(),
            asset_index: 1,
            amount: 200,
        });

        assert!(result.is_err());
    }

    #[test]
    fn test_mock_x402_settlement() {
        let mut app = MockEvmApp::new();

        app.call_precompile(MorpheumPrecompile::SettleX402 {
            protocol_id: "hyperlane".to_string(),
            raw_envelope: b"test_envelope".to_vec(),
        })
        .unwrap();

        assert_eq!(app.x402_settlements.len(), 1);
        assert_eq!(app.x402_settlements[0].protocol_id, "hyperlane");
    }

    #[test]
    fn test_mock_query_balance() {
        let mut app = MockEvmApp::new();
        app.bank.mint("0xdave", 0, 42);

        let result = app
            .query_precompile(MorpheumEvmQuery::BankBalance {
                address: "0xdave".to_string(),
                asset_index: 0,
            })
            .unwrap();

        let resp: BankBalanceResponse = serde_json::from_slice(&result).unwrap();
        assert_eq!(resp.balance, 42);
    }
}
