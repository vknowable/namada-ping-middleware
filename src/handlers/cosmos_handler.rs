// use namada_sdk::{
//     core::types::address::Address,
//     proof_of_stake::types::ValidatorState,
//     rpc,
// };
// use tendermint_rpc::{
//     endpoint::block, Client, Error,
// };
// use tendermint::block::Height;
// use axum::{
//         extract::{Path, State},
//         response::IntoResponse,
//         Json, http::StatusCode,
//     };
// use std::{
//     sync::Arc,
//     str::FromStr,
// };
// use serde_json::json;

// use crate::app::app_state::AppState;
// use crate::error::api_error::ApiError;
// use crate::model::tendermint::{LatestBlockResponse, NodeInfoResponse};

// pub async fn validators_handler(Path(address): Path<String>, State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
//   let validator = Address::from_str(&address).unwrap();
//   let epoch = rpc::query_epoch(app_state.get_client()).await.unwrap();
//   let (metadata, commission_info) = rpc::query_metadata(app_state.get_client(), &validator, None).await.unwrap();
//   let state = rpc::get_validator_state(app_state.get_client(), &validator, None).await.unwrap();
//   let parsed_state: Option<String>;
//   let mut jailed: Option<bool>;
//   match state {
//       Some(s) => {
//           jailed = Some(false);
//           match s {
//               // change to match cosmos equivalent
//               ValidatorState::Consensus => parsed_state = Some("CONSENSUS".to_string()),
//               ValidatorState::BelowCapacity => parsed_state = Some("BELOW_CAPACITY".to_string()),
//               ValidatorState::BelowThreshold => parsed_state = Some("BELOW_THRESHOLD".to_string()),
//               ValidatorState::Inactive => parsed_state = Some("INACTIVE".to_string()),
//               ValidatorState::Jailed => {
//                   parsed_state = Some("JAILED".to_string());
//                   jailed = Some(true);
//               }
//           }
//       }
//       None => {
//           parsed_state = None;
//           jailed = None;
//       }
//   }
//   let bonded_total = rpc::get_validator_stake(app_state.get_client(), epoch, &validator).await.unwrap();

//   let json_response = json!({
//       "validator": {
//           "operator_address": validator.to_string(),
//           "jailed": jailed,
//           "status": parsed_state,
//           "tokens": bonded_total.to_string_native(),
//           "description": {
//               "moniker": metadata.as_ref().unwrap().discord_handle,
//               "identity": null,
//               "website": metadata.as_ref().unwrap().website,
//               "security_contact": metadata.as_ref().unwrap().email,
//               "details": metadata.as_ref().unwrap().description,
//           },
//           "commission": {
//               "commission_rates": {
//                   "rate": commission_info.as_ref().unwrap().commission_rate.to_string(),
//                   "max_rate": null,
//                   "max_change_rate": commission_info.as_ref().unwrap().max_commission_change_per_epoch.to_string(),
//               },
//               "update_time": null
//           },
//           // there are several more fields to go here
//       }
//   });
//   Json(json_response)
// }
