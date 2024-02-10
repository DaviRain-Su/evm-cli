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

// addCurrencyPairs
pub async fn get_all_balances(
    client: &Client,
    pairs: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = OracleModule::new(oracle_addr(), Arc::new(client.clone()));
    let tx = contract.add_currency_pairs(pairs).await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

// getAllCurrencyPairs
pub async fn get_all_currency_pairs(
    client: &Client,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let contract = OracleModule::new(oracle_addr(), Arc::new(client.clone()));
    let value = contract.get_all_currency_pairs().call().await?;
    Ok(value)
}
// getDecimals
pub async fn get_decimals(client: &Client, pair: String) -> Result<u8, Box<dyn std::error::Error>> {
    let contract = OracleModule::new(oracle_addr(), Arc::new(client.clone()));
    let value = contract.get_decimals(pair).call().await?;
    Ok(value)
}

// getPrice
pub async fn get_price(
    client: &Client,
    pair: String,
) -> Result<(I256, U256, u64), Box<dyn std::error::Error>> {
    let contract = OracleModule::new(oracle_addr(), Arc::new(client.clone()));
    let value = contract.get_price(pair).call().await?;
    Ok(value)
}

// hasCurrencyPair
pub async fn has_currency_pair(
    client: &Client,
    pair: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = OracleModule::new(oracle_addr(), Arc::new(client.clone()));
    let value = contract.has_currency_pair(pair).call().await?;
    Ok(value)
}

// removeCurrencyPairs
pub async fn remove_currency_pairs(
    client: &Client,
    pairs: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = OracleModule::new(oracle_addr(), Arc::new(client.clone()));
    let tx = contract.remove_currency_pairs(pairs).send().await?.await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
