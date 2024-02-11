use crate::bear::deploy_contracts::honey::{self, honey_token_addr};
use crate::bear::nft::lunar_new_year;
use crate::constant::BERA_DECIMAL;
use crate::errors::Error;
use crate::utils::{get_all_keypairs, get_config, get_single_keypairs};
use colored::*;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Balance {
    /// Display single wallet balance
    Single { chain_id: u64 },
    /// Display multi wallet balance
    Multi(Multi),
}

impl Balance {
    pub async fn run(&self) -> Result<(), Error> {
        match self {
            Balance::Single { chain_id } => {
                let config = get_config().map_err(|e| Error::from(e.to_string()))?;

                let provider = Provider::<Http>::try_from(config.rpc_endpoint)
                    .map_err(|e| Error::Custom(e.to_string()))?;

                let single_keypair =
                    get_single_keypairs().map_err(|e| Error::Custom(e.to_string()))?;

                for keypair in single_keypair.keypairs {
                    let client = SignerMiddleware::new(
                        provider.clone(),
                        keypair.clone().with_chain_id(*chain_id),
                    );

                    let native_balance = provider
                        .get_balance(keypair.address(), None)
                        .await
                        .map_err(|e| Error::Custom(e.to_string()))?;
                    let native_balance_f64 = native_balance.as_u128() as f64 / BERA_DECIMAL;

                    let honey_decimal = honey::decimals(&client)
                        .await
                        .map_err(|e| Error::Custom(e.to_string()))?;

                    let honey_balance = honey::balance_of(&client, keypair.address())
                        .await
                        .map_err(|e| Error::Custom(e.to_string()))?;

                    let exponent: u32 = honey_decimal as u32; // 自定义指数值
                    let divisor: u128 = 10u128.pow(exponent); // 计算除数
                    let honey_balance_f64 = honey_balance.as_u128() as f64 / divisor as f64;

                    let lunar_new_year_balance =
                        lunar_new_year::balance_of(&client, keypair.address())
                            .await
                            .map_err(|e| Error::Custom(e.to_string()))?;
                    println!(
                        "{} has ({}) Bera and ({}) Honey And ({}) Lunar New Year NFT",
                        format!("{}", keypair.address()).blue(),
                        native_balance_f64.to_string().red(),
                        honey_balance_f64.to_string().blink(),
                        lunar_new_year_balance.to_string().bold()
                    );
                }
                Ok(())
            }
            Balance::Multi(multi) => multi.run().await,
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct Multi {
    #[structopt(long)]
    pub chain_id: u64,
    /// keypair file name
    #[structopt(short, long)]
    pub file_name: String,
}

impl Multi {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs =
            get_all_keypairs(&self.file_name).map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in keypairs.keypairs {
            let client = SignerMiddleware::new(
                provider.clone(),
                keypair.clone().with_chain_id(self.chain_id),
            );

            let native_balance = provider
                .get_balance(keypair.address(), None)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            let native_balance_f64 = native_balance.as_u128() as f64 / BERA_DECIMAL;

            let honey_balance = honey::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            let honey_decimal = honey::decimals(&client)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            let exponent: u32 = honey_decimal as u32; // 自定义指数值
            let divisor: u128 = 10u128.pow(exponent); // 计算除数
            let honey_balance_f64 = honey_balance.as_u128() as f64 / divisor as f64;

            let lunar_new_year_balance = lunar_new_year::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!(
                "{} has ({}) Bera and ({}) Honey And ({}) Lunar New Year NFT",
                format!("{}", keypair.address()).blue(),
                native_balance_f64.to_string().red(),
                honey_balance_f64.to_string().blink(),
                lunar_new_year_balance.to_string().bold()
            );
        }
        Ok(())
    }
}
