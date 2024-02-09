use axum::{
  extract::State,
  Json,
};
use std::sync::Arc;

use crate::app::app_state::AppState;
use crate::error::api_error::ApiError;
use crate::model::{
  bank::{SupplyResponse, SupplyDenomResponse},
  shared::{DenomAmount, PaginationInfo},
};


pub async fn supply_handler(State(_app_state): State<Arc<AppState>>) 
  -> Result<Json<SupplyResponse>, ApiError> {
  
  let response = SupplyResponse {
    supply: vec![DenomAmount {
      denom: "tnam1234".to_string(),
      amount: "1000".to_string(),
    }],
    pagination: PaginationInfo {
      next_key: None,
      total: Some("1".to_string()),
    }
};

  Ok(Json(response))
}

pub async fn supply_denom_handler(State(_app_state): State<Arc<AppState>>) 
  -> Result<Json<SupplyDenomResponse>, ApiError> {
  
  let response = SupplyDenomResponse {
    amount: DenomAmount {
      denom: "tnam1234".to_string(),
      amount: "1000".to_string(),
    },
};

  Ok(Json(response))
}