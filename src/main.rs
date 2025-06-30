use namada_query::Query;
use server::start_server;
mod cli;
mod constants;
mod handlers;
mod healthcheck;
mod metrics;
mod namada_query;
mod server;
use cli::parse_cli;
use env_logger::{Builder, Env};
use healthcheck::HealthChecker;
use log::info;
const LOG_ENV_VAR: &str = "RUST_LOG";

#[tokio::main]
async fn main() {
    let exporter_config = parse_cli();
    let q = Query::create(&exporter_config.http_rpc).unwrap();
    let env = Env::default().filter_or(LOG_ENV_VAR, "info");
    Builder::from_env(env).init();

    // Start health checker if configured
    if let Some(hc_config) = exporter_config.healthcheck.clone() {
        info!("Health check enabled with config: {:?}", hc_config);
        let health_checker = HealthChecker::new(hc_config);
        tokio::spawn(async move {
            health_checker.start_health_check_loop().await;
        });
    }

    start_server(&exporter_config, q).await;
}
