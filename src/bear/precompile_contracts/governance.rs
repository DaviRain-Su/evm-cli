use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

pub fn governance_addr() -> H160 {
    H160::from_str("0x7b5Fe22B5446f7C62Ea27B8BD71CeF94e03f3dF2").unwrap()
}

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    GovernanceModule,
    "./contract/bear/precompile_contracts/IGovernanceModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// cancelProposal
pub async fn get_all_balances(
    client: &Client,
    proposal_id: u64,
) -> Result<(u64, u64), Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(governance_addr(), Arc::new(client.clone()));
    let value = contract.cancel_proposal(proposal_id).call().await?;
    Ok(value)
}

// getConstitution
pub async fn get_constitution(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(governance_addr(), Arc::new(client.clone()));
    let value = contract.get_constitution().call().await?;
    Ok(value)
}

// getDepositParams
pub async fn get_deposit_params(
    client: &Client,
) -> Result<governance_module::DepositParams, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(governance_addr(), Arc::new(client.clone()));
    let value = contract.get_deposit_params().call().await?;
    Ok(value)
}

// getParams
pub async fn get_params(
    client: &Client,
) -> Result<governance_module::Params, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(governance_addr(), Arc::new(client.clone()));
    let value = contract.get_params().call().await?;
    Ok(value)
}

// getProposal
pub async fn get_proposal(
    client: &Client,
    proposal_id: u64,
) -> Result<governance_module::Proposal, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(governance_addr(), Arc::new(client.clone()));
    let value = contract.get_proposal(proposal_id).call().await?;
    Ok(value)
}

// getProposalDeposits
pub async fn get_proposal_deposits(
    client: &Client,
    proposal_id: u64,
) -> Result<Vec<governance_module::Coin>, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(governance_addr(), Arc::new(client.clone()));
    let value = contract.get_proposal_deposits(proposal_id).call().await?;
    Ok(value)
}

// getProposalDepositsByDepositor
//
pub async fn get_proposal_deposits_by_depositor(
    client: &Client,
    proposal_id: u64,
    depositor: Address,
) -> Result<Vec<governance_module::Coin>, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(governance_addr(), Arc::new(client.clone()));
    let value = contract
        .get_proposal_deposits_by_depositor(proposal_id, depositor)
        .call()
        .await?;
    Ok(value)
}

// getProposalTallyResult
pub async fn get_proposal_tally_result(
    client: &Client,
    proposal_id: u64,
) -> Result<governance_module::TallyResult, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(governance_addr(), Arc::new(client.clone()));
    let value = contract
        .get_proposal_tally_result(proposal_id)
        .call()
        .await?;
    Ok(value)
}

//getProposalVotes
pub async fn get_proposal_votes(
    client: &Client,
    proposal_id: u64,
    pagination: governance_module::PageRequest,
) -> Result<
    (
        Vec<governance_module::Vote>,
        governance_module::PageResponse,
    ),
    Box<dyn std::error::Error>,
> {
    let contract = GovernanceModule::new(governance_addr(), Arc::new(client.clone()));
    let value = contract
        .get_proposal_votes(proposal_id, pagination)
        .call()
        .await?;
    Ok(value)
}

// getProposalVotesByVoter
pub async fn get_proposal_votes_by_voter(
    client: &Client,
    proposal_id: u64,
    voter: Address,
) -> Result<governance_module::Vote, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(governance_addr(), Arc::new(client.clone()));
    let value = contract
        .get_proposal_votes_by_voter(proposal_id, voter)
        .call()
        .await?;
    Ok(value)
}

// getProposals
//
//
pub async fn get_proposals(
    client: &Client,
    proposal_status: i32,
    pagination: governance_module::PageRequest,
) -> Result<
    (
        Vec<governance_module::Proposal>,
        governance_module::PageResponse,
    ),
    Box<dyn std::error::Error>,
> {
    let contract = GovernanceModule::new(governance_addr(), Arc::new(client.clone()));
    let value = contract
        .get_proposals(proposal_status, pagination)
        .call()
        .await?;
    Ok(value)
}

// getTallyParams
//
pub async fn get_tally_params(
    client: &Client,
) -> Result<governance_module::TallyParams, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(governance_addr(), Arc::new(client.clone()));
    let value = contract.get_tally_params().call().await?;
    Ok(value)
}

// submitProposal
pub async fn submit_proposal(
    client: &Client,
    proposal: governance_module::MsgSubmitProposal,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(governance_addr(), Arc::new(client.clone()));
    let tx = contract.submit_proposal(proposal).send().await?.await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

// vote
//
pub async fn vote(
    client: &Client,
    proposal_id: u64,
    option: i32,
    metadata: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(governance_addr(), Arc::new(client.clone()));
    let tx = contract
        .vote(proposal_id, option, metadata)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

// voteWeighted
//
pub async fn vote_weighted(
    client: &Client,
    proposal_id: u64,
    option: Vec<governance_module::WeightedVoteOption>,
    metadata: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(governance_addr(), Arc::new(client.clone()));
    let tx = contract
        .vote_weighted(proposal_id, option, metadata)
        .send()
        .await?
        .await?;
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
