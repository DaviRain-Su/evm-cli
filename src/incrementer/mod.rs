use super::Client;
use ethers::prelude::*;
use ethers_solc::Solc;
use std::path::Path;
use std::sync::Arc;

pub mod call_incrementer;

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    Incrementer,
    "./contract/incrementer/Incrementer_ABI.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// 1. Define an asynchronous function that takes a client provider as input and returns H160
pub async fn compile_deploy_contract(client: &Client) -> Result<H160, Box<dyn std::error::Error>> {
    // 2. Define a path as the directory that hosts the smart contracts in the project
    let source = Path::new(&env!("CARGO_MANIFEST_DIR"))
        .join("contract")
        .join("incrementer");
    println!("source file: {:?}", source);

    // 检查目录是否存在
    if !source.exists() || !source.is_dir() {
        return Err(format!(
            "Contract directory does not exist or is not a directory: {:?}",
            source
        )
        .into());
    }

    // 列出目录内容
    println!("Contents of the contract directory:");
    for entry in std::fs::read_dir(&source)? {
        let entry = entry?;
        println!("  {:?}", entry.path());
    }

    // 编译合约
    let compiled = match Solc::default().compile_source(source) {
        Ok(compiled) => compiled,
        Err(e) => return Err(format!("Failed to compile contracts: {:?}", e).into()),
    };

    // 打印可用的合约
    println!("Available contracts:");
    for (name, _) in compiled.contracts.iter() {
        println!("  {}", name);
    }

    // 查找 Incrementer 合约
    let contract = match compiled.find("Incrementer") {
        Some(contract) => contract,
        None => return Err("Could not find Incrementer contract".into()),
    };

    let (abi, bytecode, _runtime_bytecode) = contract.into_parts_or_default();

    // 5. Create a contract factory which will be used to deploy instances of the contract
    let factory = ContractFactory::new(abi, bytecode, Arc::new(client.clone()));

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
    let contract = Incrementer::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.number().call().await?;
    println!("Incrementer's number is {}", value);
    Ok(value)
}

// 2. Define an asynchronous function that takes a client provider and address as input
pub async fn increment_number(
    client: &Client,
    contract_addr: &H160,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Incrementing number...");
    let contract = Incrementer::new(contract_addr.clone(), Arc::new(client.clone()));
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
    let contract = Incrementer::new(contract_addr.clone(), Arc::new(client.clone()));
    let tx = contract.reset().send().await?.await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
