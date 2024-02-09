use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

fn lunar_new_year() -> H160 {
    H160::from_str("0xDc094eaC7CC01224E798F34543a8F9e9D2559479").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    LunarNewYearModule,
    "./contract/bear/nft/lunar_new_year.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

///
pub async fn total_supply(client: &Client) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = LunarNewYearModule::new(lunar_new_year(), Arc::new(client.clone()));
    let value = contract.total_supply().await?;
    Ok(value)
}

///
pub async fn approve(
    client: &Client,
    account: Address,
    amout: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = LunarNewYearModule::new(lunar_new_year(), Arc::new(client.clone()));
    let value = contract.approve(account, amout).await?;
    Ok(value)
}

///
pub async fn balance_of(
    client: &Client,
    owner: Address,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = LunarNewYearModule::new(lunar_new_year(), Arc::new(client.clone()));
    let value = contract.balance_of(owner).await?;
    Ok(value)
}
