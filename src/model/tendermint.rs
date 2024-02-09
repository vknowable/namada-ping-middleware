use std::str::FromStr;

use serde::Serialize;
use tendermint::{node, channel, chain, Version, Time, Hash, block::Commit};
use tendermint_rpc::endpoint::block;
use tendermint_proto::{google::protobuf::Timestamp, types::CommitSig};
use namada_sdk::types::{token::Amount, address::Address};
use base64::{encode_config, STANDARD};

use crate::model::{shared::{ZERO_TIMESTAMP, PaginationInfo}, staking::ConsensusKeyInfo};


#[derive(Serialize)]
pub struct BlockResponse {
  pub block_id: BlockId,
  pub block: Block,
}

#[derive(Serialize)]
pub struct BlockId {
  pub hash: String,
  pub part_set_header: PartSetHeader,
}

#[derive(Serialize)]
pub struct PartSetHeader {
  pub total: u32,
  pub hash: String,
}

#[derive(Serialize)]
pub struct Block {
  pub header: BlockHeader,
  pub data: BlockData,
  pub evidence: BlockEvidence,
  pub last_commit: Option<LastCommit>,
}

#[derive(Serialize)]
pub struct BlockHeader {
  pub version: HeaderVersion,
  pub chain_id: chain::Id,
  pub height: tendermint::block::Height,
  pub time: Time,
  pub last_block_id: Option<BlockId>,
  pub last_commit_hash: Option<String>,
  pub data_hash: Option<String>,
  pub validators_hash: String,
  pub next_validators_hash: String,
  pub consensus_hash: String,
  pub app_hash: String,
  pub last_results_hash: Option<String>,
  pub evidence_hash: Option<String>,
  pub proposer_address: String,
}

#[derive(Serialize)]
pub struct HeaderVersion {
  pub block: u64,
  pub app: u64,
}

#[derive(Serialize)]
pub struct BlockData {
  pub txs: Vec<String>,
}

#[derive(Serialize)]
pub struct BlockEvidence {
  pub evidence: Vec<String>,
}

#[derive(Serialize)]
pub struct LastCommit {
  pub height: tendermint::block::Height,
  pub round: u32,
  pub block_id: BlockId,
  pub signatures: Vec<Signature>,
}

impl From<Commit> for LastCommit {
  fn from(value: Commit) -> Self {
    let block_id = BlockId {
      hash: hash_to_base64string(value.block_id.hash),
      part_set_header: PartSetHeader {
        total: value.block_id.part_set_header.total,
        hash: hash_to_base64string(value.block_id.part_set_header.hash),
      }
    };

    let mut last_commit = LastCommit {
      height: value.height,
      round: value.round.into(),
      block_id,
      signatures: Vec::new()
    };

    for sig in value.signatures.iter() {
      let raw_sig: CommitSig = CommitSig::from(sig.clone());
      let timestamp = match raw_sig.timestamp {
        Some(timestamp) => timestamp,
        None => ZERO_TIMESTAMP
      };

      last_commit.signatures.push(Signature {
        block_id_flag: raw_sig.block_id_flag.to_string(),
        validator_address: encode_config(&raw_sig.validator_address, STANDARD),
        timestamp,
        signature: encode_config(&raw_sig.signature, STANDARD),
      });
    }

    return last_commit
  }
}

#[derive(Serialize)]
pub struct Signature {
  pub block_id_flag: String, //make enum
  pub validator_address: String,
  pub timestamp: Timestamp,
  pub signature: String,
}

// There are some minor differences between the block object returned by Tendermint crate and the format
// expected by Ping.pub, so we need this type conversion
impl From<block::Response> for BlockResponse {
  fn from(value: block::Response) -> Self {
    
    let block_id = BlockId {
      hash: hash_to_base64string(value.block_id.hash),
      part_set_header: PartSetHeader {
        total: value.block_id.part_set_header.total,
        hash: hash_to_base64string(value.block_id.part_set_header.hash),
      }
    };

    let last_block_id = match value.block.header.last_block_id {
      Some(id) => {
        Some(BlockId {
          hash: hash_to_base64string(id.hash),
          part_set_header: PartSetHeader {
            total: id.part_set_header.total,
            hash: hash_to_base64string(id.part_set_header.hash),
          }
        })
      }
      None => None,
    };

    let version = HeaderVersion {
      block: value.block.header.version.block,
      app: value.block.header.version.app,
    };

    let app_hash_string = value.block.header.app_hash.to_string();
    let app_hash = Hash::from_str(&app_hash_string).unwrap();

    let proposer_bytes = value.block.header.proposer_address.as_bytes();
    let proposer_address = encode_config(&proposer_bytes, STANDARD);

    let mut data = BlockData {
      txs: Vec::new(),
    };
    for tx in value.block.data.iter() {
      data.txs.push(encode_config(&tx, STANDARD));
    }

    //TODO: add support for evidence
    let evidence = BlockEvidence {
      evidence: Vec::new(),
    };

    let last_commit = match value.block.last_commit {
      Some(last_commit) => Some(LastCommit::from(last_commit)),
      None => None,
    };

    let block = Block {
      header: BlockHeader {
        version,
        chain_id: value.block.header.chain_id,
        height: value.block.header.height,
        time: value.block.header.time,
        last_block_id,
        last_commit_hash: Some(hash_to_base64string(value.block.header.last_commit_hash.unwrap_or(Hash::None))),
        data_hash: Some(hash_to_base64string(value.block.header.data_hash.unwrap_or(Hash::None))),
        validators_hash: hash_to_base64string(value.block.header.validators_hash),
        next_validators_hash: hash_to_base64string(value.block.header.next_validators_hash),
        consensus_hash: hash_to_base64string(value.block.header.consensus_hash),
        app_hash: hash_to_base64string(app_hash),
        last_results_hash: Some(hash_to_base64string(value.block.header.last_results_hash.unwrap_or(Hash::None))),
        evidence_hash: Some(hash_to_base64string(value.block.header.evidence_hash.unwrap_or(Hash::None))),
        proposer_address,
      },
      data,
      evidence,
      last_commit,
    };
    BlockResponse {block_id, block }
  }
}

fn hash_to_base64string(hash: Hash) -> String {
  let bytes = hash.as_bytes();
  encode_config(&bytes, STANDARD)
}

#[derive(Serialize)]
pub struct NodeInfoResponse {
  pub default_node_info: CosmosNodeInfo,
  pub application_version: CosmosAppVersion,
}

#[derive(Serialize)]
pub struct CosmosNodeInfo {
  pub protocol_version: node::info::ProtocolVersionInfo,
  pub default_node_id: node::Id,
  pub listen_addr: node::info::ListenAddress,
  pub network: chain::Id,
  pub version: Version,
  pub channels: channel::Channels,
  pub moniker: tendermint::Moniker,
  pub other: node::info::OtherInfo,
}

impl From<node::Info> for CosmosNodeInfo {
  fn from(info: node::Info) -> Self {
    CosmosNodeInfo {
      protocol_version: info.protocol_version,
      default_node_id: info.id,
      listen_addr: info.listen_addr,
      network: info.network,
      version: info.version,
      channels: info.channels,
      moniker: info.moniker,
      other: info.other,
    }
  }
}

#[derive(Serialize)]
pub struct CosmosAppVersion {
  name: String,
  app_name: String,
  version: String,
  git_commit: String,
  build_tags: Option<String>,
  go_version: Option<String>,
  build_deps: Option<Vec<BuildDep>>,
}

impl CosmosAppVersion {
  pub fn new() -> Self {
    CosmosAppVersion {
      name: "namada".to_string(),
      app_name: "namadan".to_string(),
      version: "placeholder".to_string(),
      git_commit: "placeholder".to_string(),
      build_tags: Some("placeholder".to_string()),
      go_version: Some("placeholder".to_string()),
      build_deps: None,
    }
  }
}

#[derive(Serialize)]
struct BuildDep {
  path: String,
  version: String,
  sum: Option<String>,
}

#[derive(Serialize)]
pub struct ValidatorSetsResponse {
  pub block_height: String,
  pub validators: Vec<ValidatorInfo>,
  pub pagination: PaginationInfo,
}

#[derive(Serialize)]
pub struct ValidatorInfo {
  pub address: Address,
  pub pub_key: ConsensusKeyInfo,
  pub voting_power: Amount,
  pub proposer_priority: String,
}