use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::sync::Arc;

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    BerachefModule,
    "./contract/bear/precompile_contracts/IBerachefModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// getActiveCuttingBoard
pub async fn get_active_cutting_board(
    client: &Client,
    contract_addr: &H160,
    operator_addr: Address,
) -> Result<berachef_module::CuttingBoard, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = BerachefModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.get_active_cutting_board(operator_addr).await?;

    // 6. Return the number
    Ok(value)
}

//getDelegation
pub async fn get_delegation(
    client: &Client,
    contract_addr: &H160,
    operator_addr: Address,
) -> Result<Address, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = BerachefModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.get_delegation(operator_addr).await?;

    // 6. Return the number
    Ok(value)
}

// getQueuedCuttingBoard
pub async fn get_queued_cutting_board(
    client: &Client,
    contract_addr: &H160,
    operator_addr: Address,
) -> Result<berachef_module::CuttingBoard, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = BerachefModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.get_queued_cutting_board(operator_addr).await?;

    // 6. Return the number
    Ok(value)
}

// queueNewCuttingBoard
pub async fn queue_new_cutting_board(
    client: &Client,
    contract_addr: &H160,
    operator_addr: Address,
    weights: Vec<berachef_module::Weight>,
    start_epoch: i64,
) -> Result<bool, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = BerachefModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract
        .queue_new_cutting_board(operator_addr, weights, start_epoch)
        .await?;

    // 6. Return the number
    Ok(value)
}

// setDelegation
pub async fn set_delegation(
    client: &Client,
    contract_addr: &H160,
    delegation_address: Address,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Create contract instance
    let contract = BerachefModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // Call contract's number function
    let value = contract.set_delegation(delegation_address).await?;

    // Return the number
    Ok(value)
}

// updateFriendsOfTheChef
pub async fn update_friends_of_the_chef(
    client: &Client,
    contract_addr: &H160,
    receiver_address: Address,
    friend_of_the_chef: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Create contract instance
    let contract = BerachefModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // Call contract's number function
    let value = contract
        .update_friends_of_the_chef(receiver_address, friend_of_the_chef)
        .await?;

    // Return the number
    Ok(value)
}
