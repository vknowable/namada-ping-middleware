use serde::{Serialize, Deserialize};
use namada_sdk::types::dec::Dec;
// use namada_sdk::core::ledger::governance::utils::ProposalStatus;
use crate::model::shared::{DenomAmount, SuffixedDur, PaginationQueryParams, PaginationInfo};
use std::time::Duration;
use std::fmt::Display;


#[derive(Deserialize)]
pub struct ProposalQueryParams {
  pub proposal_status: Option<u32>,
  pub pagination: Option<PaginationQueryParams>,
}

#[derive(Serialize)]
pub struct ParamsGovResponse {
  pub voting_params: VotingParams,
  pub deposit_params: DepositParams,
  pub tally_params: TallyParams,
}

impl Default for ParamsGovResponse {
  fn default() -> Self {
    ParamsGovResponse {
      voting_params: VotingParams { voting_period: SuffixedDur(Duration::from_secs(0)) },
      deposit_params: DepositParams { min_deposit: Vec::new(), max_deposit_period: "0s".to_string() },
      tally_params: TallyParams {
        quorum: Dec::zero(),
        threshold: Dec::zero(),
        veto_threshold: Dec::zero(),
      }
    }
  }
}

#[derive(Serialize)]
pub struct VotingParams {
  pub voting_period: SuffixedDur,
}

#[derive(Serialize)]
pub struct DepositParams {
  pub min_deposit: Vec<DenomAmount>,
  pub max_deposit_period: String,
}

#[derive(Serialize)]
pub struct TallyParams {
  pub quorum: Dec,
  pub threshold: Dec,
  pub veto_threshold: Dec,
}

#[derive(Serialize)]
pub struct ProposalsResponse {
  pub proposals: Vec<ProposalItem>,
  pub pagination: Option<PaginationInfo>,
}

#[derive(Serialize)]
pub struct IndividualProposalResponse {
  pub proposal: Option<ProposalItem>,
}

#[derive(Serialize)]
pub struct ProposalItem {
  pub proposal_id: String,
  pub content: ProposalInfo,
  pub status: CosmosProposalStatus,
  pub final_tally_result: FinalTallyInfo,
  pub submit_time: String, //time
  pub deposit_end_time: String, //time
  pub total_deposit: Vec<DenomAmount>,
  pub voting_start_time: String, //time
  pub voting_end_time: String, //time
}

// TODO: in Cosmos there are different proposal types (text, community spend, parameter change, etc)
// included fields differ between each
// furthermore, the types don't correspond easily to Namada proposal types/info
#[derive(Serialize)]
pub struct ProposalInfo {
  #[serde(rename = "@type")]
  pub at_type: String,
  pub title: String,
  pub description: String,
  pub recipient: Option<String>, //address
  pub amount: Option<Vec<DenomAmount>>,
}

#[derive(Serialize)]
pub struct FinalTallyInfo {
  pub yes: String,
  pub abstain: String, // does not exist in Namada
  pub no: String,
  pub no_with_veto: String, // does not exist in Namada
}

impl Default for FinalTallyInfo {
  fn default() -> Self {
      FinalTallyInfo {
        yes: "0".to_string(),
        abstain: "0".to_string(),
        no: "0".to_string(),
        no_with_veto: "0".to_string(),
      }
  }
}

#[derive(Serialize)]
pub struct TallyResponse {
  pub tally: FinalTallyInfo,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Clone, Copy)]
pub enum CosmosProposalStatus {
  PROPOSAL_STATUS_DEPOSIT_PERIOD,
  PROPOSAL_STATUS_VOTING_PERIOD,
  PROPOSAL_STATUS_PASSED,
  PROPOSAL_STATUS_REJECTED,
  PROPOSAL_STATUS_FAILED,
}

impl From<CosmosProposalStatus> for u32 {
  fn from(status: CosmosProposalStatus) -> Self {
      match status {
        CosmosProposalStatus::PROPOSAL_STATUS_DEPOSIT_PERIOD => 1,
        CosmosProposalStatus::PROPOSAL_STATUS_VOTING_PERIOD => 2,
        CosmosProposalStatus::PROPOSAL_STATUS_PASSED => 3,
        CosmosProposalStatus::PROPOSAL_STATUS_REJECTED => 4,
        CosmosProposalStatus::PROPOSAL_STATUS_FAILED => 5,
      }
  }
}

// impl From<ProposalStatus> for CosmosProposalStatus {
//   fn from(status: ProposalStatus) -> Self {
//     match status {
//       ProposalStatus::Pending => CosmosProposalStatus::PROPOSAL_STATUS_DEPOSIT_PERIOD,
//       ProposalStatus::OnGoing => CosmosProposalStatus::PROPOSAL_STATUS_VOTING_PERIOD,
//       ProposalStatus::Ended => CosmosProposalStatus::PROPOSAL_STATUS_PASSED, // incorrect: Need to check if passed or rejected!
//     }
//   }
// }

impl Display for CosmosProposalStatus {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
          CosmosProposalStatus::PROPOSAL_STATUS_DEPOSIT_PERIOD => write!(f, "PROPOSAL_STATUS_DEPOSIT_PERIOD"),
          CosmosProposalStatus::PROPOSAL_STATUS_VOTING_PERIOD => write!(f, "PROPOSAL_STATUS_VOTING_PERIOD"),
          CosmosProposalStatus::PROPOSAL_STATUS_PASSED => write!(f, "PROPOSAL_STATUS_PASSED"),
          CosmosProposalStatus::PROPOSAL_STATUS_REJECTED => write!(f, "PROPOSAL_STATUS_REJECTED"),
          CosmosProposalStatus::PROPOSAL_STATUS_FAILED => write!(f, "PROPOSAL_STATUS_FAILED"),
      }
  }
}