use crate::errors::Error;
use structopt::StructOpt;

pub mod bank;
pub mod dex;
pub mod epoch;
pub mod honey;
pub mod nft;
pub mod reward;
pub mod swap_usdc;
pub mod wbera;

#[derive(Debug, StructOpt)]
pub enum Bera {
    /// Bera Chain Bank module
    Bank(bank::Bank),
    /// Bera Chain epoch module
    Epoch(epoch::Epoch),
    /// Bera Chain Dex module
    Dex(dex::Dex),
    /// Bera chain batch buy nft
    NFT(nft::NFT),
    /// Bera chain Honey module
    Honey(honey::Honey),
    /// Bera chain Wbera module
    WBera(wbera::WBera),
    /// reward
    Reward(reward::Reward),
    /// Mint honey
    SwapUSDC(swap_usdc::SwapUSDC),
}

impl Bera {
    pub async fn run(&self) -> Result<(), Error> {
        match self {
            Bera::Bank(bank) => bank.run().await,
            Bera::Epoch(epochs) => epochs.run().await,
            Bera::Dex(dex) => dex.run().await,
            Bera::NFT(nft) => nft.run().await,
            Bera::Honey(honey) => honey.run().await,
            Bera::WBera(wbera) => wbera.run().await,
            Bera::Reward(reward) => reward.run().await,
            Bera::SwapUSDC(mint) => mint.run().await,
        }
    }
}
