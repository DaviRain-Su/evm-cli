use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::sync::Arc;

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    GovernanceModule,
    "./contract/bear/precompile_contracts/IGovernanceModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// cancelProposal
pub async fn get_all_balances(
    client: &Client,
    contract_addr: &H160,
    proposal_id: u64,
) -> Result<(u64, u64), Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.cancel_proposal(proposal_id).await?;
    Ok(value)
}

// getConstitution
pub async fn get_constitution(
    client: &Client,
    contract_addr: &H160,
) -> Result<String, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_constitution().await?;
    Ok(value)
}
