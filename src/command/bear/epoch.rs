use crate::bear::precompile_contracts::epochs;
use crate::errors::Error;
use crate::utils::{get_config, get_single_keypairs_string_with_balance};
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers_signers::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Epoch {}

impl Epoch {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs =
            get_single_keypairs_string_with_balance().map_err(|e| Error::Custom(e.to_string()))?;
        assert!(keypairs.keypairs.len() == 1);
        let keypair = keypairs.keypairs[0].clone();
        let client = SignerMiddleware::new(
            provider.clone(),
            keypair.clone().with_chain_id(config.chain_id),
        );

        let current_epoch =
            epochs::get_current_epoch(&client, "berachain_epoch_identifier".to_string())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

        println!("Current Epoch {:?}", current_epoch);

        Ok(())
    }
}
