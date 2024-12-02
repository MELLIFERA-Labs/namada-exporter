use crate::cli::ExporterConfig;
use crate::handlers::metrics_handler;
use crate::namada_query::Query;
use axum::{response::IntoResponse, routing::get, Router};
use log::info;
async fn health_handler() -> impl IntoResponse {
    String::from("OK").into_response()
}
#[derive(Clone)]
pub struct ServerState {
    pub q: Query,
    pub config: ExporterConfig,
}
pub async fn start_server(exporter_config: &ExporterConfig, q: Query) {
    let app = Router::new()
        .route("/", get(health_handler))
        .route("/metrics", get(metrics_handler))
        .with_state(ServerState {
            q,
            config: exporter_config.clone(),
        });
    let listener = tokio::net::TcpListener::bind(exporter_config.host.clone())
        .await
        .unwrap();
    info!("Server listening on {}", exporter_config.host);
    axum::serve(listener, app).await.unwrap();
}
