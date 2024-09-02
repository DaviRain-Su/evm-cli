use crate::bear::deploy_contracts::{stg_usdc, wbera};
use crate::bear::precompile_contracts::erc20_bank::erc20_bank_addr;
use crate::bear::precompile_contracts::erc20_dex::{self, erc20_dex_module};
use crate::constant::BERA_DECIMAL;
use crate::errors::Error;
use crate::utils::calc_balance;
use crate::utils::{get_all_keypairs_string_with_balance, get_config};
use colored::*;
use ethers::abi::Bytes;
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::U256;
use ethers_core::types::Address;
use ethers_signers::Signer;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use structopt::StructOpt;

// notice must use erc20 dex
#[derive(Debug, StructOpt)]
pub struct SwapUSDC {
    /// wallet file name
    #[structopt(long)]
    pub file_name: String,
}

impl SwapUSDC {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs = get_all_keypairs_string_with_balance(&self.file_name)
            .map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in keypairs.keypairs.iter() {
            let client = SignerMiddleware::new(
                provider.clone(),
                keypair.clone().with_chain_id(config.chain_id),
            );

            let balance = provider
                .get_balance(keypair.address(), None)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            let native_balance_f64 = balance.as_u128() as f64 / BERA_DECIMAL;

            println!(
                "Address({}) have {} Bera",
                keypair.address().to_string().blue(),
                native_balance_f64.to_string().green()
            );

            let pool_id: Address = "0x7D5b5C1937ff1b18B45AbC64aeAB68663a7a58Ab"
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
            let wbera_decimal = wbera::decimals(&client)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            let base_asset_amount_f64 = calc_balance(wbera_decimal as u32, base_asset_amount);

            println!(
                "Base Asset({}) balance: {}",
                base_asset.to_string().red(),
                base_asset_amount_f64.to_string().green()
            );

            //TODO
            let half_base_swap_amount = base_asset_amount;

            let quote_asset: Address = "0x6581e59A1C8dA66eD0D313a0d4029DcE2F746Cc5"
                .parse()
                .unwrap();

            let deadline = U256::from(get_epoch_milliseconds()) + U256::from(60 * 1000);

            let stg_usdc_amount = stg_usdc::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            if base_asset_amount != U256::zero() && stg_usdc_amount == U256::zero() {
                let approve_result =
                    wbera::approve(&client, erc20_bank_addr(), base_asset_amount).await;
                println!("Approve Result:{:?}", approve_result);

                let preview_swap = erc20_dex::get_preview_swap_exact(
                    &client,
                    kind,
                    pool_id,
                    base_asset,
                    half_base_swap_amount,
                    quote_asset,
                )
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;
                println!("preview swap: {:?}", preview_swap);

                let swaps = vec![erc20_dex_module::BatchSwapStep {
                    pool_id,
                    asset_in: base_asset,
                    amount_in: half_base_swap_amount,
                    asset_out: preview_swap.0,
                    amount_out: preview_swap.1,
                    user_data: Bytes::new().into(),
                }];
                let swap_result = erc20_dex::batch_swap(&client, kind, swaps, deadline)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                println!("swap: {:?}", swap_result);
                thread::sleep(Duration::from_secs(1));
            } else {
                let balance = provider
                    .get_balance(keypair.address(), None)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                let native_balance_f64 = balance.as_u128() as f64 / BERA_DECIMAL;

                let stg_usdc_decimal = stg_usdc::decimals(&client)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                let stg_usdc_balance_f64 = calc_balance(stg_usdc_decimal as u32, stg_usdc_amount);

                let base_asset_amount = wbera::balance_of(&client, keypair.address())
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;
                let wbera_decimal = wbera::decimals(&client)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                let base_asset_amount_f64 = calc_balance(wbera_decimal as u32, base_asset_amount);

                println!(
                    "Address({}) Have {} Bera {} Wbera {} Honey",
                    keypair.address().to_string().blue(),
                    native_balance_f64.to_string().green(),
                    base_asset_amount_f64.to_string().green(),
                    stg_usdc_balance_f64.to_string().green()
                );
            }
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
