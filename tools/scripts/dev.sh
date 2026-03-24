#!/usr/bin/env bash
set -euo pipefail

# Starts all services locally without Docker.
# Requires: go, cargo, pnpm, postgres running on localhost

echo "Starting Vibetown dev environment..."
echo "Ensure PostgreSQL is running on localhost:5432"

# Start engine in background
(cd engine && go run ./cmd/vibetown-engine) &
ENGINE_PID=$!

# Start server in background
(cd server && cargo run --bin server) &
SERVER_PID=$!

# Start frontend
(cd web && pnpm dev) &
WEB_PID=$!

trap "kill $ENGINE_PID $SERVER_PID $WEB_PID 2>/dev/null" EXIT

echo "Engine PID: $ENGINE_PID"
echo "Server PID: $SERVER_PID"
echo "Web PID: $WEB_PID"
echo ""
echo "Press Ctrl+C to stop all services"

wait
