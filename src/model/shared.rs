use serde::{Serialize, Serializer, Deserialize};
use namada_sdk::core::types::token::Amount;
use std::time::Duration;

pub const NAM: &str = "nam";

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