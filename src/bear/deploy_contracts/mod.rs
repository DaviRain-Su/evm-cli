// deploy contract
pub mod erc20_bribe;
pub mod honey;
pub mod wbera;

use std::str::FromStr;

use ethers::types::H160;
pub use honey::*;

pub fn erc20_bribe_addr() -> H160 {
    H160::from_str("0x1BbACf6bA66A20CD8ad98c70EAC4ea7AaD45c3E9").unwrap()
}

pub fn erc20_honey_addr() -> H160 {
    H160::from_str("0x09ec711b81cD27A6466EC40960F2f8D85BB129D9").unwrap()
}

pub fn honey_token_addr() -> H160 {
    H160::from_str("0x7EeCA4205fF31f947EdBd49195a7A88E6A91161B").unwrap()
}

pub fn multicall3_addr() -> H160 {
    H160::from_str("0x9d1dB8253105b007DDDE65Ce262f701814B91125").unwrap()
}

pub fn wbera_addr() -> H160 {
    H160::from_str("0x5806E416dA447b267cEA759358cF22Cc41FAE80F").unwrap()
}

pub fn wbtc_addr() -> H160 {
    H160::from_str("0x9DAD8A1F64692adeB74ACa26129e0F16897fF4BB").unwrap()
}

pub fn weth_addr() -> H160 {
    H160::from_str("0x8239FBb3e3D0C2cDFd7888D8aF7701240Ac4DcA4").unwrap()
}

pub fn bgt_balance_token_addr() -> H160 {
    H160::from_str("0xAcD97aDBa1207dCf27d5C188455BEa8a32E80B8b").unwrap()
}
