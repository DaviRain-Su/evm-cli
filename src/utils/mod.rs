use super::Client;
use crate::command::generator::{KeyPairs, KeyPairsString};
use crate::config::Config;
use crate::errors::Error;
use ethers::{prelude::*, utils as ethers_utils};

pub fn get_config() -> Result<Config, Error> {
    let home_path = dirs::home_dir().ok_or(Error::Custom("can't open home dir".to_string()))?;
    let pomm_config_path = home_path.join(".config").join("evm-cli");
    let config_path = pomm_config_path.join("config.toml");

    let config_str = std::fs::read_to_string(config_path)
        .map_err(|e| Error::Custom(format!("read file content failed: Error({})", e)))?;
    let phoneix_config: Config = toml::from_str(&config_str)
        .map_err(|e| Error::Custom(format!("parse toml failed: Error({})", e)))?;

    Ok(phoneix_config)
}

pub fn get_all_keypairs(file_name: &str) -> Result<KeyPairs, Error> {
    let home_path = dirs::home_dir().ok_or(Error::Custom("can't open home dir".to_string()))?;
    let pomm_config_path = home_path.join(".config").join("evm-cli");
    let config_path = pomm_config_path.join(format!("{}_keypairs.json", file_name));
    log::info!("keypairs path: {:?}", config_path);
    let keypairs_str = KeyPairsString::read(config_path).map_err(|e| {
        let location = std::panic::Location::caller();
        Error::from(format!("Error({}): {})", location, e.to_string()))
    })?;
    let keypairs = KeyPairs::from(keypairs_str);
    Ok(keypairs)
}

// 1. Create an asynchronous function that takes a provider reference and from and to address as input
pub async fn print_balances(
    provider: &Provider<Http>,
    address_from: Address,
    address_to: Address,
) -> Result<(), Box<dyn std::error::Error>> {
    // 2. Use the get_balance function
    let balance_from = provider.get_balance(address_from, None).await?;
    let balance_to = provider.get_balance(address_to, None).await?;

    // 3. Print the resultant balance
    println!("{} has {}", address_from, balance_from);
    println!("{} has {}", address_to, balance_to);

    Ok(())
}

// 1. Define an asynchronous function that takes a client provider and the from and to addresses as input
pub async fn send_transaction(
    client: &Client,
    address_from: Address,
    address_to: Address,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Beginning transfer of 1 native currency from {} to {}.",
        address_from, address_to
    );

    // 2. Create a TransactionRequest object
    let tx = TransactionRequest::new()
        .to(address_to)
        .value(U256::from(ethers_utils::parse_ether(1)?))
        .from(address_from);

    // 3. Send the transaction with the client
    let tx = client.send_transaction(tx, None).await?.await?;

    // 4. Print out the result
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

#[test]
fn test_get_config() {
    let config = get_config().unwrap();
    println!("config: {:?}", config);
}
