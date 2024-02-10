use crate::bear::deploy_contracts::honey::{self, honey_token_addr};
use crate::bear::nft::lunar_new_year::{self, lunar_new_year_addr};
use crate::errors::Error;
use crate::utils::{get_all_keypairs, get_config, get_single_keypairs};
use colored::*;
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::types::U256;
use ethers_signers::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum NFT {
    NftBuy(NftBuy),
    NftCheck(NftCheck),
}

impl NFT {
    pub async fn run(&self) -> Result<(), Error> {
        match self {
            NFT::NftBuy(buy) => buy.run().await,
            NFT::NftCheck(check) => check.run().await,
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct NftCheck {
    #[structopt(long)]
    pub chain_id: u64,
    #[structopt(long)]
    pub file_name: String,
}

impl NftCheck {
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

            let lunar_new_year_balance = lunar_new_year::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!(
                "Address({}) have {} number Lunar New Year NFT",
                keypair.address().to_string().red(),
                lunar_new_year_balance
            );
        }

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub struct NftBuy {
    #[structopt(long)]
    pub chain_id: u64,
    #[structopt(long)]
    pub file_name: String,
}

impl NftBuy {
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

            let lunar_new_year_balance = lunar_new_year::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            if lunar_new_year_balance < U256::from(2u64) {
                log::info!(
                    "Address({:?}) have {} Lunar New Year NFT num",
                    keypair.address(),
                    lunar_new_year_balance
                );
                let honey_balance = honey::balance_of(&client, keypair.address())
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                println!(
                    "Address({:?}) have {} honey num",
                    keypair.address(),
                    honey_balance
                );

                let approve_result = honey::approve(&client, lunar_new_year_addr(), honey_balance)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                println!("approve_result: {:?}", approve_result);

                loop {
                    if let Err(e) = lunar_new_year::buy(&client).await {
                        log::warn!("Warn : {:?}", e.to_string());
                        continue;
                    } else {
                        break;
                    }
                }

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
            } else {
                log::warn!(
                    "Address({:?}) have {} Lunar New Year NFT num",
                    keypair.address(),
                    lunar_new_year_balance
                );
            }
        }

        Ok(())
    }
}
