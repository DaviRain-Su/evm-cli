// #![allow(unused_imports)]
// #![allow(dead_code)]

// 1. Import ethers crate
use ethers::prelude::LocalWallet;
use ethers::prelude::SignerMiddleware;
use ethers::prelude::Wallet;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::Signer;
use std::str::FromStr;

const BEAR_CHAIN_DECIMAL: u64 = 1_000_000_000_000_000_000;

pub mod bear;
pub mod incrementer;
pub mod utils;

// 2. Add client type
type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

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
    // let address_from = "0xe907e66a480F2822354f0a343A17B73EeF1fe8cb".parse::<Address>()?;
    // let address_to = "0xfFD34F45115CB1BB97A49b6f37E557E15d0cAD3A".parse::<Address>()?;

    // print_balances(&provider, address_from, address_to).await?;

    // 9. Call compile_deploy_contract function in main
    // let addr = compile_deploy_contract(&client).await?;

    // println!("addr: {:?}", addr);

    // let addr = H160::from_str("fff189efc3da781e7d4ec584b8304904517afac7")?;
    // let addrs = H160::from_str("0x291280135e7bb88bfe2b86caa22439632d2f4486")?;
    // println!("addr is {:?}", addrs);
    // // 7. Call read_number function in main
    // read_number(&client, &addrs).await?;

    // increment_number(&client, &addrs).await?;

    // // 7. Call read_number function in main
    // read_number(&client, &addrs).await?;

    // // 5. Call reset function in main
    // reset(&client, &addrs).await?;

    // // 7. Call read_number function in main
    // read_number(&client, &addrs).await?;

    let honey_addrs = H160::from_str("0x7EeCA4205fF31f947EdBd49195a7A88E6A91161B")?;
    println!("honey addrs {:?}", honey_addrs);

    let name = bear::honey::name(&client, &honey_addrs).await?;

    println!("honey name is {}", name);

    Ok(())
}
