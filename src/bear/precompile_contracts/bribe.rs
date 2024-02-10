use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

pub fn bribe_addr() -> H160 {
    H160::from_str("0xFCE07324E0E72E071842374E9997CF65DF990CBC").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    BribeModule,
    "./contract/bear/precompile_contracts/IBribeModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// createBribe
pub async fn create_bribe(
    client: &Client,
    operator: Address,
    start_epoch: u64,
    num_block_proposals: u64,
    bribe_per_proposal: Vec<bribe_module::Coin>,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = BribeModule::new(bribe_addr(), Arc::new(client.clone()));
    let tx = contract
        .create_bribe(
            operator,
            start_epoch,
            num_block_proposals,
            bribe_per_proposal,
        )
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

//getActiveValidatorBribes
pub async fn get_active_validator_bribes(
    client: &Client,
    operator: Address,
) -> Result<Vec<bribe_module::Bribe>, Box<dyn std::error::Error>> {
    let contract = BribeModule::new(bribe_addr(), Arc::new(client.clone()));
    let value = contract
        .get_active_validator_bribes(operator)
        .call()
        .await?;
    Ok(value)
}

// getAllValidatorBribes
pub async fn get_all_validator_bribes(
    client: &Client,
    operator: Address,
) -> Result<Vec<bribe_module::Bribe>, Box<dyn std::error::Error>> {
    let contract = BribeModule::new(bribe_addr(), Arc::new(client.clone()));
    let value = contract.get_all_validator_bribes(operator).call().await?;
    Ok(value)
}

// getBribeFees
pub async fn get_bribe_fees(
    client: &Client,
) -> Result<Vec<bribe_module::Coin>, Box<dyn std::error::Error>> {
    let contract = BribeModule::new(bribe_addr(), Arc::new(client.clone()));
    let value = contract.get_bribe_fees().call().await?;
    Ok(value)
}

// getBribes
pub async fn get_bribes(
    client: &Client,
    operator: Address,
    start_epoch: u64,
) -> Result<Vec<bribe_module::Bribe>, Box<dyn std::error::Error>> {
    let contract = BribeModule::new(bribe_addr(), Arc::new(client.clone()));
    let value = contract.get_bribes(operator, start_epoch).call().await?;
    Ok(value)
}

// updateParams
pub async fn update_params(
    client: &Client,
    fee: Vec<bribe_module::Coin>,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = BribeModule::new(bribe_addr(), Arc::new(client.clone()));
    let tx = contract.update_params(fee).send().await?.await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
