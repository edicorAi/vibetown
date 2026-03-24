#!/usr/bin/env bash
set -euo pipefail

# Runs database migrations.
# Usage: ./tools/scripts/migrate.sh [up|down|status]

ACTION="${1:-up}"

echo "Running migrations: $ACTION"
echo "TODO: Implement migration runner in Phase 2"
