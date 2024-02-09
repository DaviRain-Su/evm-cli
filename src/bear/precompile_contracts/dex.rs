use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

fn dex_addr() -> H160 {
    H160::from_str("0x9D0FBF9349F646F1435072F2B0212084752EF460").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    DexModule,
    "./contract/bear/precompile_contracts/IDexModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// getPreviewSwapExact
pub async fn get_preview_swap_exact(
    client: &Client,
    kind: u8,
    pool: String,
    base_asset: dex_module::Coin,
    quote_asset: String,
) -> Result<dex_module::Coin, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_preview_swap_exact(kind, pool, base_asset, quote_asset)
        .await?;
    Ok(value)
}
//getPreviewBatchSwap
pub async fn get_preview_batch_swap(
    client: &Client,
    kind: u8,
    swaps: Vec<dex_module::BatchSwapStep>,
    coins: Vec<dex_module::Coin>,
) -> Result<dex_module::Coin, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract.get_preview_batch_swap(kind, swaps, coins).await?;
    Ok(value)
}

pub async fn get_liquidity(
    client: &Client,
    pool: String,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract.get_liquidity(pool).await?;
    Ok(value)
}

// getTotalShares
pub async fn get_total_shares(
    client: &Client,
    pool: String,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract.get_total_shares(pool).await?;
    Ok(value)
}

// getExchangeRate
pub async fn get_exchange_rate(
    client: &Client,
    pool: String,
    base_asset: String,
    quote_asset: String,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_exchange_rate(pool, base_asset, quote_asset)
        .await?;
    Ok(value)
}

// getPreviewSharesForLiquidity
pub async fn get_preview_shares_for_liquidity(
    client: &Client,
    pool: String,
    coins: Vec<dex_module::Coin>,
) -> Result<(Vec<dex_module::Coin>, Vec<dex_module::Coin>), Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));

    let value = contract
        .get_preview_shares_for_liquidity(pool, coins)
        .await?;
    Ok(value)
}

// getPreviewAddLiquidityStaticPrice
pub async fn get_preview_add_liquidity_static_price(
    client: &Client,
    pool: String,
    liquidity: Vec<dex_module::Coin>,
) -> Result<(Vec<dex_module::Coin>, Vec<dex_module::Coin>), Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));

    let value = contract
        .get_preview_add_liquidity_static_price(pool, liquidity)
        .await?;
    Ok(value)
}

// getPreviewSharesForSingleSidedLiquidityRequest
pub async fn get_preview_shares_for_single_sided_liquidity_request(
    client: &Client,
    pool: String,
    coin: dex_module::Coin,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_preview_shares_for_single_sided_liquidity_request(pool, coin)
        .await?;

    Ok(value)
}
// getPreviewAddLiquidityNoSwap
pub async fn get_preview_add_liquidity_no_swap(
    client: &Client,
    pool: String,
    liquidity: Vec<dex_module::Coin>,
) -> Result<(Vec<dex_module::Coin>, Vec<dex_module::Coin>), Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_preview_add_liquidity_no_swap(pool, liquidity)
        .await?;
    Ok(value)
}

// getPreviewBurnShares
pub async fn get_preview_burn_shares(
    client: &Client,
    pool: String,
    shares: dex_module::Coin,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract.get_preview_burn_shares(pool, shares).await?;
    Ok(value)
}

// getRemoveLiquidityExactAmountOut
pub async fn get_remove_liquidity_exact_amount_out(
    client: &Client,
    pool: String,
    asset: dex_module::Coin,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_remove_liquidity_exact_amount_out(pool, asset)
        .await?;
    Ok(value)
}
//getRemoveLiquidityOneSideOut
pub async fn get_remove_liquidity_one_side_out(
    client: &Client,
    pool: String,
    denom: String,
    shares_in: U256,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_remove_liquidity_one_side_out(pool, denom, shares_in)
        .await?;
    Ok(value)
}
// getPoolName
pub async fn get_pool_name(
    client: &Client,
    pool: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract.get_pool_name(pool).await?;
    Ok(value)
}
/// getPoolOptions
pub async fn get_pool_options(
    client: &Client,
    pool: String,
) -> Result<dex_module::PoolOptions, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract.get_pool_options(pool).await?;
    Ok(value)
}

pub async fn get_pool_address(
    client: &Client,
    pool: String,
) -> Result<Address, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract.get_pool_address(pool).await?;
    Ok(value)
}

// swap
pub async fn swap(
    client: &Client,
    single_swap: dex_module::SingleSwap,
    limit: U256,
    deadline: U256,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract.swap(single_swap, limit, deadline).await?;
    Ok(value)
}

// batchSwap
pub async fn batch_swap(
    client: &Client,
    kind: u8, // TODO(davirain)
    swaps: Vec<dex_module::BatchSwapStep>,
    coins: Vec<dex_module::Coin>,
    deadline: U256,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract.batch_swap(kind, swaps, coins, deadline).await?;
    Ok(value)
}

// createPool
pub async fn create_pool(
    client: &Client,
    name: String,
    coins: Vec<dex_module::Coin>,
    pool_type: String,
    options: Bytes,
    creator: Address,
) -> Result<String, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract
        .create_pool(name, coins, pool_type, options, creator)
        .await?;
    Ok(value)
}

///
pub async fn add_liquidity(
    client: &Client,
    pool: String,
    account: Address,
    receiver: Address,
    coin: Vec<dex_module::Coin>,
) -> Result<(Vec<dex_module::Coin>, Vec<dex_module::Coin>), Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));

    let value = contract
        .add_liquidity(pool, account, receiver, coin)
        .await?;

    Ok(value)
}

/// removeLiquidityBurningShares
pub async fn remove_liquidity_burning_shares(
    client: &Client,
    pool: String,
    account: Address,
    receiver: Address,
    coin: dex_module::Coin,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract
        .remove_liquidity_burning_shares(pool, account, receiver, coin)
        .await?;
    Ok(value)
}

// removeLiquidityExactAmount
pub async fn remove_liquidity_exact_amount(
    client: &Client,
    pool: String,
    account: Address,
    receiver: Address,
    coin: dex_module::Coin,
    max_shares: dex_module::Coin,
) -> Result<(Vec<dex_module::Coin>, Vec<dex_module::Coin>), Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract
        .remove_liquidity_exact_amount(pool, account, receiver, coin, max_shares)
        .await?;
    Ok(value)
}
