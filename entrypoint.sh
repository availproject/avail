#!/bin/bash

da_bin=/da/bin/data-avail
da_keystore=/da/keystore

# Reload the keystore from secrets at /run/secrets/keystore.suri
if [[ ! -z "${RELOAD_KEYSTORE}" ]]; then
	for item in "babe sr25519" "gran ed25519" "imon sr25519" "audi sr25519" "auth sr25519"; do
		item_to_array=( $item )
		key_type=${item_to_array[0]}
		scheme=${item_to_array[1]}

		echo "Reloading key type ${key_type}:${scheme} into ${da_keystore}"

		${da_bin} key insert \
			--chain=${DA_CHAIN} \
			--keystore-path=${da_keystore} \
			--key-type=${key_type} \
			--scheme=${scheme} \
			--suri=/run/secrets/keystore.suri

		done;
fi

echo "Launching node ${DA_NAME} on chain ${DA_CHAIN}..."
${da_bin} \
	--base-path /da/state \
	--keystore-path ${da_keystore} \
	--offchain-worker=Always \
	--enable-offchain-indexing=true \
	--execution native-else-wasm \
	--name=${DA_NAME} \
	--chain=${DA_CHAIN} \
	--port=${DA_P2P_PORT} \
	"$@"
