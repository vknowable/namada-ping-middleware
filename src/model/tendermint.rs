use serde::Serialize;
use tendermint::{node, Version, channel, chain};
use tendermint_rpc::endpoint::block;

use crate::model::{shared::PaginationInfo, staking::ConsensusKeyInfo};


#[derive(Serialize)]
pub struct LatestBlockResponse {
    pub block: block::Response,
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
    pub address: String,
    pub pub_key: ConsensusKeyInfo,
    pub voting_power: String,
    pub proposer_priority: String,
}