use crate::bear::precompile_contracts::epochs;
use crate::errors::Error;
use crate::utils::{get_config, get_single_keypairs};
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers_signers::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Epoch {
    #[structopt(long)]
    pub chain_id: u64,
}

impl Epoch {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs = get_single_keypairs().map_err(|e| Error::Custom(e.to_string()))?;
        assert!(keypairs.keypairs.len() == 1);
        let keypair = keypairs.keypairs[0].clone();
        let client = SignerMiddleware::new(
            provider.clone(),
            keypair.clone().with_chain_id(self.chain_id),
        );

        let current_epoch =
            epochs::get_current_epoch(&client, "berachain_epoch_identifier".to_string())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

        println!("Current Epoch {:?}", current_epoch);

        Ok(())
    }
}
