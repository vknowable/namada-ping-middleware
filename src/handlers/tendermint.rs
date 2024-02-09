use tendermint_rpc::Client;
use tendermint::block::Height;
use axum::{
        extract::{Path, State},
        Json,
    };
use std::sync::Arc;
use namada_sdk::{rpc, types::storage::BlockHeight};
// use hex::FromHex;
// use base64::{encode_config, STANDARD};

use crate::app::app_state::AppState;
use crate::error::api_error::ApiError;
use crate::model::{
    tendermint::{CosmosAppVersion, NodeInfoResponse, ValidatorSetsResponse, ValidatorInfo, BlockResponse},
    staking::ConsensusKeyInfo,
    shared::PaginationInfo,
};

pub async fn latest_block_handler(State(app_state): State<Arc<AppState>>) 
  -> Result<Json<BlockResponse>, ApiError> {

  let latest_block = app_state.get_client().latest_block().await?;
  let response = BlockResponse::from(latest_block);
  Ok(Json(response))
}

pub async fn block_handler(Path(height): Path<u32>, State(app_state): State<Arc<AppState>>) 
  -> Result<Json<BlockResponse>, ApiError> {

  let block = app_state.get_client().block(Height::from(height)).await?;
  let response = BlockResponse::from(block);
  Ok(Json(response))
}

pub async fn node_info_handler(State(app_state): State<Arc<AppState>>)
  -> Result<Json<NodeInfoResponse>, ApiError> {

  let status: tendermint_rpc::endpoint::status::Response = app_state.get_client().status().await?;

  let response = NodeInfoResponse {
      default_node_info: status.node_info.into(),
      // TODO: can git commit, etc be queried from the shell?
      application_version: CosmosAppVersion::new(),
  };

  Ok(Json(response))
}

pub async fn latest_validator_sets_handler(State(app_state): State<Arc<AppState>>) 
  -> Result<Json<ValidatorSetsResponse>, ApiError> {

  // TODO: pagination support
  let height = app_state.get_client().latest_block().await?.block.header.height;
  let current_epoch = rpc::query_epoch(app_state.get_client()).await?;
  let all_vals = rpc::get_all_validators(app_state.get_client(), current_epoch).await?;

  let mut response = ValidatorSetsResponse {
    block_height: height.to_string(),
    validators: Vec::new(),
    pagination: PaginationInfo {
        next_key: None,
        total: Some("1".to_string()),
    }
  };

  for val in &all_vals {
    let stake = rpc::get_validator_stake(app_state.get_client(), current_epoch, val).await?;
    response.validators.push(ValidatorInfo {
      address: val.clone(),
        pub_key: ConsensusKeyInfo {
          //TODO
          at_type: "placeholder".to_string(),
          key: "placeholder".to_string(),
        },
      voting_power: stake,
      //TODO: how to query this?
      proposer_priority: "234141".to_string(),
    })
  }

  Ok(Json(response))
}

pub async fn validator_sets_handler(Path(height): Path<u64>, State(app_state): State<Arc<AppState>>) 
  -> Result<Json<ValidatorSetsResponse>, ApiError> {

  // TODO: pagination support
  let query_epoch = match rpc::query_epoch_at_height(app_state.get_client(), BlockHeight(height)).await? {
    Some(epoch) => epoch,
    None => rpc::query_epoch(app_state.get_client()).await?
  };

  // TODO: this query returns an empty set... not sure why
  // let all_vals = rpc::get_all_validators(app_state.get_client(), query_epoch).await?;
  let current_epoch = rpc::query_epoch(app_state.get_client()).await?; // TEMPORARY
  let all_vals = rpc::get_all_validators(app_state.get_client(), current_epoch).await?; //TEMPORARY

  let mut response = ValidatorSetsResponse {
    block_height: height.to_string(),
    validators: Vec::new(),
    pagination: PaginationInfo {
        next_key: None,
        total: Some("1".to_string()),
    }
  };

  for val in &all_vals {
    let stake = rpc::get_validator_stake(app_state.get_client(), query_epoch, val).await?;
    response.validators.push(ValidatorInfo {
      address: val.clone(),
        pub_key: ConsensusKeyInfo {
          //TODO
          at_type: "placeholder".to_string(),
          key: "placeholder".to_string(),
        },
      voting_power: stake,
      //TODO: how to query this?
      proposer_priority: "234141".to_string(),
    })
  }

  Ok(Json(response))
}
