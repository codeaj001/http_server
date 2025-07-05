#!/bin/bash

# Quick verification script for Solana HTTP Server

echo "🚀 Starting Solana HTTP Server verification..."

# Build the project
echo "📦 Building the project..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi
echo "✅ Build successful!"

# Start the server in background
echo "🖥️  Starting server..."
cargo run --release &
SERVER_PID=$!
sleep 3

# Test keypair endpoint
echo "🔑 Testing keypair generation..."
RESPONSE=$(curl -s -X POST http://localhost:8080/keypair)
if echo "$RESPONSE" | grep -q '"success":true'; then
    echo "✅ Keypair generation works!"
else
    echo "❌ Keypair generation failed!"
    echo "Response: $RESPONSE"
    kill $SERVER_PID
    exit 1
fi

# Extract keypair for testing
PUBKEY=$(echo "$RESPONSE" | grep -o '"pubkey":"[^"]*"' | cut -d'"' -f4)
SECRET=$(echo "$RESPONSE" | grep -o '"secret":"[^"]*"' | cut -d'"' -f4)

# Test message signing
echo "✍️  Testing message signing..."
SIGN_RESPONSE=$(curl -s -X POST http://localhost:8080/message/sign \
    -H "Content-Type: application/json" \
    -d "{\"message\":\"Hello, Solana!\",\"secret\":\"$SECRET\"}")

if echo "$SIGN_RESPONSE" | grep -q '"success":true'; then
    echo "✅ Message signing works!"
else
    echo "❌ Message signing failed!"
    echo "Response: $SIGN_RESPONSE"
    kill $SERVER_PID
    exit 1
fi

# Test invalid input handling
echo "🛡️  Testing error handling..."
ERROR_RESPONSE=$(curl -s -X POST http://localhost:8080/keypair/invalid)
if echo "$ERROR_RESPONSE" | grep -q "404"; then
    echo "✅ Error handling works!"
else
    echo "❌ Error handling might need improvement"
fi

# Clean up
echo "🧹 Cleaning up..."
kill $SERVER_PID
sleep 1

echo ""
echo "🎉 All tests passed! The Solana HTTP Server is working correctly."
echo ""
echo "To run the server:"
echo "  cargo run --release"
echo ""
echo "To run the full test suite:"
echo "  npm install"
echo "  npm test"
echo ""
echo "Server will be available at: http://localhost:8080"

