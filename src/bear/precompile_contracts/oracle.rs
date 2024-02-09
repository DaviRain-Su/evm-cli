use crate::Client;
use ethers::prelude::*;
use std::sync::Arc;

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    OracleModule,
    "./contract/bear/precompile_contracts/IOracleModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// addCurrencyPairs
pub async fn get_all_balances(
    client: &Client,
    contract_addr: &H160,
    pairs: Vec<String>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = OracleModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.add_currency_pairs(pairs).await?;
    Ok(value)
}

// getAllCurrencyPairs
pub async fn get_all_currency_pairs(
    client: &Client,
    contract_addr: &H160,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let contract = OracleModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_all_currency_pairs().await?;
    Ok(value)
}
// getDecimals
pub async fn get_decimals(
    client: &Client,
    contract_addr: &H160,
    pair: String,
) -> Result<u8, Box<dyn std::error::Error>> {
    let contract = OracleModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_decimals(pair).await?;
    Ok(value)
}

// getPrice
pub async fn get_price(
    client: &Client,
    contract_addr: &H160,
    pair: String,
) -> Result<(I256, U256, u64), Box<dyn std::error::Error>> {
    let contract = OracleModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_price(pair).await?;
    Ok(value)
}

// hasCurrencyPair
pub async fn has_currency_pair(
    client: &Client,
    contract_addr: &H160,
    pair: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = OracleModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.has_currency_pair(pair).await?;
    Ok(value)
}

// removeCurrencyPairs
pub async fn remove_currency_pairs(
    client: &Client,
    contract_addr: &H160,
    pairs: Vec<String>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = OracleModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.remove_currency_pairs(pairs).await?;
    Ok(value)
}
