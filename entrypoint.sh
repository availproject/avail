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
	--bootnodes=$BOOTNODE_1 \
	--bootnodes=$BOOTNODE_2 \
	--bootnodes=$BOOTNODE_3 \
	$@
