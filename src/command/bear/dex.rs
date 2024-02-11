use crate::errors::Error;
use structopt::StructOpt;

pub mod liquidity;
pub mod swap;

// notice must use erc20 dex
#[derive(Debug, StructOpt)]
pub enum Dex {
    /// swap token
    Swap(swap::Swap),
    /// liquidity
    Liquidity(liquidity::Liquidity),
}

impl Dex {
    pub async fn run(&self) -> Result<(), Error> {
        match self {
            Dex::Swap(swap) => swap.run().await,
            Dex::Liquidity(liq) => liq.run().await,
        }
    }
}
