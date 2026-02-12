#!/bin/bash
# Deploy NEARShield contract to testnet

set -e

CONTRACT_ID="nearshield.testnet"
INIT_ARGS='{"admin": "your-account.testnet", "treasury": "nearshield-treasury.testnet"}'

# Build contract
cd contracts/nearshield
cargo build --target wasm32-unknown-unknown --release
cd ../..

# Deploy
near create-account $CONTRACT_ID --masterAccount your-account.testnet --initialBalance 10 || true
near deploy $CONTRACT_ID --wasmFile contracts/nearshield/target/wasm32-unknown-unknown/release/nearshield.wasm

# Initialize
near call $CONTRACT_ID new "$INIT_ARGS" --accountId your-account.testnet

echo "✅ Contract deployed and initialized at $CONTRACT_ID"
