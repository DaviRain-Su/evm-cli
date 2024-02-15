use crate::bear::deploy_contracts::{honey, wbera};
use crate::bear::nft::balentines;
use crate::bear::nft::lunar_new_year;
use crate::command::keys::{KeyPairs, KeyPairsString};
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
    /// Check
    Check(CheckEmpty),
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

                    let balentines_balance = balentines::balance_of(&client, keypair.address())
                        .await
                        .map_err(|e| Error::Custom(e.to_string()))?;

                    let mut counter = 0;
                    let wbera_balance = loop {
                        if let Ok(v) = wbera::balance_of(&client, keypair.address()).await {
                            if (v != U256::zero()) & (counter < 3) {
                                break v;
                            } else if counter == 3 {
                                break v;
                            } else {
                                println!("Try {} time", counter.to_string().red());
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

                    let exponent: u32 = wbera_decimal as u32; // 自定义指数值
                    let divisor: u128 = 10u128.pow(exponent); // 计算除数
                    let wbera_balance_f64 = wbera_balance.as_u128() as f64 / divisor as f64;

                    println!(
                        "{} have ({}) Bera  🍤 {} Wbera 🍤 ({}) Honey 🍤 ({}) Lunar New Year NFT 🍤 {} Balentines",
                        keypair.address().to_string().blue(),
                        native_balance_f64.to_string().red(),
                        wbera_balance_f64.to_string().red(),
                        honey_balance_f64.to_string().blink(),
                        lunar_new_year_balance.to_string().green(),
                        balentines_balance.to_string().bright_yellow()
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

            let exponent: u32 = honey_decimal as u32; // 自定义指数值
            let divisor: u128 = 10u128.pow(exponent); // 计算除数
            let honey_balance_f64 = honey_balance.as_u128() as f64 / divisor as f64;

            let lunar_new_year_balance = loop {
                if let Ok(v) = lunar_new_year::balance_of(&client, keypair.address()).await {
                    break v;
                } else {
                    continue;
                }
            };

            let balentine_balance = loop {
                if let Ok(v) = balentines::balance_of(&client, keypair.address()).await {
                    break v;
                } else {
                    continue;
                }
            };

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

            let exponent: u32 = wbera_decimal as u32; // 自定义指数值
            let divisor: u128 = 10u128.pow(exponent); // 计算除数
            let wbera_balance_f64 = wbera_balance.as_u128() as f64 / divisor as f64;

            if wbera_balance == U256::zero() {
                println!(
                    "💨💨💨Warn {} Have ({}) Bera 💨💨 ({}) Wbera ({}) 💨💨 Honey  💨💨 ({}) Lunar New Year NFT  💨💨 {} balentine ",
                    keypair.address().to_string().green(),
                    native_balance_f64.to_string().red(),
                    wbera_balance_f64.to_string().green(),
                    honey_balance_f64.to_string().bright_cyan(),
                    lunar_new_year_balance.to_string().green(),
                    balentine_balance.to_string().bright_blue()
                );
            } else {
                if native_balance == U256::from(5 * BERA_DECIMAL as u128) {
                    println!(
                        "{:?} has ({}) Bera 🍤 {} Wbera ({}) 🍤 Honey 🍤 ({}) Lunar New Year NFT 🍤 {} balentine ",
                        keypair.address(),
                        native_balance_f64.to_string().red(),
                        wbera_balance_f64.to_string().bright_purple(),
                        honey_balance_f64.to_string().bright_cyan(),
                        lunar_new_year_balance.to_string().green(),
                        balentine_balance.to_string().bright_blue()
                    );
                } else {
                    println!(
                        "{} has ({}) Bera 🍤 {} Wbera ({}) 🍤 Honey 🍤 ({}) Lunar New Year NFT 🍤 {} balentine",
                        keypair.address().to_string().blue(),
                        native_balance_f64.to_string().red(),
                        wbera_balance_f64.to_string().bright_magenta(),
                        honey_balance_f64.to_string().bright_yellow(),
                        lunar_new_year_balance.to_string().green(),
                        balentine_balance.to_string().bright_blue()
                    );
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub struct CheckEmpty {
    #[structopt(long)]
    pub chain_id: u64,
    /// keypair file name
    #[structopt(short, long)]
    pub file_name: String,
}

impl CheckEmpty {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs =
            get_all_keypairs(&self.file_name).map_err(|e| Error::Custom(e.to_string()))?;

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
