# x402 Polkadot Client (Demo)

Frontend for testing x402 payment protocol.

## Setup

```bash
cp .env.example .env
npm install
npm run dev
```

## Usage

1. Click "Load Wallet from .env"
2. Click "Test /api/paid"
3. Watch automatic payment flow

## Environment

```env
VITE_SERVER_URL=http://127.0.0.1:3000
VITE_POLKADOT_NETWORK=paseo
VITE_SIGNER_MNEMONIC=your twelve word mnemonic here
```

## Features

- Auto-signs transactions using mnemonic from `.env`
- Multi-node RPC failover (Paseo testnet)
- Real-time payment progress display
