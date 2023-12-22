use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use axum::{
        routing::get,
        Router,
        http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method},
    };
use tendermint_rpc::Url;
use std::{
    sync::Arc,
    str::FromStr,
};
use dotenv::dotenv;

use namada_ping_middleware::{
    app::app_state,
    handlers::{bank, distribution, gov, mint, slashing, staking, tendermint},
};


#[tokio::main]
async fn main() {
    dotenv().ok();

    let listener: TcpListener = TcpListener::bind("0.0.0.0:1317").await.expect("Could not bind to listen address.");
    let cors = CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET])
        // .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // eg: RPC="http://localhost:26657"
    let rpc: &str = &std::env::var("RPC").expect("RPC must be set");
    let url = Url::from_str(rpc).expect("Invalid RPC address");

    let app_state = Arc::new(app_state::AppState::new(url).await);

    let app: Router = Router::new()
        // .route("/cosmos/staking/v1beta1/validators/:address", get(cosmos_handler::validators_handler))
        .route("/cosmos/bank/v1beta1/supply", get(bank::supply_handler))
        .route("/cosmos/bank/v1beta1/supply/nam", get(bank::supply_denom_handler))
        .route("/cosmos/distribution/v1beta1/community_pool", get(distribution::community_pool_handler))
        .route("/cosmos/distribution/v1beta1/params", get(distribution::distribution_params_handler))
        .route("/cosmos/gov/v1beta1/params/deposit", get(gov::params_deposit_handler))
        .route("/cosmos/gov/v1beta1/params/tallying", get(gov::params_tallying_handler))
        .route("/cosmos/gov/v1beta1/params/voting", get(gov::params_voting_handler))
        .route("/cosmos/gov/v1beta1/proposals", get(gov::all_proposals_handler))
        .route("/cosmos/gov/v1beta1/proposals/:id", get(gov::single_proposal_handler))
        .route("/cosmos/gov/v1beta1/proposals/:id/tally", get(gov::proposal_tally_handler))
        .route("/cosmos/mint/v1beta1/inflation", get(mint::inflation_handler))
        .route("/cosmos/slashing/v1beta1/params", get(slashing::slashing_params_handler))
        .route("/cosmos/slashing/v1beta1/signing_infos", get(slashing::signing_infos_handler))
        .route("/cosmos/staking/v1beta1/params", get(staking::params_handler))
        .route("/cosmos/staking/v1beta1/pool", get(staking::pool_handler))
        .route("/cosmos/staking/v1beta1/validators", get(staking::validators_handler))
        .route("/cosmos/base/tendermint/v1beta1/blocks/latest", get(tendermint::latest_block_handler))
        .route("/cosmos/base/tendermint/v1beta1/node_info", get(tendermint::node_info_handler))
        .route("/cosmos/base/tendermint/v1beta1/validatorsets/", get(tendermint::validator_sets_handler))
        .with_state(app_state)
        .layer(cors);

    println!("Starting server...");
    axum::serve(listener, app).await.unwrap();
}
