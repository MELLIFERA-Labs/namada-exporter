use crate::cli::HealthCheckConfig;
use reqwest::Client;
use tokio::time::interval;
use log::{info, error, warn};

pub struct HealthChecker {
    client: Client,
    config: HealthCheckConfig,
}

impl HealthChecker {
    pub fn new(config: HealthCheckConfig) -> Self {
        let client = Client::builder()
            .timeout(config.timeout_duration())
            .build()
            .expect("Failed to create HTTP client for health checks");
        
        Self { client, config }
    }

    pub async fn start_health_check_loop(&self) {
        let mut interval_timer = interval(self.config.ping_interval());
        
        info!("Starting health check loop, pinging {} every {}", 
              self.config.ping_url, self.config.ping_rate);
        
        loop {
            interval_timer.tick().await;
            self.ping().await;
        }
    }

    async fn ping(&self) {
        match self.client.get(&self.config.ping_url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    info!("Health check ping successful: {}", response.status());
                } else {
                    warn!("Health check ping returned non-success status: {}", response.status());
                }
            }
            Err(e) => {
                error!("Health check ping failed: {}", e);
            }
        }
    }
}