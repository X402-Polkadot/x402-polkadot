// Network configuration with multiple RPC nodes for failover
export const networks = {
  paseo: {
    name: 'Paseo Testnet',
    nodes: [
      { url: 'wss://paseo.rpc.amforc.com', name: 'Amforc' },
      { url: 'wss://paseo-rpc.dwellir.com', name: 'Dwellir' },
      { url: 'wss://rpc.ibp.network/paseo', name: 'IBP Network' },
      { url: 'wss://paseo.dotters.network', name: 'Dotters' },
    ],
    explorer: 'https://paseo.subscan.io'
  },
  westend: {
    name: 'Westend Testnet',
    nodes: [
      { url: 'wss://westend-rpc.polkadot.io', name: 'Parity' },
      { url: 'wss://westend.rpc.amforc.com', name: 'Amforc' },
    ],
    explorer: 'https://westend.subscan.io'
  },
  polkadot: {
    name: 'Polkadot Mainnet',
    nodes: [
      { url: 'wss://rpc.polkadot.io', name: 'Parity' },
      { url: 'wss://polkadot.rpc.amforc.com', name: 'Amforc' },
      { url: 'wss://polkadot-rpc.dwellir.com', name: 'Dwellir' },
    ],
    explorer: 'https://polkadot.subscan.io'
  }
};

// Check if a node is responsive
export async function checkNodeHealth(url, timeout = 5000) {
  return new Promise((resolve) => {
    const ws = new WebSocket(url);
    const timer = setTimeout(() => {
      ws.close();
      resolve(false);
    }, timeout);

    ws.onopen = () => {
      clearTimeout(timer);
      ws.close();
      resolve(true);
    };

    ws.onerror = () => {
      clearTimeout(timer);
      resolve(false);
    };
  });
}

// Find the first healthy node from the list (silently)
export async function findHealthyNode(networkId) {
  const network = networks[networkId];
  if (!network) {
    throw new Error(`Unknown network: ${networkId}`);
  }

  for (const node of network.nodes) {
    const isHealthy = await checkNodeHealth(node.url);
    if (isHealthy) {
      return node;
    }
  }

  throw new Error(`No healthy nodes found for ${network.name}`);
}

// Get network config
export function getNetworkConfig(networkId) {
  return networks[networkId] || networks.paseo;
}
