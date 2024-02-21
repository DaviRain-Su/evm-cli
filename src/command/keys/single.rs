use super::*;
use crate::errors::Error;
use ethers::core::rand::thread_rng;
use ethers_signers::LocalWallet;
use ethers_signers::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Single {
    Generator,
    Load(Load),
}

#[derive(Debug, StructOpt)]
pub struct Load {
    pub private_key: String,
}

impl Load {
    pub fn run(&self) -> Result<(), Error> {
        let wallet: LocalWallet = self
            .private_key
            .parse::<LocalWallet>()
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs = KeyPairs {
            keypairs: vec![wallet],
        };

        let keyparis_str = KeyPairsStringWithbalance::from(keypairs);

        let home_path = dirs::home_dir().ok_or(Error::Custom("can't open home dir".into()))?;
        let keypairs_path = home_path
            .join(".config")
            .join("evm-cli")
            .join("keypairs.json");

        keyparis_str
            .write(keypairs_path.clone())
            .map_err(|e| Error::Custom(e.to_string()))?;
        Ok(())
    }
}

impl Single {
    pub fn run(&self) -> Result<(), Error> {
        match self {
            Single::Generator => {
                let keypairs = KeyPairs::from_keypairs(
                    (0..1)
                        .map(|_i| LocalWallet::new(&mut thread_rng()))
                        .collect::<Vec<LocalWallet>>(),
                );
                log::info!(
                    "keypairs: {:?}",
                    keypairs
                        .keypairs
                        .iter()
                        .map(|k| k.address())
                        .collect::<Vec<_>>()
                );

                let keypairs_str = KeyPairsStringWithbalance::from(keypairs);

                let home_path =
                    dirs::home_dir().ok_or(Error::Custom("can't open home dir".into()))?;
                let keypairs_path = home_path
                    .join(".config")
                    .join("evm-cli")
                    .join("keypairs.json");

                keypairs_str
                    .write(keypairs_path.clone())
                    .map_err(|e| Error::Custom(e.to_string()))?;
                Ok(())
            }
            Single::Load(load) => load.run(),
        }
    }
}
