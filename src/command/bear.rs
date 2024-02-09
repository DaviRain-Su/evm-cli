use crate::errors::Error;
use structopt::StructOpt;

pub mod bank;
pub mod dex;
pub mod epoch;

#[derive(Debug, StructOpt)]
pub enum Bear {
    Bank(bank::Bank),
    Epoch(epoch::Epoch),
    Dex(dex::Dex),
}

impl Bear {
    pub async fn run(&self) -> Result<(), Error> {
        match self {
            Bear::Bank(bank) => bank.run().await,
            Bear::Epoch(epochs) => epochs.run().await,
            Bear::Dex(dex) => dex.run().await,
        }
    }
}
