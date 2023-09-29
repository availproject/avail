#!/bin/bash

VAL_COUNT=$1
NODE_COUNT=$2
LC_COUNT=$3
TX_COUNT=$4

# Creating an array of validators and full nodes websocket endpoints

for (( i=1; i<=$VAL_COUNT; i++ ))
do
    DIFF=$(($i - 1))
    INC=$(($DIFF * 2))
    RPC=$((26657 + $INC))
    if [[ $i -eq 1 ]]
    then
        NODE_URLS="\"ws://localhost:$RPC\""
    else
        NODE_URLS+=", \"ws://localhost:$RPC\""
    fi
done

for (( i=1; i<=$NODE_COUNT; i++ ))
do
    DIFF=$(($i - 1))
    INC=$(($DIFF * 2))
    RPC=$((9933 + $INC))
    NODE_URLS+=", \"ws://localhost:$RPC\""
done

# Creating an array of light client endpoints

for (( i=2; i<=$LC_COUNT; i++ ))
do
    DIFF=$(($i - 1))
    INC=$(($DIFF * 2))
    HTTP=$((7001 + $INC))
    if [[ $i -eq 2 ]]
    then
        LC_URL="\"http://localhost:$HTTP\""
    else
        LC_URL+=", \"http://localhost:$HTTP\""
    fi
done

# Generating config file of load test tool

echo "node_urls = [ $NODE_URLS ]
light_client_urls = [ $LC_URL ]
test_acc_mnemonic = $(cat $HOME/avail-keys/load-test.wallet.sr25519.json | jq .secretPhrase)
randomize_nodes = true
rate_limit_per_sec = 5
max_transaction_number = $TX_COUNT # Set to 0 for sustained testing
eth_node_url="http://127.0.0.1:8545"
destination_domain= 1000
da_bridge_address=\"07AF11e412ed7C343603c0F4b35645f7870686Eb\"" | tee "$HOME/avail-test/load-test-config.yaml"

# Running load test

cd $HOME/avail-test
load-test-tool -c $HOME/load-test-config.yaml