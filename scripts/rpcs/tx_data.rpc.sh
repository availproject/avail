#!/bin/bash

block_hash="0xb4b85e47a9e1f6c3251eb4f4b92cf30da60ab329e67ed58c322feab805a783d1"
# transaction_hash="0xa28a580ea7dfa03193247659120840039992fb772f59fb318ba391c1794f0a33"
filter="{\"pallet_id\": null, \"call_id\": null, \"nonce\": null, \"app_id\": null, \"ss58_address\": \"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY\", \"tx_id\": null}"
params="{\"block_id\": { \"Hash\": \"$block_hash\" }, \"fetch_events\": false, \"fetch_state\": false, \"fetch_call\": false, \"filter\": $filter}"

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
