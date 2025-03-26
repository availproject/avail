#!/bin/bash

curl -H "Content-Type: application/json" -d "{
  \"jsonrpc\": \"2.0\",
  \"method\": \"state_call\",
  \"params\": [\"SystemEventsApi_fetch_events\", \"0x0101000000\", \"0x0330ebad312c414f7e53890c8d3131bdeb393cf559b444409ee5227b2e9f0aef\"],
  \"id\": 0
}" http://127.0.0.1:9944
echo ""

# Endpoints:
#   Local:   http://127.0.0.1:9944
#   Turing:  https://turing-rpc.avail.so/rpc
#   Mainnet: https://mainnet-rpc.avail.so/rpc
