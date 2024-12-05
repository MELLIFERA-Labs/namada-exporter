# namada-exporter
> Namada exporter for Prometheus

## Grafana dashboard
todo: need to update minor changes
Check out this [repo](https://github.com/MELLIFERA-Labs/namada-exporter-validator-dashboard), or here is the link to [Grafana dashboard](https://grafana.com/grafana/dashboards/20550-namada-validators/)

## Install

### Download exporter:
```bash 
wget <release_url>
```
### Or build from source:
1. Clone repo 
2. Install rust https://www.rust-lang.org/tools/install
3. Build 
```bash 
cargo build --release 
```
4. use binary in `target/release/namada-exporter`

## Usage
### Config example

```toml
host = "127.0.0.1:4000"
validator_tm_address = "tnam1..."
http_rpc = "https://rpc.some_rpc.run/"
```
Look at [config.example.toml](config.example.toml) for more details
### Run

```bash
./namada-exporter start --config config.toml
```
### Metrics expose example
```
# HELP namada_validator_uptime_percentage Validator uptime in percentage; -1 value if validator not in active set.
# TYPE namada_validator_uptime_percentage gauge
namada_validator_uptime_percentage{chain_id="namada.5f5de2dd1b88cba30586420",validator_tm_address="tnam1q8d8ypu5j88qqvx89grct795uap82dtlqvjqjh3h",validator_hash_address="E31D476BFFF77ECE5E7933768D40C0D6CF298526"} 100
# HELP namada_validator_state Validator state; 0 - unknown, 1 - active consensus set, 2 - active below capacity set, 3 - active below threshold set, 4 - jailed, 5 - inactive.
# TYPE namada_validator_state gauge
namada_validator_state{chain_id="namada.5f5de2dd1b88cba30586420",validator_tm_address="tnam1q8d8ypu5j88qqvx89grct795uap82dtlqvjqjh3h",validator_hash_address="E31D476BFFF77ECE5E7933768D40C0D6CF298526"} 1
# HELP namada_validator_active_set_rank Validator active set rank, -1 value if not in active set.
# TYPE namada_validator_active_set_rank gauge
namada_validator_active_set_rank{chain_id="namada.5f5de2dd1b88cba30586420",validator_tm_address="tnam1q8d8ypu5j88qqvx89grct795uap82dtlqvjqjh3h",validator_hash_address="E31D476BFFF77ECE5E7933768D40C0D6CF298526"} 40
# HELP namada_validator_missed_blocks Validator missed blocks in liveness window; -1 value if not in active set.
# TYPE namada_validator_missed_blocks gauge
namada_validator_missed_blocks{chain_id="namada.5f5de2dd1b88cba30586420",validator_tm_address="tnam1q8d8ypu5j88qqvx89grct795uap82dtlqvjqjh3h",validator_hash_address="E31D476BFFF77ECE5E7933768D40C0D6CF298526"} 6
# HELP namada_validator_total_bonds Validator total bonds.
# TYPE namada_validator_total_bonds gauge
namada_validator_total_bonds{chain_id="namada.5f5de2dd1b88cba30586420",validator_tm_address="tnam1q8d8ypu5j88qqvx89grct795uap82dtlqvjqjh3h",validator_hash_address="E31D476BFFF77ECE5E7933768D40C0D6CF298526"} 474864850000
# HELP namada_validator_commission Validator commission.
# TYPE namada_validator_commission gauge
namada_validator_commission{chain_id="namada.5f5de2dd1b88cba30586420",validator_tm_address="tnam1q8d8ypu5j88qqvx89grct795uap82dtlqvjqjh3h",validator_hash_address="E31D476BFFF77ECE5E7933768D40C0D6CF298526"} 0.05
# HELP namada_network_epoch Current network epoch.
# TYPE namada_network_epoch gauge
namada_network_epoch{chain_id="namada.5f5de2dd1b88cba30586420"} 7
# HELP namada_node_catch_up Validator catch up status; 0 - not catching up, 1 - catching up.
# TYPE namada_node_catch_up gauge
namada_node_catch_up{chain_id="namada.5f5de2dd1b88cba30586420"} 0
# HELP namada_network_lowest_active_set_stake Lowest active set stake.
# TYPE namada_network_lowest_active_set_stake gauge
namada_network_lowest_active_set_stake{chain_id="namada.5f5de2dd1b88cba30586420"} 1000000000
# HELP namada_network_max_set_size Max set size.
# TYPE namada_network_max_set_size gauge
namada_network_max_set_size{chain_id="namada.5f5de2dd1b88cba30586420"} 255
# HELP namada_network_stake_threshold Stake threshold.
# TYPE namada_network_stake_threshold gauge
namada_network_stake_threshold{chain_id="namada.5f5de2dd1b88cba30586420"} 1000000000
# HELP namada_network_active_set_size Active set size.
# TYPE namada_network_active_set_size gauge
namada_network_active_set_size{chain_id="namada.5f5de2dd1b88cba30586420"} 176
# HELP namada_node_latest_block Latest block from rpc.
# TYPE namada_node_latest_block gauge
namada_node_latest_block{chain_id="namada.5f5de2dd1b88cba30586420",node_id="b7f19137e79ed78319f407c3e0fd6d86a98da5cf",moniker="technodrome-v1.0.0"} 22407
# HELP namada_validator_node_latest_block Latest block from rpc. This metric is deprecated and will be removed in future versions please use namada_node_latest_block.
# TYPE namada_validator_node_latest_block gauge
namada_validator_node_latest_block{chain_id="namada.5f5de2dd1b88cba30586420"} 22407
# EOF
```
