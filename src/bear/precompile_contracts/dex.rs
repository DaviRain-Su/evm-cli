use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::sync::Arc;

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    DexModule,
    "./contract/bear/precompile_contracts/IDexModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// getPreviewSwapExact
pub async fn get_preview_swap_exact(
    client: &Client,
    contract_addr: &H160,
    kind: u8,
    pool: String,
    base_asset: dex_module::Coin,
    quote_asset: String,
) -> Result<dex_module::Coin, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract
        .get_preview_swap_exact(kind, pool, base_asset, quote_asset)
        .await?;

    // 6. Return the number
    Ok(value)
}
//getPreviewBatchSwap
pub async fn get_preview_batch_swap(
    client: &Client,
    contract_addr: &H160,
    kind: u8,
    swaps: Vec<dex_module::BatchSwapStep>,
    coins: Vec<dex_module::Coin>,
) -> Result<dex_module::Coin, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.get_preview_batch_swap(kind, swaps, coins).await?;

    // 6. Return the number
    Ok(value)
}

pub async fn get_liquidity(
    client: &Client,
    contract_addr: &H160,
    pool: String,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.get_liquidity(pool).await?;

    // 6. Return the number
    Ok(value)
}

// getTotalShares
pub async fn get_total_shares(
    client: &Client,
    contract_addr: &H160,
    pool: String,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.get_total_shares(pool).await?;

    // 6. Return the number
    Ok(value)
}

// getExchangeRate
pub async fn get_exchange_rate(
    client: &Client,
    contract_addr: &H160,
    pool: String,
    base_asset: String,
    quote_asset: String,
) -> Result<U256, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract
        .get_exchange_rate(pool, base_asset, quote_asset)
        .await?;

    // 6. Return the number
    Ok(value)
}

// getPreviewSharesForLiquidity
pub async fn get_preview_shares_for_liquidity(
    client: &Client,
    contract_addr: &H160,
    pool: String,
    coins: Vec<dex_module::Coin>,
) -> Result<(Vec<dex_module::Coin>, Vec<dex_module::Coin>), Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract
        .get_preview_shares_for_liquidity(pool, coins)
        .await?;

    // 6. Return the number
    Ok(value)
}

// getPreviewAddLiquidityStaticPrice
pub async fn get_preview_add_liquidity_static_price(
    client: &Client,
    contract_addr: &H160,
    pool: String,
    liquidity: Vec<dex_module::Coin>,
) -> Result<(Vec<dex_module::Coin>, Vec<dex_module::Coin>), Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract
        .get_preview_add_liquidity_static_price(pool, liquidity)
        .await?;

    // 6. Return the number
    Ok(value)
}

// getPreviewSharesForSingleSidedLiquidityRequest
pub async fn get_preview_shares_for_single_sided_liquidity_request(
    client: &Client,
    contract_addr: &H160,
    pool: String,
    coin: dex_module::Coin,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract
        .get_preview_shares_for_single_sided_liquidity_request(pool, coin)
        .await?;

    // 6. Return the number
    Ok(value)
}
// getPreviewAddLiquidityNoSwap
pub async fn get_preview_add_liquidity_no_swap(
    client: &Client,
    contract_addr: &H160,
    pool: String,
    liquidity: Vec<dex_module::Coin>,
) -> Result<(Vec<dex_module::Coin>, Vec<dex_module::Coin>), Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract
        .get_preview_add_liquidity_no_swap(pool, liquidity)
        .await?;

    // 6. Return the number
    Ok(value)
}

// getPreviewBurnShares
pub async fn get_preview_burn_shares(
    client: &Client,
    contract_addr: &H160,
    pool: String,
    shares: dex_module::Coin,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.get_preview_burn_shares(pool, shares).await?;

    // 6. Return the number
    Ok(value)
}

// getRemoveLiquidityExactAmountOut
pub async fn get_remove_liquidity_exact_amount_out(
    client: &Client,
    contract_addr: &H160,
    pool: String,
    asset: dex_module::Coin,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract
        .get_remove_liquidity_exact_amount_out(pool, asset)
        .await?;

    // 6. Return the number
    Ok(value)
}
//getRemoveLiquidityOneSideOut
pub async fn get_remove_liquidity_one_side_out(
    client: &Client,
    contract_addr: &H160,
    pool: String,
    denom: String,
    shares_in: U256,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract
        .get_remove_liquidity_one_side_out(pool, denom, shares_in)
        .await?;

    // 6. Return the number
    Ok(value)
}
// getPoolName
pub async fn get_pool_name(
    client: &Client,
    contract_addr: &H160,
    pool: String,
) -> Result<String, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.get_pool_name(pool).await?;

    // 6. Return the number
    Ok(value)
}
/// getPoolOptions
pub async fn get_pool_options(
    client: &Client,
    contract_addr: &H160,
    pool: String,
) -> Result<dex_module::PoolOptions, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.get_pool_options(pool).await?;

    // 6. Return the number
    Ok(value)
}

pub async fn get_pool_address(
    client: &Client,
    contract_addr: &H160,
    pool: String,
) -> Result<Address, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.get_pool_address(pool).await?;

    // 6. Return the number
    Ok(value)
}

// swap

pub async fn swap(
    client: &Client,
    contract_addr: &H160,
    single_swap: dex_module::SingleSwap,
    limit: U256,
    deadline: U256,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.swap(single_swap, limit, deadline).await?;

    // 6. Return the number
    Ok(value)
}

// batchSwap
pub async fn batch_swap(
    client: &Client,
    contract_addr: &H160,
    kind: u8, // TODO(davirain)
    swaps: Vec<dex_module::BatchSwapStep>,
    coins: Vec<dex_module::Coin>,
    deadline: U256,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.batch_swap(kind, swaps, coins, deadline).await?;

    // 6. Return the number
    Ok(value)
}

// createPool
pub async fn create_pool(
    client: &Client,
    contract_addr: &H160,
    name: String,
    coins: Vec<dex_module::Coin>,
    pool_type: String,
    options: Bytes,
    creator: Address,
) -> Result<String, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract
        .create_pool(name, coins, pool_type, options, creator)
        .await?;

    // 6. Return the number
    Ok(value)
}

///
pub async fn add_liquidity(
    client: &Client,
    contract_addr: &H160,
    pool: String,
    account: Address,
    receiver: Address,
    coin: Vec<dex_module::Coin>,
) -> Result<(Vec<dex_module::Coin>, Vec<dex_module::Coin>), Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract
        .add_liquidity(pool, account, receiver, coin)
        .await?;

    // 6. Return the number
    Ok(value)
}

/// removeLiquidityBurningShares

pub async fn remove_liquidity_burning_shares(
    client: &Client,
    contract_addr: &H160,
    pool: String,
    account: Address,
    receiver: Address,
    coin: dex_module::Coin,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract
        .remove_liquidity_burning_shares(pool, account, receiver, coin)
        .await?;

    // 6. Return the number
    Ok(value)
}

// removeLiquidityExactAmount
pub async fn remove_liquidity_exact_amount(
    client: &Client,
    contract_addr: &H160,
    pool: String,
    account: Address,
    receiver: Address,
    coin: dex_module::Coin,
    max_shares: dex_module::Coin,
) -> Result<(Vec<dex_module::Coin>, Vec<dex_module::Coin>), Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = DexModule::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract
        .remove_liquidity_exact_amount(pool, account, receiver, coin, max_shares)
        .await?;

    // 6. Return the number
    Ok(value)
}
