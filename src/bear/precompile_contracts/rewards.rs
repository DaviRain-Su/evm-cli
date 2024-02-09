use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

fn rewards_addr() -> H160 {
    H160::from_str("0x55684E2CA2BACE0ADC512C1AFF880B15B8EA7214").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    RewardsModule,
    "./contract/bear/precompile_contracts/IRewardsModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// getCurrentRewards
pub async fn get_current_rewards(
    client: &Client,
    depositor: Address,
    receiver: Address,
) -> Result<Vec<rewards_module::Coin>, Box<dyn std::error::Error>> {
    let contract = RewardsModule::new(rewards_addr(), Arc::new(client.clone()));
    let value = contract.get_current_rewards(depositor, receiver).await?;
    Ok(value)
}

// getDepositorWithdrawAddress
pub async fn get_depositor_withdraw_address(
    client: &Client,
    depositor: Address,
) -> Result<Address, Box<dyn std::error::Error>> {
    let contract = RewardsModule::new(rewards_addr(), Arc::new(client.clone()));
    let value = contract.get_depositor_withdraw_address(depositor).await?;
    Ok(value)
}

// getOutstandingRewards
pub async fn get_outstanding_rewards(
    client: &Client,
    depositor: Address,
) -> Result<Vec<rewards_module::Coin>, Box<dyn std::error::Error>> {
    let contract = RewardsModule::new(rewards_addr(), Arc::new(client.clone()));
    let value = contract.get_outstanding_rewards(depositor).await?;
    Ok(value)
}

// setDepositorWithdrawAddress
pub async fn set_depositor_withdraw_address(
    client: &Client,
    withdraw_address: Address,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = RewardsModule::new(rewards_addr(), Arc::new(client.clone()));
    let value = contract
        .set_depositor_withdraw_address(withdraw_address)
        .await?;
    Ok(value)
}

// withdrawAllDepositorRewards
pub async fn withdraw_all_depositor_rewards(
    client: &Client,
    receiver: Address,
) -> Result<Vec<rewards_module::Coin>, Box<dyn std::error::Error>> {
    let contract = RewardsModule::new(rewards_addr(), Arc::new(client.clone()));
    let value = contract.withdraw_all_depositor_rewards(receiver).await?;
    Ok(value)
}

// withdrawDepositorRewards
pub async fn withdraw_depositor_rewards(
    client: &Client,
    receiver: Address,
    amount: U256,
) -> Result<Vec<rewards_module::Coin>, Box<dyn std::error::Error>> {
    let contract = RewardsModule::new(rewards_addr(), Arc::new(client.clone()));
    let value = contract
        .withdraw_depositor_rewards(receiver, amount)
        .await?;
    Ok(value)
}

// withdrawDepositorRewardsTo
pub async fn withdraw_depositor_rewards_to(
    client: &Client,
    receiver: Address,
    recipient: Address,
    amount: U256,
) -> Result<Vec<rewards_module::Coin>, Box<dyn std::error::Error>> {
    let contract = RewardsModule::new(rewards_addr(), Arc::new(client.clone()));
    let value = contract
        .withdraw_depositor_rewards_to(receiver, recipient, amount)
        .await?;
    Ok(value)
}
