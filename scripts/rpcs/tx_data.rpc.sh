#!/bin/bash

block_hash="{ \"Hash\": \"0x67e5f7d70a26208ad88c63a3d309c54f04754877b212ba1b378d43a9cd4b1591\" }"
block_height="{ \"Index\": 1 }"
# transaction_hash="0xa28a580ea7dfa03193247659120840039992fb772f59fb318ba391c1794f0a33"
filter="{\"pallet_id\": null, \"call_id\": null, \"nonce\": null, \"app_id\": null, \"ss58_address\": null, \"tx_id\": null}"
params="{\"block_id\": $block_height, \"fetch_events\": true, \"fetch_state\": true, \"fetch_call\": true, \"filter\": $filter}"

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
