use serde::Serialize;

#[derive(Serialize)]
pub struct InflationResponse {
  pub inflation: String,
}