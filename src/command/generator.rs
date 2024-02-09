use crate::errors::Error;
use ethers_core::rand::thread_rng;
use ethers_signers::{LocalWallet, Signer};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Generator {
    /// generator wallet numbers
    #[structopt(short, long)]
    pub wallet_num: usize,
    /// keypair file name
    #[structopt(short, long)]
    pub file_name: String,
    // chain id
    // #[structopt(short, long)]
    // pub chain_id: u64,
}

impl Generator {
    pub fn run(&self) -> Result<(), Error> {
        let keypairs = KeyPairs::from_keypairs(
            (0..self.wallet_num)
                .map(|_i| {
                    // LocalWallet::new(&mut thread_rng()).with_chain_id(self.chain_id)
                    LocalWallet::new(&mut thread_rng())
                })
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

        let keypairs_str = KeyPairsString::from(keypairs);

        let home_path = dirs::home_dir().ok_or(Error::Custom("can't open home dir".into()))?;
        let nobody_config_path = home_path.join(".config").join("evm-cli");
        let keypairs_path = nobody_config_path.join(format!("{}_keypairs.json", self.file_name));
        keypairs_str
            .write(keypairs_path.clone())
            .map_err(|e| Error::Custom(e.to_string()))?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct KeyPairs {
    pub keypairs: Vec<LocalWallet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyPairsString {
    pub keypairs: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub pubkey: String,
    pub secret: String,
}

impl KeyPairsString {
    pub fn write(&self, path: PathBuf) -> anyhow::Result<()> {
        let temp_keypairs_str = serde_json::to_string(&self)?;
        std::fs::write(path, temp_keypairs_str).map_err(|e| {
            Error::from(format!(
                "failed write keypairs_path: Error({})",
                e.to_string()
            ))
        })?;
        Ok(())
    }

    pub fn read(path: PathBuf) -> anyhow::Result<Self> {
        let temp_keypairs_str = std::fs::read_to_string(path).map_err(|e| {
            Error::from(format!(
                "failed read keypairs_path: Error({})",
                e.to_string()
            ))
        })?;
        let keypairs_str = serde_json::from_str(&temp_keypairs_str).map_err(|e| {
            Error::from(format!(
                "failed deserialze keypairs_path: Error({})",
                e.to_string()
            ))
        })?;
        Ok(keypairs_str)
    }
}

impl From<KeyPairs> for KeyPairsString {
    fn from(keypairs: KeyPairs) -> Self {
        let keypairs = keypairs
            .keypairs
            .iter()
            .map(|k| {
                let raw_keypairs = k.signer();
                Item {
                    pubkey: format!("{:?}", k.address()),
                    secret: serde_json::to_string(&raw_keypairs.to_bytes().to_vec())
                        .expect("failed ser"),
                }
            })
            .collect::<Vec<_>>();
        Self { keypairs }
    }
}

impl From<KeyPairsString> for KeyPairs {
    fn from(value: KeyPairsString) -> Self {
        let keypairs = value
            .keypairs
            .iter()
            .map(|k| {
                let raw_keypairs =
                    serde_json::from_str::<Vec<u8>>(&k.secret).expect("serde keypairs error");
                LocalWallet::from_bytes(&raw_keypairs).expect("keypairs from bytes error")
            })
            .collect::<Vec<_>>();
        Self { keypairs }
    }
}

impl KeyPairs {
    pub fn new() -> Self {
        Self { keypairs: vec![] }
    }

    pub fn from_keypairs(keypairs: Vec<LocalWallet>) -> Self {
        Self { keypairs }
    }

    pub fn push(&mut self, keypair: LocalWallet) {
        self.keypairs.push(keypair);
    }
}
