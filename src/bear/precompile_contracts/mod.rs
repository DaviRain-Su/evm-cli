use ethers::types::H160;
use std::str::FromStr;

pub mod bank;
pub mod berachef;
pub mod bgt;
pub mod bribe;
pub mod dex;
pub mod distribution;
pub mod epochs;
pub mod erc20_bank;
pub mod erc20_dex;
pub mod governance;

pub fn bank_addr() -> H160 {
    H160::from_str("0x4381dC2aB14285160c808659aEe005D51255adD7").unwrap()
}

pub fn berachef_addr() -> H160 {
    H160::from_str("0x888AF53B67D1698E04B2B9A9406AF0FFEB2EF05E").unwrap()
}

pub fn bgt_addr() -> H160 {
    H160::from_str("0x09E585D2BDEB5ECF90ADE67DCE1125070D2714A3").unwrap()
}

pub fn bribe_addr() -> H160 {
    H160::from_str("0xFCE07324E0E72E071842374E9997CF65DF990CBC").unwrap()
}

pub fn dex_addr() -> H160 {
    H160::from_str("0x9D0FBF9349F646F1435072F2B0212084752EF460").unwrap()
}

pub fn distribution_addr() -> H160 {
    H160::from_str("0x0000000000000000000000000000000000000069").unwrap()
}

pub fn epochs_addr() -> H160 {
    H160::from_str("0x612Dd8a861161819A4AD8F6f3E2A0567602877c0").unwrap()
}

pub fn erc20_bank_addr() -> H160 {
    H160::from_str("0x0000000000000000000000000000000000696969").unwrap()
}

pub fn erc20_dex_addr() -> H160 {
    H160::from_str("0x0D5862FDBDD12490F9B4DE54C236CFF63B038074").unwrap()
}

pub fn governance_addr() -> H160 {
    H160::from_str("0x7b5Fe22B5446f7C62Ea27B8BD71CeF94e03f3dF2").unwrap()
}

pub fn honey_addr() -> H160 {
    H160::from_str("0xA55E2E3846A51F6AD0ABFDFBDEA2BA0E5E0C76B5").unwrap()
}

pub fn oracle_addr() -> H160 {
    H160::from_str("0x9202Af6Ce925b26AE6B25aDfff0B2705147e195F").unwrap()
}

pub fn rewards_addr() -> H160 {
    H160::from_str("0x55684E2CA2BACE0ADC512C1AFF880B15B8EA7214").unwrap()
}

pub fn staking_addr() -> H160 {
    H160::from_str("0xd9A998CaC66092748FfEc7cFBD155Aae1737C2fF").unwrap()
}
