use crate::bear::deploy_contracts::wbera;
use crate::constant::BERA_DECIMAL;
use crate::errors::Error;
use crate::utils::{get_all_keypairs, get_config};
use colored::*;
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::U256;
use ethers_signers::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum WBera {
    /// withdraw wbera to bera
    Withdraw(Withdraw),
    /// deposit bera to wbera
    Deposit(Deposit),
}

impl WBera {
    pub async fn run(&self) -> Result<(), Error> {
        match self {
            WBera::Withdraw(wbera) => wbera.run().await,
            WBera::Deposit(deposit) => deposit.run().await,
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct Deposit {
    /// chain id
    #[structopt(long)]
    pub chain_id: u64,
    /// wallet file name
    #[structopt(long)]
    pub file_name: String,
    /// amount
    #[structopt(long)]
    pub amount: f64,
}

impl Deposit {
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
                        println!("Try {} time", counter.to_string().red());
                        counter += 1;
                        continue;
                    }
                } else {
                    continue;
                }
            };

            let native_balance_f64 = native_balance.as_u128() as f64 / BERA_DECIMAL;

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

            if (self.amount > native_balance_f64) && (wbera_balance == U256::zero()) {
                panic!(
                    "You Input amount({}) > {} have balance({})",
                    self.amount,
                    keypair.address(),
                    native_balance_f64
                );
            }
            println!(
                "deposit Before {} has {} Bera",
                format!("{}", keypair.address()).blue(),
                native_balance_f64
            );

            // check wbera balance
            if wbera_balance == U256::zero() {
                let deposit_value = U256::from((self.amount * BERA_DECIMAL) as u128);

                println!(
                    "{} has deposit {} Bera",
                    format!("{}", keypair.address()).blue(),
                    self.amount
                );

                let deposit_result = loop {
                    if let Err(e) = wbera::deposit(&client, deposit_value).await {
                        log::warn!("Warn: {:?}", e.to_string());
                        continue;
                    } else {
                        break;
                    }
                };
                println!("deposit_result: {:?}", deposit_result);

                let mut counter = 0;
                let native_balance = loop {
                    if let Ok(v) = provider.get_balance(keypair.address(), None).await {
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
                let native_balance_f64 = native_balance.as_u128() as f64 / BERA_DECIMAL;

                println!(
                    "Deposit After {} has {} Bera",
                    format!("{}", keypair.address()).blue(),
                    native_balance_f64
                );
            }
        }
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub struct Withdraw {
    /// chain id
    #[structopt(long)]
    pub chain_id: u64,
    /// wallet file name
    #[structopt(long)]
    pub file_name: String,
}

impl Withdraw {
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

            let balance = wbera::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            let native_balance_f64 = balance.as_u128() as f64 / BERA_DECIMAL;

            println!(
                "Withdraw before {} has {} Bera",
                format!("{}", keypair.address()).blue(),
                native_balance_f64
            );

            let withdraw_half_balance = balance / 2;

            let withdraw_half_balance_f64 = withdraw_half_balance.as_u128() as f64 / BERA_DECIMAL;

            println!(
                "{} has withdraw {} Bera",
                format!("{}", keypair.address()).blue(),
                withdraw_half_balance_f64
            );

            let withdra_result = wbera::withdraw(&client, withdraw_half_balance)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!("withdra_result: {:?}", withdra_result);

            let balance = wbera::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;
            let native_balance_f64 = balance.as_u128() as f64 / BERA_DECIMAL;

            println!(
                "Withdraw {} has {} Wbera",
                format!("{:?}", keypair.address()).blue(),
                native_balance_f64
            );
        }

        Ok(())
    }
}
