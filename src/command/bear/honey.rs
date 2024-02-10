use crate::bear::deploy_contracts::honey::{self, honey_token_addr};
use crate::errors::Error;
use crate::utils::{get_all_keypairs, get_config, get_single_keypairs};
use colored::*;
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::types::U256;
use ethers_signers::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Honey {
    #[structopt(long)]
    pub chain_id: u64,
    #[structopt(long)]
    pub file_name: String,
}

impl Honey {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let single_keypair = get_single_keypairs().map_err(|e| Error::Custom(e.to_string()))?;

        let single_keypair = single_keypair.keypairs[0].clone();

        let client = SignerMiddleware::new(
            provider.clone(),
            single_keypair.clone().with_chain_id(self.chain_id),
        );

        let balance = honey::balance_of(&client, single_keypair.address())
            .await
            .map_err(|e| Error::Custom(e.to_string()))?;
        println!(
            "{} has Honey {} num",
            format!("{:?}", single_keypair.address()).blue(),
            balance
        );
        let approve_result = honey::approve(&client, honey_token_addr(), balance)
            .await
            .map_err(|e| Error::Custom(e.to_string()))?;
        println!("approve result : {:?}", approve_result);

        let keypairs =
            get_all_keypairs(&self.file_name).map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in &keypairs.keypairs {
            let balance = honey::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;
            let amount = U256::from(2_000_000_000_000_000_000u64);
            if balance != amount {
                let transfer_result = loop {
                    if let Ok(result) = honey::transfer(&client, keypair.address(), amount).await {
                        break result;
                    } else {
                        continue;
                    }
                };
                println!("transfer result : {:?}", transfer_result);

                let balance = honey::balance_of(&client, keypair.address())
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;
                println!(
                    "{} has Honey {} num",
                    format!("{:?}", keypair.address()).blue(),
                    balance
                );
            }
        }

        Ok(())
    }
}
