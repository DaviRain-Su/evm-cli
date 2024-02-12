//! Cosmos SDK 分发模块负责跟踪奖励并将奖励分发给区块链网络中的验证者和委托者。该模块根据验证者和委托者对网络的参与情况，被动地将收取的费用和其他奖励分配给验证者和委托者。它还定义了社区池，社区池是链上治理控制下的资金池。
//! 该接口包括设置和获取委托奖励提现地址、检查是否开启奖励提现、提取委托人奖励、获取委托人累计奖励总额等功能。此外，它还定义了提取奖励和设置提取地址的事件，以及代表委托人对特定验证者的奖励的结构。
//! 开发人员可以使用此模块来管理使用 Cosmos SDK 构建的自定义应用程序特定区块链中的奖励分配。 Berachain 确保验证者和委托者通过该模块对网络的贡献获得公平的奖励
//! 分发模块预编译合约接口

use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

pub fn distribution_addr() -> H160 {
    H160::from_str("0x0000000000000000000000000000000000000069").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    DistributeModule,
    "./contract/bear/precompile_contracts/IDistributionModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

/// getAllDelegatorRewards
/// 返回委托人累积的全部奖励。
pub async fn get_all_delegator_rewards(
    client: &Client,
    account_address: Address, // 检索总奖励的委托人。
) -> Result<Vec<distribute_module::ValidatorReward>, Box<dyn std::error::Error>> {
    let contract = DistributeModule::new(distribution_addr(), Arc::new(client.clone()));
    let value = contract
        .get_all_delegator_rewards(account_address)
        .call()
        .await?;
    Ok(value)
}

/// # getDelegatorValidatorReward
/// 返回委托人为验证人积累的奖励。
pub async fn get_delegator_validator_reward(
    client: &Client,
    delegator: Address, // 为其获取奖励的委托人。
    validator: Address, // 用于检索奖励的验证者（运营商地址）。
) -> Result<Vec<distribute_module::Coin>, Box<dyn std::error::Error>> {
    let contract = DistributeModule::new(distribution_addr(), Arc::new(client.clone()));
    let value = contract
        .get_delegator_validator_reward(delegator, validator)
        .call()
        .await?;
    Ok(value)
}

/// getTotalDelegatorReward
/// 返回委托人累积的总奖励。
pub async fn get_total_delegator_reward(
    client: &Client,
    delegator: Address, // 检索总奖励的委托人。
) -> Result<Vec<distribute_module::Coin>, Box<dyn std::error::Error>> {
    let contract = DistributeModule::new(distribution_addr(), Arc::new(client.clone()));
    let value = contract
        .get_total_delegator_reward(delegator)
        .call()
        .await?;
    Ok(value)
}

/// # getWithdrawAddress
/// 返回将获得委托奖励的地址。
pub async fn get_withdraw_address(
    client: &Client,
    delegator: Address, // 返回提现地址的委托人。
) -> Result<Address, Box<dyn std::error::Error>> {
    let contract = DistributeModule::new(distribution_addr(), Arc::new(client.clone()));
    let value = contract.get_withdraw_address(delegator).call().await?;
    Ok(value)
}

/// getWithdrawEnabled
/// 返回是否启用提现委托奖励。
pub async fn get_withdraw_enabled(client: &Client) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = DistributeModule::new(distribution_addr(), Arc::new(client.clone()));
    let value = contract.get_withdraw_enabled().call().await?;
    Ok(value)
}

/// setWithdrawAddress
/// 调用者（msg.sender）可以设置接收委托奖励的地址。
pub async fn set_withdraw_address(
    client: &Client,
    withdraw_address: Address, // 设置为提现地址的地址。
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = DistributeModule::new(distribution_addr(), Arc::new(client.clone()));
    let tx = contract
        .set_withdraw_address(withdraw_address)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// withdrawDelegatorReward
/// 撤回调用者（msg.sender）累积的奖励。返回领取的奖励。
pub async fn withdraw_delegator_reward(
    client: &Client,
    delegator: Address, // 从中提取奖励的委托人。
    validator: Address, //从中提取奖励的验证者（运营商地址）。
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = DistributeModule::new(distribution_addr(), Arc::new(client.clone()));
    let tx = contract
        .withdraw_delegator_reward(delegator, validator)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
