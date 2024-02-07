use super::Client;
use ethers::{prelude::*, utils};
use ethers_solc::Solc;
use std::path::Path;
use std::sync::Arc;

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    Incrementer,
    "./contract/Incrementer_ABI.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// 1. Define an asynchronous function that takes a client provider as input and returns H160
async fn compile_deploy_contract(client: &Client) -> Result<H160, Box<dyn std::error::Error>> {
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
        .find("Incrementer")
        .expect("could not find contract")
        .into_parts_or_default();

    // 5. Create a contract factory which will be used to deploy instances of the contract
    let factory = ContractFactory::new(abi, bytecode, Arc::new(client.clone()));
    println!("factory {:?}", factory);

    // 6. Deploy
    let contract = factory.deploy(U256::from(5))?.send().await?;

    // 7. Print out the address
    let addr = contract.address();
    println!("Incrementer.sol has been deployed to {:?}", addr);

    // 8. Return the address
    Ok(addr)
}

// 2. Define an asynchronous function that takes a client provider and address as input and returns a U256
pub async fn read_number(
    client: &Client,
    contract_addr: &H160,
) -> Result<U256, Box<dyn std::error::Error>> {
    // 3. Create contract instance
    let contract = Incrementer::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Call contract's number function
    let value = contract.number().call().await?;

    // 5. Print out number
    println!("Incrementer's number is {}", value);

    // 6. Return the number
    Ok(value)
}

// 2. Define an asynchronous function that takes a client provider and address as input
pub async fn increment_number(
    client: &Client,
    contract_addr: &H160,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Incrementing number...");

    // 3. Create contract instance
    let contract = Incrementer::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Send contract transaction
    let tx = contract.increment(U256::from(5)).send().await?.await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

// 2. Define an asynchronous function that takes a client provider and address as input
pub async fn reset(
    client: &Client,
    contract_addr: &H160,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Resetting number...");

    // 3. Create contract instance
    let contract = Incrementer::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Send contract transaction
    let tx = contract.reset().send().await?.await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
