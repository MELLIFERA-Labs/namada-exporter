use crate::constants;
use clap::{Parser, Subcommand};
use humantime::parse_duration;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

/// Example CLI application
#[derive(Parser, Debug)]
#[command(name = env!("CARGO_CRATE_NAME"), version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"), author = env!("CARGO_PKG_AUTHORS"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the exporter with a configuration file
    Start {
        /// Path to the config.toml file
        #[arg(short = 'c', long, value_name = "CONFIG_FILE")]
        config: PathBuf,
    },
}
#[derive(Debug, Deserialize, Clone)]
pub struct ExporterConfig {
    pub host: String,
    pub validator_tm_address: String,
    pub http_rpc: String,
    pub metrics_content_type: Option<String>,
    pub healthcheck: Option<HealthCheckConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HealthCheckConfig {
    pub ping_url: String,
    pub ping_rate: String,
    pub timeout: String,
}

impl HealthCheckConfig {
    pub fn ping_interval(&self) -> Duration {
        parse_duration(&self.ping_rate)
            .unwrap_or_else(|_| Duration::from_secs(constants::DEFAULT_PING_RATE_IN_SECONDS))
    }

    pub fn timeout_duration(&self) -> Duration {
        parse_duration(&self.timeout)
            .unwrap_or_else(|_| Duration::from_secs(constants::DEFAULT_TIMEOUT_IN_SECONDS))
    }
}

pub fn parse_cli() -> ExporterConfig {
    let cli = Cli::parse();
    let config_content = match cli.command {
        Commands::Start { config } => {
            let contents =
                fs::read_to_string(config).expect("Something went wrong reading the config file");
            contents
        }
    };
    let config: ExporterConfig =
        toml::from_str(&config_content).expect("Failed to parse config file");

    config
}
