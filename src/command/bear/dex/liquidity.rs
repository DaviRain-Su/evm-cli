use crate::bear::deploy_contracts::honey;
use crate::bear::deploy_contracts::wbera;
use crate::bear::precompile_contracts::{erc20_bank::erc20_bank_addr, erc20_dex};
use crate::constant::BERA_DECIMAL;
use crate::errors::Error;
use crate::utils::calc_balance;
use crate::utils::{get_all_keypairs_string_with_balance, get_config};
use colored::Colorize;
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::U256;
use ethers_core::types::Address;
use ethers_signers::Signer;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

// notice must use erc20 dex
#[derive(Debug, StructOpt)]
pub struct Liquidity {
    /// chain id
    #[structopt(long)]
    pub chain_id: u64,
    /// wallet file name
    #[structopt(long)]
    pub file_name: String,
}

impl Liquidity {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs = get_all_keypairs_string_with_balance(&self.file_name)
            .map_err(|e| Error::Custom(e.to_string()))?;
        // let keypairs = get_single_keypairs().map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in keypairs.keypairs.iter() {
            let client = SignerMiddleware::new(
                provider.clone(),
                keypair.clone().with_chain_id(self.chain_id),
            );

            let balance = provider
                .get_balance(keypair.address(), None)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            let native_balance_f64 = balance.as_u128() as f64 / BERA_DECIMAL;

            println!(
                "Address({}) have {} Bera",
                keypair.address().to_string().green(),
                native_balance_f64.to_string().bright_blue()
            );

            let pool_id: Address = "0xa88572F08f79D28b8f864350f122c1CC0AbB0d96"
                .parse()
                .unwrap();
            let pool_name = erc20_dex::get_pool_name(&client, pool_id.clone())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!(
                "this address {} pool name is {}",
                pool_id.to_string().bright_magenta(),
                pool_name.bright_red()
            );

            // let kind = 0;
            let base_asset: Address = "0x5806e416da447b267cea759358cf22cc41fae80f"
                .parse()
                .unwrap();

            let wbera_decimal = wbera::decimals(&client)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            let base_asset_amount = wbera::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            let base_asset_amount_f64 = calc_balance(wbera_decimal as u32, base_asset_amount);

            println!(
                "Base Asset({}) balance: {}",
                base_asset.to_string().bright_blue(),
                base_asset_amount_f64.to_string().bright_red(),
            );

            let quote_asset: Address = "0x7eeca4205ff31f947edbd49195a7a88e6a91161b"
                .parse()
                .unwrap();
            let honey_decimal = honey::decimals(&client)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            let quote_asset_amount = honey::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            let quote_asset_amount_f64 = calc_balance(honey_decimal as u32, quote_asset_amount);

            println!(
                "Quote Asset({}) balance: {}",
                quote_asset.to_string().bright_blue(),
                quote_asset_amount_f64.to_string().bright_red(),
            );

            if base_asset_amount > U256::zero() && quote_asset_amount > U256::zero() {
                let wbera_addr_result =
                    wbera::approve(&client, erc20_bank_addr(), base_asset_amount)
                        .await
                        .map_err(|e| Error::Custom(e.to_string()))?;
                println!("approve wbera_addr result {:?}", wbera_addr_result);

                let receiver = keypair.address();
                let asset_in = vec![base_asset, quote_asset];
                let asset_amount = vec![base_asset_amount, quote_asset_amount];
                let result_add_liquidity =
                    erc20_dex::add_liquidity(&client, pool_id, receiver, asset_in, asset_amount)
                        .await
                        .map_err(|e| Error::Custom(e.to_string()));
                println!("add_liquidity: {:?}", result_add_liquidity);
                thread::sleep(Duration::from_secs(1));
            }
        }
        Ok(())
    }
}
