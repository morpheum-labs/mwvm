//! Simulated Morpheum environment for local SVM program testing.
//!
//! `MockSvmApp` provides a local execution environment that simulates
//! Morpheum's system program CPIs (bank balances, x402 settlement)
//! so that SVM programs can be tested without a running node.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::bindings::{
    BankBalanceResponse, AssetInfoResponse, MorpheumCpi, MorpheumSvmQuery,
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

/// Mock Morpheum SVM application for local testing.
///
/// Processes `MorpheumCpi` calls and `MorpheumSvmQuery` queries
/// against simulated state, enabling SVM program testing
/// without a live node.
#[derive(Debug, Clone)]
pub struct MockSvmApp {
    pub bank: MockBankState,
    pub x402_settlements: Vec<X402SettlementRecord>,
}

impl Default for MockSvmApp {
    fn default() -> Self {
        Self::new()
    }
}

impl MockSvmApp {
    pub fn new() -> Self {
        Self {
            bank: MockBankState::new(),
            x402_settlements: Vec::new(),
        }
    }

    /// Simulate a CPI call to a Morpheum system program.
    pub fn execute_cpi(&mut self, cpi: MorpheumCpi) -> Result<(), String> {
        match cpi {
            MorpheumCpi::MintTo {
                recipient,
                asset_index,
                amount,
            } => {
                self.bank.mint(&recipient, asset_index, amount);
                Ok(())
            }
            MorpheumCpi::BurnFrom {
                sender,
                asset_index,
                amount,
            } => self.bank.burn(&sender, asset_index, amount),
            MorpheumCpi::Transfer {
                from,
                to,
                asset_index,
                amount,
            } => self.bank.transfer(&from, &to, asset_index, amount),
            MorpheumCpi::SettleX402 {
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

    /// Execute a query against the simulated state.
    pub fn query(&self, query: MorpheumSvmQuery) -> Result<Vec<u8>, String> {
        match query {
            MorpheumSvmQuery::BankBalance {
                address,
                asset_index,
            } => {
                let balance = self.bank.balance(&address, asset_index);
                let resp = BankBalanceResponse { balance };
                serde_json::to_vec(&resp).map_err(|e| e.to_string())
            }
            MorpheumSvmQuery::AssetInfo { asset_index } => {
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
        let mut app = MockSvmApp::new();

        app.execute_cpi(MorpheumCpi::MintTo {
            recipient: "mormPK1alice".to_string(),
            asset_index: 1,
            amount: 1_000_000,
        })
        .unwrap();

        assert_eq!(app.bank.balance("mormPK1alice", 1), 1_000_000);
        assert_eq!(app.bank.supply(1), 1_000_000);
    }

    #[test]
    fn test_mock_burn() {
        let mut app = MockSvmApp::new();
        app.bank.mint("mormPK1bob", 1, 500_000);

        app.execute_cpi(MorpheumCpi::BurnFrom {
            sender: "mormPK1bob".to_string(),
            asset_index: 1,
            amount: 200_000,
        })
        .unwrap();

        assert_eq!(app.bank.balance("mormPK1bob", 1), 300_000);
    }

    #[test]
    fn test_mock_transfer() {
        let mut app = MockSvmApp::new();
        app.bank.mint("mormPK1alice", 0, 1000);

        app.execute_cpi(MorpheumCpi::Transfer {
            from: "mormPK1alice".to_string(),
            to: "mormPK1bob".to_string(),
            asset_index: 0,
            amount: 400,
        })
        .unwrap();

        assert_eq!(app.bank.balance("mormPK1alice", 0), 600);
        assert_eq!(app.bank.balance("mormPK1bob", 0), 400);
    }

    #[test]
    fn test_mock_burn_insufficient() {
        let mut app = MockSvmApp::new();
        app.bank.mint("mormPK1charlie", 1, 100);

        let result = app.execute_cpi(MorpheumCpi::BurnFrom {
            sender: "mormPK1charlie".to_string(),
            asset_index: 1,
            amount: 200,
        });

        assert!(result.is_err());
    }

    #[test]
    fn test_mock_x402_settlement() {
        let mut app = MockSvmApp::new();

        app.execute_cpi(MorpheumCpi::SettleX402 {
            protocol_id: "hyperlane".to_string(),
            raw_envelope: b"test_envelope".to_vec(),
        })
        .unwrap();

        assert_eq!(app.x402_settlements.len(), 1);
        assert_eq!(app.x402_settlements[0].protocol_id, "hyperlane");
    }

    #[test]
    fn test_mock_query_balance() {
        let mut app = MockSvmApp::new();
        app.bank.mint("mormPK1dave", 0, 42);

        let result = app
            .query(MorpheumSvmQuery::BankBalance {
                address: "mormPK1dave".to_string(),
                asset_index: 0,
            })
            .unwrap();

        let resp: BankBalanceResponse = serde_json::from_slice(&result).unwrap();
        assert_eq!(resp.balance, 42);
    }
}
