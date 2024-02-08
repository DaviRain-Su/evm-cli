use crate::Client;
use ethers::prelude::*;
use std::sync::Arc;

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    EpochsModule,
    "./contract/bear/precompile_contracts/IEpochsModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// getCurrentEpoch
pub async fn get_current_epoch(
    client: &Client,
    contract_addr: &H160,
    identifier: String,
) -> Result<(i64, i64, i64), Box<dyn std::error::Error>> {
    let contract = EpochsModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_current_epoch(identifier).await?;
    Ok(value)
}
