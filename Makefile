export PATH := $(HOME)/.cargo/bin:$(PATH)

.PHONY: help dev dev-engine dev-server dev-web build build-engine build-server build-web \
       test test-engine test-server test-web lint proto proto-lint clean install check-tools \
       docker-up docker-down test-e2e test-grpc \
       kustomize-dev kustomize-prod kustomize-build

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# ─── Development ──────────────────────────────────────────────────────────────

dev: ## Start all services via docker compose
	docker compose up

dev-engine: ## Run Go engine locally
	cd engine && go run ./cmd/vibetown-engine

dev-server: ## Run Rust server locally (with watch)
	cd server && cargo watch -x 'run --bin server'

dev-web: ## Run React frontend dev server
	cd web && pnpm dev

# ─── Build ────────────────────────────────────────────────────────────────────

build: build-engine build-server build-web ## Build all subsystems

build-engine: ## Build Go engine
	cd engine && go build -o bin/vibetown-engine ./cmd/vibetown-engine

build-server: ## Build Rust server (release)
	cd server && cargo build --release

build-web: ## Build React frontend
	cd web && pnpm build

# ─── Test ─────────────────────────────────────────────────────────────────────

test: test-engine test-server test-web ## Test all subsystems

test-engine: ## Run Go tests (excludes cmd package which needs tmux/Dolt runtime)
	cd engine && go test $$(go list ./... | grep -v /internal/cmd)

test-server: ## Run Rust tests
	cd server && cargo test

test-web: ## Run frontend type checks
	cd web && pnpm typecheck

# ─── Lint ─────────────────────────────────────────────────────────────────────

lint: proto-lint ## Lint all subsystems
	cd engine && go vet ./...
	cd server && cargo clippy --workspace -- -D warnings
	cd web && pnpm lint

# ─── Proto ────────────────────────────────────────────────────────────────────

proto: ## Generate code from proto definitions
	cd proto && buf generate

proto-lint: ## Lint proto definitions
	cd proto && buf lint

# ─── Docker ───────────────────────────────────────────────────────────────────

docker-up: ## Start docker compose services
	docker compose up -d

docker-down: ## Stop docker compose services
	docker compose down

# ─── Kustomize ───────────────────────────────────────────────────────────────

kustomize-build: ## Preview kustomize manifests (dev overlay)
	kubectl kustomize deploy/kustomize/overlays/dev

kustomize-dev: ## Deploy with kustomize (dev overlay)
	kubectl apply -k deploy/kustomize/overlays/dev

kustomize-prod: ## Deploy with kustomize (production overlay)
	kubectl apply -k deploy/kustomize/overlays/production

# ─── Integration Tests ────────────────────────────────────────────────────────

test-grpc: ## Run gRPC integration tests (Rust → Go)
	cd server && cargo test -p engine-client

test-e2e: ## Run E2E API tests (requires running server)
	./tests/e2e/test_api.sh

# ─── Utility ──────────────────────────────────────────────────────────────────

install: ## Install all dependencies
	cd engine && go mod download
	cd server && cargo fetch
	cd web && pnpm install

clean: ## Remove build artifacts
	rm -rf engine/bin
	cd server && cargo clean
	rm -rf web/packages/app/dist
	rm -rf web/node_modules/.turbo

check-tools: ## Verify required tools are installed
	@echo "Checking tools..."
	@which go >/dev/null 2>&1 && echo "  go: $$(go version)" || echo "  go: NOT FOUND (install from https://go.dev)"
	@which cargo >/dev/null 2>&1 && echo "  cargo: $$(cargo --version)" || echo "  cargo: NOT FOUND (install via https://rustup.rs)"
	@which pnpm >/dev/null 2>&1 && echo "  pnpm: $$(pnpm --version)" || echo "  pnpm: NOT FOUND (install via corepack enable)"
	@which buf >/dev/null 2>&1 && echo "  buf: $$(buf --version)" || echo "  buf: NOT FOUND (install via brew install bufbuild/buf/buf)"
	@which docker >/dev/null 2>&1 && echo "  docker: $$(docker --version)" || echo "  docker: NOT FOUND"
