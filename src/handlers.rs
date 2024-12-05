use crate::metrics::NetworkMetricsData;
use crate::metrics::NodeMetricsData;
use crate::metrics::ValidatorMetricsData;
use crate::namada_query::ValidatorData;
use crate::{metrics::NamadaMetrics, namada_query::ValidatorStake};
use axum::http::header::CONTENT_TYPE;
use axum::{
    body::Body,
    debug_handler,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use namada_sdk::proof_of_stake::PosParams;
use tendermint_rpc::endpoint::status::Response as StatusResponse;

use crate::server::ServerState;
use itertools::Itertools;
use log::{info, debug};
fn process_validator_metrics_data(
    pos_params: &PosParams,
    validator_data: &ValidatorData,
    rank: u32,
) -> ValidatorMetricsData {
    let liveness_window_check = pos_params.owned.liveness_window_check;
    let liveness_threshold: f64 = pos_params
        .owned
        .liveness_threshold
        .to_string()
        .parse::<f64>()
        .expect("Could not parse liveness_threshold");
    // Example: 
    // liveness_window_check = 10000 
    // liveness_threshold = "0.1" 
    // means that you must be live for at least 10% of the most recent 10,000 blocks, if you miss 9000 blocks in a row, then you are automatically jailed.
    let max_block_to_slash = (liveness_window_check as f64) - ((liveness_window_check as f64) * liveness_threshold);
    debug!("Max block to slash: {}", max_block_to_slash);
    let uptime_percentage = match validator_data.missed_blocks {
        Some(missed_blocks) => {
            let uptime = 1.0 - ((missed_blocks as f64) / max_block_to_slash);
            uptime * 100.0
        }
        None => -1.0,
    };
    let state = validator_data.state.number();
    let missed_blocks: i64 = validator_data.missed_blocks.map(|v| v as i64).unwrap_or(-1);
    let commission = validator_data
        .commission
        .commission_rate
        .unwrap()
        .to_string()
        .parse::<f32>()
        .unwrap_or(-1.0);

    ValidatorMetricsData {
        namada_validator_uptime_percentage: uptime_percentage.round() as i64,
        namada_validator_state: state as i64,
        namada_validator_active_set_rank: rank as i64,
        namada_missed_blocks: missed_blocks,
        namada_total_bonds: validator_data.stake.parse().unwrap(),
        validator_commission: commission,
        validator_address_hash: validator_data.address_hash.clone(),
    }
}
fn process_network_metrics(
    epoch: &String,
    response: &StatusResponse,
    sorted_validators: Vec<&ValidatorStake>,
    pos_params: &PosParams,
) -> NetworkMetricsData {
    let lowest_stake = sorted_validators.last().unwrap();
    let network_metrics = NetworkMetricsData {
        namada_network_epoch: epoch.to_string().parse::<i64>().unwrap(),
        namada_node_catch_up: response.sync_info.catching_up as i64,
        namada_network_lowest_active_set_stake: i64::try_from(lowest_stake.stake).unwrap(),
        namada_network_max_set_size: pos_params.owned.max_validator_slots as i64,
        namada_network_stake_threshold: pos_params
            .owned
            .validator_stake_threshold
            .to_string()
            .parse::<i64>()
            .unwrap(),
        namada_network_active_set_size: sorted_validators.len() as i64,
    };
    network_metrics
}
fn process_node_metrics(response: &StatusResponse) -> NodeMetricsData {
    NodeMetricsData {
        namada_node_latest_block: i64::from(response.sync_info.latest_block_height),
        node_id: response.node_info.id.to_string(),
        moniker: response.node_info.moniker.to_string(),
    }
}
#[debug_handler]
pub async fn metrics_handler(State(state): State<ServerState>) -> impl IntoResponse {
    let address = state.config.validator_tm_address.clone();
    let q = &state.q;
    info!("Querying metrics for validator: {}", address);
    info!("Queryring epoch");
    let epoch = q.query_epoch().expect("Could not query epoch");
    debug!("Queries epoch: {}", epoch);
    info!("Querying status");
    let status = q.status().expect("Could not query status");
    debug!("Queries status: {:?}", status);


    info!("Querying validator data");
    let validator = q.query_validators_data(&address).unwrap();
    debug!("Queries validator: {:?}", validator);
    info!("Querying consensus validator set");
    let validators = q.query_consensus_validator_set().unwrap();
    debug!("Queries validators: {:?}", validators);

    info!("Querying pos params");
    let pos_params = q.query_pos_params().unwrap();
    debug!("Queries pos_params: {:?}", pos_params);

    let sorted_validators = validators
        .iter()
        .sorted_by(|a, b| b.stake.cmp(&a.stake))
        .collect::<Vec<_>>();
    let storted_validators_copy: Vec<&ValidatorStake> = sorted_validators.clone();
    let validator_rank = storted_validators_copy
        .iter() // Create an iterator over the vector
        .position(|v| v.address == address) // Find the position of the element matching the condition
        .expect("Can't find validator")
        + 1;


    let validator_data = match validator {
        Some(data) => process_validator_metrics_data(&pos_params, &data, validator_rank as u32),
        None => ValidatorMetricsData::default(),
    };
    let chain_id = status.node_info.network.to_string();
    let metrics = NamadaMetrics::create(chain_id, state.config.validator_tm_address.clone());
    metrics.set_validator_metrics(&validator_data);
    let network_metrics = process_network_metrics(&epoch, &status, sorted_validators, &pos_params);
    metrics.set_network_metrics(&network_metrics);
    let node_metrics = process_node_metrics(&status);
    metrics.set_node_metrics(&node_metrics);

    Response::builder()
        .status(StatusCode::OK)
        .header(
            CONTENT_TYPE,
            "application/openmetrics-text; version=1.0.0; charset=utf-8",
        )
        .body(Body::from(metrics.render()))
        .unwrap()
}
