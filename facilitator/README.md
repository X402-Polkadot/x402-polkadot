# x402 Polkadot Facilitator (Demo)

Transaction verification and submission service.

## Setup

```bash
cp .env.example .env
cargo run
```

## Environment

```env
POLKADOT_NETWORK=paseo
POLKADOT_RPC_URL=wss://rpc.ibp.network/paseo
FACILITATOR_HOST=127.0.0.1
FACILITATOR_PORT=8080
```

## API Endpoints

| Endpoint | Description |
|----------|-------------|
| `GET /health` | Health check |
| `POST /verify` | Verify transaction |
| `POST /settle` | Submit transaction to blockchain |
