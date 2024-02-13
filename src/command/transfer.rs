use crate::constant::BERA_DECIMAL;
use crate::errors::Error;
use crate::utils::{get_all_keypairs, get_config, get_single_keypairs, send_transaction};
use colored::*;
use ethers::prelude::SignerMiddleware;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers_signers::Signer;
use pbr::ProgressBar;
use std::{thread, time};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Transfer {
    /// chain id
    #[structopt(long)]
    pub chain_id: u64,
    /// keypair file name
    #[structopt(long)]
    pub file_name: String,
    /// is one to more
    #[structopt(long)]
    pub is_one_to_more: bool,
    /// transfer amount
    #[structopt(long)]
    pub amount: f64,
}

impl Transfer {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let single_keypair = get_single_keypairs().map_err(|e| Error::Custom(e.to_string()))?;
        assert!(single_keypair.keypairs.len() == 1);

        let single_keypair = single_keypair.keypairs[0]
            .clone()
            .with_chain_id(self.chain_id);

        let client = SignerMiddleware::new(
            provider.clone(),
            single_keypair.clone().with_chain_id(self.chain_id),
        );

        let balance = provider
            .get_balance(single_keypair.address(), None)
            .await
            .map_err(|e| Error::Custom(e.to_string()))?;

        let native_balance_f64 = balance.as_u128() as f64 / BERA_DECIMAL;

        log::info!(
            "{} have {} Bera",
            format!("{}", single_keypair.address()).blue(),
            native_balance_f64.to_string().red()
        );

        let keypairs = get_all_keypairs(&self.file_name)?;

        if self.is_one_to_more {
            let mut pb = ProgressBar::new(keypairs.keypairs.len() as u64);
            let duration = time::Duration::from_millis(20);
            pb.format("╢▌▌░╟");
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

                let native_balance_f64 = native_balance.as_u128() as f64 / BERA_DECIMAL;

                log::info!(
                    "{} have {} Bera",
                    format!("{}", keypair.address()).blue(),
                    native_balance_f64.to_string().red()
                );

                // check wallet is zero
                if native_balance == U256::zero() {
                    log::info!(
                        "{} have {} Bera",
                        format!("{:?}", keypair.address()).red(),
                        native_balance_f64.to_string().red()
                    );
                    loop {
                        if let Err(e) = send_transaction(
                            &client,
                            single_keypair.address(),
                            keypair.address(),
                            self.amount,
                        )
                        .await
                        {
                            log::warn!("transfer command send_transaction have Err({:?})", e);
                            continue;
                        } else {
                            break;
                        }
                    }

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

                    log::info!(
                        "{} have {} Bera",
                        format!("{}", keypair.address()).blue(),
                        native_balance_f64.to_string().red()
                    );
                }
                pb.inc();
                thread::sleep(duration);
            }
            pb.finish_print("done");
        } else {
            let mut pb = ProgressBar::new(keypairs.keypairs.len() as u64);
            let duration = time::Duration::from_millis(20);
            pb.format("╢▌▌░╟");
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
                let native_balance_f64 = native_balance.as_u128() as f64 / BERA_DECIMAL;

                log::info!(
                    "{} have {} Bera",
                    format!("{}", keypair.address()).blue(),
                    native_balance_f64.to_string().red()
                );

                let client = SignerMiddleware::new(
                    provider.clone(),
                    keypair.clone().with_chain_id(self.chain_id),
                );

                loop {
                    if let Err(_) = send_transaction(
                        &client,
                        keypair.address(),
                        single_keypair.address(),
                        native_balance_f64 * 0.9,
                    )
                    .await
                    {
                        continue;
                    } else {
                        break;
                    }
                }

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

                log::info!(
                    "{} have {} Bera",
                    format!("{}", keypair.address()).blue(),
                    native_balance_f64.to_string().red()
                );
                pb.inc();
                thread::sleep(duration);
            }
            pb.finish_print("done");
        }

        Ok(())
    }
}
