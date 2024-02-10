use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

fn erc20_bribe_addr() -> H160 {
    H160::from_str("0x1BbACf6bA66A20CD8ad98c70EAC4ea7AaD45c3E9").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    ERC20BribeModule,
    "./contract/bear/deploy_contracts/ERC20BribeModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

/// 为验证者收受贿赂。
pub async fn bribe_module(client: &Client) -> Result<Address, Box<dyn std::error::Error>> {
    log::info!("Erc20 bribe module: Function(birbe_module)");
    let contract = ERC20BribeModule::new(erc20_bribe_addr(), Arc::new(client.clone()));
    let value = contract.bribe_module().await?;
    Ok(value)
}

/// 为验证者收受贿赂。
pub async fn get_bribes_for_validator(
    client: &Client,
    operator: Address,
    start_epoch: u64,
) -> Result<Vec<Bribe>, Box<dyn std::error::Error>> {
    log::info!(
        "Erc20 bribe module: Function(get_bribes_for_validator(Operator({:?}), start_epoch({})))",
        operator,
        start_epoch
    );
    let contract = ERC20BribeModule::new(erc20_bribe_addr(), Arc::new(client.clone()));
    let value = contract
        .get_bribes_for_validator(operator, start_epoch)
        .await?;
    Ok(value)
}

/// 为验证者获取主动贿赂。
pub async fn get_all_validator_bribes(
    client: &Client,
    operator: Address,
) -> Result<Vec<Bribe>, Box<dyn std::error::Error>> {
    log::info!(
        "Erc20 bribe module: Function(get_all_validator_bribes(Operator({:?})))",
        operator
    );
    let contract = ERC20BribeModule::new(erc20_bribe_addr(), Arc::new(client.clone()));
    let value = contract.get_all_validator_bribes(operator).await?;
    Ok(value)
}

///
pub async fn get_active_validator_bribes(
    client: &Client,
    operator: Address,
) -> Result<Vec<Bribe>, Box<dyn std::error::Error>> {
    log::info!(
        "Erc20 bribe module: Function(get_active_validator_bribes(Operator({:?})))",
        operator
    );
    let contract = ERC20BribeModule::new(erc20_bribe_addr(), Arc::new(client.clone()));
    let value = contract.get_active_validator_bribes(operator).await?;
    Ok(value)
}

/// 返回委托人为每个验证人积累的所有奖励。
pub async fn preview_claim_validator_bribes(
    client: &Client,
    delegator: Address,
) -> Result<Vec<ValidatorReward>, Box<dyn std::error::Error>> {
    log::info!(
        "Erc20 bribe module: Function(preview_claim_validator_bribes(delegator({:?})))",
        delegator
    );
    let contract = ERC20BribeModule::new(erc20_bribe_addr(), Arc::new(client.clone()));
    let value = contract.preview_claim_validator_bribes(delegator).await?;
    Ok(value)
}

/// 返回委托人的累积贿赂奖励。
pub async fn preview_claim_all_bribes(
    client: &Client,
    delegator: Address,
) -> Result<Vec<erc20_bribe_module::Reward>, Box<dyn std::error::Error>> {
    log::info!(
        "Erc20 bribe module: Function(preview_claim_all_bribes(Delegator({:?})))",
        delegator
    );
    let contract = ERC20BribeModule::new(erc20_bribe_addr(), Arc::new(client.clone()));
    let value = contract.preview_claim_all_bribes(delegator).await?;
    Ok(value)
}

/// 制造贿赂。
pub async fn create_bribe(
    client: &Client,
    delegator: Address,
    start_epoch: u64,
    num_block_proposals: u64,
    tokens: Vec<Address>,
    amounts: Vec<U256>,
) -> Result<bool, Box<dyn std::error::Error>> {
    log::info!(
        "Erc20 bribe module: Function(create_bribe(Delegator({:?}),
        StartEpoch({}), NumBlockProposals({}))), Tokens({:?}), Amounts({:?})",
        delegator,
        start_epoch,
        num_block_proposals,
        tokens,
        amounts
    );
    let contract = ERC20BribeModule::new(erc20_bribe_addr(), Arc::new(client.clone()));
    let value = contract
        .create_bribe(delegator, start_epoch, num_block_proposals, tokens, amounts)
        .await?;
    Ok(value)
}

// TODO(abi json no this api)
// pub async fn fund_bribe(
//     client: &Client,
//     contract_addr: &H160,
//     from: Address,
//     num_block_proposals: u64,
// ) -> Result<bool, Box<dyn std::error::Error>> {
//     // 3. Create contract instance
//     let contract = ERC20BribeModule::new(contract_addr.clone(), Arc::new(client.clone()));

//     // 4. Call contract's number function
//     let value = contract.fund_bribe(from, num_block_proposals).await?;

//     // 6. Return the number
//     Ok(value)
// }

/// 提取特定验证人的委托人累积的贿赂奖励。注意：委托人还必须提取他们的 BGT 奖励。
pub async fn claim_validator_bribes(
    client: &Client,
    delegator: Address,
    validator: Address,
) -> Result<bool, Box<dyn std::error::Error>> {
    log::info!(
        "Erc20 bribe module: Function(claim_validator_bribes(Delegator({:?}), Validator({:?})))",
        delegator,
        validator
    );
    let contract = ERC20BribeModule::new(erc20_bribe_addr(), Arc::new(client.clone()));
    let value = contract
        .claim_validator_bribes(delegator, validator)
        .await?;
    Ok(value)
}

/// 提取所有验证人累积给委托人的贿赂奖励。注意：委托人还必须从所有委托中提取 BGT 奖励。
pub async fn claim_all_bribes(
    client: &Client,
    delegator: Address,
) -> Result<bool, Box<dyn std::error::Error>> {
    log::info!(
        "Erc20 bribe module: Function(claim_all_bribes(Delegator({:?})))",
        delegator
    );
    let contract = ERC20BribeModule::new(erc20_bribe_addr(), Arc::new(client.clone()));
    let value = contract.claim_all_bribes(delegator).await?;
    Ok(value)
}

///
pub async fn distribution_module(client: &Client) -> Result<Address, Box<dyn std::error::Error>> {
    log::info!("Erc20 bribe module: Function(distribution_module)");
    let contract = ERC20BribeModule::new(erc20_bribe_addr(), Arc::new(client.clone()));
    let value = contract.distribution_module().await?;
    Ok(value)
}

///
pub async fn erc_20_module(client: &Client) -> Result<Address, Box<dyn std::error::Error>> {
    log::info!("Erc20 bribe module: Function(erc_20_module)");
    let contract = ERC20BribeModule::new(erc20_bribe_addr(), Arc::new(client.clone()));
    let value = contract.erc_20_module().await?;
    Ok(value)
}
