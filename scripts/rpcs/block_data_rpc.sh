#!/bin/bash

block_hash="{ \"Hash\": \"0xe43c64674ae191da404a9c780333019cb9f62018778037186ec78bb4fc4e7e04\" }"
block_height="{ \"Index\": 28 }"
transaction="{ \"TxIndex\": [ 0 ] }"
call_filter="{\"transaction\": $transaction }"
params="{ \"block_id\": $block_height, \"fetch_calls\": true, \"fetch_events\": false, \"call_filter\": $call_filter }"

curl -H "Content-Type: application/json" -d "{
  \"jsonrpc\": \"2.0\",
  \"method\": \"block_data\",
  \"params\": [$params],
  \"id\": 0
}" http://127.0.0.1:9944
echo ""

# Endpoints:
#   Local:   http://127.0.0.1:9944
#   Turing:  https://turing-rpc.avail.so/rpc
#   Mainnet: https://mainnet-rpc.avail.so/rpc
