use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::sync::Arc;

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    BankModule,
    "./contract/bear/precompile_contracts/IBankModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// get_all_balances
pub async fn get_all_balances(
    client: &Client,
    contract_addr: &H160,
    account_address: Address,
) -> Result<Vec<bank_module::Coin>, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = BankModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.get_all_balances(account_address).await?;

    // 6. Return the number
    Ok(value)
}

// getAllSpendableBalances
pub async fn get_all_spendable_balances(
    client: &Client,
    contract_addr: &H160,
    account_address: Address,
) -> Result<Vec<bank_module::Coin>, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = BankModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.get_all_spendable_balances(account_address).await?;

    // 6. Return the number
    Ok(value)
}

/// getAllSupply
pub async fn get_all_supply(
    client: &Client,
    contract_addr: &H160,
) -> Result<Vec<bank_module::Coin>, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = BankModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.get_all_supply().await?;

    // 6. Return the number
    Ok(value)
}

/// getBalance
pub async fn get_balance(
    client: &Client,
    contract_addr: &H160,
    account_address: Address,
    denom: String,
) -> Result<U256, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = BankModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.get_balance(account_address, denom).await?;

    // 6. Return the number
    Ok(value)
}

// getSpendableBalance
pub async fn get_spendable_balance(
    client: &Client,
    contract_addr: &H160,
    account_address: Address,
    denom: String,
) -> Result<U256, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = BankModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract
        .get_spendable_balance(account_address, denom)
        .await?;

    // 6. Return the number
    Ok(value)
}

// getSupply
pub async fn get_supply(
    client: &Client,
    contract_addr: &H160,
    denom: String,
) -> Result<U256, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = BankModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.get_supply(denom).await?;

    // 6. Return the number
    Ok(value)
}

/// send
pub async fn send(
    client: &Client,
    contract_addr: &H160,
    to_address: Address,
    amount: Vec<bank_module::Coin>,
) -> Result<bool, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = BankModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.send(to_address, amount).await?;

    // 6. Return the number
    Ok(value)
}
