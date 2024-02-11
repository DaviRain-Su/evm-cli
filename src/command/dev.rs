use crate::errors::Error;
use ethers::types::U256;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Dev {}

impl Dev {
    pub async fn run(&self) -> Result<(), Error> {
        let amount_in_wei: U256 = U256::from_dec_str("1000000000000000000").unwrap(); // 1 Ether in Wei
        let amount_in_eth: f64 = amount_in_wei.as_u128() as f64 / 1e18; // Convert Wei to Ether

        println!("Amount in Wei: {}", amount_in_wei);
        println!("Amount in Ether: {} eth", amount_in_eth);
        Ok(())
    }
}
