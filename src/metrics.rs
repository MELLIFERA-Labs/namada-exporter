use prometheus_client::encoding::text::encode;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;
use std::sync::atomic::AtomicU64;

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct ValidatorLabels {
    chain_id: String,
    validator_tm_address: String,
    validator_hash_address: String,
}
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct NetworkLabels {
    chain_id: String,
}
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct NodeLabels {
    chain_id: String,
    node_id: String,
    moniker: String,
}

#[derive(Debug)]
pub struct ValidatorMetricsData {
    pub namada_validator_uptime_percentage: i64,
    pub namada_validator_state: i64,
    pub namada_validator_active_set_rank: i64,
    pub namada_missed_blocks: i64,
    pub namada_total_bonds: i64,
    pub validator_commission: f32,
    pub validator_address_hash: String,
}

impl ValidatorMetricsData {
    pub fn default() -> Self {
        ValidatorMetricsData {
            namada_validator_uptime_percentage: -1,
            namada_validator_state: -1,
            namada_validator_active_set_rank: -1,
            namada_missed_blocks: -1,
            namada_total_bonds: -1,
            validator_commission: 0.0,
            validator_address_hash: "".to_string(),
        }
    }
}

pub struct NetworkMetricsData {
    pub namada_network_epoch: i64,
    pub namada_node_catch_up: i64,
    pub namada_network_lowest_active_set_stake: i64,
    pub namada_network_max_set_size: i64,
    pub namada_network_stake_threshold: i64,
    pub namada_network_active_set_size: i64,
}
pub struct NodeMetricsData {
    pub namada_node_latest_block: i64,
    pub node_id: String,
    pub moniker: String,
}
pub struct ValidatorMetricInt {
    name: String,
    help: String,
    metric: Family<ValidatorLabels, Gauge>,
}
pub struct ValidatorMetricFloat {
    name: String,
    help: String,
    metric: Family<ValidatorLabels, Gauge<f64, AtomicU64>>,
}

pub struct NetworkMetricInt {
    name: String,
    help: String,
    metric: Family<NetworkLabels, Gauge>,
}

pub struct NodeMetricInt {
    name: String,
    help: String,
    metric: Family<NodeLabels, Gauge>,
}

pub struct Metrics {
    namada_validator_uptime_percentage: ValidatorMetricInt,
    namada_validator_state: ValidatorMetricInt,
    namada_validator_active_set_rank: ValidatorMetricInt,
    namada_missed_blocks: ValidatorMetricInt,
    namada_total_bonds: ValidatorMetricInt,
    validator_commission: ValidatorMetricFloat,
    namada_network_epoch: NetworkMetricInt,
    namada_node_catch_up: NetworkMetricInt,
    namada_network_lowest_active_set_stake: NetworkMetricInt,
    namada_network_max_set_size: NetworkMetricInt,
    namada_network_stake_threshold: NetworkMetricInt,
    namada_network_active_set_size: NetworkMetricInt,
    namada_node_latest_block: NodeMetricInt,
    namada_validator_missed_blocks: NetworkMetricInt,
}
pub struct NamadaMetrics {
    chain_id: String,
    registry: Registry,
    tm_address: String,
    metrics: Metrics,
}
impl NamadaMetrics {
    pub fn create(chain_id: String, tm_address: String) -> Self {
        let mut registry = Registry::default();
        let metric = Metrics {
            namada_validator_uptime_percentage: ValidatorMetricInt {
                name: "namada_validator_uptime_percentage".to_string(),
                help: "Validator uptime in percentage; -1 value if validator not in active set".to_string(),
                metric: Family::<ValidatorLabels, Gauge>::default(),
            },
            namada_validator_state: ValidatorMetricInt {
                name: "namada_validator_state".to_string(),
                help: "Validator state; 0 - unknown, 1 - active consensus set, 2 - active below capacity set, 3 - active below threshold set, 4 - jailed, 5 - inactive".to_string(),
                metric: Family::<ValidatorLabels, Gauge>::default(),
            },
            namada_validator_active_set_rank: ValidatorMetricInt {
                name: "namada_validator_active_set_rank".to_string(),
                help: "Validator active set rank, -1 value if not in active set".to_string(),
                metric: Family::<ValidatorLabels, Gauge>::default(),
            },
            namada_missed_blocks: ValidatorMetricInt {
                name: "namada_validator_missed_blocks".to_string(),
                help: "Validator missed blocks in liveness window; -1 value if not in active set".to_string(),
                metric: Family::<ValidatorLabels, Gauge>::default(),
            },
            namada_total_bonds: ValidatorMetricInt {
                name: "namada_validator_total_bonds".to_string(),
                help: "Validator total bonds".to_string(),
                metric: Family::<ValidatorLabels, Gauge>::default(),
            },
            validator_commission: ValidatorMetricFloat {
                name: "namada_validator_commission".to_string(),
                help: "Validator commission".to_string(),
                metric: Family::<ValidatorLabels, Gauge::<f64, AtomicU64>>::default(),
            },
            namada_network_epoch: NetworkMetricInt {
                name: "namada_network_epoch".to_string(),
                help: "Current network epoch".to_string(),
                metric: Family::<NetworkLabels, Gauge>::default(),
            },
            namada_node_catch_up: NetworkMetricInt {
                name: "namada_node_catch_up".to_string(),
                help: "Validator catch up status; 0 - not catching up, 1 - catching up".to_string(),
                metric: Family::<NetworkLabels, Gauge>::default(),
            },
            namada_network_lowest_active_set_stake: NetworkMetricInt {
                name: "namada_network_lowest_active_set_stake".to_string(),
                help: "Lowest active set stake".to_string(),
                metric: Family::<NetworkLabels, Gauge>::default(),
            },
            namada_network_max_set_size: NetworkMetricInt {
                name: "namada_network_max_set_size".to_string(),
                help: "Max set size".to_string(),
                metric: Family::<NetworkLabels, Gauge>::default(),
            },
            namada_network_stake_threshold: NetworkMetricInt {
                name: "namada_network_stake_threshold".to_string(),
                help: "Stake threshold".to_string(),
                metric: Family::<NetworkLabels, Gauge>::default(),
            },
            namada_network_active_set_size: NetworkMetricInt {
                name: "namada_network_active_set_size".to_string(),
                help: "Active set size".to_string(),
                metric: Family::<NetworkLabels, Gauge>::default(),
            },
            namada_validator_missed_blocks: NetworkMetricInt {
                name: "namada_validator_node_latest_block".to_string(),
                help: "Latest block from rpc. This metric is deprecated and will be removed in future versions please use namada_node_latest_block".to_string(),
                metric: Family::<NetworkLabels, Gauge>::default(),
            },
            namada_node_latest_block: NodeMetricInt {
                name: "namada_node_latest_block".to_string(),
                help: "Latest block from rpc".to_string(),
                metric: Family::<NodeLabels, Gauge>::default(),
            },
        };
        registry.register(
            metric.namada_validator_uptime_percentage.name.as_str(),
            metric.namada_validator_uptime_percentage.help.as_str(),
            metric.namada_validator_uptime_percentage.metric.clone(),
        );
        registry.register(
            metric.namada_validator_state.name.as_str(),
            metric.namada_validator_state.help.as_str(),
            metric.namada_validator_state.metric.clone(),
        );
        registry.register(
            metric.namada_validator_active_set_rank.name.as_str(),
            metric.namada_validator_active_set_rank.help.as_str(),
            metric.namada_validator_active_set_rank.metric.clone(),
        );
        registry.register(
            metric.namada_missed_blocks.name.as_str(),
            metric.namada_missed_blocks.help.as_str(),
            metric.namada_missed_blocks.metric.clone(),
        );
        registry.register(
            metric.namada_total_bonds.name.as_str(),
            metric.namada_total_bonds.help.as_str(),
            metric.namada_total_bonds.metric.clone(),
        );
        registry.register(
            metric.validator_commission.name.as_str(),
            metric.validator_commission.help.as_str(),
            metric.validator_commission.metric.clone(),
        );
        registry.register(
            metric.namada_network_epoch.name.as_str(),
            metric.namada_network_epoch.help.as_str(),
            metric.namada_network_epoch.metric.clone(),
        );
        registry.register(
            metric.namada_node_catch_up.name.as_str(),
            metric.namada_node_catch_up.help.as_str(),
            metric.namada_node_catch_up.metric.clone(),
        );
        registry.register(
            metric.namada_network_lowest_active_set_stake.name.as_str(),
            metric.namada_network_lowest_active_set_stake.help.as_str(),
            metric.namada_network_lowest_active_set_stake.metric.clone(),
        );
        registry.register(
            metric.namada_network_max_set_size.name.as_str(),
            metric.namada_network_max_set_size.help.as_str(),
            metric.namada_network_max_set_size.metric.clone(),
        );
        registry.register(
            metric.namada_network_stake_threshold.name.as_str(),
            metric.namada_network_stake_threshold.help.as_str(),
            metric.namada_network_stake_threshold.metric.clone(),
        );
        registry.register(
            metric.namada_network_active_set_size.name.as_str(),
            metric.namada_network_active_set_size.help.as_str(),
            metric.namada_network_active_set_size.metric.clone(),
        );
        registry.register(
            metric.namada_node_latest_block.name.as_str(),
            metric.namada_node_latest_block.help.as_str(),
            metric.namada_node_latest_block.metric.clone(),
        );
        registry.register(
            metric.namada_validator_missed_blocks.name.as_str(),
            metric.namada_validator_missed_blocks.help.as_str(),
            metric.namada_validator_missed_blocks.metric.clone(),
        );
        NamadaMetrics {
            chain_id,
            registry,
            tm_address,
            metrics: metric,
        }
    }
    pub fn set_validator_metrics(&self, validator_data: &ValidatorMetricsData) {
        self.metrics
            .namada_validator_uptime_percentage
            .metric
            .get_or_create(&ValidatorLabels {
                chain_id: self.chain_id.clone(),
                validator_tm_address: self.tm_address.clone(),
                validator_hash_address: validator_data.validator_address_hash.clone(),
            })
            .set(validator_data.namada_validator_uptime_percentage);
        self.metrics
            .namada_validator_state
            .metric
            .get_or_create(&ValidatorLabels {
                chain_id: self.chain_id.clone(),
                validator_tm_address: self.tm_address.clone(),
                validator_hash_address: validator_data.validator_address_hash.clone(),
            })
            .set(validator_data.namada_validator_state);
        self.metrics
            .namada_validator_active_set_rank
            .metric
            .get_or_create(&ValidatorLabels {
                chain_id: self.chain_id.clone(),
                validator_tm_address: self.tm_address.clone(),
                validator_hash_address: validator_data.validator_address_hash.clone(),
            })
            .set(validator_data.namada_validator_active_set_rank);
        self.metrics
            .namada_missed_blocks
            .metric
            .get_or_create(&ValidatorLabels {
                chain_id: self.chain_id.clone(),
                validator_tm_address: self.tm_address.clone(),
                validator_hash_address: validator_data.validator_address_hash.clone(),
            })
            .set(validator_data.namada_missed_blocks);
        self.metrics
            .namada_total_bonds
            .metric
            .get_or_create(&ValidatorLabels {
                chain_id: self.chain_id.clone(),
                validator_tm_address: self.tm_address.clone(),
                validator_hash_address: validator_data.validator_address_hash.clone(),
            })
            .set(validator_data.namada_total_bonds);
        let rounded: f64 = format!("{:.2}", validator_data.validator_commission as f64)
            .parse()
            .expect("Failed to parse commission");
        self.metrics
            .validator_commission
            .metric
            .get_or_create(&ValidatorLabels {
                chain_id: self.chain_id.clone(),
                validator_tm_address: self.tm_address.clone(),
                validator_hash_address: validator_data.validator_address_hash.clone(),
            })
            .set(rounded);
    }
    pub fn set_network_metrics(&self, network_data: &NetworkMetricsData) {
        self.metrics
            .namada_network_epoch
            .metric
            .get_or_create(&NetworkLabels {
                chain_id: self.chain_id.clone(),
            })
            .set(network_data.namada_network_epoch);
        self.metrics
            .namada_node_catch_up
            .metric
            .get_or_create(&NetworkLabels {
                chain_id: self.chain_id.clone(),
            })
            .set(network_data.namada_node_catch_up);
        self.metrics
            .namada_network_lowest_active_set_stake
            .metric
            .get_or_create(&NetworkLabels {
                chain_id: self.chain_id.clone(),
            })
            .set(network_data.namada_network_lowest_active_set_stake);
        self.metrics
            .namada_network_max_set_size
            .metric
            .get_or_create(&NetworkLabels {
                chain_id: self.chain_id.clone(),
            })
            .set(network_data.namada_network_max_set_size);
        self.metrics
            .namada_network_stake_threshold
            .metric
            .get_or_create(&NetworkLabels {
                chain_id: self.chain_id.clone(),
            })
            .set(network_data.namada_network_stake_threshold);
        self.metrics
            .namada_network_active_set_size
            .metric
            .get_or_create(&NetworkLabels {
                chain_id: self.chain_id.clone(),
            })
            .set(network_data.namada_network_active_set_size);
    }
    pub fn set_node_metrics(&self, node_data: &NodeMetricsData) {
        self.metrics
            .namada_node_latest_block
            .metric
            .get_or_create(&NodeLabels {
                chain_id: self.chain_id.clone(),
                node_id: node_data.node_id.clone(),
                moniker: node_data.moniker.clone(),
            })
            .set(node_data.namada_node_latest_block);
        self.metrics
            .namada_validator_missed_blocks
            .metric
            .get_or_create(&NetworkLabels {
                chain_id: self.chain_id.clone(),
            })
            .set(node_data.namada_node_latest_block);
    }
    pub fn render(&self) -> String {
        let mut buffer = String::new();
        encode(&mut buffer, &self.registry).unwrap();
        buffer
    }
}
