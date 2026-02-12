#!/bin/bash
# Initialize mainnet contract (first time or re-deploy)

CONTRACT_ID="nearshield.near"
ADMIN="nearshield.near"
TREASURY="nearshield-fees.near"   # or your chosen treasury
INIT_ARGS='{"admin": "'$ADMIN'", "treasury": "'$TREASURY'"}'

near call $CONTRACT_ID new "$INIT_ARGS" --accountId $ADMIN --networkId mainnet
