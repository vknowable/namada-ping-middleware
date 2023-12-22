use axum::{
  extract::State,
  Json,
};
use std::sync::Arc;

use crate::app::app_state::AppState;
use crate::error::api_error::ApiError;
use crate::model::{
  staking::{PoolResponse, PoolInfo, ParamsResponse, ValidatorsResponse, ValidatorInfo, ValidatorDescription, ConsensusKeyInfo, CommissionInfo, RatesInfo},
  shared::PaginationInfo,
};


pub async fn pool_handler(State(_app_state): State<Arc<AppState>>) 
  -> Result<Json<PoolResponse>, ApiError> {

  let response = PoolResponse {
    pool: PoolInfo { not_bonded_tokens: "1000".to_string(), bonded_tokens: "100".to_string() },
  };

  Ok(Json(response))
}

pub async fn params_handler(State(_app_state): State<Arc<AppState>>) 
  -> Result<Json<ParamsResponse>, ApiError> {
  
  let response = ParamsResponse {
    unbonding_time: "100s".to_string(),
    max_validators: 100,
    max_entries: 7,
    historical_entries: 1000,
    bond_denom: "nam".to_string(),
    validator_bond_factor: "100".to_string(),
    global_liquid_staking_cap: "0.25000".to_string(),
    validator_liquid_staking_cap: "1.000".to_string(),
  };

  Ok(Json(response))
}

pub async fn validators_handler(State(_app_state): State<Arc<AppState>>) 
  -> Result<Json<ValidatorsResponse>, ApiError> {
  
  let response = ValidatorsResponse {
    validators: vec![ValidatorInfo {
      operator_address: "tnam2323.".to_string(),
      consensus_pubkey: ConsensusKeyInfo {
        at_type: "placeholder".to_string(),
        key: "placeholder".to_string(),
      },
      jailed: false,
      status: "BOND_STATUS_BONDED".to_string(),
      tokens: "1000".to_string(),
      delegator_shares: "10000".to_string(),
      description: ValidatorDescription {
        moniker: "TestVal".to_string(),
        identity: "".to_string(),
        website: "".to_string(),
        security_contact: "".to_string(),
        details: "A test validator".to_string(),
      },
      unbonding_height: "50000".to_string(),
      unbonding_time: "2023-09-30T06:17:37.572905825Z".to_string(),
      commission: CommissionInfo {
        commission_rates: RatesInfo {
          rate: "0.05000".to_string(),
          max_rate: "0.20000".to_string(),
          max_change_rate: "0.010000".to_string(),
        },
        update_time: "2023-09-30T06:17:37.572905825Z".to_string(),
      },
      min_self_delegation: "1".to_string(),
      unbonding_on_hold_ref_count: "0".to_string(),
      unbonding_ids: Vec::new(),
      validator_bond_shares: "500000".to_string(),
      liquid_shares: "4141341414.000000".to_string(),
    }],
    pagination: PaginationInfo {
      next_key: None,
      total: Some("1".to_string()), 
    }
  };

  Ok(Json(response))
}