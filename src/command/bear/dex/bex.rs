use crate::bear::bex::bera_croc_multi_swap;
use crate::constant::BERA_DECIMAL;
use crate::errors::Error;
use crate::utils::{get_config, get_single_keypairs_string_with_balance};
use colored::*;
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{H160, U256};
use ethers_signers::Signer;
use std::str::FromStr;
use structopt::StructOpt;

// notice must use erc20 dex
#[derive(Debug, StructOpt)]
pub struct Swap {}

impl Swap {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypair = get_single_keypairs_string_with_balance()
            .map_err(|e| Error::Custom(e.to_string()))?
            .keypairs
            .first()
            .cloned()
            .unwrap();

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

        // pool index
        let pool_idx = U256::from(36000);
        // honey base
        let base = H160::from_str("0x0E4aaF1351de4c0264C5c7056Ef3777b41BD8e03").unwrap();
        // bera quote
        let quote = H160::from_str("0x0000000000000000000000000000000000000000").unwrap();
        let is_buy = false;

        let amount = 100000000000000000;
        let mint_out = 2729383319007357558;
        let swap_result = bera_croc_multi_swap::multi_swap(
            &client, pool_idx, base, quote, is_buy, amount, mint_out,
        )
        .await
        .map_err(|e| Error::Custom(e.to_string()))?;

        println!("swap: {:?}", swap_result);

        Ok(())
    }
}
