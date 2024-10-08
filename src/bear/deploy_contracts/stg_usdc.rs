use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use ethers::utils::parse_units;
use std::str::FromStr;
use std::sync::Arc;

pub fn stg_usdc_addr() -> H160 {
    H160::from_str("0xd6D83aF58a19Cd14eF3CF6fe848C9A4d21e5727c").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    Honey,
    "./contract/bear/deploy_contracts/Honey.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

/// name 是一个公共视图方法，用于读取此 erc20 的 sdk.Coin 名称。
pub async fn name(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    let contract = Honey::new(stg_usdc_addr(), Arc::new(client.clone()));
    let value = contract.name().call().await?;
    Ok(value)
}

/// symbol 是一个公共视图方法，用于读取此 erc20 的 sdk.Coin 符号。
pub async fn symbol(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    let contract = Honey::new(stg_usdc_addr(), Arc::new(client.clone()));
    let value = contract.symbol().call().await?;
    Ok(value)
}

/// decimals 是一个公共视图方法，用于读取此 erc20 的 sdk.Coin 小数。
pub async fn decimals(client: &Client) -> Result<u8, Box<dyn std::error::Error>> {
    let contract = Honey::new(stg_usdc_addr(), Arc::new(client.clone()));
    let value = contract.decimals().call().await?;
    Ok(value)
}

/// TotalSupply 是一个公共视图方法，用于读取此 erc20 的 sdk.Coin 总供应量。
pub async fn total_supply(client: &Client) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = Honey::new(stg_usdc_addr(), Arc::new(client.clone()));
    let value = contract.total_supply().call().await?;
    Ok(value)
}

/// BalanceOf 是一个公共视图方法，用于读取此 erc20 给定地址的 sdk.Coin 余额。
pub async fn balance_of(
    client: &Client,
    address: Address,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = Honey::new(stg_usdc_addr(), Arc::new(client.clone()));
    let value = contract.balance_of(address).call().await?;
    Ok(value)
}

/// 批准是一种公共方法，用于批准给定地址花费给定数量的代币。
pub async fn approve(
    client: &Client,
    address: Address,
    amount: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let eth_max_spend = parse_units(0, 18)?;
    let contract = Honey::new(stg_usdc_addr(), Arc::new(client.clone()));
    let tx = contract
        .approve(address, amount)
        .value(eth_max_spend)
        .from(client.address())
        .gas(U256::from(50_000)) // this is crucial otherwise tx will get reverted without a reason
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// Transfer 是一种将代币转移到给定地址的公共方法。
pub async fn transfer(
    client: &Client,
    address: Address,
    amount: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = Honey::new(stg_usdc_addr(), Arc::new(client.clone()));
    let tx = contract.transfer(address, amount).send().await?.await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// TransferFrom 是一种将代币从一个地址转移到另一个地址的公共方法。
pub async fn transfer_from(
    client: &Client,
    from: Address,
    to: Address,
    amount: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = Honey::new(stg_usdc_addr(), Arc::new(client.clone()));
    let tx = contract
        .transfer_from(from, to, amount)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// TransferFrom 是一种将代币从一个地址转移到另一个地址的公共方法。
pub async fn nonces(client: &Client, address: Address) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = Honey::new(stg_usdc_addr(), Arc::new(client.clone()));
    let value = contract.nonces(address).call().await?;
    Ok(value)
}
