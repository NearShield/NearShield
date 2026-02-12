#!/bin/bash
# Deploy NEARShield to mainnet using nearshield.near as the contract account

export NEAR_NETWORK=mainnet
CONTRACT_ID="nearshield.near"          # your contract account
ADMIN="nearshield.near"             # replace with YOUR mainnet admin account
TREASURY="nearshield-fees.near"           # or create a separate treasury.near

# Build contract (Rust -> WASM)
cd contracts/nearshield
cargo build --target wasm32-unknown-unknown --release
cd ../..

# Deploy contract
near deploy $CONTRACT_ID \
  --wasmFile contracts/nearshield/target/wasm32-unknown-unknown/release/nearshield.wasm \
  --networkId mainnet \
  --accountId $ADMIN

# Initialize contract
INIT_ARGS='{"admin": "'$ADMIN'", "treasury": "'$TREASURY'"}'
near call $CONTRACT_ID new "$INIT_ARGS" \
  --accountId $ADMIN \
  --networkId mainnet

echo "✅ NEARShield deployed to $CONTRACT_ID on mainnet"
