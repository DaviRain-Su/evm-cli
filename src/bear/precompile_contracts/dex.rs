use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

pub fn dex_addr() -> H160 {
    H160::from_str("0x9D0FBF9349F646F1435072F2B0212084752EF460").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    DexModule,
    "./contract/bear/precompile_contracts/IDexModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

/// getPreviewSwapExact
/// 预览单个交换到池中的情况。
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
        .call()
        .await?;
    Ok(value)
}

/// getPreviewBatchSwap
/// 预览批量交换到一系列池中。
pub async fn get_preview_batch_swap(
    client: &Client,
    kind: u8,
    swaps: Vec<dex_module::BatchSwapStep>,
    coins: Vec<dex_module::Coin>,
) -> Result<dex_module::Coin, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_preview_batch_swap(kind, swaps, coins)
        .call()
        .await?;
    Ok(value)
}

/// getLiquidity
/// 预览当前流动性池中的代币余额。
pub async fn get_liquidity(
    client: &Client,
    pool: String,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract.get_liquidity(pool).call().await?;
    Ok(value)
}

/// getTotalShares
/// 预览流动性池的份额总数。
pub async fn get_total_shares(
    client: &Client,
    pool: String,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract.get_total_shares(pool).call().await?;
    Ok(value)
}

/// getExchangeRate
/// 预览池中两种资产之间的汇率。注意：返回的 uint 表示为最多 18 位十进制精度的值
pub async fn get_exchange_rate(
    client: &Client,
    pool: String,
    base_asset: String,
    quote_asset: String,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_exchange_rate(pool, base_asset, quote_asset)
        .call()
        .await?;
    Ok(value)
}

/// getPreviewSharesForLiquidity
/// 预览为池添加流动性而将收到的 LP 代币数量。
pub async fn get_preview_shares_for_liquidity(
    client: &Client,
    pool: String,
    coins: Vec<dex_module::Coin>,
) -> Result<(Vec<dex_module::Coin>, Vec<dex_module::Coin>), Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));

    let value = contract
        .get_preview_shares_for_liquidity(pool, coins)
        .call()
        .await?;
    Ok(value)
}

/// getPreviewAddLiquidityStaticPrice
/// 预览可以添加到池中而不影响汇率的代币数量。
pub async fn get_preview_add_liquidity_static_price(
    client: &Client,
    pool: String,
    liquidity: Vec<dex_module::Coin>,
) -> Result<(Vec<dex_module::Coin>, Vec<dex_module::Coin>), Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));

    let value = contract
        .get_preview_add_liquidity_static_price(pool, liquidity)
        .call()
        .await?;
    Ok(value)
}

/// getPreviewSharesForSingleSidedLiquidityRequest
/// 预览通过向池中添加一侧流动性而收到的股票数量。
pub async fn get_preview_shares_for_single_sided_liquidity_request(
    client: &Client,
    pool: String,
    coin: dex_module::Coin,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_preview_shares_for_single_sided_liquidity_request(pool, coin)
        .call()
        .await?;

    Ok(value)
}

/// getPreviewAddLiquidityNoSwap
/// 预览在不交换的情况下通过增加流动性而收到的代币数量。
pub async fn get_preview_add_liquidity_no_swap(
    client: &Client,
    pool: String,
    liquidity: Vec<dex_module::Coin>,
) -> Result<(Vec<dex_module::Coin>, Vec<dex_module::Coin>), Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_preview_add_liquidity_no_swap(pool, liquidity)
        .call()
        .await?;
    Ok(value)
}

/// getPreviewBurnShares
/// 预览通过燃烧 LP 代币以消除流动性将收到的代币数量。
pub async fn get_preview_burn_shares(
    client: &Client,
    pool: String,
    shares: dex_module::Coin,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_preview_burn_shares(pool, shares)
        .call()
        .await?;
    Ok(value)
}

/// getRemoveLiquidityExactAmountOut
/// 预览从池中提取特定数量的一项资产所需移除的 LP 代币数量。
pub async fn get_remove_liquidity_exact_amount_out(
    client: &Client,
    pool: String,
    asset: dex_module::Coin,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_remove_liquidity_exact_amount_out(pool, asset)
        .call()
        .await?;
    Ok(value)
}

/// getRemoveLiquidityOneSideOut
/// 预览燃烧 LP 代币将收到的一项资产的数量。
pub async fn get_remove_liquidity_one_side_out(
    client: &Client,
    pool: String,
    denom: String,
    shares_in: U256,
) -> Result<Vec<dex_module::Coin>, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_remove_liquidity_one_side_out(pool, denom, shares_in)
        .call()
        .await?;
    Ok(value)
}

/// getPoolName
/// 获取给定池地址的池名称。
pub async fn get_pool_name(
    client: &Client,
    pool: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract.get_pool_name(pool).call().await?;
    Ok(value)
}

/// getPoolOptions
/// 获取给定池地址的池选项。
pub async fn get_pool_options(
    client: &Client,
    pool: String,
) -> Result<dex_module::PoolOptions, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract.get_pool_options(pool).call().await?;
    Ok(value)
}

/// getPoolAddress
/// 将池的 bech32 地址转换为十六进制地址。
pub async fn get_pool_address(
    client: &Client,
    pool: String,
) -> Result<Address, Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let value = contract.get_pool_address(pool).call().await?;
    Ok(value)
}

/// swap
///  与单个池执行交换。注意：如果限制设置为 0，则不设置最大滑点。
/// 注意：交换的类型（GIVEN_IN 与 GIVEN_OUT）决定限制是最大输入还是最小输出。
pub async fn swap(
    client: &Client,
    single_swap: dex_module::SingleSwap,
    limit: U256,
    deadline: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let tx = contract
        .swap(single_swap, limit, deadline)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// batchSwap
/// 与一个或多个池执行一系列交换。注意：交换按 swaps 数组指定的顺序顺序执行。
/// 注意：输入金额和输出金额由 coins 参数中设置的值确定。注意：与单次互换类似，限制由互换类型决定。
/// GIVEN_IN 表示限制是最小输出，GIVEN_OUT 表示限制是最大输入。注意：如果没有设置限制，则没有最大滑点。
pub async fn batch_swap(
    client: &Client,
    kind: u8, // TODO(davirain)
    swaps: Vec<dex_module::BatchSwapStep>,
    coins: Vec<dex_module::Coin>,
    deadline: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let tx = contract
        .batch_swap(kind, swaps, coins, deadline)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// createPool
///创建一个新池。
pub async fn create_pool(
    client: &Client,
    name: String,
    coins: Vec<dex_module::Coin>,
    pool_type: String,
    options: Bytes,
    creator: Address,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let tx = contract
        .create_pool(name, coins, pool_type, options, creator)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// addLiquidity
/// 为资金池增加流动性。
pub async fn add_liquidity(
    client: &Client,
    pool: String,
    account: Address,
    receiver: Address,
    coin: Vec<dex_module::Coin>,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));

    let tx = contract
        .add_liquidity(pool, account, receiver, coin)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// removeLiquidityBurningShares
/// 通过销毁股票来消除池中的流动性。
pub async fn remove_liquidity_burning_shares(
    client: &Client,
    pool: String,
    account: Address,
    receiver: Address,
    coin: dex_module::Coin,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let tx = contract
        .remove_liquidity_burning_shares(pool, account, receiver, coin)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// removeLiquidityExactAmount
/// 从池中移除特定数量的流动性，并烧毁最大数量的股票。
pub async fn remove_liquidity_exact_amount(
    client: &Client,
    pool: String,
    account: Address,
    receiver: Address,
    coin: dex_module::Coin,
    max_shares: dex_module::Coin,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = DexModule::new(dex_addr(), Arc::new(client.clone()));
    let tx = contract
        .remove_liquidity_exact_amount(pool, account, receiver, coin, max_shares)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
