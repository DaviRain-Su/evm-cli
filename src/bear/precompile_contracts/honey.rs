use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::sync::Arc;

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    HoneyModule,
    "./contract/bear/precompile_contracts/IHoneyModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// getAMOCurrentLimit
pub async fn get_amo_current_limt(
    client: &Client,
    contract_addr: &H160,
    amo_type: String,
    amo_addr: Address,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_amo_current_limit(amo_type, amo_addr).await?;
    Ok(value)
}

/// getParams
pub async fn get_params(
    client: &Client,
    contract_addr: &H160,
) -> Result<Vec<honey_module::Exchangeable>, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_params().await?;
    Ok(value)
}

// getTotalCollateral
pub async fn get_total_collateral(
    client: &Client,
    contract_addr: &H160,
) -> Result<Vec<honey_module::Coin>, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_total_collateral().await?;
    Ok(value)
}

// getTotalSupply
pub async fn get_total_supply(
    client: &Client,
    contract_addr: &H160,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_total_supply().await?;
    Ok(value)
}

// mint
pub async fn mint(
    client: &Client,
    contract_addr: &H160,
    to: Address,
    collateral: honey_module::Coin,
) -> Result<honey_module::Coin, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.mint(to, collateral).await?;
    Ok(value)
}

// previewExactOutCollateral
pub async fn preview_exact_out_collateral(
    client: &Client,
    contract_addr: &H160,
    collateral_out: honey_module::Coin,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract
        .preview_exact_out_collateral(collateral_out)
        .await?;
    Ok(value)
}

// previewMint
//
pub async fn preview_mint(
    client: &Client,
    contract_addr: &H160,
    coin: honey_module::Coin,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.preview_mint(coin).await?;
    Ok(value)
}

// previewRedeem

pub async fn preview_redeem(
    client: &Client,
    contract_addr: &H160,
    amount: U256,
    denom_out: String,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.preview_redeem(amount, denom_out).await?;
    Ok(value)
}

// previewRequiredCollateral

pub async fn preview_required_collateral(
    client: &Client,
    contract_addr: &H160,
    honey_out: U256,
    denom_in: String,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract
        .preview_required_collateral(honey_out, denom_in)
        .await?;
    Ok(value)
}

// redeem
pub async fn redeem(
    client: &Client,
    contract_addr: &H160,
    from: Address,
    amount: U256,
    denom_out: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.redeem(from, amount, denom_out).await?;
    Ok(value)
}

// requestHoney
pub async fn request_honey(
    client: &Client,
    contract_addr: &H160,
    to: Address,
    amount: U256,
    amo_type: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.request_honey(to, amount, amo_type).await?;
    Ok(value)
}

// updateParams
pub async fn update_params(
    client: &Client,
    contract_addr: &H160,
    params: Vec<honey_module::Exchangeable>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.update_params(params).await?;
    Ok(value)
}
