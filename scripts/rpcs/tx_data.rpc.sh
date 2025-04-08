#!/bin/bash

block_hash="{ \"Hash\": \"0xe43c64674ae191da404a9c780333019cb9f62018778037186ec78bb4fc4e7e04\" }"
block_height="{ \"Index\": 2 }"
tx_index="{ \"Index\": 0 }"
# transaction_hash="0xa28a580ea7dfa03193247659120840039992fb772f59fb318ba391c1794f0a33"
filter="{\"pallet_id\": null, \"call_id\": null, \"nonce\": null, \"app_id\": null, \"ss58_address\": null, \"tx_id\": null}"
extension="{ \"fetch_call\": true, \"enable_call_encoding\": true, \"enable_call_decoding\": true, \"fetch_events\": true, \"enable_event_encoding\": true, \"enable_event_decoding\": true }"
params="{\"block_id\": $block_height, \"extension\": $extension, \"filter\": $filter}"

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
