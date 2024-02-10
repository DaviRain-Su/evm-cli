use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

pub fn rewards_addr() -> H160 {
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
    let value = contract
        .get_current_rewards(depositor, receiver)
        .call()
        .await?;
    Ok(value)
}

// getDepositorWithdrawAddress
pub async fn get_depositor_withdraw_address(
    client: &Client,
    depositor: Address,
) -> Result<Address, Box<dyn std::error::Error>> {
    let contract = RewardsModule::new(rewards_addr(), Arc::new(client.clone()));
    let value = contract
        .get_depositor_withdraw_address(depositor)
        .call()
        .await?;
    Ok(value)
}

// getOutstandingRewards
pub async fn get_outstanding_rewards(
    client: &Client,
    depositor: Address,
) -> Result<Vec<rewards_module::Coin>, Box<dyn std::error::Error>> {
    let contract = RewardsModule::new(rewards_addr(), Arc::new(client.clone()));
    let value = contract.get_outstanding_rewards(depositor).call().await?;
    Ok(value)
}

// setDepositorWithdrawAddress
pub async fn set_depositor_withdraw_address(
    client: &Client,
    withdraw_address: Address,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = RewardsModule::new(rewards_addr(), Arc::new(client.clone()));
    let tx = contract
        .set_depositor_withdraw_address(withdraw_address)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

// withdrawAllDepositorRewards
pub async fn withdraw_all_depositor_rewards(
    client: &Client,
    receiver: Address,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = RewardsModule::new(rewards_addr(), Arc::new(client.clone()));
    let tx = contract
        .withdraw_all_depositor_rewards(receiver)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

// withdrawDepositorRewards
pub async fn withdraw_depositor_rewards(
    client: &Client,
    receiver: Address,
    amount: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = RewardsModule::new(rewards_addr(), Arc::new(client.clone()));
    let tx = contract
        .withdraw_depositor_rewards(receiver, amount)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

// withdrawDepositorRewardsTo
pub async fn withdraw_depositor_rewards_to(
    client: &Client,
    receiver: Address,
    recipient: Address,
    amount: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = RewardsModule::new(rewards_addr(), Arc::new(client.clone()));
    let tx = contract
        .withdraw_depositor_rewards_to(receiver, recipient, amount)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
