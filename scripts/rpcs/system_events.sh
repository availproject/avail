#!/bin/bash

curl -H "Content-Type: application/json" -d "{
  \"jsonrpc\": \"2.0\",
  \"method\": \"state_call\",
  \"params\": [\"SystemEventsApi_fetch_events\", \"0x040000000000\", \"0x696a6dce1a780404a8d7da80f29950f7eb8dd6a43b985e0746ff0bd00fd58db1\"],
  \"id\": 0
}" http://127.0.0.1:9944
echo ""

# Endpoints:
#   Local:   http://127.0.0.1:9944
#   Turing:  https://turing-rpc.avail.so/rpc
#   Mainnet: https://mainnet-rpc.avail.so/rpc
