use crate::config::Config;
use crate::constant::DEFAULT_CONFIG_FILE;
use crate::errors::Error;
use crate::incrementer::{compile_deploy_contract, increment_number, read_number, reset};
use crate::utils::{get_all_keypairs, get_config, get_single_keypairs};
use crate::Client;
use ethers::prelude::*;
use ethers_solc::Solc;
use std::{path::Path, sync::Arc};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct DeploySc {
    #[structopt(long)]
    pub chain_id: u64,
    #[structopt(long)]
    pub file_name: String,
}

impl DeploySc {
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

            let addr = compile_deploy_contract(&client)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            read_number(&client, &addr)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            increment_number(&client, &addr)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            read_number(&client, &addr)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            reset(&client, &addr)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            read_number(&client, &addr)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;
        }

        Ok(())
    }
}
