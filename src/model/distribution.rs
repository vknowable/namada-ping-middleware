use serde::Serialize;
use namada_sdk::types::dec::Dec;

use crate::model::shared::DenomAmount;

#[derive(Serialize)]
pub struct CommunityPoolResponse {
  pub pool: Vec<DenomAmount>,
}

#[derive(Serialize)]
pub struct DistributionParamsResponse {
  pub params: DistibutionParamsInfo,
}

#[derive(Serialize)]
pub struct DistibutionParamsInfo {
  pub community_tax: Dec,
  pub base_proposer_reward: Dec,
  pub bonus_proposer_reward: Dec,
  pub withdraw_addr_enabled: bool,
}