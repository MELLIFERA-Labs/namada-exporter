import { Query } from '../../shared/src'
import { Command } from 'commander'
import fs from 'fs'
import BigNumber from 'bignumber.js'
const program = new Command()
import packageJson from '../package.json'
import * as toml from 'toml'
import {
  uptimeMetric,
  activeSetRankMetric,
  stateStatusMetric,
  missedBlocksMetric,
  totalBondsMetric,
  validatorCommissionMetric,
  lowestActiveSetStakeMetric,
  catchUpMetric,
  epochMetric,
} from './metrics.ts'
import Prometheus from 'prom-client'
program
  .name(packageJson.name)
  .version(packageJson.version)
  .description(packageJson.description)

enum ValidatorState {
  UNKNOWN,
  ACTIVE_CONSENSUS_SET,
  ACTIVE_BELOW_CAPACITY_SET,
  ACTIVE_BELOW_THRESHOLD_SET,
  JAILED,
  INACTIVE,
}
function mapValidatorState(state: string): ValidatorState {
  switch (state) {
    case 'active_consensus_set':
      return ValidatorState.ACTIVE_CONSENSUS_SET
    case 'active_below_capacity_set':
      return ValidatorState.ACTIVE_BELOW_CAPACITY_SET
    case 'active_below_threshold_set':
      return ValidatorState.ACTIVE_BELOW_THRESHOLD_SET
    case 'jailed':
      return ValidatorState.JAILED
    case 'inactive':
      return ValidatorState.INACTIVE
    default:
      return ValidatorState.UNKNOWN
  }
}

async function collectMetrics(config: any) {
    const statusUrl = new URL('/status', config['validator_http_rpc'])
    const status = await fetch(statusUrl).then((data) => data.json())
    const chainId = status['result']['node_info']['network']
    const catchUp = status['result']['sync_info']['catching_up']
    const nodeValAddress = status['result']['validator_info']['address']
    if (catchUp) {
        console.warn('WARNING: Node is catching up may not provide accurate data')
    }

    const q = new Query(config['validator_http_rpc'])
    /** request data */
    const [epoch, validator, validators, posParams] = await Promise.all([
        q.query_epoch(),
        q.query_validator_data(config['validator_tm_address']),
        q.query_consensus_validator_set(),
        q.query_pos_params(),
    ])
    const sortedValidators = validators.sort(
        (a: { stake: string }, b: { stake: string }) =>
            BigNumber(b.stake).comparedTo(a.stake),
    )
    const valActiveRank = sortedValidators.findIndex(
        (v: { address: string }) => v.address === validator.address,
    )
    const lowestActiveSetStake = sortedValidators.at(-1).stake
    const valAddressMetrics = {
        validator_tm_address: validator.address,
        validator_hash_address: validator.address_hash,
    }
    if (validator.address_hash !== nodeValAddress) {
        console.warn(
            'WARNING: Using not validator rpc, this exporter suppose to be run with validator rpc',
        )
    }
    Prometheus.register.setDefaultLabels({ chain_id: chainId })
    /** set default value if validator is not in active set */
    activeSetRankMetric.set(
        {
            ...valAddressMetrics,
        },
        -1,
    )
    missedBlocksMetric.set(
        {
            ...valAddressMetrics,
        },
        -1,
    )
    uptimeMetric.set(
        {
            ...valAddressMetrics,
        },
        -1,
    )

    if (valActiveRank !== -1) {
        activeSetRankMetric.set(
            {
                ...valAddressMetrics,
            },
            valActiveRank + 1,
        )

        const livenessWindowCheck = posParams.owned.liveness_window_check
        const livenessThreshold = Number(posParams.owned.liveness_threshold)
        const maxBlockToSlash = livenessWindowCheck * livenessThreshold
        const uptimeProcent =
            validator?.missed_blocks != null
                ? (1 - Number(validator.missed_blocks) / maxBlockToSlash) * 100
                : null
        if (uptimeProcent) {
            missedBlocksMetric.set(
                {
                    ...valAddressMetrics,
                },
                Number(validator.missed_blocks),
            )
            uptimeMetric.set(
                {
                    ...valAddressMetrics,
                },
                Number(uptimeProcent.toFixed(2)),
            )
        }
    }
    stateStatusMetric.set(
        {
            ...valAddressMetrics,
        },
        mapValidatorState(validator.state),
    )
    totalBondsMetric.set(
        { ...valAddressMetrics },
        BigNumber(validator.stake).toNumber(),
    )
    validatorCommissionMetric.set(
        { ...valAddressMetrics },
        Number(validator.commission.commission_rate),
    )
    lowestActiveSetStakeMetric.set(Number(lowestActiveSetStake))
    catchUpMetric.set(
        {
            ...valAddressMetrics,
        },
        catchUp ? 1 : 0,
    )
    epochMetric.set({ chain_id: chainId }, epoch)
    const metrics = await Prometheus.register.metrics()
}

program
  .command('start')
  .description('run exporter')
  .option('--config <config>', 'Path to config file', './config.toml')
  .action(async (opt) => {
    const config = toml.parse(fs.readFileSync(opt.config, 'utf-8'))
    Bun.serve({
          port: config.port,
          async fetch(req) {
              await collectMetrics(config)
              const metrics = await Prometheus.register.metrics();
              return new Response(metrics)
          }})
      console.log(`Metrics server started on http://localhost:${config.port}`)
  })
program.parse()
