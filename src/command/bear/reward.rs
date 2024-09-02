use crate::bear::precompile_contracts::rewards;
use crate::errors::Error;
use crate::utils::{get_all_keypairs_string_with_balance, get_config};
use colored::*;
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers_signers::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Reward {
    /// wallet file name
    #[structopt(long)]
    pub file_name: String,
}

impl Reward {
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

            // TODO(have error)
            let withdraw_result =
                rewards::withdraw_all_depositor_rewards(&client, keypair.address())
                    .await
                    .map_err(|e| Error::Custom(e.to_string()));

            println!(
                "{}: {:#?}",
                keypair.address().to_string().blue(),
                withdraw_result
            );
        }
        Ok(())
    }
}
