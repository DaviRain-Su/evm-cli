use crate::bear::nft::lunar_new_year;
use crate::errors::Error;
use crate::utils::{get_all_keypairs, get_config, get_single_keypairs};
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers_signers::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct NFT {
    #[structopt(long)]
    pub chain_id: u64,
    #[structopt(long)]
    pub file_name: String,
}

impl NFT {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs = get_single_keypairs().map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in &keypairs.keypairs {
            let client = SignerMiddleware::new(
                provider.clone(),
                keypair.clone().with_chain_id(self.chain_id),
            );

            let total_supply = lunar_new_year::total_supply(&client)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;
            println!("Total supply {:#?}", total_supply);

            let balance = lunar_new_year::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!("Address({:?}) have {} num", keypair.address(), balance);
        }

        Ok(())
    }
}
