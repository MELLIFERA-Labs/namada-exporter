use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use toml;

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
        #[arg(short='c', long, value_name = "CONFIG_FILE")]
        config: PathBuf,
    },
}
#[derive(Deserialize, Debug, Clone)]
pub struct ExporterConfig {
    pub host: String,
    pub validator_tm_address: String,
    pub http_rpc: String,
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
