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

/// 如果未找到账户地址，则返回空数组。
/// 按地址返回所有面额的账户余额
pub async fn get_all_balances(
    client: &Client,
    account_address: Address,
) -> Result<Vec<bank_module::Coin>, Box<dyn std::error::Error>> {
    let contract = BankModule::new(bank_addr(), Arc::new(client.clone()));
    let value = contract.get_all_balances(account_address).await?;
    Ok(value)
}

/// getAllSpendableBalances
/// 如果未找到账户地址，则返回空数组
/// 按地址返回所有硬币面额的账户余额
pub async fn get_all_spendable_balances(
    client: &Client,
    account_address: Address,
) -> Result<Vec<bank_module::Coin>, Box<dyn std::error::Error>> {
    let contract = BankModule::new(bank_addr(), Arc::new(client.clone()));
    let value = contract.get_all_spendable_balances(account_address).await?;
    Ok(value)
}

/// getAllSupply
/// 返回所有代币的总供应量
pub async fn get_all_supply(
    client: &Client,
) -> Result<Vec<bank_module::Coin>, Box<dyn std::error::Error>> {
    let contract = BankModule::new(bank_addr(), Arc::new(client.clone()));
    let value = contract.get_all_supply().await?;
    Ok(value)
}

/// getBalance
/// 如果未找到面额，则返回 0
/// 按地址返回给定硬币面额的帐户余额 amount
pub async fn get_balance(
    client: &Client,
    account_address: Address,
    denom: String,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = BankModule::new(bank_addr(), Arc::new(client.clone()));
    let value = contract.get_balance(account_address, denom).await?;
    Ok(value)
}

/// getSpendableBalance
/// 如果未找到面额，则返回 0
/// 按地址返回给定硬币面额的帐户余额 amount
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

/// getSupply
/// 返回单个代币的总供应量
pub async fn get_supply(
    client: &Client,
    denom: String,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = BankModule::new(bank_addr(), Arc::new(client.clone()));
    let value = contract.get_supply(denom).await?;
    Ok(value)
}

/// send
/// 如果发送者没有足够的余额，则返回 false
/// 将硬币从 msg.sender 发送到另一个
pub async fn send(
    client: &Client,
    to_address: Address,
    amount: Vec<bank_module::Coin>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = BankModule::new(bank_addr(), Arc::new(client.clone()));
    let value = contract.send(to_address, amount).await?;
    Ok(value)
}
