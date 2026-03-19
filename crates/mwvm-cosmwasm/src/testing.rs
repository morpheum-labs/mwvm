//! Testing helpers for CosmWasm contracts targeting Morpheum.
//!
//! Provides utilities for:
//! - Generating mock `Env` and `MessageInfo` with Morpheum addresses
//! - Creating test instantiation and execution contexts
//! - Validating MorpheumMsg responses from contract executions

use cosmwasm_std::{
    Addr, BlockInfo, ContractInfo, Env, MessageInfo, Timestamp, TransactionInfo,
};

/// Creates a mock `Env` with Morpheum-style configuration.
pub fn mock_morpheum_env() -> Env {
    Env {
        block: BlockInfo {
            height: 12345,
            time: Timestamp::from_seconds(1700000000),
            chain_id: "morpheum-testnet-1".to_string(),
        },
        transaction: Some(TransactionInfo { index: 0 }),
        contract: ContractInfo {
            address: Addr::unchecked("morm1contract0000000000000000000000000000"),
        },
    }
}

/// Creates a mock `MessageInfo` from a Morpheum sender address.
pub fn mock_morpheum_info(sender: &str, funds: &[cosmwasm_std::Coin]) -> MessageInfo {
    MessageInfo {
        sender: Addr::unchecked(sender),
        funds: funds.to_vec(),
    }
}

/// Generates a deterministic Morpheum test address.
pub fn test_address(index: u32) -> String {
    format!("morm1test{:036x}", index)
}

/// Validates that a contract response contains the expected MorpheumMsg.
pub fn assert_has_morpheum_msg(
    response: &cosmwasm_std::Response<crate::bindings::MorpheumMsg>,
    expected_msg_type: &str,
) {
    let found = response.messages.iter().any(|sub_msg| {
        if let cosmwasm_std::CosmosMsg::Custom(ref msg) = sub_msg.msg {
            let msg_type = match msg {
                crate::bindings::MorpheumMsg::MintTo { .. } => "mint_to",
                crate::bindings::MorpheumMsg::BurnFrom { .. } => "burn_from",
                crate::bindings::MorpheumMsg::SettleX402 { .. } => "settle_x402",
            };
            msg_type == expected_msg_type
        } else {
            false
        }
    });
    assert!(
        found,
        "expected MorpheumMsg::{expected_msg_type} in response messages"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_env() {
        let env = mock_morpheum_env();
        assert_eq!(env.block.chain_id, "morpheum-testnet-1");
        assert!(env.contract.address.as_str().starts_with("morm1"));
    }

    #[test]
    fn test_mock_info() {
        let info = mock_morpheum_info("morm1sender", &[]);
        assert_eq!(info.sender.as_str(), "morm1sender");
        assert!(info.funds.is_empty());
    }

    #[test]
    fn test_address_generation() {
        let addr = test_address(0);
        assert!(addr.starts_with("morm1test"));
        assert_eq!(addr.len(), "morm1test".len() + 36);
    }
}
