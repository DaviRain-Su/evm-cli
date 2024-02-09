use crate::Client;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

fn erc20_bank_addr() -> H160 {
    H160::from_str("0x0000000000000000000000000000000000696969").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    ERC20BankModule,
    "./contract/bear/precompile_contracts/IERC20BankModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// coinDenomForERC20Address
pub async fn coin_denom_for_erc20_address(
    client: &Client,
    token: Address,
) -> Result<String, Box<dyn std::error::Error>> {
    let contract = ERC20BankModule::new(erc20_bank_addr(), Arc::new(client.clone()));
    let value = contract.coin_denom_for_erc20_address(token).await?;
    Ok(value)
}

/// erc20AddressForCoinDenom
pub async fn erc_20_address_for_coin_denom(
    client: &Client,
    denom: String,
) -> Result<Address, Box<dyn std::error::Error>> {
    let contract = ERC20BankModule::new(erc20_bank_addr(), Arc::new(client.clone()));
    let value = contract.erc_20_address_for_coin_denom(denom).await?;
    Ok(value)
}

/// performBankTransfer
pub async fn perform_bank_transfer(
    client: &Client,
    owner: Address,
    recipient: Address,
    amount: U256,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = ERC20BankModule::new(erc20_bank_addr(), Arc::new(client.clone()));
    let value = contract
        .perform_bank_transfer(owner, recipient, amount)
        .await?;
    Ok(value)
}

// transferCoinToERC20
pub async fn transfer_coin_to_erc20(
    client: &Client,
    denom: String,
    amount: U256,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = ERC20BankModule::new(erc20_bank_addr(), Arc::new(client.clone()));
    let value = contract.transfer_coin_to_erc20(denom, amount).await?;
    Ok(value)
}

// transferCoinToERC20From
pub async fn transfer_coin_to_erc20_from(
    client: &Client,
    denom: String,
    owner: Address,
    recipient: Address,
    amount: U256,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = ERC20BankModule::new(erc20_bank_addr(), Arc::new(client.clone()));
    let value = contract
        .transfer_coin_to_erc20_from(denom, owner, recipient, amount)
        .await?;
    Ok(value)
}

/// transferCoinToERC20To
pub async fn transfer_coin_to_erc20_to(
    client: &Client,
    denom: String,
    recipient: Address,
    amount: U256,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = ERC20BankModule::new(erc20_bank_addr(), Arc::new(client.clone()));
    let value = contract
        .transfer_coin_to_erc20_to(denom, recipient, amount)
        .await?;
    Ok(value)
}

///transferERC20ToCoin
pub async fn transfer_erc20_to_coin(
    client: &Client,
    token: Address,
    amount: U256,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = ERC20BankModule::new(erc20_bank_addr(), Arc::new(client.clone()));
    let value = contract.transfer_erc20_to_coin(token, amount).await?;
    Ok(value)
}

/// transferERC20ToCoinFrom
pub async fn transfer_erc20_to_coin_from(
    client: &Client,
    token: Address,
    owner: Address,
    recipient: Address,
    amount: U256,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = ERC20BankModule::new(erc20_bank_addr(), Arc::new(client.clone()));
    let value = contract
        .transfer_erc20_to_coin_from(token, owner, recipient, amount)
        .await?;
    Ok(value)
}

// transferERC20ToCoinTo
//
pub async fn transfer_erc20_to_coin_to(
    client: &Client,
    token: Address,
    recipient: Address,
    amount: U256,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = ERC20BankModule::new(erc20_bank_addr(), Arc::new(client.clone()));
    let value = contract
        .transfer_erc20_to_coin_to(token, recipient, amount)
        .await?;
    Ok(value)
}
