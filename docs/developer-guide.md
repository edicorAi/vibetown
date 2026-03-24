# Developer Guide

## Prerequisites

- **Go** 1.25+ (`brew install go`)
- **Rust** nightly-2025-12-04 (installed via `rustup`, pinned in `rust-toolchain.toml`)
- **Node.js** 20+ with **pnpm** 10+ (`corepack enable`)
- **Docker** with Compose v2
- **buf** (`brew install bufbuild/buf/buf`) — for proto linting/generation

## Quick Start

```bash
# Check all tools are installed
make check-tools

# Install all dependencies
make install

# Start development (via Docker Compose)
make dev

# Or run each service independently:
make dev-engine   # Go engine on :50051
make dev-server   # Rust server on :3000
make dev-web      # React frontend on :5173
```

## Building

```bash
make build          # Build all
make build-engine   # Go binary → engine/bin/vibetown-engine
make build-server   # Rust binary → server/target/release/server
make build-web      # React → web/packages/app/dist/
```

## Testing

```bash
make test           # Run all tests
make test-engine    # Go tests
make test-server    # Rust tests
make test-web       # Frontend type checks
make test-grpc      # gRPC integration tests (engine-client)
make test-e2e       # E2E API tests (requires running server)
```

## Project Layout

| Directory | Language | What |
|-----------|----------|------|
| `engine/` | Go | Orchestration engine (65 packages from Gastown) |
| `server/` | Rust | API server (14 crates from Vibe-Kanban + 2 new) |
| `web/` | TypeScript/React | Frontend (3 packages: app, web-core, ui) |
| `proto/` | Protobuf | gRPC service definitions |
| `deploy/` | YAML/Docker | Helm chart + Dockerfiles |
| `tests/` | Shell | E2E test suite |

## Adding Features

### New API endpoint
1. Add route handler in `server/crates/server/src/routes/`
2. Register in `routes/mod.rs`
3. Add corresponding frontend hook in `web/packages/web-core/src/hooks/`
4. Add page/component in `web/packages/app/src/`

### New shadcn component
```bash
cd web/packages/ui
npx shadcn@latest add [component-name]
```

### New protobuf service method
1. Edit proto in `proto/vibetown/`
2. `make proto` to regenerate Go code
3. Rust code regenerates on `cargo build` via build.rs
4. Implement handler in `engine/internal/grpcapi/`
5. Add client method in `server/crates/engine-client/src/client.rs`

### New database table
1. Create migration in `server/crates/db/migrations/`
2. Add sqlx model in `server/crates/db/src/models/`
3. Add API types in `server/crates/api-types/`

## Environment Variables

| Variable | Default | Purpose |
|----------|---------|---------|
| `VT_DATABASE_URL` | `sqlite://vibetown.db` | Database connection |
| `VT_ENGINE_GRPC_ADDR` | `localhost:50051` | Go engine gRPC address |
| `VT_PORT` | `3000` | HTTP server port |
| `VT_LOG_LEVEL` | `info` | Tracing level |
| `VT_DEFAULT_AGENT` | `claude` | Default agent runtime |
