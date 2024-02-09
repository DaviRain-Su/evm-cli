use crate::Client;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

fn epochs_addr() -> H160 {
    H160::from_str("0x612Dd8a861161819A4AD8F6f3E2A0567602877c0").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    EpochsModule,
    "./contract/bear/precompile_contracts/IEpochsModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// getCurrentEpoch
pub async fn get_current_epoch(
    client: &Client,
    identifier: String,
) -> Result<(i64, i64, i64), Box<dyn std::error::Error>> {
    let contract = EpochsModule::new(epochs_addr(), Arc::new(client.clone()));
    let value = contract.get_current_epoch(identifier).await?;
    Ok(value)
}
