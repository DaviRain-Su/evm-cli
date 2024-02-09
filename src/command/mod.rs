use crate::errors::Error;
use structopt::StructOpt;

pub mod auto;
pub mod generator;
use crate::command::auto::Auto;
use crate::command::generator::Generator;

#[derive(Debug, StructOpt)]
pub enum Command {
    /// auto generate config.toml file to ~/.config/pomm/config.toml
    #[structopt(name = "auto")]
    Auto(Auto),
    /// generate new keypair
    #[structopt(name = "generator")]
    Generator(Generator),
}

impl Command {
    pub async fn run(&self) -> Result<(), Error> {
        match &self {
            Command::Auto(auto) => auto.run(),
            Command::Generator(generator) => generator.run(),
        }
    }
}
