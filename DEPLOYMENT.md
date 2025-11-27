# Deployment Guide (Fly.io - Free, No Cold Starts)

## Overview

This guide deploys all 3 services on Fly.io's free tier:
- **Facilitator** - Rust backend (verifies payments)
- **Server** - Rust backend (serves protected content)
- **Client** - Static frontend (user interface)

**Why Fly.io?**
- Services stay running 24/7 (no spin-down)
- No cold starts during demos
- 3 free VMs included
- No charges on free tier

## Prerequisites

- GitHub account
- Code pushed to GitHub repository
- Credit card (for verification only - won't be charged on free tier)

---

## Step 1: Install Fly CLI

```bash
brew install flyctl
```

---

## Step 2: Create Fly.io Account

```bash
fly auth signup
```

This opens a browser to create your account. Add a credit card for verification (you won't be charged on free tier).

If you already have an account:
```bash
fly auth login
```

---

## Step 3: Deploy Facilitator (First)

1. Navigate to the facilitator directory:
```bash
cd facilitator
```

2. Create the Fly app:
```bash
fly launch --no-deploy
```

When prompted:
- App name: `x402-facilitator` (or let it auto-generate)
- Region: Choose closest to you
- Database: No
- Redis: No

3. Edit the generated `fly.toml`:
```toml
app = 'x402-facilitator'
primary_region = 'sjc'

[build]

[http_service]
  internal_port = 8080
  force_https = true
  auto_stop_machines = false
  auto_start_machines = true
  min_machines_running = 1
  processes = ['app']

[[vm]]
  memory = '256mb'
  cpu_kind = 'shared'
  cpus = 1
```

4. Set environment variables:
```bash
fly secrets set POLKADOT_NETWORK=paseo POLKADOT_RPC_URL=wss://rpc.ibp.network/paseo FACILITATOR_HOST=0.0.0.0 FACILITATOR_PORT=8080
```

5. Deploy:
```bash
fly deploy
```

6. Get your URL:
```bash
fly status
```

Your facilitator URL: `https://x402-facilitator.fly.dev`

---

## Step 4: Deploy Server (Second)

1. Navigate to the server directory:
```bash
cd ../server
```

2. Create the Fly app:
```bash
fly launch --no-deploy
```

3. Edit `fly.toml`:
```toml
app = 'x402-server'
primary_region = 'sjc'

[build]

[http_service]
  internal_port = 3000
  force_https = true
  auto_stop_machines = false
  auto_start_machines = true
  min_machines_running = 1
  processes = ['app']

[[vm]]
  memory = '256mb'
  cpu_kind = 'shared'
  cpus = 1
```

4. Set environment variables:
```bash
fly secrets set SERVER_HOST=0.0.0.0 SERVER_PORT=3000 FACILITATOR_URL=https://facilitator.fly.dev RECEIVER_WALLET_ADDRESS=your_wallet_address_here DEFAULT_PRICE=1000000000000 POLKADOT_NETWORK=paseo
```

> ⚠️ Replace `FACILITATOR_URL` with your actual URL from Step 3

5. Deploy:
```bash
fly deploy
```

Your server URL: `https://x402-server.fly.dev`

---

## Step 5: Deploy Client (Last)

1. Navigate to the client directory:
```bash
cd ../client
```

2. Create the Fly app:
```bash
fly launch --no-deploy
```

3. Edit `fly.toml`:
```toml
app = 'x402-client'
primary_region = 'sjc'

[build]

[http_service]
  internal_port = 80
  force_https = true
  auto_stop_machines = false
  auto_start_machines = true
  min_machines_running = 1
  processes = ['app']

[[vm]]
  memory = '256mb'
  cpu_kind = 'shared'
  cpus = 1
```

4. Set environment variables:
```bash
fly secrets set VITE_SERVER_URL=https://x402-server.fly.dev VITE_POLKADOT_NETWORK=paseo
```

> ⚠️ Replace `VITE_SERVER_URL` with your actual URL from Step 4

5. Deploy:
```bash
fly deploy
```

Your client URL: `https://x402-client.fly.dev`

---

## Step 6: Test Your Deployment

1. Open `https://x402-client.fly.dev` in browser
2. Wallet interface loads instantly (no cold start!)
3. Test the payment flow

---

## Service URLs Summary

| Service | URL |
|---------|-----|
| Facilitator | `https://x402-facilitator.fly.dev` |
| Server | `https://x402-server.fly.dev` |
| Client | `https://x402-client.fly.dev` |

---

## Quick Reference Commands

```bash
# Check app status
fly status -a x402-facilitator

# View logs
fly logs -a x402-facilitator

# List all apps
fly apps list

# Redeploy after changes
fly deploy

# Destroy an app
fly apps destroy x402-facilitator
```

---

## Free Tier Limits

Fly.io free tier includes:
- **3 shared-cpu VMs** (256MB RAM each)
- **160GB outbound bandwidth**/month
- **3GB persistent storage**

Your 3 services fit perfectly in the free tier!

---

## Troubleshooting

### Build fails?
```bash
fly logs -a your-app-name
```

### App not starting?
```bash
fly status -a your-app-name
```

### Services can't communicate?
- Check secrets: `fly secrets list -a your-app-name`
- Ensure URLs use `https://`

### Multiple machines running?
```bash
fly scale count 1 -a your-app-name
```
