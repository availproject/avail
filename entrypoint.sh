#!/bin/sh
cat /entrypoint.sh;

trap cleanup 1 2 3 6

cleanup()
{
  echo "Done cleanup ... quitting."
  exit 1
}

/da/bin/data-avail \
	--validator \
	--base-path /da/state \
	--keystore-path /da/keystore \
	--execution=NativeElseWasm \
	--offchain-worker=Always \
	--enable-offchain-indexing=true \
	--name $DA_NAME \
	--chain $DA_CHAIN \
	--port $DA_P2P_PORT \
	--bootnodes=/ip4/52.47.205.129/tcp/30333/p2p/12D3KooW9tVuCzq3eknsevL5uyqQ3LpVcuqtkTqropjNccbhsWBz \
	--bootnodes=/ip4/15.237.127.118/tcp/30333/p2p/12D3KooWQtxig5HukFDwQzshGWgQEZAqGqdCN7AQBW7cQRJWCyxL \
	--bootnodes=/ip4/52.47.205.129/tcp/30333/p2p/12D3KooW9tVuCzq3eknsevL5uyqQ3LpVcuqtkTqropjNccbhsWBz \
	$@
