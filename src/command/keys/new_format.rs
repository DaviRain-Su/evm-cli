use super::*;
use crate::errors::Error;
use crate::utils::get_all_keypairs_string;
use structopt::StructOpt;

/// NewFormat
#[derive(Debug, StructOpt)]
pub struct NewFormat {
    #[structopt(short, long)]
    pub file_name: String,
}

impl NewFormat {
    pub fn run(&self) -> Result<(), Error> {
        let all_keypairs =
            get_all_keypairs_string(&self.file_name).map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs_str = KeyPairsString::from(all_keypairs);
        let keypairs_str_with_balance = KeyPairsStringWithbalance::from(keypairs_str);

        let home_path = dirs::home_dir().ok_or(Error::Custom("can't open home dir".into()))?;
        let keypairs_path = home_path
            .join(".config")
            .join("evm-cli")
            .join(format!("{}_new_format.json", self.file_name));

        keypairs_str_with_balance
            .write(keypairs_path.clone())
            .map_err(|e| Error::Custom(e.to_string()))?;

        Ok(())
    }
}
