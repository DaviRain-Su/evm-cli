use crate::errors::Error;
use structopt::StructOpt;

pub mod bank;
pub mod dex;
pub mod epoch;
pub mod honey;
pub mod nft;

#[derive(Debug, StructOpt)]
pub enum Bear {
    Bank(bank::Bank),
    Epoch(epoch::Epoch),
    Dex(dex::Dex),
    NFT(nft::NFT),
    Honey(honey::Honey),
}

impl Bear {
    pub async fn run(&self) -> Result<(), Error> {
        match self {
            Bear::Bank(bank) => bank.run().await,
            Bear::Epoch(epochs) => epochs.run().await,
            Bear::Dex(dex) => dex.run().await,
            Bear::NFT(nft) => nft.run().await,
            Bear::Honey(honey) => honey.run().await,
        }
    }
}
