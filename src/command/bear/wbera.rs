use crate::bear::deploy_contracts::wbera;
use crate::errors::Error;
use crate::utils::{get_all_keypairs, get_config, get_single_keypairs};
use colored::*;
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::U256;
use ethers_signers::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum WBera {
    Withdraw(Withdraw),
    Deposit(Deposit),
}

impl WBera {
    pub async fn run(&self) -> Result<(), Error> {
        match self {
            WBera::Withdraw(wbera) => wbera.run().await,
            WBera::Deposit(deposit) => deposit.run().await,
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct Deposit {
    #[structopt(long)]
    pub chain_id: u64,
    #[structopt(long)]
    pub file_name: String,
}

impl Deposit {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs = get_single_keypairs().map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in keypairs.keypairs {
            let client = SignerMiddleware::new(
                provider.clone(),
                keypair.clone().with_chain_id(self.chain_id),
            );

            let native_balance = provider
                .get_balance(keypair.address(), None)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!(
                "{} has Bera {} num",
                format!("{:?}", keypair.address()).blue(),
                native_balance
            );

            let deposit_half_native_balance = native_balance / 2000;

            let deposit_result = loop {
                if let Err(e) = wbera::deposit(&client, deposit_half_native_balance).await {
                    log::warn!("Warn: {:?}", e.to_string());
                    continue;
                } else {
                    break;
                }
            };
            println!("deposit_result: {:?}", deposit_result);

            let balance = wbera::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;
            println!(
                "{} has Wbera {} num",
                format!("{:?}", keypair.address()).blue(),
                balance
            );
        }
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub struct Withdraw {
    #[structopt(long)]
    pub chain_id: u64,
    #[structopt(long)]
    pub file_name: String,
}

impl Withdraw {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs =
            get_all_keypairs(&self.file_name).map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in keypairs.keypairs {
            let client = SignerMiddleware::new(
                provider.clone(),
                keypair.clone().with_chain_id(self.chain_id),
            );

            let balance = wbera::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!(
                "{} has Wbera {} num",
                format!("{:?}", keypair.address()).blue(),
                balance
            );

            let withdraw_half_balance = balance / 2;

            let withdra_result = wbera::withdraw(&client, withdraw_half_balance)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!("withdra_result: {:?}", withdra_result);

            let balance = wbera::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;
            println!(
                "{} has Wbera {} num",
                format!("{:?}", keypair.address()).blue(),
                balance
            );
        }

        Ok(())
    }
}
