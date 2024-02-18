use crate::bear::deploy_contracts::honey::{self, honey_token_addr};
use crate::errors::Error;
use crate::utils::{
    get_all_keypairs_string_with_balance, get_config, get_single_keypairs_string_with_balance,
};
use colored::*;
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::types::U256;
use ethers_signers::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Honey {
    /// chain id
    #[structopt(long)]
    pub chain_id: u64,
    /// wallet file name
    #[structopt(long)]
    pub file_name: String,
}

impl Honey {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let single_keypair =
            get_single_keypairs_string_with_balance().map_err(|e| Error::Custom(e.to_string()))?;

        let single_keypair = single_keypair.keypairs[0].clone();

        let client = SignerMiddleware::new(
            provider.clone(),
            single_keypair.clone().with_chain_id(self.chain_id),
        );

        let honey_balance = honey::balance_of(&client, single_keypair.address())
            .await
            .map_err(|e| Error::Custom(e.to_string()))?;

        let honey_decimal = honey::decimals(&client)
            .await
            .map_err(|e| Error::Custom(e.to_string()))?;

        let exponent: u32 = honey_decimal as u32; // 自定义指数值
        let divisor: u128 = 10u128.pow(exponent); // 计算除数
        let honey_balance_f64 = honey_balance.as_u128() as f64 / divisor as f64;

        println!(
            "{} have {} Honey",
            single_keypair.address().to_string().blue(),
            honey_balance_f64
        );

        let approve_result = loop {
            if let Ok(result) = honey::approve(&client, honey_token_addr(), honey_balance).await {
                break result;
            } else {
                continue;
            }
        };
        println!("approve result : {:?}", approve_result);

        let keypairs = get_all_keypairs_string_with_balance(&self.file_name)
            .map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in &keypairs.keypairs {
            let balance = honey::balance_of(&client, keypair.address())
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;
            let amount = U256::from(2_000_000_000_000_000_000u64);
            if balance != amount {
                let transfer_result = loop {
                    if let Ok(result) = honey::transfer(&client, keypair.address(), amount).await {
                        break result;
                    } else {
                        continue;
                    }
                };
                println!("transfer result: {:?}", transfer_result);

                let honey_balance = honey::balance_of(&client, keypair.address())
                    .await
                    .map_err(|e| Error::Custom(e.to_string()))?;
                let honey_balance_f64 = honey_balance.as_u128() as f64 / divisor as f64;

                println!(
                    "{} have {} Honey",
                    keypair.address().to_string().blue(),
                    honey_balance_f64
                );
            }
        }

        Ok(())
    }
}
