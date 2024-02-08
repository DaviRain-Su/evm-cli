use crate::Client;
use ethers::prelude::*;
use std::sync::Arc;

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    ERC20DexModule,
    "./contract/bear/precompile_contracts/IERC20DexModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// addLiquidity
pub async fn add_liquidity(
    client: &Client,
    contract_addr: &H160,
    pool: Address,
    receiver: Address,
    assets_in: Vec<Address>,
    amounts_in: Vec<U256>,
) -> Result<(Vec<Address>, Vec<U256>, Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract
        .add_liquidity(pool, receiver, assets_in, amounts_in)
        .await?;
    Ok(value)
}

/// batchSwap
pub async fn batch_swap(
    client: &Client,
    contract_addr: &H160,
    kind: u8,
    swaps: Vec<erc20_dex_module::BatchSwapStep>,
    deadline: U256,
) -> Result<(Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.batch_swap(kind, swaps, deadline).await?;
    Ok(value)
}

// createPool
pub async fn create_pool(
    client: &Client,
    contract_addr: &H160,
    name: String,
    assets_in: Vec<Address>,
    amounts_in: Vec<U256>,
    pool_type: String,
    options: erc20_dex_module::PoolOptions,
) -> Result<Address, Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract
        .create_pool(name, assets_in, amounts_in, pool_type, options)
        .await?;
    Ok(value)
}

/// getExchangeRate
pub async fn get_exchange_rate(
    client: &Client,
    contract_addr: &H160,
    pool: Address,
    base_asset: Address,
    quote_asset: Address,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract
        .get_exchange_rate(pool, base_asset, quote_asset)
        .await?;
    Ok(value)
}

// getLiquidity
pub async fn get_liquidity(
    client: &Client,
    contract_addr: &H160,
    pool: Address,
) -> Result<(Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_liquidity(pool).await?;
    Ok(value)
}

/// getPoolName
pub async fn get_pool_name(
    client: &Client,
    contract_addr: &H160,
    pool: Address,
) -> Result<String, Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_pool_name(pool).await?;
    Ok(value)
}

// getPoolOptions
pub async fn get_pool_options(
    client: &Client,
    contract_addr: &H160,
    pool: Address,
) -> Result<erc20_dex_module::PoolOptions, Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_pool_options(pool).await?;
    Ok(value)
}

// getPreviewAddLiquidityNoSwap
pub async fn get_preview_add_liquidity_no_swap(
    client: &Client,
    contract_addr: &H160,
    pool: Address,
    assets: Vec<Address>,
    amounts: Vec<U256>,
) -> Result<(Vec<Address>, Vec<U256>, Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract
        .get_preview_add_liquidity_no_swap(pool, assets, amounts)
        .await?;
    Ok(value)
}

// getPreviewAddLiquidityStaticPrice
pub async fn get_preview_add_liquidity_static_price(
    client: &Client,
    contract_addr: &H160,
    pool: Address,
    liquidity: Vec<Address>,
    amounts: Vec<U256>,
) -> Result<(Vec<Address>, Vec<U256>, Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract
        .get_preview_add_liquidity_static_price(pool, liquidity, amounts)
        .await?;
    Ok(value)
}

// getPreviewBatchSwap
pub async fn get_preview_batch_swap(
    client: &Client,
    contract_addr: &H160,
    kind: u8,
    swaps: Vec<erc20_dex_module::BatchSwapStep>,
) -> Result<(Address, U256), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_preview_batch_swap(kind, swaps).await?;
    Ok(value)
}

// getPreviewBurnShares
pub async fn get_preview_burn_shares(
    client: &Client,
    contract_addr: &H160,
    pool: Address,
    asset: Address,
    amount: U256,
) -> Result<(Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract
        .get_preview_burn_shares(pool, asset, amount)
        .await?;
    Ok(value)
}

// getPreviewSharesForLiquidity
pub async fn get_preview_shares_for_liquidity(
    client: &Client,
    contract_addr: &H160,
    pool: Address,
    assets: Vec<Address>,
    amounts: Vec<U256>,
) -> Result<(Vec<Address>, Vec<U256>, Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract
        .get_preview_shares_for_liquidity(pool, assets, amounts)
        .await?;
    Ok(value)
}

// getPreviewSharesForSingleSidedLiquidityRequest
pub async fn get_preview_shares_for_single_sided_liquidity_request(
    client: &Client,
    contract_addr: &H160,
    pool: Address,
    asset: Address,
    amount: U256,
) -> Result<(Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract
        .get_preview_shares_for_single_sided_liquidity_request(pool, asset, amount)
        .await?;
    Ok(value)
}

// getPreviewSwapExact
pub async fn get_preview_swap_exact(
    client: &Client,
    contract_addr: &H160,
    kind: u8,
    pool: Address,
    base_asset: Address,
    base_asset_amount: U256,
    quote_asset: Address,
) -> Result<(Address, U256), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract
        .get_preview_swap_exact(kind, pool, base_asset, base_asset_amount, quote_asset)
        .await?;
    Ok(value)
}

/// getRemoveLiquidityExactAmountOut
pub async fn get_remove_liquidity_exact_amount_out(
    client: &Client,
    contract_addr: &H160,
    pool: Address,
    asset_in: Address,
    asset_amount: U256,
) -> Result<(Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract
        .get_remove_liquidity_exact_amount_out(pool, asset_in, asset_amount)
        .await?;
    Ok(value)
}

// getRemoveLiquidityOneSideOut
pub async fn get_remove_liquidity_one_side_out(
    client: &Client,
    contract_addr: &H160,
    pool: Address,
    asset_out: Address,
    shares_in: U256,
) -> Result<(Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract
        .get_remove_liquidity_one_side_out(pool, asset_out, shares_in)
        .await?;
    Ok(value)
}

// getTotalShares
pub async fn get_total_shares(
    client: &Client,
    contract_addr: &H160,
    pool: Address,
) -> Result<(Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_total_shares(pool).await?;
    Ok(value)
}

//removeLiquidityBurningShares
pub async fn remove_liquidity_burning_shares(
    client: &Client,
    contract_addr: &H160,
    pool: Address,
    withdraw_address: Address,
    asset_in: Address,
    amount_in: U256,
) -> Result<(Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract
        .remove_liquidity_burning_shares(pool, withdraw_address, asset_in, amount_in)
        .await?;
    Ok(value)
}

/// removeLiquidityExactAmount
pub async fn remove_liquidity_exact_amount(
    client: &Client,
    contract_addr: &H160,
    pool: Address,
    withdraw_address: Address,
    asset_out: Address,
    amount_out: U256,
    shares_in: Address,
    max_shares_in: U256,
) -> Result<(Vec<Address>, Vec<U256>, Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract
        .remove_liquidity_exact_amount(
            pool,
            withdraw_address,
            asset_out,
            amount_out,
            shares_in,
            max_shares_in,
        )
        .await?;
    Ok(value)
}

// swap
pub async fn swap(
    client: &Client,
    contract_addr: &H160,
    kind: u8,
    pool_id: Address,
    asset_in: Address,
    amount_in: U256,
    asset_out: Address,
    amount_out: U256,
    deadline: U256,
) -> Result<(Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract
        .swap(
            kind, pool_id, asset_in, amount_in, asset_out, amount_out, deadline,
        )
        .await?;
    Ok(value)
}
