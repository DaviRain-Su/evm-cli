use crate::errors::Error;
use ethers_core::rand::thread_rng;
use ethers_signers::{LocalWallet, Signer};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Generator {
    Single(Single),
    Multi(Multi),
}

impl Generator {
    pub fn run(&self) -> Result<(), Error> {
        match self {
            Generator::Single(single) => single.run(),
            Generator::Multi(multi) => multi.run(),
        }
    }
}

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

        let keyparis_str = KeyPairsString::from(keypairs);

        let home_path = dirs::home_dir().ok_or(Error::Custom("can't open home dir".into()))?;
        let nobody_config_path = home_path.join(".config").join("evm-cli");
        let keypairs_path = nobody_config_path.join("keypairs.json");
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
                // write single wallet
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

                let keypairs_str = KeyPairsString::from(keypairs);

                let home_path =
                    dirs::home_dir().ok_or(Error::Custom("can't open home dir".into()))?;
                let nobody_config_path = home_path.join(".config").join("evm-cli");
                let keypairs_path = nobody_config_path.join("keypairs.json");
                keypairs_str
                    .write(keypairs_path.clone())
                    .map_err(|e| Error::Custom(e.to_string()))?;
                Ok(())
            }
            Single::Load(load) => load.run(),
        }
    }
}

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
