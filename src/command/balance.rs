use crate::bear::deploy_contracts::{honey, stg_usdc, wbera};
use crate::command::keys::{KeyPairs, KeyPairsString};
use crate::constant::BERA_DECIMAL;
use crate::errors::Error;
use crate::utils::calc_balance;
use crate::utils::{
    get_all_keypairs_string_with_balance, get_config, get_single_keypairs_string_with_balance,
};
use colored::*;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Balance {
    /// Display single wallet balance
    Single,
    /// Display multi wallet balance
    Multi(Multi),
    /// Check
    Check(CheckEmpty),
}

impl Balance {
    pub async fn run(&self) -> Result<(), Error> {
        match self {
            Balance::Single => {
                let config = get_config().map_err(|e| Error::from(e.to_string()))?;

                let provider = Provider::<Http>::try_from(config.rpc_endpoint)
                    .map_err(|e| Error::Custom(e.to_string()))?;

                let single_keypair = get_single_keypairs_string_with_balance()
                    .map_err(|e| Error::Custom(e.to_string()))?;

                for keypair in single_keypair.keypairs {
                    let client = SignerMiddleware::new(
                        provider.clone(),
                        keypair.clone().with_chain_id(config.chain_id),
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

                    let honey_balance_f64 = calc_balance(honey_decimal as u32, honey_balance);

                    let wbera_balance = wbera::balance_of(&client, keypair.address())
                        .await
                        .map_err(|e| Error::Custom(e.to_string()))?;

                    let wbera_decimal = wbera::decimals(&client)
                        .await
                        .map_err(|e| Error::Custom(e.to_string()))?;

                    let wbera_balance_f64 = calc_balance(wbera_decimal as u32, wbera_balance);

                    println!(
                        "{} have ({}) Bera  🍤 {} Wbera 🍤 ({}) Honey 🍤 ",
                        keypair.address().to_string().blue(),
                        native_balance_f64.to_string().red(),
                        wbera_balance_f64.to_string().red(),
                        honey_balance_f64.to_string().blink(),
                    );
                }
                Ok(())
            }
            Balance::Multi(multi) => multi.run().await,
            Balance::Check(check) => check.run().await,
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct Multi {
    /// keypair file name
    #[structopt(short, long)]
    pub file_name: String,
}

impl Multi {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs = get_all_keypairs_string_with_balance(&self.file_name)
            .map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in keypairs.keypairs {
            let client = SignerMiddleware::new(
                provider.clone(),
                keypair.clone().with_chain_id(config.chain_id),
            );

            let mut counter = 0;
            let native_balance = loop {
                if let Ok(v) = provider.get_balance(keypair.address(), None).await {
                    if (v != U256::zero()) & (counter < 3) {
                        break v;
                    } else if counter == 3 {
                        break v;
                    } else {
                        log::warn!("Try {} time", counter.to_string().red());
                        counter += 1;
                        continue;
                    }
                } else {
                    continue;
                }
            };

            let native_balance_f64 = native_balance.as_u128() as f64 / BERA_DECIMAL;

            let honey_balance = loop {
                if let Ok(v) = honey::balance_of(&client, keypair.address()).await {
                    break v;
                } else {
                    continue;
                }
            };

            let honey_decimal = loop {
                if let Ok(v) = honey::decimals(&client).await {
                    break v;
                } else {
                    continue;
                }
            };

            let honey_balance_f64 = calc_balance(honey_decimal as u32, honey_balance);

            let mut counter = 0;
            let wbera_balance = loop {
                if let Ok(v) = wbera::balance_of(&client, keypair.address()).await {
                    if (v != U256::zero()) & (counter < 3) {
                        break v;
                    } else if counter == 3 {
                        break v;
                    } else {
                        log::warn!("Try {} time", counter.to_string().red());
                        counter += 1;
                        continue;
                    }
                } else {
                    continue;
                }
            };

            let wbera_decimal = loop {
                if let Ok(v) = wbera::decimals(&client).await {
                    break v;
                } else {
                    continue;
                }
            };

            let wbera_balance_f64 = calc_balance(wbera_decimal as u32, wbera_balance);

            let mut counter = 0;
            let stg_usdc_balance = loop {
                if let Ok(v) = stg_usdc::balance_of(&client, keypair.address()).await {
                    if (v != U256::zero()) & (counter < 3) {
                        break v;
                    } else if counter == 3 {
                        break v;
                    } else {
                        log::warn!("Try {} time", counter.to_string().red());
                        counter += 1;
                        continue;
                    }
                } else {
                    continue;
                }
            };

            let stg_usdc_decimal = loop {
                if let Ok(v) = stg_usdc::decimals(&client).await {
                    break v;
                } else {
                    continue;
                }
            };

            let stg_usdc_balance_f64 = calc_balance(stg_usdc_decimal as u32, stg_usdc_balance);

            if wbera_balance == U256::zero() {
                println!(
                    "💨💨💨Warn {} Have ({}) Bera 💨💨 ({}) Wbera ({}) 💨💨 Honey 💨💨 {} STGUSDC ",
                    keypair.address().to_string().green(),
                    native_balance_f64.to_string().red(),
                    wbera_balance_f64.to_string().green(),
                    honey_balance_f64.to_string().bright_cyan(),
                    stg_usdc_balance_f64.to_string().bright_red(),
                );
            } else {
                if native_balance == U256::from(5 * BERA_DECIMAL as u128) {
                    println!(
                        "{:?} has ({}) Bera 🍤 {} Wbera ({}) 🍤 Honey 🍤 {} STGUSDC ",
                        keypair.address(),
                        native_balance_f64.to_string().red(),
                        wbera_balance_f64.to_string().bright_purple(),
                        honey_balance_f64.to_string().bright_cyan(),
                        stg_usdc_balance_f64.to_string().bright_red(),
                    );
                } else {
                    println!(
                        "{} has ({}) Bera 🍤 {} Wbera ({}) 🍤 Honey 🍤 {} STGUSDC",
                        keypair.address().to_string().blue(),
                        native_balance_f64.to_string().red(),
                        wbera_balance_f64.to_string().bright_magenta(),
                        honey_balance_f64.to_string().bright_yellow(),
                        stg_usdc_balance_f64.to_string().bright_red()
                    );
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub struct CheckEmpty {
    /// keypair file name
    #[structopt(short, long)]
    pub file_name: String,
}

impl CheckEmpty {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs = get_all_keypairs_string_with_balance(&self.file_name)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let mut no_zero_balance_balance = vec![];
        for keypair in keypairs.keypairs {
            let mut counter = 0;
            let native_balance = loop {
                if let Ok(v) = provider.get_balance(keypair.address(), None).await {
                    if (v != U256::zero()) & (counter < 3) {
                        break v;
                    } else if counter == 3 {
                        break v;
                    } else {
                        log::warn!("Try {} time", counter.to_string().red());
                        counter += 1;
                        continue;
                    }
                } else {
                    continue;
                }
            };

            if native_balance != U256::zero() {
                no_zero_balance_balance.push(keypair);
            } else {
                println!("Address({:?}) native balanze is zero", keypair.address());
            }
        }

        let keypairs_str = KeyPairsString::from(KeyPairs {
            keypairs: no_zero_balance_balance,
        });

        let home_path = dirs::home_dir().ok_or(Error::Custom("can't open home dir".into()))?;
        let keypairs_path = home_path
            .join(".config")
            .join("evm-cli")
            .join(format!("{}_not_zero_balance_keypairs.json", self.file_name));

        keypairs_str
            .write(keypairs_path.clone())
            .map_err(|e| Error::Custom(e.to_string()))?;

        Ok(())
    }
}
