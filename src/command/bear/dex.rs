use crate::bear::precompile_contracts::erc20_dex;
use crate::errors::Error;
use crate::utils::{get_config, get_single_keypairs};
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers_core::types::Address;
use ethers_signers::Signer;
use structopt::StructOpt;

// notice must use erc20 dex
#[derive(Debug, StructOpt)]
pub struct Dex {
    #[structopt(long)]
    pub chain_id: u64,
    #[structopt(long)]
    pub file_name: String,
}

impl Dex {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs = get_single_keypairs().map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in keypairs.keypairs.iter() {
            let client = SignerMiddleware::new(
                provider.clone(),
                keypair.clone().with_chain_id(self.chain_id),
            );
            let pool: Address = "a88572F08f79D28b8f864350f122c1CC0AbB0d96".parse().unwrap();
            let pool_name = erc20_dex::get_pool_name(&client, pool.clone())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!("this address {:?} pool name is {}", pool, pool_name)
        }
        Ok(())
    }
}
