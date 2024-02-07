use super::super::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::sync::Arc;

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    Honey,
    "./contract/bear/Honey.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

/// name 是一个公共视图方法，用于读取此 erc20 的 sdk.Coin 名称。
pub async fn name(
    client: &Client,
    contract_addr: &H160,
) -> Result<String, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = Honey::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.name().call().await?;

    // 6. Return the number
    Ok(value)
}

/// symbol 是一个公共视图方法，用于读取此 erc20 的 sdk.Coin 符号。
pub async fn symbol(
    client: &Client,
    contract_addr: &H160,
) -> Result<String, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = Honey::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.symbol().call().await?;

    // 6. Return the number
    Ok(value)
}

/// decimals 是一个公共视图方法，用于读取此 erc20 的 sdk.Coin 小数。
pub async fn decimals(
    client: &Client,
    contract_addr: &H160,
) -> Result<u8, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = Honey::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.decimals().call().await?;

    // 6. Return the number
    Ok(value)
}

/// TotalSupply 是一个公共视图方法，用于读取此 erc20 的 sdk.Coin 总供应量。
pub async fn total_supply(
    client: &Client,
    contract_addr: &H160,
) -> Result<U256, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = Honey::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.total_supply().call().await?;

    // 6. Return the number
    Ok(value)
}

/// BalanceOf 是一个公共视图方法，用于读取此 erc20 给定地址的 sdk.Coin 余额。
pub async fn balance_of(
    client: &Client,
    contract_addr: &H160,
    address: Address,
) -> Result<U256, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = Honey::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.balance_of(address).await?;

    // 6. Return the number
    Ok(value)
}

/// 批准是一种公共方法，用于批准给定地址花费给定数量的代币。
pub async fn approve(
    client: &Client,
    contract_addr: &H160,
    address: Address,
    amount: U256,
) -> Result<bool, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = Honey::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.approve(address, amount).await?;

    // 6. Return the number
    Ok(value)
}

/// Transfer 是一种将代币转移到给定地址的公共方法。
pub async fn transfer(
    client: &Client,
    contract_addr: &H160,
    address: Address,
    amount: U256,
) -> Result<bool, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = Honey::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.transfer(address, amount).await?;

    // 6. Return the number
    Ok(value)
}

/// TransferFrom 是一种将代币从一个地址转移到另一个地址的公共方法。
pub async fn transfer_from(
    client: &Client,
    contract_addr: &H160,
    from: Address,
    to: Address,
    amount: U256,
) -> Result<bool, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = Honey::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.transfer_from(from, to, amount).await?;

    // 6. Return the number
    Ok(value)
}
