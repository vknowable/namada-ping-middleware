use axum::{
  http::{StatusCode, header},
  response::{IntoResponse, Json},
};
use serde::Serialize;
use serde_json::json;
use namada_sdk::error::Error;
use tendermint_rpc::Error as TendermintError;


// TODO: Make sure this error structure matches the Cosmos API error json
/// Error json returned to client when an Api call fails due to e.g. a server side data query error
#[derive(Debug)]
pub struct ApiError {
  pub error: String,
  pub code: StatusCode,
  pub message: Option<String>,
  pub details: Vec<ErrDetails>,
}

impl IntoResponse for ApiError {
  
  fn into_response(self) -> axum::response::Response {

    (
      self.code,
      [(header::CONTENT_TYPE, "application/json")],
      Json(json!({
        "error": self.error,
        "code": self.code.as_u16(),
        "message": self.message,
        "details": self.details,
      }))
    ).into_response()
  }
}

// Conversion from Namada sdk error::Error
// TODO: more informative errors, ie: fill other fields instead of blank/generic strings
impl From<Error> for ApiError {
  fn from(err: Error) -> Self {
    ApiError {
      error: err.to_string(),
      code: StatusCode::INTERNAL_SERVER_ERROR,
      message: Some("An error occurred".to_string()),
      details: vec![ErrDetails {
          type_url: "".to_string(),
          value: "".to_string(),
      }],
    }
  }
}

impl From<TendermintError> for ApiError {
  fn from(err: TendermintError) -> Self {
    ApiError {
      error: err.to_string(),
      code: StatusCode::INTERNAL_SERVER_ERROR,
      message: Some("An error occurred".to_string()),
      details: vec![ErrDetails {
          type_url: "".to_string(),
          value: "".to_string(),
      }],
    }
  }
}

impl Default for ApiError {
  fn default() -> Self {
    ApiError {
      error: "api error".to_string(),
      code: StatusCode::INTERNAL_SERVER_ERROR,
      message: Some("An unknown error occurred".to_string()),
      details: vec![ErrDetails {
          type_url: "".to_string(),
          value: "".to_string(),
      }],
    }
  }
}


#[derive(Debug, Serialize)]
pub struct ErrDetails {
  pub type_url: String,
  pub value: String,
}

// TODO: add a type for when the Cosmos rest endpoint is invalid, eg: 'not implemented'