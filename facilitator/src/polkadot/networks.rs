use std::time::Duration;
use tokio::time::timeout;
use tracing::{info, warn, debug};
use futures::future::join_all;

#[derive(Debug, Clone)]
pub struct RpcNode {
    pub url: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub name: String,
    pub nodes: Vec<RpcNode>,
    pub default_index: usize,
}

impl NetworkConfig {
    pub fn paseo() -> Self {
        Self {
            name: "Paseo Testnet".to_string(),
            nodes: vec![
                RpcNode { url: "wss://rpc.ibp.network/paseo".to_string(), name: "IBP Network".to_string() },
                RpcNode { url: "wss://paseo.rpc.amforc.com".to_string(), name: "Amforc".to_string() },
                RpcNode { url: "wss://paseo.dotters.network".to_string(), name: "Dotters".to_string() },
                RpcNode { url: "wss://paseo-rpc.dwellir.com".to_string(), name: "Dwellir".to_string() },
            ],
            default_index: 0,
        }
    }

    pub fn westend() -> Self {
        Self {
            name: "Westend Testnet".to_string(),
            nodes: vec![
                RpcNode { url: "wss://westend-rpc.polkadot.io".to_string(), name: "Parity".to_string() },
                RpcNode { url: "wss://westend.rpc.amforc.com".to_string(), name: "Amforc".to_string() },
            ],
            default_index: 0,
        }
    }

    pub fn polkadot() -> Self {
        Self {
            name: "Polkadot Mainnet".to_string(),
            nodes: vec![
                RpcNode { url: "wss://rpc.polkadot.io".to_string(), name: "Parity".to_string() },
                RpcNode { url: "wss://polkadot.rpc.amforc.com".to_string(), name: "Amforc".to_string() },
                RpcNode { url: "wss://polkadot-rpc.dwellir.com".to_string(), name: "Dwellir".to_string() },
            ],
            default_index: 0,
        }
    }

    pub fn from_network_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "paseo" => Self::paseo(),
            "westend" => Self::westend(),
            "polkadot" => Self::polkadot(),
            _ => Self::paseo(),
        }
    }
}

/// Check if an RPC node is healthy by attempting a WebSocket connection
async fn check_node_health(url: &str, timeout_ms: u64) -> bool {
    use tokio_tungstenite::connect_async;

    let connect_future = connect_async(url);

    match timeout(Duration::from_millis(timeout_ms), connect_future).await {
        Ok(Ok((_, _))) => true,
        Ok(Err(_)) => false,
        Err(_) => false, // Timeout
    }
}

/// Find a healthy RPC node. Checks all nodes in parallel, returns default if healthy,
/// otherwise returns first healthy alternative.
pub async fn find_healthy_node(config: &NetworkConfig) -> Option<RpcNode> {
    let timeout_ms = 5000u64;

    debug!("Checking {} RPC nodes in parallel", config.nodes.len());

    // Check all nodes in parallel
    let health_checks: Vec<_> = config.nodes.iter()
        .map(|node| {
            let url = node.url.clone();
            async move {
                let healthy = check_node_health(&url, timeout_ms).await;
                (healthy, url)
            }
        })
        .collect();

    let results = join_all(health_checks).await;

    // Check if default node is healthy
    if let Some((healthy, _)) = results.get(config.default_index) {
        if *healthy {
            let node = &config.nodes[config.default_index];
            info!("Using default RPC: {} ({})", node.name, node.url);
            return Some(node.clone());
        }
    }

    // Default is down, find first healthy alternative
    warn!("Default RPC node is down, searching for alternative");

    for (i, (healthy, _)) in results.iter().enumerate() {
        if *healthy && i != config.default_index {
            let node = &config.nodes[i];
            info!("Using alternative RPC: {} ({})", node.name, node.url);
            return Some(node.clone());
        }
    }

    warn!("No healthy RPC nodes found");
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_config_paseo() {
        let config = NetworkConfig::paseo();
        assert_eq!(config.name, "Paseo Testnet");
        assert!(!config.nodes.is_empty());
    }

    #[test]
    fn test_from_network_name() {
        let config = NetworkConfig::from_network_name("paseo");
        assert_eq!(config.name, "Paseo Testnet");
    }
}
