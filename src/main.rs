use namada_query::Query;
use server::start_server;
use tokio::runtime::Runtime;
mod cli;
mod handlers;
mod metrics;
mod namada_query;
mod server;
use cli::parse_cli;
use env_logger::{Builder, Env};

fn main() {
    let exporter_config = parse_cli();
    let rt = Runtime::new().unwrap();
    let q = Query::create(&exporter_config.http_rpc).unwrap();
    let env = Env::default().filter_or("RUST_LOG", "info");
    Builder::from_env(env).init();
    rt.block_on(start_server(&exporter_config, q));
}
