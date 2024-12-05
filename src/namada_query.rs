use namada_sdk::address::Address;
use namada_sdk::key::PublicKeyTmRawHash;
use namada_sdk::proof_of_stake::types::{
    CommissionPair, ValidatorMetaData, ValidatorState as ValidatorStateType,
};
use namada_sdk::proof_of_stake::{self, PosParams};
use namada_sdk::rpc;
use namada_sdk::rpc::query_storage_value;
use namada_sdk::storage::DbKeySeg;
use namada_sdk::uint::Uint;
use std::error::Error;
use std::str::FromStr;
use tendermint_rpc::endpoint::status::Response;
use tendermint_rpc::{Client, HttpClient, Url};
use tokio::runtime::Handle;
use tokio::task::block_in_place;
#[derive(Debug)]
pub enum ValidatorState {
    Unknown,
    ActiveConsensusSet,
    ActiveBelowCapacitySet,
    ActiveBelowThresholdSet,
    Jailed,
    Inactive,
}

impl ValidatorState {
    pub fn number(&self) -> u8 {
        match self {
            ValidatorState::Unknown => 0,
            ValidatorState::ActiveConsensusSet => 1,
            ValidatorState::ActiveBelowCapacitySet => 2,
            ValidatorState::ActiveBelowThresholdSet => 3,
            ValidatorState::Jailed => 4,
            ValidatorState::Inactive => 5,
        }
    }
}

#[derive(Debug)]

pub struct ValidatorData {
    #[allow(dead_code)]
    pub metadata: Option<ValidatorMetaData>,
    pub stake: String,
    pub commission: CommissionPair,
    pub address_hash: String,
    pub missed_blocks: Option<u64>,
    #[allow(dead_code)]
    pub address: String,
    pub state: ValidatorState,
}
#[derive(Debug)]
pub struct ValidatorStake {
    pub address: String,
    pub stake: Uint,
}

#[derive(Clone)]
pub struct Query {
    client: HttpClient,
}

impl Query {
    pub fn create(rpc: &str) -> Result<Query, Box<dyn Error>> {
        let url = Url::from_str(rpc)?;
        let http_client = HttpClient::new(url)?;
        Ok(Query {
            client: http_client,
        })
    }
    pub fn query_epoch(&self) -> Result<String, Box<dyn Error>> {
        let result = block_in_place(|| {
            let runtime_handle = Handle::current();
            runtime_handle.block_on(rpc::query_epoch(&self.client))
        })?;
        Ok(result.to_string())
    }
    #[allow(dead_code)]
    pub async fn query_epoch_async(&self) -> Result<String, Box<dyn Error>> {
        let epoch = rpc::query_epoch(&self.client).await?;
        Ok(epoch.to_string())
    }

    pub async fn query_validators_async(
        &self,
        address: &str,
    ) -> Result<Option<ValidatorData>, Box<dyn Error>> {
        let addr: Address = Address::from_str(&address)?;
        let commission = rpc::query_commission_rate(&self.client, &addr, None).await?;
        let (validator_metadata_or_none, _) =
            rpc::query_metadata(&self.client, &addr, None).await?;

        let epoch = rpc::query_epoch(&self.client).await?;
        // todo: For some reason to this method we require epoch and not Option(Epoch), but then it's pass as Option. wierd mb need to make PR
        let stake = rpc::get_validator_stake(&self.client, epoch, &addr).await?;
        let liveness_key = proof_of_stake::storage_key::liveness_sum_missed_votes_key();
        let val_key = rpc::query_validator_consensus_keys(&self.client, &addr)
            .await?
            .expect("Could not get validator consensus key");

        let missed_key = liveness_key
            .push(&DbKeySeg::StringSeg("data".to_string()))
            .expect("Could not create storage key")
            .push(&DbKeySeg::AddressSeg(addr.clone()))
            .expect("Could not create storage key");

        let (state_or_none, _) = rpc::get_validator_state(&self.client, &addr, None).await?;
        let missed_block: Result<u64, namada_sdk::error::Error> =
            query_storage_value(&self.client, &missed_key).await;
        let missed_blocks_maybe = match missed_block {
            Ok(missed_blocks) => Some(missed_blocks),
            _ => None,
        };

        let state: ValidatorState = match state_or_none {
            Some(state) => match state {
                ValidatorStateType::Consensus => ValidatorState::ActiveConsensusSet,
                ValidatorStateType::BelowCapacity => ValidatorState::ActiveBelowCapacitySet,
                ValidatorStateType::BelowThreshold => ValidatorState::ActiveBelowThresholdSet,
                ValidatorStateType::Jailed => ValidatorState::Jailed,
                ValidatorStateType::Inactive => ValidatorState::Inactive,
            },
            None => ValidatorState::Unknown,
        };
        let validator_data = ValidatorData {
            commission,
            metadata: validator_metadata_or_none,
            stake: stake.to_string(),
            address_hash: val_key.tm_raw_hash(),
            missed_blocks: missed_blocks_maybe,
            address: address.to_string(),
            state,
        };

        Ok(Some(validator_data))
    }
    pub fn query_validators_data(
        &self,
        address: &str,
    ) -> Result<Option<ValidatorData>, Box<dyn Error>> {
        let result = block_in_place(|| {
            let runtime_handle = Handle::current();
            runtime_handle.block_on(self.query_validators_async(address))
        })?;
        Ok(result)
    }
    pub async fn query_consensus_validator_set_async(
        &self,
    ) -> Result<Vec<ValidatorStake>, Box<dyn Error>> {
        let epoch = rpc::query_epoch(&self.client).await?;
        let consensus_set = rpc::get_all_consensus_validators(&self.client, epoch).await?;
        let result: Vec<_> = consensus_set
            .iter()
            .map(|val| ValidatorStake {
                address: val.address.clone().to_string(),
                stake: val.bonded_stake.clone().into(),
            })
            .collect();

        Ok(result)
    }

    pub fn query_consensus_validator_set(&self) -> Result<Vec<ValidatorStake>, Box<dyn Error>> {
        let result = block_in_place(|| {
            let runtime_handle = Handle::current();
            runtime_handle.block_on(self.query_consensus_validator_set_async())
        })?;
        Ok(result)
    }
    pub async fn query_pos_params_async(&self) -> Result<PosParams, Box<dyn Error>> {
        let result = rpc::get_pos_params(&self.client).await?;
        Ok(result)
    }
    pub fn query_pos_params(&self) -> Result<PosParams, Box<dyn Error>> {
        let result = block_in_place(|| {
            let runtime_handle = Handle::current();
            runtime_handle.block_on(self.query_pos_params_async())
        })?;
        Ok(result)
    }
    pub async fn status_async(&self) -> Result<Response, Box<dyn Error>> {
        let result = self.client.status().await?;
        Ok(result)
    }
    pub fn status(&self) -> Result<Response, Box<dyn Error>> {
        let result = block_in_place(|| {
            let runtime_handle = Handle::current();
            runtime_handle.block_on(self.status_async())
        })?;
        Ok(result)
    }
}
