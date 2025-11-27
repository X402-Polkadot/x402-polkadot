#!/bin/bash

# x402 Polkadot - Startup Script
# Starts Facilitator, Server, and Client

set -e

echo "🚀 Starting x402 Polkadot Payment System..."
echo ""

# Color codes
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Cleanup function
cleanup() {
    echo ""
    echo "${YELLOW}🛑 Shutting down services...${NC}"
    kill $FACILITATOR_PID $SERVER_PID $CLIENT_PID 2>/dev/null
    echo "✅ All services stopped"
    exit 0
}

trap cleanup SIGINT SIGTERM

# Start Facilitator
echo "${BLUE}[1/3] Starting Facilitator on port 8080...${NC}"
cd facilitator
cargo run 2>&1 | sed 's/^/[FACILITATOR] /' &
FACILITATOR_PID=$!
cd ..
sleep 3

# Start Server
echo "${BLUE}[2/3] Starting Server on port 3000...${NC}"
cd server
cargo run 2>&1 | sed 's/^/[SERVER] /' &
SERVER_PID=$!
cd ..
sleep 3

# Start Client
echo "${BLUE}[3/3] Starting Client on port 5173...${NC}"
cd client

# Check if node_modules exists
if [ ! -d "node_modules" ]; then
    echo "${YELLOW}Installing client dependencies...${NC}"
    npm install
fi

npm run dev 2>&1 | sed 's/^/[CLIENT] /' &
CLIENT_PID=$!
cd ..

echo ""
echo "${GREEN}✅ All services started!${NC}"
echo ""
echo "📍 Service URLs:"
echo "   Facilitator: http://127.0.0.1:8080"
echo "   Server:      http://127.0.0.1:3000"
echo "   Client:      http://localhost:5173"
echo ""
echo "🌐 Open your browser to: ${GREEN}http://localhost:5173${NC}"
echo ""
echo "Press Ctrl+C to stop all services"
echo ""

# Wait for all processes
wait
