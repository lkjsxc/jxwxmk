#!/bin/bash

echo "=== JXWXMK Test Script ==="

BASE_URL="${1:-http://localhost:8080}"

echo "Testing endpoints at $BASE_URL"
echo ""

# Test health endpoint
echo -n "Health endpoint: "
if curl -sf "$BASE_URL/health" > /dev/null; then
    echo "✓ OK"
else
    echo "✗ FAIL"
    exit 1
fi

# Test metrics endpoint
echo -n "Metrics endpoint: "
if curl -sf "$BASE_URL/metrics" > /dev/null; then
    echo "✓ OK"
else
    echo "✗ FAIL"
    exit 1
fi

# Test static assets
echo -n "Static assets (index.html): "
if curl -sf "$BASE_URL/" > /dev/null; then
    echo "✓ OK"
else
    echo "✗ FAIL"
    exit 1
fi

echo ""
echo "All tests passed!"
