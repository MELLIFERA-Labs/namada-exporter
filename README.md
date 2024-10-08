# namada-exporter
> Namada exporter for Prometheus

## Install

### Download exporter:
```bash 
wget <release_url>
```
### Or build from source:
1. Install bun(JavaScript runtime) https://bun.sh Rust Cargo and Node.js
2. Clone this repo
3. Install dependencies for shared in /shared folder:
```bash
bun install 
```
4. build shared library in /shared folder:
```bash
bun run prepublish
```
5. Install dependencies for exporter in /exporter folder:
```bash
bun install --production --frozen-lockfile
```
6. build binary in root project:
```bash
bun build ./exporter/src/index.ts --compile --outfile namada-exporter  
```
## Usage
### Config example
```toml
port = 3000
validator_tm_address = "tnam..."
validator_http_rpc = "http://localhost:26657"
```
Look at [config.example.toml](config.example.toml) for more details
### Run
```bash
./namada-exporter start --config config.toml
```
### Metrics expose example
```
# HELP validator_uptime_percentage Validator uptime in percentage; -1 value if validator not in active set
# TYPE validator_uptime_percentage gauge
validator_uptime_percentage{validator_tm_address="tnam1qxrc2njv8lnvfqsmj5dlyzuys06nzmz3ggx2s3dc",validator_hash_address="3BA4CBA3D0DE25094DF41C8B1A07A8BD163B045E",chain_id="shielded-expedition.88f17d1d14"} 100

# HELP validator_state Validator state; 0 - unknown, 1 - active consensus set, 2 - active below capacity set, 3 - active below threshold set, 4 - jailed, 5 - inactive
# TYPE validator_state gauge
validator_state{validator_tm_address="tnam1qxrc2njv8lnvfqsmj5dlyzuys06nzmz3ggx2s3dc",validator_hash_address="3BA4CBA3D0DE25094DF41C8B1A07A8BD163B045E",chain_id="shielded-expedition.88f17d1d14"} 1

# HELP validator_active_set_rank Validator active set rank, -1 value if not in active set
# TYPE validator_active_set_rank gauge
validator_active_set_rank{validator_tm_address="tnam1qxrc2njv8lnvfqsmj5dlyzuys06nzmz3ggx2s3dc",validator_hash_address="3BA4CBA3D0DE25094DF41C8B1A07A8BD163B045E",chain_id="shielded-expedition.88f17d1d14"} 125

# HELP network_epoch Current network epoch
# TYPE network_epoch gauge
network_epoch{chain_id="shielded-expedition.88f17d1d14"} 11

# HELP node_catch_up Validator catch up status; 0 - not catching up, 1 - catching up
# TYPE node_catch_up gauge
node_catch_up{validator_tm_address="tnam1qxrc2njv8lnvfqsmj5dlyzuys06nzmz3ggx2s3dc",validator_hash_address="3BA4CBA3D0DE25094DF41C8B1A07A8BD163B045E",chain_id="shielded-expedition.88f17d1d14"} 0

# HELP network_lowest_active_set_stake Lowest active set stake
# TYPE network_lowest_active_set_stake gauge
network_lowest_active_set_stake{chain_id="shielded-expedition.88f17d1d14"} 1000000000

# HELP validator_missed_blocks Validator missed blocks in liveness window; -1 value if not in active set
# TYPE validator_missed_blocks gauge
validator_missed_blocks{validator_tm_address="tnam1qxrc2njv8lnvfqsmj5dlyzuys06nzmz3ggx2s3dc",validator_hash_address="3BA4CBA3D0DE25094DF41C8B1A07A8BD163B045E",chain_id="shielded-expedition.88f17d1d14"} 0

# HELP validator_total_bonds Validator total bonds
# TYPE validator_total_bonds gauge
validator_total_bonds{validator_tm_address="tnam1qxrc2njv8lnvfqsmj5dlyzuys06nzmz3ggx2s3dc",validator_hash_address="3BA4CBA3D0DE25094DF41C8B1A07A8BD163B045E",chain_id="shielded-expedition.88f17d1d14"} 20031000000

# HELP validator_commission Validator commission
# TYPE validator_commission gauge
validator_commission{validator_tm_address="tnam1qxrc2njv8lnvfqsmj5dlyzuys06nzmz3ggx2s3dc",validator_hash_address="3BA4CBA3D0DE25094DF41C8B1A07A8BD163B045E",chain_id="shielded-expedition.88f17d1d14"} 0.1

# HELP network_max_set_size Max set size
# TYPE network_max_set_size gauge
network_max_set_size{chain_id="shielded-expedition.88f17d1d14"} 257

# HELP network_stake_threshold Stake threshold
# TYPE network_stake_threshold gauge
network_stake_threshold{chain_id="shielded-expedition.88f17d1d14"} 1000000000

# HELP network_active_set_size Active set size
# TYPE network_active_set_size gauge
network_active_set_size{chain_id="shielded-expedition.88f17d1d14"} 227

# HELP validator_node_latest_block Latest block from rpc
# TYPE validator_node_latest_block gauge
validator_node_latest_block{chain_id="shielded-expedition.88f17d1d14"} 42850

```
