use super::Client;
use ethers::prelude::*;
use ethers_solc::Solc;
use std::path::Path;
use std::sync::Arc;

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    CallerContract,
    "./contract/CallerContract_ABI.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// 1. Define an asynchronous function that takes a client provider as input and returns H160
pub async fn compile_deploy_contract(client: &Client) -> Result<H160, Box<dyn std::error::Error>> {
    // 2. Define a path as the directory that hosts the smart contracts in the project
    let source = Path::new(&env!("CARGO_MANIFEST_DIR"));
    let source = source.join("contract");
    println!("source file: {:?}", source);

    // 3. Compile all of the smart contracts
    let compiled = Solc::default()
        .compile_source(source)
        .expect("Could not compile contracts");

    // 4. Get ABI & Bytecode for Incrementer.sol
    let (abi, bytecode, _runtime_bytecode) = compiled
        .find("CallerContract")
        .expect("could not find contract")
        .into_parts_or_default();

    // 5. Create a contract factory which will be used to deploy instances of the contract
    let factory = ContractFactory::new(abi, bytecode, Arc::new(client.clone()));

    let incrementer_address = "0xf214745d896fc2f7ee9be9d5b7c91044b81ea783"
        .parse::<Address>()
        .unwrap();

    // 6. Deploy
    let contract = factory.deploy(incrementer_address)?.send().await?;

    // 7. Print out the address
    let addr = contract.address();
    println!("CallerContract.sol has been deployed to {:?}", addr);

    // 8. Return the address
    Ok(addr)
}

// 2. Define an asynchronous function that takes a client provider and address as input and returns a U256
pub async fn call_target_function(
    client: &Client,
    contract_addr: &H160,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = CallerContract::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.call_target_function().call().await?;
    Ok(value)
}
