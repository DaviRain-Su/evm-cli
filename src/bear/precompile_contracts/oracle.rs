//! Oracle 模块（Skip 的 Slinky）负责跟踪和管理货币对及其相关价格。它提供有关货币对、价格、时间戳、区块高度以及每对小数位数的信息。
//! Oracle 模块提供多种功能，包括添加和删除货币对、检查货币对是否正在被跟踪、检索货币对的价格和元数据以及获取给定货币对的小数位数。当添加或删除货币对时，它还会发出事件，从而允许系统的其他部分对这些变化做出反应。
//! 该模块可用于需要访问各种货币对的最新价格信息的智能合约系统。通过集成该预言机模块，Berachain 上的用户和 dapp 可以获得准确可靠的价格数据。
//! Oracle模块预编译合约接口

use crate::Client;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

pub fn oracle_addr() -> H160 {
    H160::from_str("0x9202Af6Ce925b26AE6B25aDfff0B2705147e195F").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    OracleModule,
    "./contract/bear/precompile_contracts/IOracleModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

/// # addCurrencyPairs
/// write 方法添加要由 oracle 模块跟踪的货币对。
pub async fn get_all_balances(
    client: &Client,
    pairs: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = OracleModule::new(oracle_addr(), Arc::new(client.clone()));
    let tx = contract.add_currency_pairs(pairs).await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// # getAllCurrencyPairs
/// 返回预言机模块跟踪的所有货币对。
pub async fn get_all_currency_pairs(
    client: &Client,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let contract = OracleModule::new(oracle_addr(), Arc::new(client.clone()));
    let value = contract.get_all_currency_pairs().call().await?;
    Ok(value)
}

/// # getDecimals
/// 返回给定货币对的小数位数。
pub async fn get_decimals(client: &Client, pair: String) -> Result<u8, Box<dyn std::error::Error>> {
    let contract = OracleModule::new(oracle_addr(), Arc::new(client.clone()));
    let value = contract.get_decimals(pair).call().await?;
    Ok(value)
}

/// # getPrice
/// 返回给定货币对的价格以及元数据。
pub async fn get_price(
    client: &Client,
    pair: String,
) -> Result<(I256, U256, u64), Box<dyn std::error::Error>> {
    let contract = OracleModule::new(oracle_addr(), Arc::new(client.clone()));
    let value = contract.get_price(pair).call().await?;
    Ok(value)
}

/// # hasCurrencyPair
/// 如果预言机模块正在跟踪货币对，则返回。
pub async fn has_currency_pair(
    client: &Client,
    pair: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = OracleModule::new(oracle_addr(), Arc::new(client.clone()));
    let value = contract.has_currency_pair(pair).call().await?;
    Ok(value)
}

/// # removeCurrencyPairs
/// write 方法将货币对从预言机模块的跟踪中删除。
pub async fn remove_currency_pairs(
    client: &Client,
    pairs: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = OracleModule::new(oracle_addr(), Arc::new(client.clone()));
    let tx = contract.remove_currency_pairs(pairs).send().await?.await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
