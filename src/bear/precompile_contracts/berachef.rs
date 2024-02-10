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

// getActiveCuttingBoard
pub async fn get_active_cutting_board(
    client: &Client,
    operator_addr: Address,
) -> Result<berachef_module::CuttingBoard, Box<dyn std::error::Error>> {
    let contract = BerachefModule::new(berachef_addr(), Arc::new(client.clone()));
    let value = contract.get_active_cutting_board(operator_addr).await?;
    Ok(value)
}

//getDelegation
pub async fn get_delegation(
    client: &Client,
    operator_addr: Address,
) -> Result<Address, Box<dyn std::error::Error>> {
    let contract = BerachefModule::new(berachef_addr(), Arc::new(client.clone()));
    let value = contract.get_delegation(operator_addr).await?;
    Ok(value)
}

// getQueuedCuttingBoard
pub async fn get_queued_cutting_board(
    client: &Client,
    operator_addr: Address,
) -> Result<berachef_module::CuttingBoard, Box<dyn std::error::Error>> {
    let contract = BerachefModule::new(berachef_addr(), Arc::new(client.clone()));
    let value = contract.get_queued_cutting_board(operator_addr).await?;
    Ok(value)
}

// queueNewCuttingBoard
pub async fn queue_new_cutting_board(
    client: &Client,
    operator_addr: Address,
    weights: Vec<berachef_module::Weight>,
    start_epoch: i64,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = BerachefModule::new(berachef_addr(), Arc::new(client.clone()));
    let value = contract
        .queue_new_cutting_board(operator_addr, weights, start_epoch)
        .await?;
    Ok(value)
}

// setDelegation
pub async fn set_delegation(
    client: &Client,
    delegation_address: Address,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = BerachefModule::new(berachef_addr(), Arc::new(client.clone()));
    let value = contract.set_delegation(delegation_address).await?;
    Ok(value)
}

// updateFriendsOfTheChef
pub async fn update_friends_of_the_chef(
    client: &Client,
    receiver_address: Address,
    friend_of_the_chef: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = BerachefModule::new(berachef_addr(), Arc::new(client.clone()));
    let value = contract
        .update_friends_of_the_chef(receiver_address, friend_of_the_chef)
        .await?;
    Ok(value)
}
