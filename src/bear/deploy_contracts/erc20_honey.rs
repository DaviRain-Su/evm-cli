use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

pub fn erc20_honey_addr() -> H160 {
    H160::from_str("0x09ec711b81cD27A6466EC40960F2f8D85BB129D9").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    ERC20HoneyModule,
    "./contract/bear/deploy_contracts/ERC20HoneyModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

/// erc20Module
pub async fn erc_20_module(client: &Client) -> Result<Address, Box<dyn std::error::Error>> {
    let contract = ERC20HoneyModule::new(erc20_honey_addr(), Arc::new(client.clone()));
    let value = contract.erc_20_module().call().await?;
    Ok(value)
}

/// getAMOCurrentLimit
pub async fn get_amo_current_limit(
    client: &Client,
    amo_type: String,
    amo_addr: Address,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = ERC20HoneyModule::new(erc20_honey_addr(), Arc::new(client.clone()));
    let value = contract
        .get_amo_current_limit(amo_type, amo_addr)
        .call()
        .await?;
    Ok(value)
}

/// getExchangable
pub async fn get_exchangable(
    client: &Client,
) -> Result<Vec<Erc20Exchangable>, Box<dyn std::error::Error>> {
    let contract = ERC20HoneyModule::new(erc20_honey_addr(), Arc::new(client.clone()));
    let value = contract.get_exchangable().call().await?;
    Ok(value)
}

// getTotalCollateral
pub async fn get_total_collateral(
    client: &Client,
) -> Result<(Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20HoneyModule::new(erc20_honey_addr(), Arc::new(client.clone()));
    let value = contract.get_total_collateral().call().await?;
    Ok(value)
}

/// getTotalSupply
pub async fn get_total_supply(client: &Client) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = ERC20HoneyModule::new(erc20_honey_addr(), Arc::new(client.clone()));
    let value = contract.get_total_supply().call().await?;
    Ok(value)
}

/// honey
pub async fn honey(client: &Client) -> Result<Address, Box<dyn std::error::Error>> {
    let contract = ERC20HoneyModule::new(erc20_honey_addr(), Arc::new(client.clone()));
    let value = contract.honey().call().await?;
    Ok(value)
}

/// honeyModule
pub async fn honey_module(client: &Client) -> Result<Address, Box<dyn std::error::Error>> {
    let contract = ERC20HoneyModule::new(erc20_honey_addr(), Arc::new(client.clone()));
    let value = contract.honey_module().call().await?;
    Ok(value)
}

/// mint
pub async fn mint(
    client: &Client,
    to: Address,
    collateral: Address,
    amount: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = ERC20HoneyModule::new(erc20_honey_addr(), Arc::new(client.clone()));
    let tx = contract.mint(to, collateral, amount).send().await?.await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// previewExactOutCollateral
pub async fn preview_exact_out_collateral(
    client: &Client,
    amount_out: U256,
    asset_out: Address,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = ERC20HoneyModule::new(erc20_honey_addr(), Arc::new(client.clone()));
    let tx = contract
        .preview_exact_out_collateral(amount_out, asset_out)
        .call()
        .await?;

    Ok(tx)
}

/// previewMint
pub async fn preview_mint(
    client: &Client,
    collateral: Address,
    amount: U256,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = ERC20HoneyModule::new(erc20_honey_addr(), Arc::new(client.clone()));
    let tx = contract.preview_mint(collateral, amount).call().await?;

    Ok(tx)
}

// previewRedeem
pub async fn preview_redeem(
    client: &Client,
    collateral: Address,
    amount: U256,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = ERC20HoneyModule::new(erc20_honey_addr(), Arc::new(client.clone()));
    let tx = contract.preview_redeem(collateral, amount).call().await?;

    Ok(tx)
}

// previewRequiredCollateral
pub async fn preview_required_collateral(
    client: &Client,
    honey_out: U256,
    asset_in: Address,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = ERC20HoneyModule::new(erc20_honey_addr(), Arc::new(client.clone()));
    let tx = contract
        .preview_required_collateral(honey_out, asset_in)
        .call()
        .await?;

    Ok(tx)
}

/// redeem
pub async fn redeem(
    client: &Client,
    to: Address,
    amount: U256,
    collateral: Address,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = ERC20HoneyModule::new(erc20_honey_addr(), Arc::new(client.clone()));
    let tx = contract
        .redeem(to, amount, collateral)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// updateParams
pub async fn update_params(
    client: &Client,
    params: Vec<Erc20Exchangable>,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = ERC20HoneyModule::new(erc20_honey_addr(), Arc::new(client.clone()));
    let tx = contract.update_params(params).send().await?.await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
