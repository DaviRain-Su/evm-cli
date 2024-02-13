use crate::bear::deploy_contracts::wbera::{self, wbera_addr};
use crate::bear::precompile_contracts::erc20_dex::{self};
use crate::constant::BERA_DECIMAL;
use crate::errors::Error;
use crate::utils::{get_all_keypairs, get_config};
use colored::*;
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::U256;
use ethers_core::types::Address;
use ethers_signers::Signer;
use std::time::{SystemTime, UNIX_EPOCH};
use structopt::StructOpt;

// notice must use erc20 dex
#[derive(Debug, StructOpt)]
pub struct Swap {
    /// chain id
    #[structopt(long)]
    pub chain_id: u64,
    /// wallet file name
    #[structopt(long)]
    pub file_name: String,
}

impl Swap {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs =
            get_all_keypairs(&self.file_name).map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in keypairs.keypairs.iter() {
            let client = SignerMiddleware::new(
                provider.clone(),
                keypair.clone().with_chain_id(self.chain_id),
            );

            let mut counter = 0;
            let balance = loop {
                if let Ok(v) = provider.get_balance(keypair.address(), None).await {
                    if (v != U256::zero()) & (counter < 3) {
                        break v;
                    } else if counter == 3 {
                        break v;
                    } else {
                        log::warn!("Try {} time", counter.to_string().red());
                        counter += 1;
                        continue;
                    }
                } else {
                    continue;
                }
            };

            let native_balance_f64 = balance.as_u128() as f64 / BERA_DECIMAL;

            println!(
                "Address({}) have {} Bera",
                keypair.address().to_string().blue(),
                native_balance_f64.to_string().green()
            );

            let pool_id: Address = "0xa88572F08f79D28b8f864350f122c1CC0AbB0d96"
                .parse()
                .unwrap();
            let pool_name = erc20_dex::get_pool_name(&client, pool_id.clone())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!(
                "this address {} pool name is {}",
                pool_id.to_string().red(),
                pool_name.to_string().green()
            );

            let kind = 0;
            let base_asset: Address = "0x5806e416da447b267cea759358cf22cc41fae80f"
                .parse()
                .unwrap();

            let base_asset_amount = wbera::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;
            let wbera_decimal = loop {
                if let Ok(v) = wbera::decimals(&client).await {
                    break v;
                } else {
                    continue;
                }
            };

            let exponent: u32 = wbera_decimal as u32; // 自定义指数值
            let divisor: u128 = 10u128.pow(exponent); // 计算除数
            let base_asset_amount_f64 = base_asset_amount.as_u128() as f64 / divisor as f64;

            println!(
                "Base Asset({}) balance: {}",
                base_asset.to_string().red(),
                base_asset_amount_f64.to_string().green()
            );

            let base_swap_amount = base_asset_amount / 2;

            let mut counter = 0;
            loop {
                if let Err(result) = wbera::approve(&client, wbera_addr(), base_asset_amount).await
                {
                    if counter == 3 {
                        break;
                    } else {
                        log::warn!("Wbera approce Error({:?})", result);
                        counter += 1;
                        continue;
                    }
                } else {
                    break;
                }
            }

            let quote_asset: Address = "0x7eeca4205ff31f947edbd49195a7a88e6a91161b"
                .parse()
                .unwrap();

            let preview_swap = loop {
                let result = erc20_dex::get_preview_swap_exact(
                    &client,
                    kind,
                    pool_id,
                    base_asset,
                    base_swap_amount,
                    quote_asset,
                )
                .await;
                if let Ok(r) = result {
                    break r;
                } else {
                    continue;
                }
            };
            println!("preview swap: {:?}", preview_swap);

            let deadline = U256::from(get_epoch_milliseconds()) + U256::from(60 * 1000);

            let mut counter = 0;
            let swap_result = loop {
                log::info!("Process erc20 dex Swap");
                if let Err(result) = erc20_dex::swap(
                    &client,
                    kind,
                    pool_id,
                    base_asset,
                    base_swap_amount,
                    preview_swap.0,
                    preview_swap.1,
                    deadline,
                )
                .await
                {
                    if counter == 3 {
                        break;
                    } else {
                        println!("Warn Error({})", result.to_string().red());
                        counter += 1;
                        continue;
                    }
                } else {
                    break;
                }
            };
            println!("swap: {:?}", swap_result);
        }
        Ok(())
    }
}

fn get_epoch_milliseconds() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
