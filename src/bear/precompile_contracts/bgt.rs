use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::sync::Arc;

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    BGTModule,
    "./contract/bear/precompile_contracts/IBGTModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// redeem
pub async fn redeem(
    client: &Client,
    contract_addr: &H160,
    receiver: Address,
    amount: U256,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = BGTModule::new(contract_addr.clone(), Arc::new(client.clone()));

    let value = contract.redeem(receiver, amount).await?;

    Ok(value)
}
