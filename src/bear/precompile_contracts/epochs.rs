use crate::Client;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

fn epochs_addr() -> H160 {
    H160::from_str("0x612Dd8a861161819A4AD8F6f3E2A0567602877c0").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    EpochsModule,
    "./contract/bear/precompile_contracts/IEpochsModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

/// # Epoch
///
/// Epoch 代表一个时间间隔。有许多不同的纪元，但是大多数 PoL dApp 使用的纪元可以通过 berachain_epoch_identifier 来识别。
/// Berachain 追踪以下纪元：
///  - day
/// - hour
/// - week
/// - berachain_epoch_identifier
/// 一天的秒数 = (24 * 60 * 60) seconds = 86400
/// 给定 5 秒的区块时间，一天的一个纪元将持续大约： 86400/5 = 17280
/// 根据此数学计算，每个 17280 块将开始一个新纪元。
///
/// ## getCurrentEpoch
///
/// 根据标识符获取当前纪元。注意： berachain_epoch_identifier 是 berachain poof 流动性 dApp 的默认纪元标识符。
pub async fn get_current_epoch(
    client: &Client,
    identifier: String,
) -> Result<(i64, i64, i64), Box<dyn std::error::Error>> {
    let contract = EpochsModule::new(epochs_addr(), Arc::new(client.clone()));
    let value = contract.get_current_epoch(identifier).await?;
    Ok(value)
}
