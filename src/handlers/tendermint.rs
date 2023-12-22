use tendermint_rpc::{
    endpoint::block, Client,
};
use axum::{
        extract::State,
        Json,
    };
use std::sync::Arc;

use crate::app::app_state::AppState;
use crate::error::api_error::ApiError;
use crate::model::{
    tendermint::{CosmosAppVersion, NodeInfoResponse, ValidatorSetsResponse, ValidatorInfo},
    staking::ConsensusKeyInfo,
    shared::PaginationInfo,
};

// TODO: there are some differences between tendermint block response and cosmos block response not accounted for here
pub async fn latest_block_handler(State(app_state): State<Arc<AppState>>) 
  -> Result<Json<block::Response>, ApiError> {

  let latest_block = app_state.get_client().latest_block().await?;
  Ok(Json(latest_block))
}

pub async fn node_info_handler(State(app_state): State<Arc<AppState>>)
  -> Result<Json<NodeInfoResponse>, ApiError> {

  let status: tendermint_rpc::endpoint::status::Response = app_state.get_client().status().await?;

  let response = NodeInfoResponse {
      default_node_info: status.node_info.into(),
      application_version: CosmosAppVersion::new(),
  };

  Ok(Json(response))
}

pub async fn validator_sets_handler(State(_app_state): State<Arc<AppState>>) 
  -> Result<Json<ValidatorSetsResponse>, ApiError> {
  
  let response = ValidatorSetsResponse {
    block_height: "12860".to_string(),
    validators: vec![ValidatorInfo {
        address: "tnam1234".to_string(),
        pub_key: ConsensusKeyInfo {
            at_type: "placeholder".to_string(),
            key: "placeholder".to_string(),
        },
        voting_power: "1232".to_string(),
        proposer_priority: "234141".to_string(),
    }],
    pagination: PaginationInfo {
        next_key: None,
        total: Some("1".to_string()),
    }
  };

  Ok(Json(response))
}
