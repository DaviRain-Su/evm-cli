//! 该预编译负责配置从验证器到 berachain 中流动性提供者的 BGT 排放。
//! Berachef模块预编译合约接口

use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

pub fn berachef_addr() -> H160 {
    H160::from_str("0x888AF53B67D1698E04B2B9A9406AF0FFEB2EF05E").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    BerachefModule,
    "./contract/bear/precompile_contracts/IBerachefModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

/// getActiveCuttingBoard
/// 返回具有给定operatorAddr的验证器的活动切割板
pub async fn get_active_cutting_board(
    client: &Client,
    operator_addr: Address,
) -> Result<berachef_module::CuttingBoard, Box<dyn std::error::Error>> {
    let contract = BerachefModule::new(berachef_addr(), Arc::new(client.clone()));
    let value = contract
        .get_active_cutting_board(operator_addr)
        .call()
        .await?;
    Ok(value)
}

/// getDelegation
/// 返回验证者的委托地址。
pub async fn get_delegation(
    client: &Client,
    operator_addr: Address,
) -> Result<Address, Box<dyn std::error::Error>> {
    let contract = BerachefModule::new(berachef_addr(), Arc::new(client.clone()));
    let value = contract.get_delegation(operator_addr).call().await?;
    Ok(value)
}

/// getQueuedCuttingBoard
/// 返回具有给定operatorAddr的验证器的排队切板
pub async fn get_queued_cutting_board(
    client: &Client,
    operator_addr: Address,
) -> Result<berachef_module::CuttingBoard, Box<dyn std::error::Error>> {
    let contract = BerachefModule::new(berachef_addr(), Arc::new(client.clone()));
    let value = contract
        .get_queued_cutting_board(operator_addr)
        .call()
        .await?;
    Ok(value)
}

/// queueNewCuttingBoard
/// 切板的重量加起来必须为 100。也只能使用列入白名单的池。
/// 使用给定的operatorAddr将新的切板添加到验证器的队列中。
pub async fn queue_new_cutting_board(
    client: &Client,
    operator_addr: Address,
    weights: Vec<berachef_module::Weight>,
    start_epoch: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = BerachefModule::new(berachef_addr(), Arc::new(client.clone()));
    let tx = contract
        .queue_new_cutting_board(operator_addr, weights, start_epoch)
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// setDelegation
/// 设置一个可以代表验证者设置切板的地址。
pub async fn set_delegation(
    client: &Client,
    delegation_address: Address,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = BerachefModule::new(berachef_addr(), Arc::new(client.clone()));
    let tx = contract
        .set_delegation(delegation_address)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// updateFriendsOfTheChef
/// 该函数的调用者必须是治理模块账户。
/// 更新厨师的好友以更新 LP 池是否列入白名单的状态。
pub async fn update_friends_of_the_chef(
    client: &Client,
    receiver_address: Address,
    friend_of_the_chef: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = BerachefModule::new(berachef_addr(), Arc::new(client.clone()));
    let tx = contract
        .update_friends_of_the_chef(receiver_address, friend_of_the_chef)
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
