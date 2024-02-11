use crate::errors::Error;
use structopt::StructOpt;

pub mod auto;
pub mod balance;
pub mod bear;
pub mod deploy_sc;
pub mod dev;
pub mod keys;
pub mod transfer;

#[derive(Debug, StructOpt)]
pub enum Command {
    /// auto generate config.toml file to ~/.config/evm-cli/config.toml
    #[structopt(name = "auto")]
    Auto(auto::Auto),
    /// generate new keypair, support single or multi keypair generate and load single keypair
    #[structopt(name = "generator")]
    Generator(keys::Generator),
    /// Display single or multi Wallet Balance
    #[structopt(name = "balance")]
    Balance(balance::Balance),
    /// Support batch transfer
    #[structopt(name = "transfer")]
    Transfer(transfer::Transfer),
    /// Now support deploy incrementer smart contract
    #[structopt(name = "deploy-sc")]
    Deploy(deploy_sc::DeploySc),
    /// bera module
    #[structopt(name = "bear")]
    Bera(bear::Bera),
    /// dev for test
    #[structopt(name = "dev")]
    Dev(dev::Dev),
}

impl Command {
    pub async fn run(&self) -> Result<(), Error> {
        match &self {
            Command::Auto(auto) => auto.run(),
            Command::Generator(generator) => generator.run(),
            Command::Balance(balance) => balance.run().await,
            Command::Transfer(trnasfer) => trnasfer.run().await,
            Command::Deploy(deploy_sc) => deploy_sc.run().await,
            Command::Bera(bera) => bera.run().await,
            Command::Dev(dev) => dev.run().await,
        }
    }
}
