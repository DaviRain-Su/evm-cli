use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

fn bank_addr() -> H160 {
    H160::from_str("0x4381dC2aB14285160c808659aEe005D51255adD7").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    BankModule,
    "./contract/bear/precompile_contracts/IBankModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// get_all_balances
pub async fn get_all_balances(
    client: &Client,
    account_address: Address,
) -> Result<Vec<bank_module::Coin>, Box<dyn std::error::Error>> {
    let contract = BankModule::new(bank_addr(), Arc::new(client.clone()));
    let value = contract.get_all_balances(account_address).await?;
    Ok(value)
}

// getAllSpendableBalances
pub async fn get_all_spendable_balances(
    client: &Client,
    account_address: Address,
) -> Result<Vec<bank_module::Coin>, Box<dyn std::error::Error>> {
    let contract = BankModule::new(bank_addr(), Arc::new(client.clone()));
    let value = contract.get_all_spendable_balances(account_address).await?;
    Ok(value)
}

/// getAllSupply
pub async fn get_all_supply(
    client: &Client,
) -> Result<Vec<bank_module::Coin>, Box<dyn std::error::Error>> {
    let contract = BankModule::new(bank_addr(), Arc::new(client.clone()));
    let value = contract.get_all_supply().await?;
    Ok(value)
}

/// getBalance
pub async fn get_balance(
    client: &Client,
    account_address: Address,
    denom: String,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = BankModule::new(bank_addr(), Arc::new(client.clone()));
    let value = contract.get_balance(account_address, denom).await?;
    Ok(value)
}

// getSpendableBalance
pub async fn get_spendable_balance(
    client: &Client,
    account_address: Address,
    denom: String,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = BankModule::new(bank_addr(), Arc::new(client.clone()));
    let value = contract
        .get_spendable_balance(account_address, denom)
        .await?;
    Ok(value)
}

// getSupply
pub async fn get_supply(
    client: &Client,
    denom: String,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = BankModule::new(bank_addr(), Arc::new(client.clone()));
    let value = contract.get_supply(denom).await?;
    Ok(value)
}

/// send
pub async fn send(
    client: &Client,
    to_address: Address,
    amount: Vec<bank_module::Coin>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = BankModule::new(bank_addr(), Arc::new(client.clone()));
    let value = contract.send(to_address, amount).await?;
    Ok(value)
}
