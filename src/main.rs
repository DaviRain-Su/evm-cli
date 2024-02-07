#![allow(unused_imports)]
#![allow(dead_code)]

// 1. Import ethers crate
use ethers::prelude::LocalWallet;
use ethers::prelude::SignerMiddleware;
use ethers::prelude::Wallet;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::Signer;
use ethers_solc::Solc;
use std::str::FromStr;
use std::{path::Path, sync::Arc};

// 1. Add to imports
use ethers::{prelude::*, utils};
use ethers_solc::CompilerInput;

const BEAR_CHAIN_DECIMAL: u64 = 1_000_000_000_000_000_000;

// 2. Add client type
type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    Incrementer,
    "./contract/Incrementer_ABI.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// 2. Define an asynchronous function that takes a client provider and address as input and returns a U256
async fn read_number(
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

// 1. Create an asynchronous function that takes a provider reference and from and to address as input
async fn print_balances(
    provider: &Provider<Http>,
    address_from: Address,
    address_to: Address,
) -> Result<(), Box<dyn std::error::Error>> {
    // 2. Use the get_balance function
    let balance_from = provider.get_balance(address_from, None).await?;
    let balance_to = provider.get_balance(address_to, None).await?;

    // 3. Print the resultant balance
    println!("{} has {}", address_from, balance_from);
    println!("{} has {}", address_to, balance_to);

    Ok(())
}

// 1. Define an asynchronous function that takes a client provider and the from and to addresses as input
async fn send_transaction(
    client: &Client,
    address_from: Address,
    address_to: Address,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Beginning transfer of 1 native currency from {} to {}.",
        address_from, address_to
    );

    // 2. Create a TransactionRequest object
    let tx = TransactionRequest::new()
        .to(address_to)
        .value(U256::from(utils::parse_ether(1)?))
        .from(address_from);

    // 3. Send the transaction with the client
    let tx = client.send_transaction(tx, None).await?.await?;

    // 4. Print out the result
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

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

// 3. Add annotation
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 4. Use try_from with RPC endpoint
    let provider = Provider::<Http>::try_from("https://artio.rpc.berachain.com/")?;
    // 5. Use a private key to create a wallet
    // Do not include the private key in plain text in any production code
    // This is just for demonstration purposes
    // Do not include '0x' at the start of the private key
    let wallet: LocalWallet = "eca31d121880412e02e16295069348dcd18db64ea0f179b24a9e7ecfeb66983d"
        .parse::<LocalWallet>()?
        .with_chain_id(80085u64);

    // 6. Wrap the provider and wallet together to create a signer client
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    println!("client: {:?}", client);

    // 2. Add from and to address
    let address_from = "0xe907e66a480F2822354f0a343A17B73EeF1fe8cb".parse::<Address>()?;
    let address_to = "0xfFD34F45115CB1BB97A49b6f37E557E15d0cAD3A".parse::<Address>()?;

    print_balances(&provider, address_from, address_to).await?;

    // 9. Call compile_deploy_contract function in main
    let addr = compile_deploy_contract(&client).await?;

    println!("addr: {:?}", addr);

    // let addr = H160::from_str("fff189efc3da781e7d4ec584b8304904517afac7")?;
    // let addrs = H160::from_str("0x291280135e7bb88bfe2b86caa22439632d2f4486")?;
    // println!("addr is {:?}", addrs);
    // 7. Call read_number function in main
    read_number(&client, &addr).await?;

    Ok(())
}
