use crate::bear::deploy_contracts::{
    multicall3_addr,
    wbera::{self, wbera_addr},
};
use crate::bear::precompile_contracts::dex;

use crate::bear::precompile_contracts::{
    erc20_bank::{self, erc20_bank_addr},
    erc20_dex::{self, erc20_dex_addr},
};
use crate::errors::Error;
use crate::utils::{get_all_keypairs, get_config, get_single_keypairs};
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::U256;
use ethers_core::types::Address;
use ethers_signers::Signer;
use std::time::{SystemTime, UNIX_EPOCH};
use structopt::StructOpt;
use time::OffsetDateTime;

pub mod swap;

// notice must use erc20 dex
#[derive(Debug, StructOpt)]
pub enum Dex {
    Swap(swap::Swap),
}

impl Dex {
    pub async fn run(&self) -> Result<(), Error> {
        match self {
            Dex::Swap(swap) => swap.run().await,
        }
    }
}
