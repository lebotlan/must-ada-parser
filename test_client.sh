#!/usr/bin/env bash
# Simple test harness for the C client using the Python mock server above.
# Usage: bash c-client/tests/test_client.sh
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"
PYTHON="${PYTHON:-python3}"

# Build C client
cd "$ROOT/c-client"
make

# Create a temporary ADA file
TF="$(mktemp /tmp/test_ada.XXXXXX.ada)"
echo 'procedure Test is begin null; end Test;' > "$TF"

# Start mock server in background
$PYTHON "$ROOT/c-client/tests/mock_server.py" &
SERVER_PID=$!
sleep 0.2

# Run C client (it connects to 127.0.0.1:46000 by design)
./ada_client "$TF" > /tmp/ada_client_out.txt 2>&1 || true

# Wait for server to finish
wait $SERVER_PID || true

# Check output for JSON AST
if grep -q '{"kind":"Program' /tmp/ada_client_out.txt; then
    echo "C client test: OK"
    rm "$TF"
    exit 0
else
    echo "C client test: FAILED"
    echo "==== OUTPUT ===="
    cat /tmp/ada_client_out.txt
    rm "$TF"
    exit 2
fi