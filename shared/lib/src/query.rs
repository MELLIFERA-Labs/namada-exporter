
use namada_sdk::address::Address;

use namada_sdk::collections::{HashMap};

use namada_sdk::queries::RPC;
use namada_sdk::rpc::{
    get_public_key_at,
    query_storage_value,
};
use namada_sdk::token;
use namada_sdk::proof_of_stake;
use namada_sdk::proof_of_stake::{OwnedPosParams, Dec};
use namada_sdk::storage::DbKeySeg;
use namada_sdk::key::PublicKeyTmRawHash;

use std::str::FromStr;
use wasm_bindgen::prelude::*;
use namada_sdk::proof_of_stake::types::{CommissionPair, ValidatorMetaData, ValidatorState};

use crate::rpc_client::HttpClient;

use crate::utils::{set_panic_hook, to_js_result};
use serde::Serialize;


#[wasm_bindgen]
/// Represents an API for querying the ledger
pub struct Query {
    client: HttpClient,
}

#[warn(dead_code)]
#[derive(Serialize)]
#[serde(remote = "CommissionPair")]
struct Commission {
    commission_rate: Option<Dec>,
    max_commission_change_per_epoch: Option<Dec>
}

#[derive(Serialize)]
#[serde(remote = "OwnedPosParams")]
#[warn(dead_code)]
struct OwnedPos {
    max_validator_slots: u64,
    pipeline_len: u64,
    unbonding_len: u64,
    tm_votes_per_token: Dec,
    block_proposer_reward: Dec,
    block_vote_reward: Dec,
    max_inflation_rate: Dec,
    target_staked_ratio: Dec,
    duplicate_vote_min_slash_rate: Dec,
    light_client_attack_min_slash_rate: Dec,
    cubic_slashing_window_length: u64,
    validator_stake_threshold: token::Amount,
    liveness_window_check: u64,
    liveness_threshold: Dec,
    rewards_gain_p: Dec,
    rewards_gain_d: Dec
}

#[derive(Serialize)]
#[warn(dead_code)]
struct PosParamsJs {
    #[serde(with = "OwnedPos")]
    owned: OwnedPosParams,
    max_proposal_period: u64
}
#[derive(Serialize)]
struct ConsensusWight {

}
#[derive(Serialize)]
struct ValidatorGeneral {
    address: String,
    #[serde(with = "Commission")]
    commission: CommissionPair,
    stake: String,
    state: String
}

#[derive(Serialize)]
struct CategorizedVals {
    active: Vec<ValidatorGeneral>,
    inactive: Vec<ValidatorGeneral>,
}

#[derive(Serialize)]
struct ValidatorData {
    metadata: Option<ValidatorMetaData>,
    stake: String,
    #[serde(with = "Commission")]
    commission: CommissionPair,
    // consensus_key: ValidatorConsensusKeys,
    address_hash: String,
    missed_blocks: Option<u64>,
    address: String,
    state: String
}

#[wasm_bindgen]
impl Query {
    #[wasm_bindgen(constructor)]
    pub fn new(url: String) -> Query {
        set_panic_hook();
        let client = HttpClient::new(url);
        Query { client }
    }

    pub async fn query_epoch(&self) -> Result<JsValue, JsError> {
        let epoch = RPC.shell().epoch(&self.client).await?;

        to_js_result(epoch)
    }

    pub async fn query_validator_data(&self, address: String) -> Result<JsValue, JsError> {
        let addr: Address = Address::from_str(&address)?;
        let addr_copy = addr.clone();
    
        // Directly await the RPC call
        let commission = RPC.vp().pos().validator_commission(&self.client, &addr, &None).await
            .map_err(|_| JsError::new("Error retrieving commission"))?; // Handle errors here
    
        // At this point, commission is of type CommissionPair
        let stake = RPC.vp().pos().validator_stake(&self.client, &addr, &None).await?;
        let val_metadata = RPC.vp().pos().validator_metadata(&self.client, &addr).await?;
        let val_key = RPC.vp().pos().consensus_key(&self.client, &addr).await?;
        let liveness_key = proof_of_stake::storage_key::liveness_sum_missed_votes_key();
    
        let missed_key = liveness_key
            .push(&DbKeySeg::StringSeg("data".to_string()))
            .expect("Could not create storage key")
            .push(&DbKeySeg::AddressSeg(addr))
            .expect("Could not create storage key");
    
        let missed_blocks: Result<u64, namada_sdk::error::Error> = query_storage_value(&self.client, &missed_key).await;
        let missed_blocks_maybe = match missed_blocks {
            Ok(missed_blocks) => Some(missed_blocks),
            _ => None,
        };
    
        let state = RPC.vp().pos().validator_state(&self.client, &addr_copy, &None).await?;

        // Destructure the tuple
        let (state_option, _epoch) = state; // You can ignore the Epoch part if you don't need it
        
        let final_state = match state_option {
            Some(validator_state) => match validator_state {
                ValidatorState::Consensus => "active_consensus_set".to_string(),
                ValidatorState::BelowCapacity => "active_below_capacity_set".to_string(),
                ValidatorState::BelowThreshold => "active_below_threshold_set".to_string(),
                ValidatorState::Inactive => "inactive".to_string(),
                ValidatorState::Jailed => "jailed".to_string(),
            },
            None => "unknown".to_string(),
        };
        
    
        let validator_data = ValidatorData {
            commission, // Directly use commission since it's already a CommissionPair
            stake: stake.unwrap().to_string(),
            metadata: val_metadata,
            missed_blocks: missed_blocks_maybe,
            address_hash: val_key.unwrap().tm_raw_hash(),
            address,
            state: final_state,
        };
    
        to_js_result(validator_data)
    }
    

   pub async fn query_missed_blocks(&self, address: String) -> Result<JsValue, JsError> {
       let addr: Address = Address::from_str(&address)?;
       let liveness_key = proof_of_stake::storage_key::liveness_sum_missed_votes_key();

       let missed_key = liveness_key
           .push(&DbKeySeg::StringSeg("data".to_string()))
           .expect("Could not create storge key")
           .push(&DbKeySeg::AddressSeg(addr))
           .expect("Could not create storage key");

       let missed_blocks: Result<u64, namada_sdk::error::Error> = query_storage_value(&self.client, &missed_key).await;
       let missed_blocks_maybe  = match missed_blocks {
           Ok(missed_blocks) => Some(missed_blocks),
           _ => None
       };
       to_js_result(missed_blocks_maybe)
   }

   pub async fn query_consensus_validator_set(&self) -> Result<JsValue, JsError> {
       let consensus_set = RPC.vp().pos().consensus_validator_set(&self.client, &None).await?;
       let result: Vec<_> =  consensus_set.iter().map(|val| {
           let mut val_map = HashMap::new();
           val_map.insert("address", val.address.clone().to_string());
           val_map.insert("stake", val.bonded_stake.clone().to_string());
           val_map
       }).collect();
       to_js_result(result)
   }

    pub async fn query_pos_params(&self) -> Result<JsValue, JsError> {
        let pos_params = RPC.vp().pos().pos_params(&self.client).await?;
        let pos_params_js = PosParamsJs {
            owned: pos_params.owned.clone(),
            max_proposal_period: pos_params.max_proposal_period
        };
        to_js_result(pos_params_js)
    }



    /// Gets all active validator addresses
    ///
    /// # Errors
    ///
    /// Returns an error if the RPC call fails
    pub async fn query_all_validator_addresses(&self) -> Result<JsValue, JsError> {
        let validator_addresses = RPC
            .vp()
            .pos()
            .validator_addresses(&self.client, &None)
            .await?;

        to_js_result(validator_addresses)
    }

    /// Gets total bonds by validator address
    ///
    /// # Errors
    ///
    /// Returns an error if the RPC call fails
    pub async fn query_total_bonds(&self, address: String) -> Result<JsValue, JsError> {
        let address = Address::from_str(&address)?;

        let total_bonds = RPC
            .vp()
            .pos()
            .validator_stake(&self.client, &address, &None)
            .await?;

        to_js_result(total_bonds)
    }


    pub async fn query_public_key(&self, address: &str) -> Result<JsValue, JsError> {
        let addr = Address::from_str(address).map_err(JsError::from)?;
        let pk = get_public_key_at(&self.client, &addr, 0).await?;

        let result = match pk {
            Some(v) => Some(v.to_string()),
            None => None,
        };

        to_js_result(result)
    }
}
