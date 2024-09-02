use serde::Deserialize;

/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// rpc endpoint
    pub rpc_endpoint: String,
    /// chain id
    pub chain_id: u64,
}
