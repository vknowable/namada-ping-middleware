use axum::{
  extract::State,
  Json,
};
use std::sync::Arc;

use crate::app::app_state::AppState;
use crate::error::api_error::ApiError;
use crate::model::mint::InflationResponse;


pub async fn inflation_handler(State(_app_state): State<Arc<AppState>>) 
  -> Result<Json<InflationResponse>, ApiError> {
  
  let response = InflationResponse {
    inflation: "0.1200000".to_string(),
  };

  Ok(Json(response))
}