use crate::errors::Error;
use structopt::StructOpt;

pub mod auto;
pub mod balance;
pub mod keys;
pub mod transfer;
use crate::command::auto::Auto;
use crate::command::keys::Generator;

#[derive(Debug, StructOpt)]
pub enum Command {
    /// auto generate config.toml file to ~/.config/pomm/config.toml
    #[structopt(name = "auto")]
    Auto(Auto),
    /// generate new keypair
    #[structopt(name = "generator")]
    Generator(Generator),
    /// get balance
    #[structopt(name = "balance")]
    Balance(balance::Balance),
}

impl Command {
    pub async fn run(&self) -> Result<(), Error> {
        match &self {
            Command::Auto(auto) => auto.run(),
            Command::Generator(generator) => generator.run(),
            Command::Balance(balance) => balance.run().await,
        }
    }
}
