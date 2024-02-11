//! 银行预编译为 Cosmos SDK 银行模块提供接口，该模块负责管理代币转账和余额。
//! 该接口定义了与银行模块交互的各种事件和方法。
//! Bank 模块发出的所有支持的 Cosmos 事件的接口
//!

use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

pub fn bank_addr() -> H160 {
    H160::from_str("0x4381dC2aB14285160c808659aEe005D51255adD7").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    BankModule,
    "./contract/bear/precompile_contracts/IBankModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

/// getAllBalances
/// 如果未找到账户地址，则返回空数组。
/// 按地址返回所有面额的账户余额
pub async fn get_all_balances(
    client: &Client,
    account_address: Address,
) -> Result<Vec<bank_module::Coin>, Box<dyn std::error::Error>> {
    let contract = BankModule::new(bank_addr(), Arc::new(client.clone()));
    let value = contract.get_all_balances(account_address).call().await?;
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
    let value = contract
        .get_all_spendable_balances(account_address)
        .call()
        .await?;
    Ok(value)
}

/// getAllSupply
/// 返回所有代币的总供应量
pub async fn get_all_supply(
    client: &Client,
) -> Result<Vec<bank_module::Coin>, Box<dyn std::error::Error>> {
    let contract = BankModule::new(bank_addr(), Arc::new(client.clone()));
    let value = contract.get_all_supply().call().await?;
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
    let value = contract.get_balance(account_address, denom).call().await?;
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
        .call()
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
    let value = contract.get_supply(denom).call().await?;
    Ok(value)
}

/// send
/// 如果发送者没有足够的余额，则返回 false
/// 将硬币从 msg.sender 发送到另一个
/// - to_address: 收件人地址
/// - amount: 发送的 Cosmos 币数量
pub async fn send(
    client: &Client,
    to_address: Address,
    amount: Vec<bank_module::Coin>,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = BankModule::new(bank_addr(), Arc::new(client.clone()));
    let tx = contract.send(to_address, amount).send().await?.await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
