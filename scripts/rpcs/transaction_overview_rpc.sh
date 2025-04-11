#!/bin/bash

# Check if the transaction hash argument is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <transaction_hash>"
    exit 1
fi

transaction_hash="$1"

curl -H "Content-Type: application/json" -d "{
  \"jsonrpc\": \"2.0\",
  \"method\": \"transaction_overview\",
  \"params\": [\"$transaction_hash\", false],
  \"id\": 0
}" http://127.0.0.1:9944
echo ""

# Endpoints:
#   Local:   http://127.0.0.1:9944
#   Turing:  https://turing-rpc.avail.so/rpc
#   Mainnet: https://mainnet-rpc.avail.so/rpc
