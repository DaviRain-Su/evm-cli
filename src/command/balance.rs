use crate::errors::Error;
use crate::utils::{get_all_keypairs, get_config, get_single_keypairs};
use colored::*;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Balance {
    Single,
    Multi(Multi),
}

impl Balance {
    pub async fn run(&self) -> Result<(), Error> {
        match self {
            Balance::Single => {
                let config = get_config().map_err(|e| Error::from(e.to_string()))?;

                let provider = Provider::<Http>::try_from(config.rpc_endpoint)
                    .map_err(|e| Error::Custom(e.to_string()))?;

                let single_keypair =
                    get_single_keypairs().map_err(|e| Error::Custom(e.to_string()))?;

                for keypair in single_keypair.keypairs {
                    // let block_number = BlockId::from(BlockNumber::Finalized);
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
            Balance::Multi(multi) => multi.run().await,
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct Multi {
    /// keypair file name
    #[structopt(short, long)]
    pub file_name: String,
}

impl Multi {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs =
            get_all_keypairs(&self.file_name).map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in keypairs.keypairs {
            let mut count = 0;
            loop {
                let balance = provider
                    .get_balance(keypair.address(), None)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;
                if balance == U256::zero() && count <= 3 {
                    log::warn!(
                        "WARN: Address({:?}) balance is Zero({})",
                        keypair.address(),
                        balance
                    );
                    count += 1;
                    continue;
                } else {
                    println!(
                        "{} has {}",
                        format!("{:?}", keypair.address()).blue(),
                        balance.to_string().red()
                    );
                    break;
                }
            }
        }
        Ok(())
    }
}
