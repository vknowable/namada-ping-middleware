use axum::{
  extract::{Path, Query, State}, http::StatusCode, Json
};
use futures::future::try_join_all;
use tendermint::v0_34::abci::response;
use tendermint_rpc::HttpClient;
use std::{
  sync::Arc,
  str::FromStr,
  time::Duration,
};
use namada_sdk::{
  rpc,
  types::{
    // token::Amount,
    dec::Dec,
    // time,
    storage::Key,
  },
  // core::ledger::parameters::{storage, EpochDuration},
};
use namada_sdk::governance::{storage::keys as governance_storage, utils::{ProposalStatus, ProposalResult, TallyResult}};
use namada_sdk::proof_of_stake::Epoch;
use namada_sdk::types::parameters::EpochDuration;
use namada_parameters::storage;

use crate::app::app_state::AppState;
use crate::error::api_error::ApiError;
use crate::model::{
  gov::{ParamsGovResponse, TallyResponse, FinalTallyInfo, ProposalInfo, ProposalsResponse, IndividualProposalResponse, ProposalItem, ProposalQueryParams, CosmosProposalStatus},
  shared::{DEFAULT_TIMESTAMP, DenomAmount, SuffixedDur},
};


pub async fn params_voting_handler(State(app_state): State<Arc<AppState>>) 
  -> Result<Json<ParamsGovResponse>, ApiError> {
  
  let gov_params = rpc::query_governance_parameters(app_state.get_client()).await;
  let epoch_dur: EpochDuration = rpc::query_storage_value(app_state.get_client(), &storage::get_epoch_duration_storage_key()).await?;
  
  let mut response = ParamsGovResponse::default();

  // TODO: Namada voting period can be anything within an allowed range
  // Cosmos has fixed voting period so we're just returning Namada's min_duration value here
  // perhaps it could be handled differently?
  let voting_period = Duration::from(epoch_dur.min_duration) * (gov_params.min_proposal_voting_period as u32);
  response.voting_params.voting_period = SuffixedDur(voting_period);

  Ok(Json(response))
}

pub async fn params_deposit_handler(State(app_state): State<Arc<AppState>>) 
  -> Result<Json<ParamsGovResponse>, ApiError> {
  
  let gov_params = rpc::query_governance_parameters(app_state.get_client()).await;

  // TODO: can't find an equivalent to max_deposit_period
  let mut response = ParamsGovResponse::default();
  let deposit = DenomAmount::nam_amount(gov_params.min_proposal_fund);
  response.deposit_params.min_deposit.push(deposit);

  Ok(Json(response))
}

pub async fn params_tallying_handler(State(_app_state): State<Arc<AppState>>) 
  -> Result<Json<ParamsGovResponse>, ApiError> {
  
  // TODO: do these params have equivalents in Namada?
  // looks like quorum might be dependant on proposal type (see enum 'TallyType')
  // hardcoding for now...
  let mut response = ParamsGovResponse::default();
  
  response.tally_params.quorum = Dec::zero();
  response.tally_params.threshold = Dec::from_str("0.67").unwrap();
  response.tally_params.veto_threshold = Dec::one();

  Ok(Json(response))
}

pub async fn all_proposals_handler(query: Query<ProposalQueryParams>, State(app_state): State<Arc<AppState>>) 
  -> Result<Json<ProposalsResponse>, ApiError> {

  let last_proposal_id_key = governance_storage::get_counter_key();
  let last_proposal_id: u64 = rpc::query_storage_value(app_state.get_client(), &last_proposal_id_key).await?;

  // get deposit amount
  let gov_params = rpc::query_governance_parameters(app_state.get_client()).await;
  let deposit = DenomAmount::nam_amount(gov_params.min_proposal_fund);

  let mut tasks = Vec::new();

  // Spawn a task for each entry in consensus set
  for id in 0..last_proposal_id {
      let task = fetch_proposal_info(app_state.get_client(), id, deposit.clone());
      tasks.push(task);
  }

  // Collect results of all tasks concurrently
  let all_proposals: Vec<Option<ProposalItem>> = try_join_all(tasks).await?;
  // Remove any 'None' values that might have been returned when querying a proposal id
  let valid_proposals: Vec<ProposalItem> = all_proposals.into_iter()
    .filter_map(|opt_proposal| opt_proposal)
    .collect();

  let response = ProposalsResponse {
    proposals: valid_proposals,
    pagination: None,
  };

  Ok(Json(response))
}

async fn fetch_proposal_info(client: &HttpClient, id: u64, deposit: DenomAmount) -> Result<Option<ProposalItem>, ApiError> {
  if let Some(proposal) = rpc::query_proposal_by_id(client, id).await? {
    let mut proposal_item = ProposalItem::from(proposal);
    proposal_item.total_deposit = vec![deposit];
    return Ok(Some(proposal_item));
  }
  
  Ok(None)
}

pub async fn single_proposal_handler(Path(id): Path<u64>, State(app_state): State<Arc<AppState>>) 
  -> Result<Json<IndividualProposalResponse>, ApiError> {

  let current_epoch = rpc::query_epoch(app_state.get_client()).await?;
  let proposal = get_proposal(app_state, id, current_epoch, None).await?;
  let response = IndividualProposalResponse { proposal };

  Ok(Json(response))
  }

pub async fn proposal_tally_handler(Path(id): Path<u64>, State(app_state): State<Arc<AppState>>) 
  -> Result<Json<TallyResponse>, ApiError> {
  
  let mut final_tally = FinalTallyInfo::default();
  let proposal_result = get_proposal_result(app_state, id).await?;
  //
  if let Some(proposal_result) = proposal_result {
    final_tally.yes = proposal_result.total_yay_power.to_string_native();
    final_tally.no = proposal_result.total_nay_power.to_string_native();
    let response = TallyResponse {tally: final_tally };
    return Ok(Json(response))
  }

  return Err(ApiError {
    error: "proposal result not found".to_string(),
    code: StatusCode::INTERNAL_SERVER_ERROR,
    message: None,
    details: Vec::new(),
  })

  // Ok(Json(response))
}

/// retrieves proposal info by id and formats it into a ProposalItem struct
async fn get_proposal(app_state: Arc<AppState>, id: u64, current_epoch: Epoch, requested_status: Option<u32>) -> Result<Option<ProposalItem>, ApiError> {
  match rpc::query_proposal_by_id(app_state.get_client(), id).await? {
    // the rpc query may return None; also, if the status doesn't match the filter parmas we will return None
    // neither of these cases will be considered errors, it just means nothing needs to be appended to the eventual Api response
    Some(proposal) => {
      let proposal_status = proposal.get_status(current_epoch);
      let proposal_result = get_proposal_result(app_state.clone(), id).await?;

      if let Some(proposal_result) = proposal_result {
//
        let status: CosmosProposalStatus = map_status_namada_to_cosmos(proposal_status, proposal_result.result);

        if filter_proposal_by_status(requested_status, status) {
          let mut final_tally_result = FinalTallyInfo::default();
          final_tally_result.yes = proposal_result.total_yay_power.to_string_native();
          final_tally_result.no = proposal_result.total_nay_power.to_string_native();

          // TODO: how do we find this value?
          let submit_time = DEFAULT_TIMESTAMP.to_string();
          // TODO: properly convert these from epochs to timestamps
          let deposit_end_time = proposal.voting_start_epoch.to_string(); // same as voting_start_time?

          // get deposit amount
          let gov_params = rpc::query_governance_parameters(app_state.get_client()).await;
          let deposit = DenomAmount::nam_amount(gov_params.min_proposal_fund);

          // TODO: properly convert these from epochs to timestamps
          let voting_start_time = proposal.voting_start_epoch.to_string();
          let voting_end_time = proposal.voting_end_epoch.to_string();

          Ok(Some(ProposalItem {
            proposal_id: proposal.id.to_string(),
            content: ProposalInfo {
              at_type: proposal.r#type.to_string(),
              title: proposal.content.get("title").unwrap_or(&"".to_string()).clone(),
              // TODO: concatenate 'abstract', 'motivation', and 'details'?
              description: proposal.content.get("details").unwrap_or(&"".to_string()).clone(),
              recipient: None,
              amount: None,
            },
            status,
            final_tally_result,
            submit_time,
            deposit_end_time,
            total_deposit: vec![deposit],
            voting_start_time,
            voting_end_time,
          }))
        }
        else { Ok(None) }
      //
      }
      else { Ok(None) } // if proposal doesn't match requested status
    }
    None => Ok(None) // if rpc query doesn't find a proposal matching the id
  }
}

/// Attempts to query a proposal result from storage
async fn get_proposal_result(app_state: Arc<AppState>, id: u64) -> Result<Option<ProposalResult>, ApiError> {
  // let proposal_result_key: Key = governance_storage::get_proposal_result_key(id);
  // let proposal_result: ProposalResult = rpc::query_storage_value(app_state.get_client(), &proposal_result_key).await?;
  match rpc::query_proposal_result(app_state.get_client(), id).await? {
    Some(proposal_result) => Ok(Some(proposal_result)),
    None => Ok(None)
  }

  // Ok(proposal_result)
}

/// Maps a Namada proposal status enum to a Cosmos proposal status enum
#[allow(unreachable_patterns)]
fn map_status_namada_to_cosmos(namada_status: ProposalStatus, result: TallyResult) -> CosmosProposalStatus {
  match namada_status {
    ProposalStatus::Pending => CosmosProposalStatus::PROPOSAL_STATUS_DEPOSIT_PERIOD,
    ProposalStatus::OnGoing => CosmosProposalStatus::PROPOSAL_STATUS_VOTING_PERIOD,
    ProposalStatus::Ended => {
      match result {
        TallyResult::Passed => CosmosProposalStatus::PROPOSAL_STATUS_PASSED,
        TallyResult::Rejected => CosmosProposalStatus::PROPOSAL_STATUS_REJECTED,
      }
    }
    // currently unused
    _ => CosmosProposalStatus::PROPOSAL_STATUS_FAILED
  }
}

fn filter_proposal_by_status(query_status: Option<u32>, proposal_status: CosmosProposalStatus) -> bool {
  let status_u32: u32 = proposal_status.into();
  match query_status {
    Some(query_status) => query_status == status_u32,
    // no status specified; return all
    None => true
  }
}