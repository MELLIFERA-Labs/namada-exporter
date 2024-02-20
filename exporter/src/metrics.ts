import Prometheus from 'prom-client'
export const uptimeMetric = new Prometheus.Gauge({
  name: 'validator_uptime_percentage',
  help: 'Validator uptime in percentage; -1 value if validator not in active set',
  labelNames: ['chain_id', 'validator_tm_address', 'validator_hash_address'],
})

export const stateStatusMetric = new Prometheus.Gauge({
  name: 'validator_state',
  help: 'Validator state; 0 - unknown, 1 - active consensus set, 2 - active below capacity set, 3 - active below threshold set, 4 - jailed, 5 - inactive',
  labelNames: ['chain_id', 'validator_tm_address', 'validator_hash_address'],
})

export const activeSetRankMetric = new Prometheus.Gauge({
  name: 'validator_active_set_rank',
  help: 'Validator active set rank, -1 value if not in active set',
  labelNames: ['chain_id', 'validator_tm_address', 'validator_hash_address'],
})

export const epochMetric = new Prometheus.Gauge({
  name: 'network_epoch',
  help: 'Current network epoch',
  labelNames: ['chain_id'],
})
export const catchUpMetric = new Prometheus.Gauge({
  name: 'node_catch_up',
  help: 'Validator catch up status; 0 - not catching up, 1 - catching up',
  labelNames: ['chain_id', 'validator_tm_address', 'validator_hash_address'],
})
export const lowestActiveSetStakeMetric = new Prometheus.Gauge({
  name: 'network_lowest_active_set_stake',
  help: 'Lowest active set stake',
  labelNames: ['chain_id'],
})
export const missedBlocksMetric = new Prometheus.Gauge({
  name: 'validator_missed_blocks',
  help: 'Validator missed blocks in liveness window; -1 value if not in active set',
  labelNames: ['chain_id', 'validator_tm_address', 'validator_hash_address'],
})
export const totalBondsMetric = new Prometheus.Gauge({
  name: 'validator_total_bonds',
  help: 'Validator total bonds',
  labelNames: ['chain_id', 'validator_tm_address', 'validator_hash_address'],
})
export const validatorCommissionMetric = new Prometheus.Gauge({
  name: 'validator_commission',
  help: 'Validator commission',
  labelNames: ['chain_id', 'validator_tm_address', 'validator_hash_address'],
})
export const validatorMaxSetSizeMetric = new Prometheus.Gauge({
  name: 'network_max_set_size',
  help: 'Max set size',
  labelNames: ['chain_id'],
})

export const validatorStakeThresholdMetric = new Prometheus.Gauge({
  name: 'network_stake_threshold',
  help: 'Stake threshold',
  labelNames: ['chain_id'],
})
export const validatorActiveSetSizeMetric = new Prometheus.Gauge({
  name: 'network_active_set_size',
  help: 'Active set size',
  labelNames: ['chain_id'],
})
export const validatorLatestBlockMetric = new Prometheus.Gauge({
  name: 'network_latest_block',
  help: 'Latest block from rpc',
  labelNames: ['chain_id'],
})
