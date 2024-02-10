use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

pub fn bgt_addr() -> H160 {
    H160::from_str("0x09E585D2BDEB5ECF90ADE67DCE1125070D2714A3").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    BGTModule,
    "./contract/bear/precompile_contracts/IBGTModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// redeem
pub async fn redeem(
    client: &Client,
    receiver: Address,
    amount: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = BGTModule::new(bgt_addr(), Arc::new(client.clone()));
    let tx = contract.redeem(receiver, amount).send().await?.await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
