use ethers::types::H160;
use std::str::FromStr;

pub mod bank;
pub mod berachef;
pub mod bgt;
pub mod bribe;
pub mod dex;
pub mod distribution;
pub mod erc20_bank;
pub mod erc20_dex;
pub mod governance;
pub mod honey;
pub mod oracle;
pub mod rewards;
pub mod staking;

pub fn staking_addr() -> H160 {
    H160::from_str("0xd9A998CaC66092748FfEc7cFBD155Aae1737C2fF").unwrap()
}
