use crate::errors::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum NFT {
    /// buy nft
    NftBuy(NftBuy),
    /// check nft have
    NftCheck(NftCheck),
    /// booba buy
    Booba(BoobaOnBera),
    /// Balentines
    Balentines(Balentines),
}

impl NFT {
    pub async fn run(&self) -> Result<(), Error> {
        println!("nft!");
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub struct BoobaOnBera {
    /// wallet file name
    #[structopt(long)]
    pub file_name: String,
}

impl BoobaOnBera {
    pub async fn run(&self) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub struct Balentines {
    /// wallet file name
    #[structopt(long)]
    pub file_name: String,
}

impl Balentines {
    pub async fn run(&self) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub struct NftCheck {
    /// wallet file name
    #[structopt(long)]
    pub file_name: String,
}

impl NftCheck {
    pub async fn run(&self) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub struct NftBuy {
    /// wallet file name
    #[structopt(long)]
    pub file_name: String,
}

impl NftBuy {
    pub async fn run(&self) -> Result<(), Error> {
        Ok(())
    }
}
