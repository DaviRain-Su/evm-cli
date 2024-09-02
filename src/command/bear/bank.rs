use crate::bear::precompile_contracts::bank;
use crate::errors::Error;
use crate::utils::{get_all_keypairs_string_with_balance, get_config};
use colored::*;
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers_signers::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Bank {
    /// wallet file name
    #[structopt(long)]
    pub file_name: String,
}

impl Bank {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs = get_all_keypairs_string_with_balance(&self.file_name)
            .map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in &keypairs.keypairs {
            let client = SignerMiddleware::new(
                provider.clone(),
                keypair.clone().with_chain_id(config.chain_id),
            );

            let all_balance = bank::get_all_balances(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!(
                "{} has All Balance {:#?}",
                keypair.address().to_string().blue(),
                all_balance
            );
        }
        Ok(())
    }
}
