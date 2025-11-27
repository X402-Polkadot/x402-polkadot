# x402 Polkadot Server (Demo)

API server implementing x402 payment protocol.

## Setup

```bash
cp .env.example .env
cargo run
```

## Environment

```env
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
FACILITATOR_URL=http://127.0.0.1:8080
RECEIVER_WALLET_ADDRESS=your_wallet_address
DEFAULT_PRICE=1000000000000
POLKADOT_NETWORK=paseo
```

## API Endpoints

| Endpoint | Description |
|----------|-------------|
| `GET /api/health` | Health check |
| `GET /api/free` | Free endpoint |
| `GET /api/paid` | Paid endpoint (x402) |

## x402 Flow

1. Client requests `/api/paid` → Returns 402 with payment requirements
2. Client sends signed transaction in `X-PAYMENT` header
3. Server verifies via facilitator
4. Server settles via facilitator
5. Returns protected content
