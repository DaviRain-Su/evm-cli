use super::super::Client;
use ethers::prelude::*;
use std::sync::Arc;

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    Honey,
    "./contract/bear/Honey.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// 2. Define an asynchronous function that takes a client provider and address as input and returns a U256
pub async fn name(
    client: &Client,
    contract_addr: &H160,
) -> Result<String, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = Honey::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.name().call().await?;

    // 6. Return the number
    Ok(value)
}
