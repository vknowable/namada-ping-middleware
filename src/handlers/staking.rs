use axum::{
  extract::{Query, State},
  Json,
};
use std::{
  sync::Arc,
  time::Duration, ops::Div,
};
use namada_sdk::{
  proof_of_stake::{PosParams, types::ValidatorState},
  rpc,
  // core::ledger::parameters::{storage},
  types::{token::NATIVE_SCALE, dec::Dec},
};
use namada_sdk::types::parameters::EpochDuration;
use namada_parameters::storage;

use crate::{app::app_state::AppState, model::staking::CosmosValStatus};
use crate::error::api_error::ApiError;
use crate::model::{
  staking::{PoolResponse, PoolInfo, ParamsResponse, ValidatorsQueryParams, ValidatorsResponse, ValidatorInfo, ValidatorDescription, ConsensusKeyInfo, CommissionInfo, RatesInfo},
  shared::{NAM, DEFAULT_TIMESTAMP, PaginationInfo, SuffixedDur},
};


pub async fn pool_handler(State(app_state): State<Arc<AppState>>) 
  -> Result<Json<PoolResponse>, ApiError> {

  let current_epoch = rpc::query_epoch(app_state.get_client()).await?;
  let bonded_tokens = rpc::get_total_staked_tokens(app_state.get_client(), current_epoch).await?.div(NATIVE_SCALE as u64);
  
  //TODO: not_bonded tokens
  let response = PoolResponse {
    pool: PoolInfo { not_bonded_tokens: "1000000000".to_string(), bonded_tokens },
  };

  Ok(Json(response))
}

pub async fn params_handler(State(app_state): State<Arc<AppState>>) 
  -> Result<Json<ParamsResponse>, ApiError> {
  
  let pos_params: PosParams = rpc::get_pos_params(app_state.get_client()).await?;
  let epoch_dur: EpochDuration = rpc::query_storage_value(app_state.get_client(), &storage::get_epoch_duration_storage_key()).await?;
  let unbonding_period = Duration::from(epoch_dur.min_duration) * (pos_params.owned.unbonding_len as u32);

  let response = ParamsResponse {
    unbonding_time: SuffixedDur(unbonding_period),
    max_validators: pos_params.owned.max_validator_slots as u32,
    // TODO: need to find out what these two values mean
    max_entries: 7,
    historical_entries: 1000,
    bond_denom: NAM.to_string(),
  };

  Ok(Json(response))
}

pub async fn validators_handler(query: Query<ValidatorsQueryParams>, State(app_state): State<Arc<AppState>>) 
  -> Result<Json<ValidatorsResponse>, ApiError> {

  // TODO: pagination support
  let current_epoch = rpc::query_epoch(app_state.get_client()).await?;
  let all_vals = rpc::get_all_validators(app_state.get_client(), current_epoch).await?;
  let mut response = ValidatorsResponse::new();

  for val in &all_vals {
    // check validator status first
    let state = rpc::get_validator_state(app_state.get_client(), val, Some(current_epoch)).await?;
    let (jailed, status) = match state {
      Some(state) => map_status_namada_to_cosmos(state),
      None => (true, CosmosValStatus::BOND_STATUS_UNBONDED)
    };

    if filter_validator_by_status(query.status, status) {
      let (metadata, commission_info) = rpc::query_metadata(app_state.get_client(), val, Some(current_epoch)).await?;
      let description: ValidatorDescription = match metadata {
        Some(metadata) => {
          ValidatorDescription {
            moniker: val.clone(),
            identity: metadata.discord_handle,
            website: metadata.website,
            security_contact: Some(metadata.email),
            details: metadata.description,
          }
        }
        None => ValidatorDescription::empty(val)
      };

      let commission = match commission_info {
        Some(commission_info) => {
          CommissionInfo {
            commission_rates: RatesInfo {
              rate: commission_info.commission_rate,
              max_rate: Dec::one(), // no such paramater in Namada
              max_change_rate: commission_info.max_commission_change_per_epoch,
            },
            //TODO: placeholder... how to query this?
            update_time: DEFAULT_TIMESTAMP.to_string(),
          }
        }
        None => CommissionInfo::default()
      };

      let stake = rpc::get_validator_stake(app_state.get_client(), current_epoch, val).await?;

      // Contruct response info
      let validator_info = ValidatorInfo {
        operator_address: val.clone(),
        // TODO: Consensus key
        consensus_pubkey: ConsensusKeyInfo {
          at_type: "placeholder".to_string(),
          key: "placeholder".to_string(),
        },
        jailed,
        status,
        tokens: stake.div(NATIVE_SCALE as u64),
        delegator_shares: stake.to_string_native(),
        description,
        //TODO: how to query this info
        unbonding_height: "0".to_string(),
        unbonding_time: DEFAULT_TIMESTAMP.to_string(),
        commission,
        min_self_delegation: "1".to_string(),
      };
      response.validators.push(validator_info);
    }
  }

  response.pagination = PaginationInfo {
      next_key: None,
      total: Some("1".to_string()), 
    };

  Ok(Json(response))
}

/// Maps Namada validator state to Cosmos validator state
fn map_status_namada_to_cosmos(namada_status: ValidatorState) -> (bool, CosmosValStatus) {
  match namada_status {
    // TODO: how best to map these statuses?
    // do we have to query the chain state to differentiate between UNBONDING and UNBONDED?
    ValidatorState::Consensus => (false, CosmosValStatus::BOND_STATUS_BONDED),
    // ValidatorStates::BelowCapacity =>
    // ValidatorStates::BelowThreshold =>
    // ValidatorStates::Inactive =>
    // ValidatorStates::Jailed =>
    _ => (true, CosmosValStatus::BOND_STATUS_UNBONDED)
  }
}

fn filter_validator_by_status(query_status: Option<CosmosValStatus>, val_status: CosmosValStatus) -> bool {
  match query_status {
    Some(query_status) => query_status == val_status,
    // no status specified; return all
    None => true
  }
}