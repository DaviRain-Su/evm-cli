use crate::errors::Error;
use structopt::StructOpt;

pub mod bex;
pub mod liquidity;
pub mod swap;

// notice must use erc20 dex
#[derive(Debug, StructOpt)]
pub enum Dex {
    /// swap token
    Swap(swap::Swap),
    /// liquidity
    Liquidity(liquidity::Liquidity),
    /// bex
    Bex(bex::Swap),
}

impl Dex {
    pub async fn run(&self) -> Result<(), Error> {
        match self {
            Dex::Swap(swap) => swap.run().await,
            Dex::Liquidity(liq) => liq.run().await,
            Dex::Bex(bex) => bex.run().await,
        }
    }
}
