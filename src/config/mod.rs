use crate::errors::Error;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Optionally include your keypair path. Defaults to your Solana CLI config file.
    pub keypair_path: Option<String>,
    /// Optionally include your RPC endpoint. Use "local", "dev", "main" for default endpoints. Defaults to your Solana CLI config file.
    pub rpc_endpoint: Option<String>,
    /// Optionally include a commitment level. Defaults to your Solana CLI config file.
    pub commitment: Option<String>,
}
