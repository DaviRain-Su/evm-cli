use crate::Client;
use ethers::prelude::*;
use ethers::utils::parse_units;
use std::str::FromStr;
use std::sync::Arc;

pub fn erc20_dex_addr() -> H160 {
    H160::from_str("0x0D5862FDBDD12490F9B4DE54C236CFF63B038074").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    ERC20DexModule,
    "./contract/bear/precompile_contracts/IERC20DexModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

/// #  addLiquidity
/// 为资金池增加流动性。
pub async fn add_liquidity(
    client: &Client,
    pool: Address,
    receiver: Address,
    assets_in: Vec<Address>,
    amounts_in: Vec<U256>,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let tx = contract
        .add_liquidity(pool, receiver, assets_in, amounts_in)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// # batchSwap
/// 与单个池执行交换。注意：如果限制设置为 0，则不设置最大滑点。
/// 注意：交换的类型（GIVEN_IN 与 GIVEN_OUT）决定限制是最大输入还是最小输出。
pub async fn batch_swap(
    client: &Client,
    kind: u8,
    swaps: Vec<erc20_dex_module::BatchSwapStep>,
    deadline: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let tx = contract
        .batch_swap(kind, swaps, deadline)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// # createPool
/// 创建一个新池。
pub async fn create_pool(
    client: &Client,
    name: String,
    assets_in: Vec<Address>,
    amounts_in: Vec<U256>,
    pool_type: String,
    options: erc20_dex_module::PoolOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let tx = contract
        .create_pool(name, assets_in, amounts_in, pool_type, options)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// # getExchangeRate
/// 预览池中两种资产之间的汇率。注意：返回的 uint 表示为最多 18 位十进制精度的值
pub async fn get_exchange_rate(
    client: &Client,
    pool: Address,
    base_asset: Address,
    quote_asset: Address,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_exchange_rate(pool, base_asset, quote_asset)
        .call()
        .await?;
    Ok(value)
}

/// # getLiquidity
/// 预览当前流动性池中的代币余额。
pub async fn get_liquidity(
    client: &Client,
    pool: Address,
) -> Result<(Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let value = contract.get_liquidity(pool).call().await?;
    Ok(value)
}

/// # getPoolName
/// 获取给定池地址的池名称。
pub async fn get_pool_name(
    client: &Client,
    pool: Address,
) -> Result<String, Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let value = contract.get_pool_name(pool).call().await?;
    Ok(value)
}

/// # getPoolOptions
/// 获取给定池地址的池选项。
pub async fn get_pool_options(
    client: &Client,
    pool: Address,
) -> Result<erc20_dex_module::PoolOptions, Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let value = contract.get_pool_options(pool).call().await?;
    Ok(value)
}

/// # getPreviewAddLiquidityNoSwap
/// 预览在不交换的情况下通过增加流动性而收到的代币数量。
pub async fn get_preview_add_liquidity_no_swap(
    client: &Client,
    pool: Address,
    assets: Vec<Address>,
    amounts: Vec<U256>,
) -> Result<(Vec<Address>, Vec<U256>, Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_preview_add_liquidity_no_swap(pool, assets, amounts)
        .call()
        .await?;
    Ok(value)
}

/// # getPreviewAddLiquidityStaticPrice
/// 预览可以添加到池中而不影响汇率的代币数量。
pub async fn get_preview_add_liquidity_static_price(
    client: &Client,
    pool: Address,
    liquidity: Vec<Address>,
    amounts: Vec<U256>,
) -> Result<(Vec<Address>, Vec<U256>, Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_preview_add_liquidity_static_price(pool, liquidity, amounts)
        .call()
        .await?;
    Ok(value)
}

/// # getPreviewBatchSwap
/// 预览批量交换。
pub async fn get_preview_batch_swap(
    client: &Client,
    kind: u8,
    swaps: Vec<erc20_dex_module::BatchSwapStep>,
) -> Result<(Address, U256), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let value = contract.get_preview_batch_swap(kind, swaps).call().await?;

    Ok(value)
}

/// # getPreviewBurnShares
/// 预览通过燃烧 LP 代币以消除流动性将收到的代币数量。
pub async fn get_preview_burn_shares(
    client: &Client,
    pool: Address,
    asset: Address,
    amount: U256,
) -> Result<(Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_preview_burn_shares(pool, asset, amount)
        .call()
        .await?;
    Ok(value)
}

/// # getPreviewSharesForLiquidity
/// 预览为池添加流动性而将收到的 LP 代币数量。
pub async fn get_preview_shares_for_liquidity(
    client: &Client,
    pool: Address,
    assets: Vec<Address>,
    amounts: Vec<U256>,
) -> Result<(Vec<Address>, Vec<U256>, Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_preview_shares_for_liquidity(pool, assets, amounts)
        .call()
        .await?;
    Ok(value)
}

/// # getPreviewSharesForSingleSidedLiquidityRequest
/// 预览通过向池中添加一侧流动性而收到的股票数量。
pub async fn get_preview_shares_for_single_sided_liquidity_request(
    client: &Client,
    pool: Address,
    asset: Address,
    amount: U256,
) -> Result<(Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_preview_shares_for_single_sided_liquidity_request(pool, asset, amount)
        .call()
        .await?;
    Ok(value)
}

/// # getPreviewSwapExact
/// 预览单个交换到池中的情况。
pub async fn get_preview_swap_exact(
    client: &Client,
    kind: u8,
    pool: Address,
    base_asset: Address,
    base_asset_amount: U256,
    quote_asset: Address,
) -> Result<(Address, U256), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_preview_swap_exact(kind, pool, base_asset, base_asset_amount, quote_asset)
        .call()
        .await?;
    Ok(value)
}

/// # getRemoveLiquidityExactAmountOut
/// 预览从池中提取特定数量的一项资产所需移除的 LP 代币数量。
pub async fn get_remove_liquidity_exact_amount_out(
    client: &Client,
    pool: Address,
    asset_in: Address,
    asset_amount: U256,
) -> Result<(Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_remove_liquidity_exact_amount_out(pool, asset_in, asset_amount)
        .call()
        .await?;
    Ok(value)
}

/// # getRemoveLiquidityOneSideOut
/// 预览燃烧 LP 代币将收到的一项资产的数量。
pub async fn get_remove_liquidity_one_side_out(
    client: &Client,
    pool: Address,
    asset_out: Address,
    shares_in: U256,
) -> Result<(Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let value = contract
        .get_remove_liquidity_one_side_out(pool, asset_out, shares_in)
        .call()
        .await?;
    Ok(value)
}

/// # getTotalShares
/// 预览流动性池的份额总数。
pub async fn get_total_shares(
    client: &Client,
    pool: Address,
) -> Result<(Vec<Address>, Vec<U256>), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let value = contract.get_total_shares(pool).call().await?;
    Ok(value)
}

/// # removeLiquidityBurningShares
/// 通过销毁股票来消除池中的流动性。
pub async fn remove_liquidity_burning_shares(
    client: &Client,
    pool: Address,
    withdraw_address: Address,
    asset_in: Address,
    amount_in: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let tx = contract
        .remove_liquidity_burning_shares(pool, withdraw_address, asset_in, amount_in)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// # removeLiquidityExactAmount
/// 从池中移除特定数量的流动性，并烧毁最大数量的股票。
pub async fn remove_liquidity_exact_amount(
    client: &Client,
    pool: Address,
    withdraw_address: Address,
    asset_out: Address,
    amount_out: U256,
    shares_in: Address,
    max_shares_in: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let tx = contract
        .remove_liquidity_exact_amount(
            pool,
            withdraw_address,
            asset_out,
            amount_out,
            shares_in,
            max_shares_in,
        )
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

/// # swap
/// 与单个池执行交换。注意：如果限制设置为 0，则不设置最大滑点。
// 注意：交换的类型（GIVEN_IN 与 GIVEN_OUT）决定限制是最大输入还是最小输出。
pub async fn swap(
    client: &Client,
    kind: u8,
    pool_id: Address,
    asset_in: Address,
    amount_in: U256,
    asset_out: Address,
    amount_out: U256,
    deadline: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = ERC20DexModule::new(erc20_dex_addr(), Arc::new(client.clone()));
    let eth_max_spend = parse_units(0, 18)?;
    // 设置交易的 gas 价格
    // 1_599_422_605
    // let gas_price = U256::from(4_000_000_000u64); // 以太坊网络上的标准 gas 价格，你可以根据情况调整
    let tx = contract
        .swap(
            kind, pool_id, asset_in, amount_in, asset_out, amount_out, deadline,
        )
        .value(amount_in)
        .from(client.address())
        // .gas_price(gas_price) // 设置交易的 gas 价格
        .gas(U256::from(30_000_000)) // this is crucial otherwise tx will get reverted without a reason
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
