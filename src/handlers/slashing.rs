use axum::{
  extract::State,
  Json,
};
use std::sync::Arc;

use crate::app::app_state::AppState;
use crate::error::api_error::ApiError;
use crate::model::{
  slashing::{SigningInfos, SigningInfosResponse, SlashingParamsInfo, SlashingParamsResponse},
  shared::PaginationInfo,
};


pub async fn slashing_params_handler(State(_app_state): State<Arc<AppState>>) 
  -> Result<Json<SlashingParamsResponse>, ApiError> {
  
  let response = SlashingParamsResponse {
    params: SlashingParamsInfo {
      signed_blocks_window: "1000".to_string(),
      min_signed_per_window: "0.0500000".to_string(),
      downtime_jail_duration: "600s".to_string(),
      slash_fraction_double_sign: "0.050000000000000".to_string(),
      slash_fraction_downtime: "0.00010000".to_string(),
    },
  };

  Ok(Json(response))
}

pub async fn signing_infos_handler(State(_app_state): State<Arc<AppState>>) 
  -> Result<Json<SigningInfosResponse>, ApiError> {
  
  let response = SigningInfosResponse {
    info: vec![SigningInfos {
      address: "tnam1234".to_string(),
      start_height: "0".to_string(),
      index_offset: "23414".to_string(),
      jailed_until: "1970-01-01T00:00:00Z".to_string(),
      tombstoned: false,
      missed_blocks_counter: "2".to_string(),
    }],
    pagination: PaginationInfo {
      next_key: None,
      total: Some("1".to_string()),
    }
  };

  Ok(Json(response))
}