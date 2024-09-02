use crate::Client;
use ethers::prelude::*;
use ethers::utils::parse_units;
use std::str::FromStr;
use std::sync::Arc;

pub fn bera_croc_multi_swap() -> H160 {
    H160::from_str("0x21e2C0AFd058A89FCf7caf3aEA3cB84Ae977B73D").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    BeraCrocMultiSwap,
    "./contract/bex/BeraCrocMultiSwap.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

/// crocSwapDex
pub async fn croc_swap_dex(client: &Client) -> Result<Address, Box<dyn std::error::Error>> {
    let contract = BeraCrocMultiSwap::new(bera_croc_multi_swap(), Arc::new(client.clone()));
    let value = contract.croc_swap_dex().call().await?;
    Ok(value)
}

/// multiSwap
pub async fn multi_swap(
    client: &Client,
    pool_idx: U256,
    base: H160,
    quote: H160,
    is_buy: bool,
    amount: u128,
    min_out: u128,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = BeraCrocMultiSwap::new(bera_croc_multi_swap(), Arc::new(client.clone()));
    let eth_max_spend = parse_units(1, 17)?;
    let gas_price = U256::from(10_624_066_551u64); // 以太坊网络上的标准 gas 价格，你可以根据情况调整
                                                   //let value = contract.multi_swap().call().await?;
    let steps = vec![SwapStep {
        pool_idx,
        base,
        quote,
        is_buy,
    }];
    let tx = contract
        .multi_swap(steps, amount, min_out)
        .value(eth_max_spend)
        .from(client.address())
        .gas_price(gas_price) // 设置交易的 gas 价格
        .gas(U256::from(400_000u64)) // this is crucial otherwise tx will get reverted without a reason
        .send()
        .await?
        .await?;

    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);
    Ok(())
}
