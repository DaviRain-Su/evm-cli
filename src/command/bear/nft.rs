use crate::bear::deploy_contracts::honey::{self, honey_token_addr};
use crate::bear::nft::lunar_new_year::{self, lunar_new_year_addr};
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

        let keypairs =
            get_all_keypairs(&self.file_name).map_err(|e| Error::Custom(e.to_string()))?;

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

            println!(
                "Address({:?}) have {} Lunar New Year NFT num",
                keypair.address(),
                balance
            );

            let honey_balance = honey::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!(
                "Address({:?}) have {} honey num",
                keypair.address(),
                balance
            );

            let approve_result = honey::approve(&client, lunar_new_year_addr(), honey_balance)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!("approve_result: {:?}", approve_result);

            let buy_result = lunar_new_year::buy(&client)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!("buy_result: {:?}", approve_result);

            let balance = lunar_new_year::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!(
                "Address({:?}) have {}  num Lunar New Year NFT",
                keypair.address(),
                balance
            );

            let honey_balance = honey::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!(
                "Address({:?}) have {} honey num",
                keypair.address(),
                balance
            );
        }

        Ok(())
    }
}
