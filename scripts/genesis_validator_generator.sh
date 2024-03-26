UNIQUE_ID=$RANDOM
NODE_LOCATION="./target/release/avail-node"
CONTROLLER_LOC_SR="./keys/${UNIQUE_ID}/controller_sr25519.txt"
CONTROLLER_LOC_ED="./keys/${UNIQUE_ID}/controller_ed25519.txt"
STASH_LOC="./keys/${UNIQUE_ID}/stash.txt"
CHAIN_SPEC_LOC="./keys/${UNIQUE_ID}/chain_spec.js"
SCRIPT="./keys/${UNIQUE_ID}/script.txt"

mkdir -p "./keys/${UNIQUE_ID}"
${NODE_LOCATION} key generate > ${CONTROLLER_LOC_SR}
${NODE_LOCATION} key generate > ${STASH_LOC}

CONTROLLER_SR_SEED="$(awk 'NR==3{print $3}' ${CONTROLLER_LOC_SR})"
CONTROLLER_SR_HEX="$(awk 'NR==4{print $4}' ${CONTROLLER_LOC_SR})"
CONTROLLER_SR_SS58="$(awk 'NR==6{print $4}' ${CONTROLLER_LOC_SR})"
STASH_SEED="$(awk 'NR==3{print $3}' ${STASH_LOC})"
STASH_HEX="$(awk 'NR==4{print $4}' ${STASH_LOC})"
STASH_SS58="$(awk 'NR==6{print $4}' ${STASH_LOC})"

${NODE_LOCATION} key inspect --scheme ed25519 "${CONTROLLER_SR_SEED}" > ${CONTROLLER_LOC_ED}

CONTROLLER_ED_HEX="$(awk 'NR==4{print $4}' ${CONTROLLER_LOC_ED})"
CONTROLLER_ED_SS58="$(awk 'NR==6{print $4}' ${CONTROLLER_LOC_ED})"

echo "Controller Private Seed (hex): ${CONTROLLER_SR_SEED}"
echo "Controller Public key (hex) (SR): ${CONTROLLER_SR_HEX}"
echo "Controller Public key (SS58) (SR): ${CONTROLLER_SR_SS58}"
echo "Controller Public key (hex) (ED): ${CONTROLLER_ED_HEX}"
echo "Controller Public key (SS58) (ED): ${CONTROLLER_ED_SS58}"
echo "Stash Private Seed (hex): ${STASH_SEED}"
echo "Stash Public key (hex): ${STASH_HEX}"
echo "Stash Public key (SS58): ${STASH_SS58}"

echo "" > "$CHAIN_SPEC_LOC"

BALANCES="[[\"${STASH_SS58}\", 1000000000000000000000000]]"
echo "Chain Spec:"
echo "balances:" >> "$CHAIN_SPEC_LOC"
jq <<< $BALANCES >> "$CHAIN_SPEC_LOC"

SESSION="[\"${STASH_SS58}\", \"${STASH_SS58}\", {\"authority_discovery\": \"${CONTROLLER_SR_SS58}\",\"babe\": \"${CONTROLLER_SR_SS58}\", \"grandpa\": \"${CONTROLLER_ED_SS58}\", \"im_online\": \"${CONTROLLER_SR_SS58}\"}]"
echo "session_keys:" >> "$CHAIN_SPEC_LOC"
jq <<< $SESSION >> "$CHAIN_SPEC_LOC"

STAKING="[\"${STASH_SS58}\", \"${STASH_SS58}\", 10000000000000000000000, \"Validator\"]"
echo "staking_stakers:" >> "$CHAIN_SPEC_LOC"
jq <<< $STAKING >> "$CHAIN_SPEC_LOC"

cat "${CHAIN_SPEC_LOC}"

echo ""
echo "Manual Script To Inject Keys:"
echo "seed=\"${CONTROLLER_SR_SEED}\"  // SR Controller Secret seed
edkey=\"${CONTROLLER_ED_HEX}\" // ED Controller Public key (hex)
srkey=\"${CONTROLLER_SR_HEX}\" // SR Controller Public key (hex)

curl --trace-ascii here.txt http://localhost:9933 -H \"Content-Type:application/json;charset=utf-8\" -d \"{\\\"jsonrpc\\\":\\\"2.0\\\",\\\"id\\\":1,\\\"method\\\":\\\"author_insertKey\\\",\\\"params\\\": [\\\"gran\\\",\\\"$CONTROLLER_SR_SEED\\\",\\\"$CONTROLLER_ED_HEX\\\"]}\"
curl --trace-ascii here.txt http://localhost:9933 -H \"Content-Type:application/json;charset=utf-8\" -d \"{\\\"jsonrpc\\\":\\\"2.0\\\",\\\"id\\\":1,\\\"method\\\":\\\"author_insertKey\\\",\\\"params\\\": [\\\"babe\\\",\\\"$CONTROLLER_SR_SEED\\\",\\\"$CONTROLLER_SR_HEX\\\"]}\"
curl --trace-ascii here.txt http://localhost:9933 -H \"Content-Type:application/json;charset=utf-8\" -d \"{\\\"jsonrpc\\\":\\\"2.0\\\",\\\"id\\\":1,\\\"method\\\":\\\"author_insertKey\\\",\\\"params\\\": [\\\"imon\\\",\\\"$CONTROLLER_SR_SEED\\\",\\\"$CONTROLLER_SR_HEX\\\"]}\"
curl --trace-ascii here.txt http://localhost:9933 -H \"Content-Type:application/json;charset=utf-8\" -d \"{\\\"jsonrpc\\\":\\\"2.0\\\",\\\"id\\\":1,\\\"method\\\":\\\"author_insertKey\\\",\\\"params\\\": [\\\"audi\\\",\\\"$CONTROLLER_SR_SEED\\\",\\\"$CONTROLLER_SR_HEX\\\"]}\"
" > "${SCRIPT}"

cat "${SCRIPT}"
