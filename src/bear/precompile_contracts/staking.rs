use crate::Client;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

pub fn staking_addr() -> H160 {
    H160::from_str("0x9202Af6Ce925b26AE6B25aDfff0B2705147e195F").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    StatkingModule,
    "./contract/bear/precompile_contracts/IStakingModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

/// # beginRedelegate
pub async fn begin_redelegate(
    client: &Client,
    src_validator: Address,
    dst_validator: Address,
    amount: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = StatkingModule::new(staking_addr(), Arc::new(client.clone()));
    let tx = contract
        .begin_redelegate(src_validator, dst_validator, amount)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

// cancelUnbondingDelegation
pub async fn cancel_unbonding_delegation(
    client: &Client,
    validator_address: Address,
    amount: U256,
    creation_height: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = StatkingModule::new(staking_addr(), Arc::new(client.clone()));
    let tx = contract
        .cancel_unbonding_delegation(validator_address, amount, creation_height)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

// delegate
pub async fn delegate(
    client: &Client,
    validator_address: Address,
    amount: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = StatkingModule::new(staking_addr(), Arc::new(client.clone()));
    let tx = contract
        .delegate(validator_address, amount)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

// getBondedValidators
pub async fn get_bonded_validators(
    client: &Client,
    pagination: PageRequest,
) -> Result<(Vec<Validator>, PageResponse), Box<dyn std::error::Error>> {
    let contract = StatkingModule::new(staking_addr(), Arc::new(client.clone()));
    let tx = contract.get_bonded_validators(pagination).call().await?;

    Ok(tx)
}

// getBondedValidatorsByPower
pub async fn get_bonded_validators_by_power(
    client: &Client,
) -> Result<Vec<Address>, Box<dyn std::error::Error>> {
    let contract = StatkingModule::new(staking_addr(), Arc::new(client.clone()));
    let tx = contract.get_bonded_validators_by_power().call().await?;
    Ok(tx)
}

// getDelegation
pub async fn get_delegation(
    client: &Client,
    delegation_address: Address,
    validator_address: Address,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = StatkingModule::new(staking_addr(), Arc::new(client.clone()));
    let tx = contract
        .get_delegation(delegation_address, validator_address)
        .call()
        .await?;
    Ok(tx)
}

// getDelegatorUnbondingDelegations
pub async fn get_delegator_unbonding_delegations(
    client: &Client,
    delegation_address: Address,
    pagination: PageRequest,
) -> Result<(Vec<UnbondingDelegation>, PageResponse), Box<dyn std::error::Error>> {
    let contract = StatkingModule::new(staking_addr(), Arc::new(client.clone()));
    let tx = contract
        .get_delegator_unbonding_delegations(delegation_address, pagination)
        .call()
        .await?;
    Ok(tx)
}

// getDelegatorValidators
pub async fn get_delegator_validators(
    client: &Client,
    delegation_address: Address,
    pagination: PageRequest,
) -> Result<(Vec<Validator>, PageResponse), Box<dyn std::error::Error>> {
    let contract = StatkingModule::new(staking_addr(), Arc::new(client.clone()));
    let tx = contract
        .get_delegator_validators(delegation_address, pagination)
        .call()
        .await?;
    Ok(tx)
}

// getRedelegations
pub async fn get_redelegations(
    client: &Client,
    delegation_address: Address,
    src_validator: Address,
    dst_validator: Address,
    pagination: PageRequest,
) -> Result<(Vec<RedelegationEntry>, PageResponse), Box<dyn std::error::Error>> {
    let contract = StatkingModule::new(staking_addr(), Arc::new(client.clone()));
    let tx = contract
        .get_redelegations(delegation_address, src_validator, dst_validator, pagination)
        .call()
        .await?;
    Ok(tx)
}

// getUnbondingDelegation
pub async fn get_unbonding_delegation(
    client: &Client,
    delegation_address: Address,
    validator_address: Address,
) -> Result<Vec<UnbondingDelegationEntry>, Box<dyn std::error::Error>> {
    let contract = StatkingModule::new(staking_addr(), Arc::new(client.clone()));
    let tx = contract
        .get_unbonding_delegation(delegation_address, validator_address)
        .call()
        .await?;
    Ok(tx)
}

// getValAddressFromConsAddress
pub async fn get_val_address_from_cons_address(
    client: &Client,
    cons_addr: Bytes,
) -> Result<Address, Box<dyn std::error::Error>> {
    let contract = StatkingModule::new(staking_addr(), Arc::new(client.clone()));
    let tx = contract
        .get_val_address_from_cons_address(cons_addr)
        .call()
        .await?;
    Ok(tx)
}

// getValidator
pub async fn get_validator(
    client: &Client,
    validator_address: Address,
) -> Result<Validator, Box<dyn std::error::Error>> {
    let contract = StatkingModule::new(staking_addr(), Arc::new(client.clone()));
    let tx = contract.get_validator(validator_address).call().await?;
    Ok(tx)
}

// getValidatorDelegations
pub async fn get_validator_delegations(
    client: &Client,
    validator_address: Address,
    pagination: PageRequest,
) -> Result<(Vec<Delegation>, PageResponse), Box<dyn std::error::Error>> {
    let contract = StatkingModule::new(staking_addr(), Arc::new(client.clone()));
    let tx = contract
        .get_validator_delegations(validator_address, pagination)
        .call()
        .await?;
    Ok(tx)
}

// getValidators
pub async fn get_validators(
    client: &Client,
    pagination: PageRequest,
) -> Result<(Vec<Validator>, PageResponse), Box<dyn std::error::Error>> {
    let contract = StatkingModule::new(staking_addr(), Arc::new(client.clone()));
    let tx = contract.get_validators(pagination).call().await?;
    Ok(tx)
}

// undelegate
pub async fn undelegate(
    client: &Client,
    validator_address: Address,
    amount: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = StatkingModule::new(staking_addr(), Arc::new(client.clone()));
    let tx = contract
        .undelegate(validator_address, amount)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
