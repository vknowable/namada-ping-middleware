use serde::Serialize;
use crate::model::shared::{PaginationInfo, DenomAmount};


#[derive(Serialize)]
pub struct SupplyDenomResponse {
  pub amount: DenomAmount,
}

#[derive(Serialize)]
pub struct SupplyResponse {
  pub supply: Vec<DenomAmount>,
  pub pagination: PaginationInfo,
}