use crate::errors::Error;
use crate::utils::{get_all_keypairs, get_config, get_single_keypairs, send_transaction};
use colored::*;
use ethers::prelude::SignerMiddleware;
use ethers::prelude::Wallet;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers_signers::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Transfer {
    #[structopt(long)]
    pub chain_id: u64,
    /// keypair file name
    #[structopt(long)]
    pub file_name: String,
    /// is one to more
    #[structopt(long)]
    pub is_one_to_more: bool,
    /// transfer amount
    #[structopt(long)]
    pub amount: f64,
}

impl Transfer {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let single_keypair = get_single_keypairs().map_err(|e| Error::Custom(e.to_string()))?;
        assert!(single_keypair.keypairs.len() == 1);

        let single_keypair = single_keypair.keypairs[0]
            .clone()
            .with_chain_id(self.chain_id);

        let client = SignerMiddleware::new(
            provider.clone(),
            single_keypair.clone().with_chain_id(self.chain_id),
        );

        let balance = provider
            .get_balance(single_keypair.address(), None)
            .await
            .map_err(|e| Error::Custom(e.to_string()))?;

        println!(
            "{} has {}",
            format!("{:?}", single_keypair.address()).blue(),
            balance.to_string().red()
        );

        let keypairs = get_all_keypairs(&self.file_name)?;
        if self.is_one_to_more {
            for keypair in keypairs.keypairs {
                let balance = provider
                    .get_balance(keypair.address(), None)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                println!(
                    "{} has {}",
                    format!("{:?}", keypair.address()).blue(),
                    balance.to_string().red()
                );

                send_transaction(
                    &client,
                    single_keypair.address(),
                    keypair.address(),
                    self.amount,
                )
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

                let balance = provider
                    .get_balance(keypair.address(), None)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                println!(
                    "{} has {}",
                    format!("{:?}", keypair.address()).blue(),
                    balance.to_string().red()
                );
            }
        } else {
            for keypair in keypairs.keypairs {
                let balance = provider
                    .get_balance(keypair.address(), None)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                println!(
                    "{} has {}",
                    format!("{:?}", keypair.address()).blue(),
                    balance.to_string().red()
                );

                let client = SignerMiddleware::new(
                    provider.clone(),
                    keypair.clone().with_chain_id(self.chain_id),
                );
                send_transaction(
                    &client,
                    keypair.address(),
                    single_keypair.address(),
                    self.amount,
                )
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

                let balance = provider
                    .get_balance(keypair.address(), None)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                println!(
                    "{} has {}",
                    format!("{:?}", keypair.address()).blue(),
                    balance.to_string().red()
                );
            }
        }

        Ok(())
    }
}
