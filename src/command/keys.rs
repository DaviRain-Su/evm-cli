use crate::errors::Error;
use structopt::StructOpt;

pub mod multi;
pub mod new_format;
pub mod new_store;
pub mod single;
pub mod types;
pub use types::*;

#[derive(Debug, StructOpt)]
pub enum Keys {
    /// Single wallet generate or load
    Single(single::Single),
    /// multi wallet generate
    Multi(multi::Multi),
    /// generator new key file
    NewStore(new_store::NewStore),
    /// new format
    NewFormat(new_format::NewFormat),
}

impl Keys {
    pub fn run(&self) -> Result<(), Error> {
        match self {
            Keys::Single(single) => single.run(),
            Keys::Multi(multi) => multi.run(),
            Keys::NewStore(new_store) => new_store.run(),
            Keys::NewFormat(new_format) => new_format.run(),
        }
    }
}
