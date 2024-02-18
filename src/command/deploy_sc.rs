use crate::errors::Error;
use crate::incrementer::call_incrementer::{
    call_target_function, compile_deploy_contract as call_incrementer_compile_deploy_contract,
};
use crate::incrementer::{compile_deploy_contract, increment_number, read_number, reset};
use crate::utils::{get_all_keypairs_string_with_balance, get_config};
use ethers::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Deploy {
    Incrementer(Incrementer),
    CallIncrementer(CallIncrementer),
}

impl Deploy {
    pub async fn run(&self) -> Result<(), Error> {
        match self {
            Deploy::Incrementer(inc) => inc.run().await,
            Deploy::CallIncrementer(call) => call.run().await,
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct Incrementer {
    /// chain id
    #[structopt(long)]
    pub chain_id: u64,
    /// wallet file name
    #[structopt(long)]
    pub file_name: String,
}

impl Incrementer {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs = get_all_keypairs_string_with_balance(&self.file_name)
            .map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in keypairs.keypairs {
            let client = SignerMiddleware::new(
                provider.clone(),
                keypair.clone().with_chain_id(self.chain_id),
            );

            println!("Address({:?}) deploy contract", keypair.address());

            let mut counter = 0;
            'a: loop {
                let result = compile_deploy_contract(&client)
                    .await
                    .map_err(|e| Error::Custom(e.to_string()));
                log::info!("compile_deploy_contract Result: {:?}", result);
                if let Ok(addr) = result {
                    loop {
                        if let Err(_) = read_number(&client, &addr).await {
                            log::warn!("read_number have error");
                            continue;
                        } else {
                            break;
                        }
                    }

                    loop {
                        if let Err(_) = increment_number(&client, &addr).await {
                            // log::warn!("increment_number have error");
                            continue;
                        } else {
                            break;
                        }
                    }

                    loop {
                        if let Err(e) = read_number(&client, &addr).await {
                            log::warn!("read_number have error: {e:?}");
                            continue;
                        } else {
                            break;
                        }
                    }

                    loop {
                        if let Err(e) = reset(&client, &addr).await {
                            log::warn!("reset have error: {e:?}");
                            continue;
                        } else {
                            break;
                        }
                    }

                    loop {
                        if let Err(e) = read_number(&client, &addr).await {
                            log::warn!("read_number have error: {e:?}");
                            continue;
                        } else {
                            break 'a;
                        }
                    }
                } else if counter == 3 {
                    break;
                } else {
                    counter += 1;
                    continue;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub struct CallIncrementer {
    /// chain id
    #[structopt(long)]
    pub chain_id: u64,
    /// wallet file name
    #[structopt(long)]
    pub file_name: String,
}

impl CallIncrementer {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;

        let provider = Provider::<Http>::try_from(config.rpc_endpoint)
            .map_err(|e| Error::Custom(e.to_string()))?;

        let keypairs = get_all_keypairs_string_with_balance(&self.file_name)
            .map_err(|e| Error::Custom(e.to_string()))?;

        for keypair in keypairs.keypairs {
            let client = SignerMiddleware::new(
                provider.clone(),
                keypair.clone().with_chain_id(self.chain_id),
            );

            println!("Address({:?}) deploy contract", keypair.address());

            let addr = call_incrementer_compile_deploy_contract(&client)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;
            println!("compile_deploy_contract Result: {:?}", addr);

            let value = call_target_function(&client, &addr).await;
            println!("call_target_function: {:?}", value);
        }

        Ok(())
    }
}
