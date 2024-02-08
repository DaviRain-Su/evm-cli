use crate::Client;
use ethers::core::types::Address;
use ethers::prelude::*;
use std::sync::Arc;

// 1. Generate a type-safe interface for the Incrementer smart contract
abigen!(
    GovernanceModule,
    "./contract/bear/precompile_contracts/IGovernanceModule.abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// cancelProposal
pub async fn get_all_balances(
    client: &Client,
    contract_addr: &H160,
    proposal_id: u64,
) -> Result<(u64, u64), Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.cancel_proposal(proposal_id).await?;
    Ok(value)
}

// getConstitution
pub async fn get_constitution(
    client: &Client,
    contract_addr: &H160,
) -> Result<String, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_constitution().await?;
    Ok(value)
}

// getDepositParams
pub async fn get_deposit_params(
    client: &Client,
    contract_addr: &H160,
) -> Result<governance_module::DepositParams, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_deposit_params().await?;
    Ok(value)
}

// getParams
pub async fn get_params(
    client: &Client,
    contract_addr: &H160,
) -> Result<governance_module::Params, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_params().await?;
    Ok(value)
}

// getProposal
pub async fn get_proposal(
    client: &Client,
    contract_addr: &H160,
    proposal_id: u64,
) -> Result<governance_module::Proposal, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_proposal(proposal_id).await?;
    Ok(value)
}

// getProposalDeposits
pub async fn get_proposal_deposits(
    client: &Client,
    contract_addr: &H160,
    proposal_id: u64,
) -> Result<Vec<governance_module::Coin>, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_proposal_deposits(proposal_id).await?;
    Ok(value)
}

// getProposalDepositsByDepositor
//
pub async fn get_proposal_deposits_by_depositor(
    client: &Client,
    contract_addr: &H160,
    proposal_id: u64,
    depositor: Address,
) -> Result<Vec<governance_module::Coin>, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract
        .get_proposal_deposits_by_depositor(proposal_id, depositor)
        .await?;
    Ok(value)
}

// getProposalTallyResult
pub async fn get_proposal_tally_result(
    client: &Client,
    contract_addr: &H160,
    proposal_id: u64,
) -> Result<governance_module::TallyResult, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_proposal_tally_result(proposal_id).await?;
    Ok(value)
}

//getProposalVotes
pub async fn get_proposal_votes(
    client: &Client,
    contract_addr: &H160,
    proposal_id: u64,
    pagination: governance_module::PageRequest,
) -> Result<
    (
        Vec<governance_module::Vote>,
        governance_module::PageResponse,
    ),
    Box<dyn std::error::Error>,
> {
    let contract = GovernanceModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_proposal_votes(proposal_id, pagination).await?;
    Ok(value)
}

// getProposalVotesByVoter
pub async fn get_proposal_votes_by_voter(
    client: &Client,
    contract_addr: &H160,
    proposal_id: u64,
    voter: Address,
) -> Result<governance_module::Vote, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract
        .get_proposal_votes_by_voter(proposal_id, voter)
        .await?;
    Ok(value)
}

// getProposals
//
//
pub async fn get_proposals(
    client: &Client,
    contract_addr: &H160,
    proposal_status: i32,
    pagination: governance_module::PageRequest,
) -> Result<
    (
        Vec<governance_module::Proposal>,
        governance_module::PageResponse,
    ),
    Box<dyn std::error::Error>,
> {
    let contract = GovernanceModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_proposals(proposal_status, pagination).await?;
    Ok(value)
}

// getTallyParams
//
pub async fn get_tally_params(
    client: &Client,
    contract_addr: &H160,
) -> Result<governance_module::TallyParams, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.get_tally_params().await?;
    Ok(value)
}

// submitProposal
pub async fn submit_proposal(
    client: &Client,
    contract_addr: &H160,
    proposal: governance_module::MsgSubmitProposal,
) -> Result<u64, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.submit_proposal(proposal).await?;
    Ok(value)
}

// vote
//
pub async fn vote(
    client: &Client,
    contract_addr: &H160,
    proposal_id: u64,
    option: i32,
    metadata: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract.vote(proposal_id, option, metadata).await?;
    Ok(value)
}

// voteWeighted
//
pub async fn vote_weighted(
    client: &Client,
    contract_addr: &H160,
    proposal_id: u64,
    option: Vec<governance_module::WeightedVoteOption>,
    metadata: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let contract = GovernanceModule::new(contract_addr.clone(), Arc::new(client.clone()));
    let value = contract
        .vote_weighted(proposal_id, option, metadata)
        .await?;
    Ok(value)
}
