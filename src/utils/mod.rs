// 1. Add to imports
use super::Client;
use ethers::{prelude::*, utils as ethers_utils};

// 1. Create an asynchronous function that takes a provider reference and from and to address as input
pub async fn print_balances(
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
pub async fn send_transaction(
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
        .value(U256::from(ethers_utils::parse_ether(1)?))
        .from(address_from);

    // 3. Send the transaction with the client
    let tx = client.send_transaction(tx, None).await?.await?;

    // 4. Print out the result
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
