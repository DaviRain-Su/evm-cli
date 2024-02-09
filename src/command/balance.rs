use crate::errors::Error;
use crate::utils::{get_all_keypairs, get_config};
use colored::*;
use ethers::prelude::SignerMiddleware;
use ethers::prelude::Wallet;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers_signers::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Balance {
    /// keypair file name
    #[structopt(short, long)]
    pub file_name: String,
}

impl Balance {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs =
            get_all_keypairs(&self.file_name).map_err(|e| Error::Custom(e.to_string()))?;

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
        }
        Ok(())
    }
}
