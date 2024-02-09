use crate::errors::Error;
use structopt::StructOpt;

pub mod bank;

#[derive(Debug, StructOpt)]
pub enum Bear {
    Bank(bank::Bank),
}

impl Bear {
    pub async fn run(&self) -> Result<(), Error> {
        match self {
            Bear::Bank(bank) => bank.run().await,
        }
    }
}
