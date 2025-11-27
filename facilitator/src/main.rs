mod api;
mod config;
mod error;
mod polkadot;

use anyhow::Result;
use axum::{
    routing::{get, post},
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
    polkadot::PolkadotClient,
};

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();

    info!("Starting x402 Polkadot Facilitator");

    let config = Config::from_env()?;
    info!("Configuration loaded: network={}", config.polkadot_network);

    let polkadot_client = PolkadotClient::new(
        config.polkadot_rpc_url.clone(),
        config.polkadot_network.clone(),
        config.signer_seed.clone(),
    )
    .await?;

    let state: AppState = Arc::new(AppStateInner {
        config: config.clone(),
        polkadot_client,
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
        .route("/health", get(api::routes::health))
        .route("/verify", post(api::routes::verify))
        .route("/settle", post(api::routes::settle))
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
