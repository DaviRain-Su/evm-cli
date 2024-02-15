use crate::bear::deploy_contracts::honey::{self};
use crate::bear::nft::balentines::{self, balentines_addr};
use crate::bear::nft::booba_on_bera::{self};
use crate::bear::nft::lunar_new_year::{self, lunar_new_year_addr};
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
pub enum NFT {
    /// buy nft
    NftBuy(NftBuy),
    /// check nft have
    NftCheck(NftCheck),
    /// booba buy
    Booba(BoobaOnBera),
    /// Balentines
    Balentines(Balentines),
}

impl NFT {
    pub async fn run(&self) -> Result<(), Error> {
        match self {
            NFT::NftBuy(buy) => buy.run().await,
            NFT::NftCheck(check) => check.run().await,
            NFT::Booba(booba) => booba.run().await,
            NFT::Balentines(balentines) => balentines.run().await,
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct BoobaOnBera {
    /// chain id
    #[structopt(long)]
    pub chain_id: u64,
    /// wallet file name
    #[structopt(long)]
    pub file_name: String,
}

impl BoobaOnBera {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs =
            get_all_keypairs(&self.file_name).map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in &keypairs.keypairs {
            let client = SignerMiddleware::new(
                provider.clone(),
                keypair.clone().with_chain_id(self.chain_id),
            );

            let native_token_balance = provider
                .get_balance(keypair.address(), None)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            let native_balance_f64 = native_token_balance.as_u128() as f64 / BERA_DECIMAL;

            println!(
                "{} have {} Bera",
                keypair.address().to_string().blue(),
                native_balance_f64.to_string().red()
            );

            let erc20_total_supply = booba_on_bera::erc_20_total_supply(&client)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;
            println!("Erc20 Total supply {}", erc20_total_supply);

            let booba_decimal = booba_on_bera::decimal(&client)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;
            println!("booba_decimal {}", booba_decimal);

            let exponent: u32 = booba_decimal as u32; // 自定义指数值
            let divisor: u128 = 10u128.pow(exponent); // 计算除数
            let mint_num = U256::from(50 * divisor);

            let mut counter = 0;
            let booba_result = loop {
                if let Err(e) = booba_on_bera::booba_mint(&client, mint_num, true).await {
                    if counter == 3 {
                        break;
                    } else {
                        counter += 1;
                        log::warn!("Warn : {:?}", e.to_string());
                        continue;
                    }
                } else {
                    break;
                }
            };

            println!("booba_result: {:?}", booba_result);

            let balance = booba_on_bera::erc_721_balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!(
                "Address({}) have {} Booba On Bera NFT",
                keypair.address().to_string().blue(),
                balance
            );
        }

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub struct Balentines {
    /// chain id
    #[structopt(long)]
    pub chain_id: u64,
    /// wallet file name
    #[structopt(long)]
    pub file_name: String,
}

impl Balentines {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs =
            get_all_keypairs(&self.file_name).map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in &keypairs.keypairs {
            let client = SignerMiddleware::new(
                provider.clone(),
                keypair.clone().with_chain_id(self.chain_id),
            );

            let native_token_balance = provider
                .get_balance(keypair.address(), None)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            let honey_balance = honey::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            let honey_decimal = honey::decimals(&client)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            let exponent: u32 = honey_decimal as u32; // 自定义指数值
            let divisor: u128 = 10u128.pow(exponent); // 计算除数
            let honey_balance_f64 = honey_balance.as_u128() as f64 / divisor as f64;

            let total_supply = lunar_new_year::total_supply(&client)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;
            println!("Total supply {}", total_supply);

            let balentines_balance = balentines::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            if balentines_balance < U256::from(1u64)
                && native_token_balance > U256::zero()
                && honey_balance >= U256::from((2.2 * divisor as f64) as u128)
            {
                println!(
                    "Address({}) have {} Balentines",
                    keypair.address().to_string().blue(),
                    balentines_balance.to_string().bright_red()
                );
                let honey_balance = honey::balance_of(&client, keypair.address())
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                let exponent: u32 = honey_decimal as u32; // 自定义指数值
                let divisor: u128 = 10u128.pow(exponent); // 计算除数
                let honey_balance_f64 = honey_balance.as_u128() as f64 / divisor as f64;

                println!(
                    "Address({}) have {} honey num",
                    keypair.address().to_string().blue(),
                    honey_balance_f64
                );

                let approve_result = honey::approve(&client, balentines_addr(), honey_balance)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                println!("approve_result: {:?}", approve_result);

                loop {
                    if let Err(e) = balentines::buy(&client).await {
                        log::warn!("Warn : {:?}", e.to_string());
                        continue;
                    } else {
                        break;
                    }
                }

                println!("buy_result: {:?}", approve_result);

                let balance = balentines::balance_of(&client, keypair.address())
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                println!(
                    "Address({}) have {} Balentines",
                    keypair.address().to_string().blue(),
                    balance
                );

                let honey_balance = honey::balance_of(&client, keypair.address())
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                let honey_decimal = honey::decimals(&client)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                let exponent: u32 = honey_decimal as u32; // 自定义指数值
                let divisor: u128 = 10u128.pow(exponent); // 计算除数
                let honey_balance_f64 = honey_balance.as_u128() as f64 / divisor as f64;

                println!(
                    "Address({}) have {} honey",
                    keypair.address().to_string().red(),
                    honey_balance_f64.to_string().bright_blue()
                );
            } else {
                println!(
                    "Address({}) have {} Balentines",
                    keypair.address().to_string().red(),
                    balentines_balance
                );
                println!(
                    "Address({}) have {} Bera",
                    keypair.address().to_string().red(),
                    native_token_balance
                );
                println!(
                    "Address({}) have {} Honey",
                    keypair.address().to_string().red(),
                    honey_balance_f64
                );
            }
        }

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub struct NftCheck {
    /// chain id
    #[structopt(long)]
    pub chain_id: u64,
    /// wallet file name
    #[structopt(long)]
    pub file_name: String,
}

impl NftCheck {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs =
            get_all_keypairs(&self.file_name).map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in &keypairs.keypairs {
            let client = SignerMiddleware::new(
                provider.clone(),
                keypair.clone().with_chain_id(self.chain_id),
            );

            let lunar_new_year_balance = lunar_new_year::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            println!(
                "Address({}) have {} number Lunar New Year NFT",
                keypair.address().to_string().red(),
                lunar_new_year_balance
            );
        }

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub struct NftBuy {
    /// chain id
    #[structopt(long)]
    pub chain_id: u64,
    /// wallet file name
    #[structopt(long)]
    pub file_name: String,
}

impl NftBuy {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs =
            get_all_keypairs(&self.file_name).map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in &keypairs.keypairs {
            let client = SignerMiddleware::new(
                provider.clone(),
                keypair.clone().with_chain_id(self.chain_id),
            );

            let native_token_balance = provider
                .get_balance(keypair.address(), None)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            let honey_balance = honey::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            let honey_decimal = honey::decimals(&client)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            let exponent: u32 = honey_decimal as u32; // 自定义指数值
            let divisor: u128 = 10u128.pow(exponent); // 计算除数
            let honey_balance_f64 = honey_balance.as_u128() as f64 / divisor as f64;

            let total_supply = lunar_new_year::total_supply(&client)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;
            println!("Total supply {}", total_supply);

            let lunar_new_year_balance = lunar_new_year::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            if lunar_new_year_balance < U256::from(2u64)
                && native_token_balance > U256::zero()
                && honey_balance >= U256::from(2_000_000_000_000_000_000u64)
            {
                println!(
                    "Address({}) have {} Lunar New Year NFT",
                    keypair.address().to_string().blue(),
                    lunar_new_year_balance
                );
                let honey_balance = honey::balance_of(&client, keypair.address())
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                let exponent: u32 = honey_decimal as u32; // 自定义指数值
                let divisor: u128 = 10u128.pow(exponent); // 计算除数
                let honey_balance_f64 = honey_balance.as_u128() as f64 / divisor as f64;

                println!(
                    "Address({}) have {} honey num",
                    keypair.address().to_string().blue(),
                    honey_balance_f64
                );

                let approve_result = honey::approve(&client, lunar_new_year_addr(), honey_balance)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                println!("approve_result: {:?}", approve_result);

                loop {
                    if let Err(e) = lunar_new_year::buy(&client).await {
                        log::warn!("Warn : {:?}", e.to_string());
                        continue;
                    } else {
                        break;
                    }
                }

                println!("buy_result: {:?}", approve_result);

                let balance = lunar_new_year::balance_of(&client, keypair.address())
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                println!(
                    "Address({}) have {} num Lunar New Year NFT",
                    keypair.address().to_string().blue(),
                    balance
                );

                let honey_balance = honey::balance_of(&client, keypair.address())
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                let honey_decimal = honey::decimals(&client)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;

                let exponent: u32 = honey_decimal as u32; // 自定义指数值
                let divisor: u128 = 10u128.pow(exponent); // 计算除数
                let honey_balance_f64 = honey_balance.as_u128() as f64 / divisor as f64;

                println!(
                    "Address({}) have {} honey num",
                    keypair.address().to_string().red(),
                    honey_balance_f64
                );
            } else {
                println!(
                    "Address({}) have {} Lunar New Year NFT",
                    keypair.address().to_string().red(),
                    lunar_new_year_balance
                );
                println!(
                    "Address({}) have {} Bera",
                    keypair.address().to_string().red(),
                    native_token_balance
                );
                println!(
                    "Address({}) have {} Honey",
                    keypair.address().to_string().red(),
                    honey_balance_f64
                );
            }
        }

        Ok(())
    }
}
