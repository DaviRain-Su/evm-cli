use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

pub fn distribution_addr() -> H160 {
    H160::from_str("0x0000000000000000000000000000000000000069").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    DistributeModule,
    "./contract/bear/precompile_contracts/IDistributionModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// getAllDelegatorRewards
pub async fn get_all_delegator_rewards(
    client: &Client,
    account_address: Address,
) -> Result<Vec<distribute_module::ValidatorReward>, Box<dyn std::error::Error>> {
    let contract = DistributeModule::new(distribution_addr(), Arc::new(client.clone()));
    let value = contract
        .get_all_delegator_rewards(account_address)
        .call()
        .await?;
    Ok(value)
}

/// getDelegatorValidatorReward
pub async fn get_delegator_validator_reward(
    client: &Client,
    delegator: Address,
    validator: Address,
) -> Result<Vec<distribute_module::Coin>, Box<dyn std::error::Error>> {
    let contract = DistributeModule::new(distribution_addr(), Arc::new(client.clone()));
    let value = contract
        .get_delegator_validator_reward(delegator, validator)
        .call()
        .await?;
    Ok(value)
}

/// getTotalDelegatorReward
pub async fn get_total_delegator_reward(
    client: &Client,
    delegator: Address,
) -> Result<Vec<distribute_module::Coin>, Box<dyn std::error::Error>> {
    let contract = DistributeModule::new(distribution_addr(), Arc::new(client.clone()));
    let value = contract
        .get_total_delegator_reward(delegator)
        .call()
        .await?;
    Ok(value)
}

///getWithdrawAddress
pub async fn get_withdraw_address(
    client: &Client,
    delegator: Address,
) -> Result<Address, Box<dyn std::error::Error>> {
    let contract = DistributeModule::new(distribution_addr(), Arc::new(client.clone()));
    let value = contract.get_withdraw_address(delegator).call().await?;
    Ok(value)
}

///getWithdrawEnabled
pub async fn get_withdraw_enabled(client: &Client) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = DistributeModule::new(distribution_addr(), Arc::new(client.clone()));
    let value = contract.get_withdraw_enabled().call().await?;
    Ok(value)
}

// setWithdrawAddress
pub async fn set_withdraw_address(
    client: &Client,
    withdraw_address: Address,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = DistributeModule::new(distribution_addr(), Arc::new(client.clone()));
    let tx = contract
        .set_withdraw_address(withdraw_address)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

// withdrawDelegatorReward
pub async fn withdraw_delegator_reward(
    client: &Client,
    delegator: Address,
    validator: Address,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = DistributeModule::new(distribution_addr(), Arc::new(client.clone()));
    let tx = contract
        .withdraw_delegator_reward(delegator, validator)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
