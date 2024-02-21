use super::*;
use crate::errors::Error;
use crate::utils::get_all_keypairs_string_with_balance;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct NewStore {
    #[structopt(short, long)]
    pub file_name: String,
}

impl NewStore {
    pub fn run(&self) -> Result<(), Error> {
        let all_keypairs = get_all_keypairs_string_with_balance(&self.file_name)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keyparis_str = KeyPairsString::convert_from_keypairs(all_keypairs);

        let home_path = dirs::home_dir().ok_or(Error::Custom("can't open home dir".into()))?;
        let keypairs_path = home_path
            .join(".config")
            .join("evm-cli")
            .join(format!("{}_convert.json", self.file_name));

        keyparis_str
            .write(keypairs_path.clone())
            .map_err(|e| Error::Custom(e.to_string()))?;

        Ok(())
    }
}
