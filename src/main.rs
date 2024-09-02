// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(dead_code)]

use ethers::prelude::SignerMiddleware;
use ethers::prelude::Wallet;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use structopt::StructOpt;

pub mod bear;
pub mod command;
pub mod config;
pub mod constant;
pub mod errors;
pub mod incrementer;
pub mod utils;

type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

use crate::command::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let opt = Command::from_args();
    opt.run().await?;

    Ok(())
}
