//! 该预编译负责管理针对网络中验证器的贿赂。以下是允许查看和创建贿赂的功能。

use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

pub fn bribe_addr() -> H160 {
    H160::from_str("0xFCE07324E0E72E071842374E9997CF65DF990CBC").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    BribeModule,
    "./contract/bear/precompile_contracts/IBribeModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

/// # createBribe
/// 制造新的贿赂。只有有权限的人才能调用该方法。注意：每个时期每个验证者只能创建一次贿赂。
pub async fn create_bribe(
    client: &Client,
    operator: Address, // 验证器操作员地址。
    start_epoch: u64,  // 贿赂开始的纪元。
    num_block_proposals: u64,
    bribe_per_proposal: Vec<bribe_module::Coin>,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = BribeModule::new(bribe_addr(), Arc::new(client.clone()));
    let tx = contract
        .create_bribe(
            operator,
            start_epoch,
            num_block_proposals,
            bribe_per_proposal,
        )
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// # getActiveValidatorBribes
/// 从商店获取给定验证器的所有活跃贿赂。
pub async fn get_active_validator_bribes(
    client: &Client,
    operator: Address, // 验证器操作员地址。
) -> Result<Vec<bribe_module::Bribe>, Box<dyn std::error::Error>> {
    let contract = BribeModule::new(bribe_addr(), Arc::new(client.clone()));
    let value = contract
        .get_active_validator_bribes(operator)
        .call()
        .await?;
    Ok(value)
}

/// # getAllValidatorBribes
/// 从商店获取给定验证器的所有贿赂。
pub async fn get_all_validator_bribes(
    client: &Client,
    operator: Address, // 验证器操作员地址。
) -> Result<Vec<bribe_module::Bribe>, Box<dyn std::error::Error>> {
    let contract = BribeModule::new(bribe_addr(), Arc::new(client.clone()));
    let value = contract.get_all_validator_bribes(operator).call().await?;
    Ok(value)
}

/// # getBribeFees
/// 从商店获取贿赂费。
pub async fn get_bribe_fees(
    client: &Client,
) -> Result<Vec<bribe_module::Coin>, Box<dyn std::error::Error>> {
    let contract = BribeModule::new(bribe_addr(), Arc::new(client.clone()));
    let value = contract.get_bribe_fees().call().await?;
    Ok(value)
}

/// # getBribes
/// 获取给定验证器的贿赂，启动纪元对。
pub async fn get_bribes(
    client: &Client,
    operator: Address, // 验证器操作员地址。
    start_epoch: u64,  // 贿赂开始的纪元。
) -> Result<Vec<bribe_module::Bribe>, Box<dyn std::error::Error>> {
    let contract = BribeModule::new(bribe_addr(), Arc::new(client.clone()));
    let value = contract.get_bribes(operator, start_epoch).call().await?;
    Ok(value)
}

/// # updateParams
/// 更新参数。只有有权限的人才能调用该方法。
pub async fn update_params(
    client: &Client,
    fee: Vec<bribe_module::Coin>,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = BribeModule::new(bribe_addr(), Arc::new(client.clone()));
    let tx = contract.update_params(fee).send().await?.await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
