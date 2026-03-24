#!/usr/bin/env bash
set -euo pipefail

# E2E API test suite for Vibetown
# Usage: ./tests/e2e/test_api.sh [base_url]
#
# Requires: curl, jq

BASE_URL="${1:-http://localhost:3000/api}"
PASS=0
FAIL=0

pass() { echo "  ✓ $1"; PASS=$((PASS + 1)); }
fail() { echo "  ✗ $1: $2"; FAIL=$((FAIL + 1)); }

test_endpoint() {
    local method="$1" path="$2" expected_status="$3" desc="$4"
    local body="${5:-}"

    local args=(-s -o /dev/null -w "%{http_code}" -X "$method")
    if [ -n "$body" ]; then
        args+=(-H "Content-Type: application/json" -d "$body")
    fi

    local status
    status=$(curl "${args[@]}" "${BASE_URL}${path}")

    if [ "$status" = "$expected_status" ]; then
        pass "$desc"
    else
        fail "$desc" "expected $expected_status, got $status"
    fi
}

test_json_field() {
    local method="$1" path="$2" field="$3" expected="$4" desc="$5"
    local body="${6:-}"

    local args=(-s -X "$method")
    if [ -n "$body" ]; then
        args+=(-H "Content-Type: application/json" -d "$body")
    fi

    local value
    value=$(curl "${args[@]}" "${BASE_URL}${path}" | jq -r "$field")

    if [ "$value" = "$expected" ]; then
        pass "$desc"
    else
        fail "$desc" "expected $expected, got $value"
    fi
}

echo "═══════════════════════════════════════════════════════"
echo " Vibetown E2E API Tests"
echo " Base URL: $BASE_URL"
echo "═══════════════════════════════════════════════════════"

echo ""
echo "── Health ──"
test_endpoint GET /health 200 "GET /health returns 200"
test_endpoint GET /ready 200 "GET /ready returns 200"

echo ""
echo "── Orchestration ──"
test_endpoint GET /orchestration/town 200 "GET /orchestration/town"
test_endpoint GET /orchestration/rigs 200 "GET /orchestration/rigs"
test_endpoint GET /orchestration/agents 200 "GET /orchestration/agents"
test_endpoint GET /orchestration/convoys 200 "GET /orchestration/convoys"
test_endpoint GET /orchestration/merge-queue 200 "GET /orchestration/merge-queue"

test_endpoint POST /orchestration/town 200 "POST /orchestration/town" \
    '{"name":"test-town","owner":"tester"}'

test_endpoint POST /orchestration/agents/spawn 200 "POST /orchestration/agents/spawn" \
    '{"name":"test-agent","role":"polecat","rig_id":"default","runtime":"claude"}'

echo ""
echo "── Feed ──"
test_endpoint GET "/feed/events?limit=10" 200 "GET /feed/events"

echo ""
echo "── Mail ──"
test_endpoint GET /mail/inbox 200 "GET /mail/inbox"
test_endpoint POST /mail/send 200 "POST /mail/send" \
    '{"from_addr":"mayor@town","to_addr":"polecat@town","subject":"Test","body":"Hello","priority":"normal"}'

echo ""
echo "── Work Items ──"
test_endpoint GET /work-items 200 "GET /work-items"
test_endpoint POST /work-items 200 "POST /work-items" \
    '{"item_type":"task","title":"Test task","description":"A test"}'

echo ""
echo "═══════════════════════════════════════════════════════"
echo " Results: $PASS passed, $FAIL failed"
echo "═══════════════════════════════════════════════════════"

[ "$FAIL" -eq 0 ] || exit 1
