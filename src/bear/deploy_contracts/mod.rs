// deploy contract
pub mod erc20_bribe;
pub mod honey;
pub mod wbera;
pub mod wbtc;
pub mod weth;

use std::str::FromStr;

use ethers::types::H160;
pub use honey::*;

pub fn erc20_honey_addr() -> H160 {
    H160::from_str("0x09ec711b81cD27A6466EC40960F2f8D85BB129D9").unwrap()
}

pub fn multicall3_addr() -> H160 {
    H160::from_str("0x9d1dB8253105b007DDDE65Ce262f701814B91125").unwrap()
}

pub fn bgt_balance_token_addr() -> H160 {
    H160::from_str("0xAcD97aDBa1207dCf27d5C188455BEa8a32E80B8b").unwrap()
}
