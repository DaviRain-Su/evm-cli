//! Honey 的抵押品只能通过治理投票来更新。

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
/// 获取指定AMO的当前限制。
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
/// 获取honey模块的参数。
pub async fn get_params(
    client: &Client,
) -> Result<Vec<honey_module::Exchangeable>, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(honey_addr(), Arc::new(client.clone()));
    let value = contract.get_params().call().await?;
    Ok(value)
}

/// # getTotalCollateral
/// 获取 Honey 模块中锁定的总抵押品。
pub async fn get_total_collateral(
    client: &Client,
) -> Result<Vec<honey_module::Coin>, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(honey_addr(), Arc::new(client.clone()));
    let value = contract.get_total_collateral().call().await?;
    Ok(value)
}

/// # getTotalSupply
/// 获取蜂蜜的总供应量。
pub async fn get_total_supply(client: &Client) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(honey_addr(), Arc::new(client.clone()));
    let value = contract.get_total_supply().call().await?;
    Ok(value)
}

/// # mint
/// 使用给定的抵押品为给定的帐户铸造新的蜂蜜币。
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
/// 预览兑换一定数量的目标抵押品所需的蜂蜜数量。
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
/// 使用给定的抵押品预览蜂蜜的量。
pub async fn preview_mint(
    client: &Client,
    coin: honey_module::Coin,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(honey_addr(), Arc::new(client.clone()));
    let value = contract.preview_mint(coin).call().await?;
    Ok(value)
}

/// # previewRedeem
/// 预览兑换蜂蜜所需返还的抵押品金额。
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
/// 预览铸造一定数量蜂蜜所需的抵押品数量。
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
/// 使用给定的抵押品为给定的帐户兑换蜂蜜币。
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
/// 向阿莫要蜂蜜。
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
/// 该函数的调用者必须是治理模块账户。
/// 更新 Honey 模块的参数。
pub async fn update_params(
    client: &Client,
    params: Vec<honey_module::Exchangeable>,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = HoneyModule::new(honey_addr(), Arc::new(client.clone()));
    let tx = contract.update_params(params).send().await?.await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
