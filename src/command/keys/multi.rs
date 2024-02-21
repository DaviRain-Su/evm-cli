use super::*;
use crate::errors::Error;
use ethers::core::rand::thread_rng;
use ethers_signers::LocalWallet;
use ethers_signers::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Multi {
    /// generator wallet numbers
    #[structopt(short, long)]
    pub wallet_num: usize,
    /// keypair file name
    #[structopt(short, long)]
    pub file_name: String,
}

impl Multi {
    pub fn run(&self) -> Result<(), Error> {
        // write multi wallet
        let keypairs = KeyPairs::from_keypairs(
            (0..self.wallet_num)
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

        let home_path = dirs::home_dir().ok_or(Error::Custom("can't open home dir".into()))?;
        let keypairs_path = home_path
            .join(".config")
            .join("evm-cli")
            .join(format!("{}_keypairs.json", self.file_name));

        keypairs_str
            .write(keypairs_path.clone())
            .map_err(|e| Error::Custom(e.to_string()))?;

        Ok(())
    }
}
