use crate::bear::deploy_contracts::{
    multicall3_addr,
    wbera::{self, wbera_addr},
};
use crate::bear::precompile_contracts::dex;

use crate::bear::precompile_contracts::{
    erc20_bank::{self, erc20_bank_addr},
    erc20_dex::{self, erc20_dex_addr},
};
use crate::errors::Error;
use crate::utils::{get_all_keypairs, get_config, get_single_keypairs};
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::U256;
use ethers_core::types::Address;
use ethers_signers::Signer;
use std::time::{SystemTime, UNIX_EPOCH};
use structopt::StructOpt;
use time::OffsetDateTime;

// notice must use erc20 dex
#[derive(Debug, StructOpt)]
pub struct Swap {
    #[structopt(long)]
    pub chain_id: u64,
    #[structopt(long)]
    pub file_name: String,
}

impl Swap {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        // let keypairs =
        // get_all_keypairs(&self.file_name).map_err(|e| Error::Custom(e.to_string()))?;
        let keypairs = get_single_keypairs().map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in keypairs.keypairs.iter() {
            let client = SignerMiddleware::new(
                provider.clone(),
                keypair.clone().with_chain_id(self.chain_id),
            );

            // let block_number = BlockId::from(BlockNumber::Finalized);
            let balance = provider
                .get_balance(keypair.address(), None)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!("Address({:?}) have {}", keypair.address(), balance);

            let pool_id: Address = "0xa88572F08f79D28b8f864350f122c1CC0AbB0d96"
                .parse()
                .unwrap();
            let pool_name = erc20_dex::get_pool_name(&client, pool_id.clone())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!("this address {:?} pool name is {}", pool_id, pool_name);

            let liquidity = erc20_dex::get_liquidity(&client, pool_id.clone())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!("this address {:?} liquidity {:?}", pool_id, liquidity);

            let kind = 0;
            let base_asset: Address = "0x5806e416da447b267cea759358cf22cc41fae80f"
                .parse()
                .unwrap();
            let base_asset_amount = wbera::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;
            println!(
                "Base Asset({:?}) balance: {}",
                base_asset, base_asset_amount
            );

            let wbera_addr_result = wbera::approve(&client, wbera_addr(), base_asset_amount)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;
            println!("approve wbera_addr result {:?}", wbera_addr_result);

            let quote_asset: Address = "0x7eeca4205ff31f947edbd49195a7a88e6a91161b"
                .parse()
                .unwrap();
            let preview_swap = erc20_dex::get_preview_swap_exact(
                &client,
                kind,
                pool_id,
                base_asset,
                base_asset_amount / 100,
                quote_asset,
            )
            .await
            .map_err(|e| Error::Custom(e.to_string()))?;
            println!("preview swap: {:?}", preview_swap);

            let deadline = U256::from(get_epoch_milliseconds()) + U256::from(60 * 1000);

            // todo(davirain)
            // ERROR(Error: Custom("Contract call reverted with data: 0x"))

            let result_swap = erc20_dex::swap(
                &client,
                kind,
                pool_id,
                base_asset,
                base_asset_amount / 100,
                preview_swap.0,
                preview_swap.1,
                deadline,
            )
            .await
            .map_err(|e| Error::Custom(e.to_string()))?;
            println!("swap: {:?}", result_swap);

            // let total_shares = erc20_dex::get_total_shares(&client, pool_id.clone())
            //     .await
            //     .map_err(|e| Error::Custom(e.to_string()))?;
            // println!("total shares: {:?}", total_shares);

            // let exchange_rate =
            //     erc20_dex::get_exchange_rate(&client, pool_id.clone(), base_asset, quote_asset)
            //         .await
            //         .map_err(|e| Error::Custom(e.to_string()))?;

            // println!("the exchange rate is {:?}", exchange_rate);

            // let options = erc20_dex::get_pool_options(&client, pool_id.clone())
            //     .await
            //     .map_err(|e| Error::Custom(e.to_string()))?;

            // println!("pool options: {:?}", options);
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
