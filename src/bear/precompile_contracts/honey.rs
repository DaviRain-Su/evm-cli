use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

pub fn honey_addr() -> H160 {
    H160::from_str("0xA55E2E3846A51F6AD0ABFDFBDEA2BA0E5E0C76B5").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    HoneyModule,
    "./contract/bear/precompile_contracts/IHoneyModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

/// # getAMOCurrentLimit
pub async fn get_amo_current_limt(
    client: &Client,
    amo_type: String,
    amo_addr: Address,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(honey_addr(), Arc::new(client.clone()));
    let value = contract
        .get_amo_current_limit(amo_type, amo_addr)
        .call()
        .await?;
    Ok(value)
}

/// # getParams
pub async fn get_params(
    client: &Client,
) -> Result<Vec<honey_module::Exchangeable>, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(honey_addr(), Arc::new(client.clone()));
    let value = contract.get_params().call().await?;
    Ok(value)
}

/// # getTotalCollateral
pub async fn get_total_collateral(
    client: &Client,
) -> Result<Vec<honey_module::Coin>, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(honey_addr(), Arc::new(client.clone()));
    let value = contract.get_total_collateral().call().await?;
    Ok(value)
}

/// # getTotalSupply
pub async fn get_total_supply(client: &Client) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(honey_addr(), Arc::new(client.clone()));
    let value = contract.get_total_supply().call().await?;
    Ok(value)
}

/// # mint
pub async fn mint(
    client: &Client,
    to: Address,
    collateral: honey_module::Coin,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(honey_addr(), Arc::new(client.clone()));
    let tx = contract.mint(to, collateral).send().await?.await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// # previewExactOutCollateral
pub async fn preview_exact_out_collateral(
    client: &Client,
    collateral_out: honey_module::Coin,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(honey_addr(), Arc::new(client.clone()));
    let value = contract
        .preview_exact_out_collateral(collateral_out)
        .call()
        .await?;
    Ok(value)
}

/// # previewMint
pub async fn preview_mint(
    client: &Client,
    coin: honey_module::Coin,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(honey_addr(), Arc::new(client.clone()));
    let value = contract.preview_mint(coin).call().await?;
    Ok(value)
}

/// # previewRedeem
pub async fn preview_redeem(
    client: &Client,
    amount: U256,
    denom_out: String,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(honey_addr(), Arc::new(client.clone()));
    let value = contract.preview_redeem(amount, denom_out).call().await?;
    Ok(value)
}

/// # previewRequiredCollateral
pub async fn preview_required_collateral(
    client: &Client,
    honey_out: U256,
    denom_in: String,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(honey_addr(), Arc::new(client.clone()));
    let value = contract
        .preview_required_collateral(honey_out, denom_in)
        .call()
        .await?;
    Ok(value)
}

/// # redeem
pub async fn redeem(
    client: &Client,
    from: Address,
    amount: U256,
    denom_out: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(honey_addr(), Arc::new(client.clone()));
    let tx = contract
        .redeem(from, amount, denom_out)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// # requestHoney
pub async fn request_honey(
    client: &Client,
    to: Address,
    amount: U256,
    amo_type: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(honey_addr(), Arc::new(client.clone()));
    let tx = contract
        .request_honey(to, amount, amo_type)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// # updateParams
pub async fn update_params(
    client: &Client,
    params: Vec<honey_module::Exchangeable>,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(honey_addr(), Arc::new(client.clone()));
    let tx = contract.update_params(params).send().await?.await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
