#!/bin/bash

block_hash="0x6f364ce7181b4e9f08b107f4bb098977492ce916ffa0bfd557872250483b44c4"
# transaction_hash="0xa28a580ea7dfa03193247659120840039992fb772f59fb318ba391c1794f0a33"
params="{\"block_id\": { \"Hash\": \"$block_hash\" }, \"fetch_events\": true, \"fetch_state\": true, \"fetch_call\": true}"

curl -H "Content-Type: application/json" -d "{
  \"jsonrpc\": \"2.0\",
  \"method\": \"transaction_data\",
  \"params\": [$params],
  \"id\": 0
}" http://127.0.0.1:9944
echo ""

# Endpoints:
#   Local:   http://127.0.0.1:9944
#   Turing:  https://turing-rpc.avail.so/rpc
#   Mainnet: https://mainnet-rpc.avail.so/rpc
