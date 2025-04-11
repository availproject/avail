#!/bin/bash

block_hash="{ \"Hash\": \"0xe43c64674ae191da404a9c780333019cb9f62018778037186ec78bb4fc4e7e04\" }"
block_height="{ \"Index\": 24 }"
params="{ \"block_id\": $block_height }"

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
