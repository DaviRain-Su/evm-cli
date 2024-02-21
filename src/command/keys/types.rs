use crate::errors::Error;
use ethers_signers::LocalWallet;
use ethers_signers::Signer;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

impl KeyPairsString {
    pub fn convert_from_keypairs(keypairs: KeyPairs) -> Self {
        let keypairs = keypairs
            .keypairs
            .iter()
            .map(|k| {
                let raw_keypairs = k.signer();
                let secret_hex: String = raw_keypairs
                    .to_bytes()
                    .to_vec()
                    .iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect();

                Item {
                    pubkey: format!("{:?}", k.address()),
                    secret: secret_hex,
                }
            })
            .collect::<Vec<_>>();
        Self { keypairs }
    }

    pub fn convert_to_new_format(keypairs: KeyPairs) -> Self {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyPairsStringWithbalance {
    pub keypairs: Vec<ItemWithBalance>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemWithBalance {
    pub pubkey: String,
    pub secret: String,
    pub balance: f64,
    pub verify: bool,
    /// is or not call 💦
    pub on_task: bool,
    /// call 💦 time
    pub update_time: String,
}

impl KeyPairsStringWithbalance {
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

impl From<KeyPairs> for KeyPairsStringWithbalance {
    fn from(keypairs: KeyPairs) -> Self {
        let keypairs = keypairs
            .keypairs
            .iter()
            .map(|k| {
                let raw_keypairs = k.signer();
                ItemWithBalance {
                    pubkey: format!("{:?}", k.address()),
                    secret: serde_json::to_string(&raw_keypairs.to_bytes().to_vec())
                        .expect("failed ser"),
                    balance: 0.0,
                    verify: false,
                    on_task: false,
                    update_time: String::new(),
                }
            })
            .collect::<Vec<_>>();
        Self { keypairs }
    }
}

impl From<KeyPairsStringWithbalance> for KeyPairsString {
    fn from(value: KeyPairsStringWithbalance) -> Self {
        let keypairs = value
            .keypairs
            .iter()
            .map(|k| Item {
                pubkey: k.pubkey.clone(),
                secret: k.secret.clone(),
            })
            .collect::<Vec<_>>();
        Self { keypairs }
    }
}

impl From<KeyPairsString> for KeyPairsStringWithbalance {
    fn from(value: KeyPairsString) -> Self {
        let keypairs = value
            .keypairs
            .iter()
            .map(|k| ItemWithBalance {
                pubkey: k.pubkey.clone(),
                secret: k.secret.clone(),
                balance: 0.0,
                verify: false,
                on_task: false,
                update_time: String::new(),
            })
            .collect::<Vec<_>>();
        Self { keypairs }
    }
}

impl From<KeyPairsStringWithbalance> for KeyPairs {
    fn from(value: KeyPairsStringWithbalance) -> Self {
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
