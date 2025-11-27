mod api;
mod config;
mod error;
mod facilitator;
mod x402;

use anyhow::Result;
use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::{
    api::{routes::AppStateInner, AppState},
    config::Config,
    facilitator::FacilitatorClient,
};

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();

    info!("Starting x402 Polkadot Server");

    let config = Config::from_env()?;
    info!("Configuration loaded: network={}", config.polkadot_network);

    let facilitator_client = FacilitatorClient::new(config.facilitator_url.clone());

    let state: AppState = Arc::new(AppStateInner {
        config: config.clone(),
        facilitator_client,
    });

    let app = create_router(state);

    let bind_addr = config.bind_address();
    info!("Server listening on http://{}", bind_addr);

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/health", get(api::routes::health))
        .route("/api/free", get(api::routes::free))
        .route("/api/paid", get(api::routes::paid))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

fn init_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
}
