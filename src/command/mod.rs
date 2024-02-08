use structopt::StructOpt;

pub mod auto;
use crate::command::auto::Auto;

#[derive(Debug, StructOpt)]
pub enum Command {
    /// auto generate config.toml file to ~/.config/pomm/config.toml
    #[structopt(name = "auto")]
    Auto(Auto),
}
