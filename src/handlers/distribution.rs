use axum::{
  extract::State,
  Json,
};
use std::sync::Arc;
use namada_sdk::{
  proof_of_stake::PosParams,
  rpc,
  types::{
    token::Amount,
    dec::Dec,
  },
};

use crate::app::app_state::AppState;
use crate::error::api_error::ApiError;
use crate::model::{
  shared::DenomAmount,
  distribution::{CommunityPoolResponse, DistibutionParamsInfo, DistributionParamsResponse},
};


pub async fn community_pool_handler(State(_app_state): State<Arc<AppState>>) 
  -> Result<Json<CommunityPoolResponse>, ApiError> {
  
  // TODO: is there a pgf pool we can query the balance of?
  let amount = Amount::from_u64(0);
  let response = CommunityPoolResponse {
    pool: vec![DenomAmount::nam_amount(amount)],
  };

  Ok(Json(response))
}

pub async fn distribution_params_handler(State(app_state): State<Arc<AppState>>) 
  -> Result<Json<DistributionParamsResponse>, ApiError> {
  
  let pos_params: PosParams = rpc::get_pos_params(app_state.get_client()).await?;

  // TODO: are these values mapped properly from Namada to Cosmos? 
  // eg: should community_tax be set to pgf inflation rate? is base_proposer_reward equivalent to block_vote_reward? etc.
  let community_tax: Dec = Dec::zero();
  let withdraw_addr_enabled: bool = true; // is this constant?
  let base_proposer_reward: Dec = pos_params.owned.block_vote_reward;
  let bonus_proposer_reward: Dec = pos_params.owned.block_proposer_reward;

  let response = DistributionParamsResponse {
    params: DistibutionParamsInfo {
      community_tax,
      base_proposer_reward,
      bonus_proposer_reward,
      withdraw_addr_enabled,
    }
  };

  Ok(Json(response))
}