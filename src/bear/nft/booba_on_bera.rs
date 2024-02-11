use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

pub fn booba_on_bera_addr() -> H160 {
    H160::from_str("0x1F136a43101D12F98c9887D46D7cDbEFACdd573D").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    BooBaOnBeraModule,
    "./contract/bear/nft/booba_on_bera.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

pub async fn approve(
    client: &Client,
    account: Address,
    amout: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = BooBaOnBeraModule::new(booba_on_bera_addr(), Arc::new(client.clone()));
    let tx = contract.approve(account, amout).send().await?.await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// boobaMint
pub async fn booba_mint(
    client: &Client,
    account: Address,
    value: U256,
    mint_nft: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = BooBaOnBeraModule::new(booba_on_bera_addr(), Arc::new(client.clone()));
    let tx = contract.booba_mint(value, mint_nft).send().await?.await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

///
pub async fn erc_20_balance_of(
    client: &Client,
    owner: Address,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = BooBaOnBeraModule::new(booba_on_bera_addr(), Arc::new(client.clone()));
    let value = contract.erc_20_balance_of(owner).call().await?;
    Ok(value)
}

///
pub async fn erc_721_balance_of(
    client: &Client,
    owner: Address,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = BooBaOnBeraModule::new(booba_on_bera_addr(), Arc::new(client.clone()));
    let value = contract.erc_721_balance_of(owner).call().await?;
    Ok(value)
}

///
pub async fn erc_20_total_supply(client: &Client) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = BooBaOnBeraModule::new(booba_on_bera_addr(), Arc::new(client.clone()));
    let value = contract.erc_20_total_supply().call().await?;
    Ok(value)
}

///
pub async fn erc_721_total_supply(client: &Client) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = BooBaOnBeraModule::new(booba_on_bera_addr(), Arc::new(client.clone()));
    let value = contract.erc_721_total_supply().call().await?;
    Ok(value)
}

///
pub async fn decimal(client: &Client) -> Result<u8, Box<dyn std::error::Error>> {
    let contract = BooBaOnBeraModule::new(booba_on_bera_addr(), Arc::new(client.clone()));
    let value = contract.decimals().call().await?;
    Ok(value)
}
