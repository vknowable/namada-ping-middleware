use serde::{Serialize, Serializer, Deserialize};
use namada_sdk::types::token::Amount;
use std::time::Duration;
use tendermint_proto::google::protobuf::Timestamp;

pub const NAM: &str = "nam";
pub const DEFAULT_TIMESTAMP: &str = "1970-01-01T00:00:00Z";
pub const ZERO_TIMESTAMP: Timestamp = Timestamp {
  seconds: -62135596800,
  nanos: 0,
};

#[derive(Deserialize)]
pub struct PaginationQueryParams {
  pub limit: Option<u32>,
  pub offset: Option<u32>,
  pub count_total: Option<bool>,
  pub reverse: Option<bool>,
}

#[derive(Serialize)]
pub struct PaginationInfo {
  pub next_key: Option<String>,
  pub total: Option<String>,
}

impl Default for PaginationInfo {
  fn default() -> Self {
    PaginationInfo {
      next_key: None,
      total: None,
    }
  }
}

#[derive(Serialize)]
pub struct DenomAmount {
  pub denom: String,
  pub amount: String,
}

impl DenomAmount {
  pub fn nam_amount(amount: Amount) -> Self {
    DenomAmount {
      denom: NAM.to_string(),
      amount: amount.to_string_native(),
    }
  }
}

pub struct SuffixedDur(pub Duration);

impl Serialize for SuffixedDur {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer {
      
      let seconds = self.0.as_secs();
      let formatted_duration = format!("{}s", seconds);

      serializer.serialize_str(&formatted_duration)
  }
}