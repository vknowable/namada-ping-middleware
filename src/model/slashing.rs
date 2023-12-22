use serde::Serialize;
use crate::model::shared::PaginationInfo;


#[derive(Serialize)]
pub struct SlashingParamsResponse {
  pub params: SlashingParamsInfo,
}

#[derive(Serialize)]
pub struct SlashingParamsInfo {
  pub signed_blocks_window: String,
  pub min_signed_per_window: String,
  pub downtime_jail_duration: String,
  pub slash_fraction_double_sign: String,
  pub slash_fraction_downtime: String,
}

#[derive(Serialize)]
pub struct SigningInfosResponse {
  pub info: Vec<SigningInfos>,
  pub pagination: PaginationInfo,
}

#[derive(Serialize)]
pub struct SigningInfos {
  pub address: String,
  pub start_height: String,
  pub index_offset: String,
  pub jailed_until: String, //time
  pub tombstoned: bool,
  pub missed_blocks_counter: String,
}