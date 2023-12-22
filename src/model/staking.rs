use serde::Serialize;
use crate::model::shared::PaginationInfo;

#[derive(Serialize)]
pub struct PoolResponse {
  pub pool: PoolInfo,
}

#[derive(Serialize)]
pub struct PoolInfo {
  pub not_bonded_tokens: String,
  pub bonded_tokens: String,
}

#[derive(Serialize)]
pub struct ParamsResponse {
  pub unbonding_time: String,
  pub max_validators: u32,
  pub max_entries: u32,
  pub historical_entries: u32,
  pub bond_denom: String,
  pub validator_bond_factor: String,
  pub global_liquid_staking_cap: String,
  pub validator_liquid_staking_cap: String,
}

#[derive(Serialize)]
pub struct ValidatorsQueryParams {
  pub pagination_limit: Option<u32>,
  pub pagination_offset: Option<u32>,
  pub pagination_reverse: Option<bool>,
  pub status: Option<String>,
}

#[derive(Serialize)]
pub struct ValidatorsResponse {
  pub validators: Vec<ValidatorInfo>,
  pub pagination: PaginationInfo,
}

#[derive(Serialize)]
pub struct ValidatorInfo {
  pub operator_address: String,
  pub consensus_pubkey: ConsensusKeyInfo,
  pub jailed: bool,
  pub status: String, //change to enum
  pub tokens: String,
  pub delegator_shares: String,
  pub description: ValidatorDescription,
  pub unbonding_height: String,
  pub unbonding_time: String,
  pub commission: CommissionInfo,
  pub min_self_delegation: String,
  pub unbonding_on_hold_ref_count: String,
  pub unbonding_ids: Vec<String>,
  pub validator_bond_shares: String,
  pub liquid_shares: String,
}

#[derive(Serialize)]
pub struct ConsensusKeyInfo {
  #[serde(rename = "@type")]
  pub at_type: String,
  pub key: String,
}

#[derive(Serialize)]
pub struct ValidatorDescription {
  pub moniker: String,
  pub identity: String,
  pub website: String,
  pub security_contact: String,
  pub details: String,
}

#[derive(Serialize)]
pub struct CommissionInfo {
  pub commission_rates: RatesInfo,
  pub update_time: String, // time
}

#[derive(Serialize)]
pub struct RatesInfo {
  pub rate: String,
  pub max_rate: String,
  pub max_change_rate: String,
}